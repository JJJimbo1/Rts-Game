use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::{Collider, Velocity};
use xtrees::Quad;
use crate::*;

#[derive(Default)]
pub struct CombatPlugin;

impl CombatPlugin {
    fn process_commands(
        mut command_reader: EventReader<UnitCommandEvent>,
        pathing_space: Res<GridSpace>,
        grid_map: Res<OGrid>,
        mut rand : ResMut<Random<WichmannHill>>,
        mut pathfinders: Query<(Entity, &Transform, &mut GroundPathFinder, &mut Controller)>,
    ) {
        for command in command_reader.iter() {
            match command.command_type {
                UnitCommandType::Move(destination) => {
                    println!("MOVE!");
                    let spread = (command.units.len() as f32).sqrt() * 2.0;
                    pathfinders.iter_mut().filter(|(entity, _, _, _)| command.units.contains(entity)).for_each(|(_, transform, mut pathfinder, mut controller)| {
                        pathfinder.start = transform.translation.xz();
                        let end = destination + Vec2::new(rand.range(-spread, spread), rand.range(-spread, spread));
                        let (end_x, end_y) = pathing_space.position_to_index(end);
                        let end = grid_map.0.get_cell(end_x, end_y).and_then(|c| grid_map.0.closest_unblocked_cell(*c)).map_or(end, |c| pathing_space.index_to_position(c.index()));
                        pathfinder.end = end;
                        controller.follow = true;
                        controller.pursuant = None;
                    });
                },
                UnitCommandType::Attack(target) => {
                    println!("ATTACK!");
                    pathfinders.iter_mut().filter(|(entity, _, _, _)| command.units.contains(entity)).for_each(|(_, _, _, mut controller)| {
                        controller.pursuant = Some(target);
                    });
                }
            }
        }
    }

    fn teamplayer_world_updater(
        actors: Res<Actors>,
        bounds: Res<MapBounds>,
        mut teamplayer_world: ResMut<TeamPlayerWorld>,
        query: Query<(Entity, &Transform, &Collider, &TeamPlayer)>
    ) {
        if actors.is_changed() || bounds.is_changed() {
            *teamplayer_world = TeamPlayerWorld::new(&actors, &bounds);
        } else {
            teamplayer_world.clear_trees();
        }
        query.for_each(|(ent, tran, _, tp)| {
            //TODO: Fix Extents
            let quad = Quad::new(tran.translation.x, tran.translation.z, 0.5, 0.5);
            teamplayer_world.insert(*tp, ent, quad);
        })
    }

    fn targeting_system(
        pathing_space: Res<GridSpace>,
        grid_map: Res<OGrid>,
        teamplayer_world: Res<TeamPlayerWorld>,
        transforms: Query<&Transform>,
        mut query: Query<(&Transform, &mut GroundPathFinder, &mut Controller, &mut WeaponSet, &TeamPlayer,)>,
    ) {

        //TODO: Make sure weapons can only target the target if they are able to.
        query.for_each_mut(|(transform, mut pathfinder, mut controller, mut weapon_set, teamplayer)| {
            // println!("{:?}", mobile_object.pursuant);
            match controller.pursuant {
                Some(target) => {
                    if let Ok(target_transform) = transforms.get(target) {

                        let dir = Vec2::new(transform.translation.x - target_transform.translation.x, transform.translation.z - target_transform.translation.z).normalize() * weapon_set.closing_range;
                        if d2::distance_magnitude((transform.translation.x, transform.translation.z), (target_transform.translation.x, target_transform.translation.z)) > dir.length_squared() {
                            pathfinder.start = transform.translation.xz();
                            let (end_x, end_y) = pathing_space.position_to_index(target_transform.translation.xz() + dir);
                            let end = grid_map.0.get_cell(end_x, end_y).and_then(|c| grid_map.0.closest_unblocked_cell(*c)).map_or(target_transform.translation.xz() + dir, |c| pathing_space.index_to_position(c.index()));
                            pathfinder.end = end;
                            controller.follow = true;
                        } else {
                            controller.follow = false;
                        }

                        for weapon in weapon_set.weapons.iter_mut() {
                            if d2::distance_magnitude((transform.translation.x, transform.translation.z), (target_transform.translation.x, target_transform.translation.z)) < weapon.range.powi(2) {
                                weapon.target = Target::ManualTarget(target);
                            } else if let Target::AutoTarget(_) = weapon.target {

                            } else {
                                weapon.target = Target::None
                            }
                        }
                    } else {
                        controller.pursuant = None;
                    }
                },
                None => {
                    for weapon in weapon_set.weapons.iter_mut() {
                        if let Target::ManualTarget(_) = weapon.target {
                            weapon.target = Target::None;
                        } else if let Target::AutoTarget(target) = weapon.target {
                            if let Ok(target_transform) = transforms.get(target) {
                                if d2::distance_magnitude((transform.translation.x, transform.translation.z), (target_transform.translation.x, target_transform.translation.z)) > weapon.range.powi(2) {
                                    weapon.target = Target::None;
                                }
                            } else {
                                weapon.target = Target::None;
                            }
                        }
                        if let Target::None = weapon.target {
                            let targets = teamplayer_world.search_targets(*teamplayer, transform.translation, weapon);
                            if let Some(e) = targets.first() {
                                weapon.target = Target::AutoTarget(*e);
                            }
                        }
                    }
                }
            }
        });
    }

