

pub mod commander;
pub mod navigation;
pub mod pathfinder;
pub mod reference;
pub mod select;
pub mod snowflake;
pub mod squad;
pub mod teamplayer;

pub use commander::*;
pub use navigation::*;
pub use pathfinder::*;
pub use reference::*;
pub use select::*;
pub use snowflake::*;
pub use squad::*;
pub use teamplayer::*;

// pub use pathing::*;

use bevy::prelude::*;
use avian3d::prelude::{Collider, LinearVelocity};
use pathing::DS2Map;
use xtrees::Quad;
use crate::*;

#[derive(Debug, Clone)]
#[derive(Event)]
pub struct CommandEvent {
    pub player: TeamPlayer,
    pub objects: Vec<Entity>,
    pub command: CommandType,
}

impl CommandEvent {
    pub fn activate(&self) -> Option<&Vec<Entity>> {
        if self.command.is_activate() {
            Some(&self.objects)
        } else {
            None
        }
    }

    pub fn attack(&self) -> Option<&Vec<Entity>> {
        if self.command.is_attack() {
            Some(&self.objects)
        } else {
            None
        }
    }

    pub fn build(&self) -> Option<&Vec<Entity>> {
        if self.command.is_build() {
            Some(&self.objects)
        } else {
            None
        }
    }

