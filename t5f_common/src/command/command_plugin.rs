
use bevy::prelude::*;
use bevy_rapier3d::{geometry::Collider, dynamics::Velocity};
use pathing::DS2Map;
use t5f_utility::random::Random;
use xtrees::Quad;

use crate::*;

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
        mut teamplayer_world: ResMut<TeamPlayerWorld>,
        query: Query<(Entity, &Transform, &Collider, &TeamPlayer)>
    ) {
        if actors.is_changed() || bounds.is_changed() {
            *teamplayer_world = TeamPlayerWorld::new(&actors, &bounds);
        } else {
            teamplayer_world.clear_trees();
        }
        query.iter().for_each(|(ent, tran, _, tp)| {
            //TODO: Fix Extents
            let quad = Quad::new(tran.translation.x, tran.translation.z, 0.5, 0.5);
            teamplayer_world.insert(*tp, ent, quad);
        })
    }

    fn follow_path(
        mut gizmos: Gizmos,
        time: Res<Time>,
        mut followers : Query<(&mut PathFinder, &mut Transform, &mut Velocity, &Navigator)>,
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

        //TODO: Implement nice looking driving behaviour.
        followers.iter_mut().for_each(|(mut pathfinder, mut transform, mut velocity, navigator)| {
            let Some(path) = pathfinder.path_mut() else { return; };
            let y = transform.translation.y;
            match (path.get(0).cloned(), path.get(1).cloned()) {
                (Some(first), Some(_second)) => {
                    let desired_rotation = transform.looking_at(first.extend(y).xzy(), Vec3::Y).rotation;
                    let difference = transform.rotation.angle_between(desired_rotation);
                    let speed = (1.5 / difference) * time.delta_secs();
                    let new_rotation = transform.rotation.slerp(desired_rotation, speed.clamp(0.0, 1.0));
                    transform.rotation = new_rotation;

                    let distance = first.distance(transform.translation.xz());
                    if distance > 1.0 {
                        velocity.linvel = transform.rotation * -Vec3::Z * navigator.max_forward_speed.abs();
                    } else {
                        path.remove(0);
                    }
                },
                (Some(first), None) => {
                    let desired_rotation = transform.looking_at(first.extend(y).xzy(), Vec3::Y).rotation;
                    let difference = transform.rotation.angle_between(desired_rotation);
                    let speed = (1.5 / difference) * time.delta_secs();
                    let new_rotation = transform.rotation.slerp(desired_rotation, speed.clamp(0.0, 1.0));
                    transform.rotation = new_rotation;

                    let distance = first.distance(transform.translation.xz());
                    if distance > 0.05 {
                        velocity.linvel = transform.rotation * -Vec3::Z  * navigator.max_forward_speed.abs().min(distance * 3.0);
                    } else {
                        transform.translation = first.extend(y).xzy();
                        path.remove(0);
                    }
                },
                _ => {
                    velocity.linvel.x = 0.0;
                    velocity.linvel.z = 0.0;
                    *pathfinder = PathFinder::Idle;
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
        app.world_mut().get_resource_or_insert_with(|| TeamPlayerWorld::new(&commanders, &bounds));

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