    fn weapons_system(
        time : Res<Time>,
        mut weapons : Query<&mut WeaponSet>,
        mut healths : Query<&mut Health>
    ) {
        weapons.for_each_mut(|mut wep| {
            for weapon in wep.weapons.iter_mut() {
                if weapon.cooldown > 0.0 {
                    weapon.cooldown -= time.delta_seconds();
                }
                if weapon.cooldown > 0.0 {
                    continue;
                }
                if let Some(mut health) = weapon.target.get_target().and_then(|target| healths.get_mut(target).ok()) {
                    health.damage(weapon.damage, weapon.damage_types);
                    weapon.cooldown = weapon.fire_rate;
                }
            }
        });
    }

    fn health_system(
        mut objects_killed_writer : EventWriter<ObjectKilledEvent>,
        query : Query<(Entity, &Health)>
    ) {
        query.for_each(|(entity, health)| {
            if health.is_dead() {
                objects_killed_writer.send(ObjectKilledEvent(entity));
            }
        });
    }

    fn follow_path(
        mut debug : ResMut<DebugLines>,
        mut followers : Query<(&mut Path, &mut Transform, &mut Velocity, &mut Controller)>,
    ) {
        followers.for_each_mut(|(path, tran, _, _)| {
            if let Some(x) = path.0.first() {
                // println!("{}", x);
                debug.line_colored(tran.translation.xz().extend(1.0).xzy(), x.extend(1.0).xzy(), 0.0, Color::Rgba{ red : 1.0, green : 0.2, blue : 0.2, alpha : 1.0});
            }
            for i in 1..path.0.len() {
                let previous = path.0[i - 1];
                let current = path.0[i];
                debug.line_colored(previous.extend(1.0).xzy(), current.extend(1.0).xzy(), 0.0, Color::Rgba{ red : 1.0, green : 0.2, blue : 0.2, alpha : 1.0});
            }
        });

        //TODO: Implement nice looking driving behaviour.
        followers.for_each_mut(|(mut path, mut transform, mut velocity, mut controller)| {
            if !controller.follow { velocity.linvel.x = 0.0; velocity.linvel.z = 0.0; return; }
            let y = transform.translation.y;
            match (path.0.get(0).cloned(), path.0.get(1).cloned()) {
                (Some(first), Some(_second)) => {

                    let desired_rotation = transform.looking_at(first.extend(y).xzy(), Vec3::Y).rotation;
                    let difference = transform.rotation.angle_between(desired_rotation);
                    let speed = 0.015 / difference;
                    let new_rotation = transform.rotation.slerp(desired_rotation, speed.clamp(0.0, 1.0));
                    transform.rotation = new_rotation;

                    let distance = first.distance(transform.translation.xz());
                    if distance > 1.0 {
                        velocity.linvel = transform.rotation * -Vec3::Z * controller.max_forward_speed.abs();
                    } else {
                        path.0.remove(0);
                    }
                },
                (Some(first), None) => {
                    let desired_rotation = transform.looking_at(first.extend(y).xzy(), Vec3::Y).rotation;
                    let difference = transform.rotation.angle_between(desired_rotation);
                    let speed = 0.015 / difference;
                    let new_rotation = transform.rotation.slerp(desired_rotation, speed.clamp(0.0, 1.0));
                    transform.rotation = new_rotation;

                    let distance = first.distance(transform.translation.xz());
                    if distance > 0.05 {
                        velocity.linvel = transform.rotation * -Vec3::Z  * controller.max_forward_speed.abs().min(distance * 3.0);
                    } else {
                        path.0.remove(0);
                        transform.translation = first.extend(y).xzy();
                    }
                },
                _ => {
                    velocity.linvel.x = 0.0;
                    velocity.linvel.z = 0.0;
                    controller.follow = false;
                }
            }
        });
    }
}

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        let actors = app.world.get_resource_or_insert_with(|| Actors::default()).clone();
        let bounds = app.world.get_resource_or_insert_with(|| MapBounds::default()).clone();
        app.world.get_resource_or_insert_with(|| TeamPlayerWorld::new(&actors, &bounds));

        app.world.get_resource_or_insert_with(|| OGrid(GridMap::new(0, 0).with_cells(|x, z| GridCell::new(x, z, false))));
        app.world.get_resource_or_insert_with(|| DefaultPather::default());
        app.world.get_resource_or_insert_with(|| GridSpace::default());

        app

            .add_plugin(PathFindingPlugin::<GroundPathFinder>::default())

            .add_system(Self::process_commands.before(PathFindingSystems::PathFindingSystem))
            .add_system(Self::teamplayer_world_updater.after(Self::process_commands))
            .add_system(Self::targeting_system.after(Self::teamplayer_world_updater))
            .add_system(Self::weapons_system.after(Self::targeting_system))
            .add_system(Self::health_system.after(Self::weapons_system))
            .add_system(Self::follow_path.after(PathFindingSystems::PathFindingSystem))

        ;
    }
}