    pub fn r#move(&self) -> Option<&Vec<Entity>> {
        if self.command.is_move() {
            Some(&self.objects)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandType {
    Activate,
    Attack(Entity),
    Build(BuildStatus),
    Move(Vec2),
}

impl CommandType {
    pub fn is_activate(&self) -> bool {
        match self {
            Self::Activate => true,
            _ => false,
        }
    }

    pub fn is_attack(&self) -> bool {
        match self {
            Self::Attack(_) => true,
            _ => false,
        }
    }

    pub fn is_build(&self) -> bool {
        match self {
            Self::Build(_) => true,
            _ => false,
        }
    }

    pub fn is_move(&self) -> bool {
        match self {
            Self::Move(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BuildStatus {
    Begin(ObjectType),
    Finish(Transform),
}

#[derive(Default)]
pub struct CommandPlugin;

impl CommandPlugin {
    pub fn process_commands(
        mut commands: EventReader<CommandEvent>,
        mut rand: ResMut<Random>,
        mut pathfinders: Query<(Entity, &Transform, &mut PathFinder, &mut Navigator)>,
    ) {
        for command in commands.read() {
            match command.command {
                CommandType::Move(destination) => {
                    let spread = (command.objects.len() as f32).sqrt();
                    pathfinders.iter_mut().filter(|(entity, _, _, _)| command.objects.contains(entity)).for_each(|(_, transform, mut pathfinder, mut navigator)| {
                        let start = transform.translation.xz();
                        let end = destination + Vec2::new(rand.range(-spread, spread), rand.range(-spread, spread));
                        pathfinder.set_trip((start, end));
                        navigator.pursue = None;
                    });
                },
                CommandType::Attack(target) => {
                    pathfinders.iter_mut().filter(|(entity, _, _, _)| command.objects.contains(entity)).for_each(|(_, _, _, mut navigator)| {
                        navigator.pursue = Some(target);
                    });
                }
                _ => { },
            }
        }
    }

    fn teamplayer_world_updater(
        actors: Res<Commanders>,
        bounds: Res<MapBounds>,
        mut combat_world: ResMut<CombatWorld>,
        query: Query<(Entity, &Transform, &Collider, &TeamPlayer)>
    ) {
        if actors.is_changed() || bounds.is_changed() {
            *combat_world = CombatWorld::new(&actors, &bounds);
        } else {
            combat_world.clear_trees();
        }
        query.iter().for_each(|(ent, tran, _, tp)| {
            //TODO: Fix Extents
            let quad = Quad::new(tran.translation.x, tran.translation.z, 0.5, 0.5);
            combat_world.insert(*tp, ent, quad);
        })
    }

    fn follow_path(
        mut gizmos: Gizmos,
        time: Res<Time>,
        mut followers: Query<(&mut PathFinder, &mut Transform, &mut LinearVelocity, &Navigator)>,
    ) {
        followers.iter_mut().for_each(|(pathfinder, tran, _, _)| {
            let Some(path) = pathfinder.path() else { return; };
            if let Some(x) = path.first() {
                gizmos.line(tran.translation.xz().extend(1.0).xzy(), x.extend(1.0).xzy(), Color::srgba( 1.0, 0.2, 0.2, 1.0));
            }
            for path in path.windows(2) {
                let previous = path[0];
                let current = path[1];
                gizmos.line(previous.extend(1.0).xzy(), current.extend(1.0).xzy(), Color::srgba(1.0, 0.2, 0.2, 1.0));
            }
        });

        followers.iter_mut().for_each(|(mut pathfinder, mut transform, mut velocity, navigator)| {
            let Some(path) = pathfinder.path_mut() else { return; };
            let y = transform.translation.y;
            match (path.get(0).cloned(), path.get(1).cloned()) {
                (Some(first), Some(_second)) => {
                    let distance = first.distance(transform.translation.xz());
                    let new_velocity = transform.rotation * -Vec3::Z  * navigator.max_forward_speed.abs().min(distance * 3.0);
                    let desired_rotation = transform.looking_at(first.extend(y).xzy(), Vec3::Y).rotation;

                    let max_turn_speed = navigator.max_turn_speed.unwrap_or(f32::INFINITY);

                    let turn_speed = navigator.max_turn_speed.unwrap_or(f32::INFINITY)
                        * time.delta_secs()
                        * if distance > 2.0 * new_velocity.length() / max_turn_speed * (transform.rotation.angle_between(desired_rotation).abs() / 2.0).sin() { 1.0 } else { -1.0 };
                    
                    let new_rotation = transform.rotation.rotate_towards(desired_rotation, turn_speed);
                    transform.rotation = new_rotation;

                    if distance > 1.0 {
                        velocity.0 = transform.rotation * -Vec3::Z * navigator.max_forward_speed.abs();
                    } else {
                        path.remove(0);
                    }
                },
                (Some(first), None) => {
                    let distance = first.distance(transform.translation.xz());
                    let new_velocity = transform.rotation * -Vec3::Z  * navigator.max_forward_speed.abs().min(distance * 3.0);
                    let desired_rotation = transform.looking_at(first.extend(y).xzy(), Vec3::Y).rotation;

                    let max_turn_speed = navigator.max_turn_speed.unwrap_or(f32::INFINITY);

                    let turn_speed = navigator.max_turn_speed.unwrap_or(f32::INFINITY)
                        * time.delta_secs()
                        * if distance > 2.0 * new_velocity.length() / max_turn_speed * (transform.rotation.angle_between(desired_rotation).abs() / 2.0).sin() { 1.0 } else { -1.0 };
                    
                    let new_rotation = transform.rotation.rotate_towards(desired_rotation, turn_speed);
                    transform.rotation = new_rotation;

                    if distance > 0.05 {
                        velocity.0 = new_velocity;
                    } else {
                        transform.translation = first.extend(y).xzy();
                        path.remove(0);
                    }
                },
                _ => {
                    velocity.0.x = 0.0;
                    velocity.0.z = 0.0;
                    pathfinder.clear_path();
                }
            }
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct CommandSystems;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandEvent>();
        let commanders = app.world_mut().get_resource_or_insert_with(|| Commanders::default()).clone();
        let bounds = app.world_mut().get_resource_or_insert_with(|| MapBounds::default()).clone();
        app.world_mut().get_resource_or_insert_with(|| CombatWorld::new(&commanders, &bounds));

        app.world_mut().get_resource_or_insert_with(|| GridMap(DS2Map::new()));
        app.world_mut().get_resource_or_insert_with(|| GridSpace::default());

        app
            .add_plugins(PathFindingPlugin)
            .add_systems(Update, (
                Self::process_commands.before(PathFindingSystems::PathFindingSystem),
                Self::teamplayer_world_updater.after(Self::process_commands),
            ).in_set(CommandSystems))
            .add_systems(Update,
                Self::follow_path.after(PathFindingSystems::PathFindingSystem)
            )
        ;
    }
}