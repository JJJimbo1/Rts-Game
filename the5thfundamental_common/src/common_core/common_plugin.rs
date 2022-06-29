use bevy::{math::Vec3Swizzles, prelude::*, utils::hashbrown::HashMap};
use bevy_pathfinding::{Path, PathFindingSystems, PathFindingPlugin, d2::{GridMap, GridCell}, GridSpace, DefaultPather};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::{Collider, Velocity};
use simple_random::Random;
use xtrees::Quad;

use crate::{Actors, Health, MapBounds, Target, TeamPlayer, TeamPlayerWorld, WeaponSet, MobileObject, UnitCommand, UnitCommandType, ResourceProvider, Queues, EconomicObject, ObjectSpawnEvent, ObjectSpawnEventData, Snowflake, ObjectKilled, GroundPathFinder};

#[derive(Default)]
pub struct CommonPlugin;

impl CommonPlugin {
    fn process_commands(
        mut command_reader: EventReader<UnitCommand>,
        pathing_space: Res<GridSpace>,
        grid_map: Res<GridMap>,
        mut rand : ResMut<Random>,
        mut pathfinders: Query<(Entity, &Transform, &mut GroundPathFinder, &mut MobileObject)>,
    ) {
        for command in command_reader.iter() {
            match command.command_type {
                UnitCommandType::Move(destination) => {
                    println!("MOVE!");
                    let spread = (command.units.len() as f32).sqrt() * 2.0;
                    pathfinders.iter_mut().filter(|(entity, _, _, _)| command.units.contains(entity)).for_each(|(_, transform, mut pathfinder, mut mobile_object)| {
                        pathfinder.start = transform.translation.xz();
                        let end = destination + Vec2::new(rand.range(-spread, spread), rand.range(-spread, spread));
                        let (end_x, end_y) = pathing_space.position_to_index(end);
                        let end = grid_map.get_cell(end_x, end_y).and_then(|c| grid_map.closest_unblocked_cell(*c)).map_or(end, |c| pathing_space.index_to_position(c.index()));
                        pathfinder.end = end;
                        mobile_object.follow = true;
                        mobile_object.pursuant = None;
                    });
                },
                UnitCommandType::Attack(target) => {
                    println!("ATTACK!");
                    pathfinders.iter_mut().filter(|(entity, _, _, _)| command.units.contains(entity)).for_each(|(_, _, _, mut mobile_object)| {
                        mobile_object.pursuant = Some(target);
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
        grid_map: Res<GridMap>,
        teamplayer_world: Res<TeamPlayerWorld>,
        transforms: Query<&Transform>,
        mut query: Query<(&Transform, &mut GroundPathFinder, &mut MobileObject, &mut WeaponSet, &TeamPlayer,)>,
    ) {

        //TODO: Make sure weapons can only target the target if they are able to.
        query.for_each_mut(|(transform, mut pathfinder, mut mobile_object, mut weapon_set, teamplayer)| {
            // println!("{:?}", mobile_object.pursuant);
            match mobile_object.pursuant {
                Some(target) => {
                    if let Ok(target_transform) = transforms.get(target) {

                        let dir = Vec2::new(transform.translation.x - target_transform.translation.x, transform.translation.z - target_transform.translation.z).normalize() * weapon_set.closing_range;
                        if mathfu::D2::distance_magnitude((transform.translation.x, transform.translation.z), (target_transform.translation.x, target_transform.translation.z)) > dir.length_squared() {
                            pathfinder.start = transform.translation.xz();
                            let (end_x, end_y) = pathing_space.position_to_index(target_transform.translation.xz() + dir);
                            let end = grid_map.get_cell(end_x, end_y).and_then(|c| grid_map.closest_unblocked_cell(*c)).map_or(target_transform.translation.xz() + dir, |c| pathing_space.index_to_position(c.index()));
                            pathfinder.end = end;
                            mobile_object.follow = true;
                        } else {
                            mobile_object.follow = false;
                        }

                        for weapon in weapon_set.weapons.iter_mut() {
                            if mathfu::D2::distance_magnitude((transform.translation.x, transform.translation.z), (target_transform.translation.x, target_transform.translation.z)) < weapon.range.powi(2) {
                                weapon.target = Target::ManualTarget(target);
                            } else if let Target::AutoTarget(_) = weapon.target {

                            } else {
                                weapon.target = Target::None
                            }
                        }
                    } else {
                        mobile_object.pursuant = None;
                    }
                },
                None => {
                    for weapon in weapon_set.weapons.iter_mut() {
                        if let Target::ManualTarget(_) = weapon.target {
                            weapon.target = Target::None;
                        } else if let Target::AutoTarget(target) = weapon.target {
                            if let Ok(target_transform) = transforms.get(target) {
                                if mathfu::D2::distance_magnitude((transform.translation.x, transform.translation.z), (target_transform.translation.x, target_transform.translation.z)) > weapon.range.powi(2) {
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
        mut weapons : Query<(Entity, &Transform, &mut WeaponSet)>,
        mut healths : Query<&mut Health>
    ) {
        weapons.for_each_mut(|(_, tran, mut wep)| {
            for weapon in wep.weapons.iter_mut() {
                if weapon.cooldown > 0.0 {
                    weapon.cooldown -= time.delta_seconds();
                }
                if weapon.cooldown > 0.0 {
                    continue;
                }
                if let Some(target) = weapon.target.get_target() {
                    if let Ok(mut health) = healths.get_mut(target) {
                        health.damage(weapon.damage, weapon.damage_types);
                        weapon.cooldown = weapon.fire_rate;
                    }
                }
            }
        });
    }

    fn health_system(
        mut objects_killed_writer : EventWriter<ObjectKilled>,
        query : Query<(Entity, &Health)>
    ) {
        query.for_each(|(entity, health)| {
            if health.is_dead() {
                objects_killed_writer.send(ObjectKilled(entity));
            }
        });
    }

    fn score_calculator_system(
        mut actors : ResMut<Actors>,
        query : Query<(&TeamPlayer, Option<&ResourceProvider>, Option<&Queues>, Option<&WeaponSet>)>,
    ) {
        actors.reset_ratings();
        query.for_each(|(tp, res, que, wep)| {
            if let Some(a) = actors.actors.get_mut(tp) {
                if let Some(x) = res {
                    a.rating.economy_score += x.strength;
                }
                if let Some(x) = que {
                    a.rating.production_score += x.count() as f64;
                }
                if let Some(x) = wep {
                    a.rating.power_score += x.weapons.len() as f64;
                }
            }
        });
    }

    fn follow_path(
        mut debug : ResMut<DebugLines>,
        mut followers : Query<(&mut Path, &mut Transform, &mut Velocity, &mut MobileObject)>,
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
        followers.for_each_mut(|(mut path, mut transform, mut velocity, mut mobile_object)| {
            if !mobile_object.follow { velocity.linvel.x = 0.0; velocity.linvel.z = 0.0; return; }
            let y = transform.translation.y;
            match (path.0.get(0).cloned(), path.0.get(1).cloned()) {
                (Some(first), Some(_second)) => {
                    transform.look_at(first.extend(y).xzy(), Vec3::Y);
                    let direction = (first - transform.translation.xz()).normalize_or_zero();
                    let distance = first.distance(transform.translation.xz());
                    if distance > 1.0 {
                        velocity.linvel.x =  direction.x * mobile_object.max_forward_speed.abs();
                        velocity.linvel.z =  direction.y * mobile_object.max_forward_speed.abs();
                    } else {
                        path.0.remove(0);
                    }
                },
                (Some(first), None) => {
                    transform.look_at(first.extend(y).xzy(), Vec3::Y);
                    let direction = (first - transform.translation.xz()).normalize_or_zero();
                    let distance = first.distance(transform.translation.xz());
                    if distance > 0.05 {
                        velocity.linvel.x =  direction.x * (mobile_object.max_forward_speed.abs()).min(distance * 3.0);
                        velocity.linvel.z =  direction.y * (mobile_object.max_forward_speed.abs()).min(distance * 3.0);
                    } else {
                        path.0.remove(0);
                        transform.translation = first.extend(y).xzy();
                    }
                },
                _ => {
                    velocity.linvel.x = 0.0;
                    velocity.linvel.z = 0.0;
                    mobile_object.follow = false;
                }
            }
        });
    }

    fn resource_adder_system(
        time : Res<Time>,
        mut actors : ResMut<Actors>,
        query : Query<(&TeamPlayer, &EconomicObject)>
    ) {
        let mut add : HashMap<TeamPlayer, (u32, f64)> = HashMap::new();
        for a in actors.actors.iter() {
            add.insert(*a.0, (0, 0.0));
        }
        query.for_each(|(tp, res)| {
            if let Some(x) = add.get_mut(tp) {
                x.0 += 1;
                x.1 += res.resource_gen - res.resource_drain;
            }
        });
        for (id, actor) in actors.actors.iter_mut() {
            let mut to_add = add[id];
            to_add.1 *= time.delta_seconds() as f64;
            actor.economy.add_resources(to_add);
        }
    }

    fn queue_system(
        mut spawn_events: EventWriter<ObjectSpawnEvent>,
        time : Res<Time>,
        mut actors : ResMut<Actors>,
        mut queues : Query<(&Transform, &TeamPlayer, &mut Queues)>
    ) {
        queues.for_each_mut(|(transform, team_player, mut queues)| {
            if let Some(actor) = actors.actors.get_mut(team_player) {
                for queue in queues.queues.values_mut() {
                    if let Some(object) = queue.zip_queue.get_next() {
                        let cost_this_frame = object.cost as f64 / object.time_to_build.as_secs_f64() * queue.time_left(time.delta_seconds_f64());
                        if actor.economy.remove_resources(cost_this_frame) && { queue.update(time.delta_seconds_f64()); queue.is_ready() } {
                            let data = queue.advance().unwrap();
                            if data.buffered {
                                queue.push_to_buffer(data);
                            } else {
                                let mut transform = *transform;
                                transform.translation += transform.forward() * 20.0;
                                let spawn_data = ObjectSpawnEventData { snowflake: Snowflake::new(), object_type: data.object_type, team_player: *team_player, transform};
                                spawn_events.send(ObjectSpawnEvent(spawn_data));
                            }
                        }
                    }
                }
            }
        })
    }
}

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        let actors = app.world.get_resource_or_insert_with(|| Actors::default()).clone();
        let bounds = app.world.get_resource_or_insert_with(|| MapBounds::default()).clone();
        let _teamplayer_world = app.world.get_resource_or_insert_with(|| TeamPlayerWorld::new(&actors, &bounds)).clone();

        app.world.get_resource_or_insert_with(|| GridMap::new(0, 0).with_cells(|x, z| GridCell::new(x, z, false)));
        app.world.get_resource_or_insert_with(|| DefaultPather::default());
        app.world.get_resource_or_insert_with(|| GridSpace::default());

        app
            .add_plugin(PathFindingPlugin::<GroundPathFinder>::default())

            .add_system(Self::process_commands.before(PathFindingSystems::PathFindingSystem))
            .add_system(Self::teamplayer_world_updater.after(Self::process_commands))
            .add_system(Self::targeting_system.after(Self::teamplayer_world_updater))
            .add_system(Self::weapons_system.after(Self::targeting_system))
            .add_system(Self::health_system.after(Self::weapons_system))

            .add_system(Self::score_calculator_system)
            .add_system(Self::follow_path.after(PathFindingSystems::PathFindingSystem))

            .add_system(Self::resource_adder_system)
            .add_system(Self::queue_system.after(Self::resource_adder_system))
            ;
    }
}
