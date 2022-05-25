pub use systems::*;
mod systems {
    use bevy::{math::Vec3Swizzles, prelude::*};
    use bevy_pathfinding::PathFinder;
    use bevy_rapier3d::prelude::Collider;
    use xtrees::Quad;

    use crate::{Actors, AttackCommand, CommonSystemSets, DirtyEntities, Health, Identifiers, MapBounds, Target, TeamPlayer, TeamPlayerWorld, WeaponSet, MobileObject};

    #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
    pub enum CombatSystems {
        TeamPlayerWorldUpdaterSystem,
        ManualTargetingSystem,
        AutoTargetingSystem,
        WeaponsSystem,
        HealthSystem,
    }

    #[derive(Default)]
    pub struct CombatBundle;

    // impl Plugin for CombatBundle {
    //     fn build(&self, app: &mut App) {
    //         app.add_startup_system(combat_startup_system)
    //             .add_system_set(SystemSet::new().label(CommonSystemSets::Combat)
    //                 .with_system(team_player_world_updater_system.label(CombatSystems::TeamPlayerWorldUpdaterSystem))
    //                 .with_system(manual_targeting_system.label(CombatSystems::ManualTargetingSystem).after(CombatSystems::TeamPlayerWorldUpdaterSystem))
    //                 .with_system(auto_targeting_system.label(CombatSystems::AutoTargetingSystem).after(CombatSystems::ManualTargetingSystem))
    //                 .with_system(weapons_look_at_system.label(CombatSystems::WeaponsSystem).after(CombatSystems::AutoTargetingSystem))
    //                 .with_system(weapons_system.label(CombatSystems::WeaponsSystem).after(CombatSystems::AutoTargetingSystem))
    //                 .with_system(health_system.label(CombatSystems::HealthSystem).after(CombatSystems::WeaponsSystem)));
    //             }
    //         }

    ///Make sure you include the startup system.
    pub fn combat_system_set(set : SystemSet) -> SystemSet {
        set.label(CommonSystemSets::Combat)
            .with_system(team_player_world_updater_system.label(CombatSystems::TeamPlayerWorldUpdaterSystem))
            .with_system(manual_targeting_system.label(CombatSystems::ManualTargetingSystem).after(CombatSystems::TeamPlayerWorldUpdaterSystem))
            .with_system(auto_targeting_system.label(CombatSystems::AutoTargetingSystem).after(CombatSystems::ManualTargetingSystem))
            .with_system(weapons_look_at_system.label(CombatSystems::WeaponsSystem).after(CombatSystems::AutoTargetingSystem))
            .with_system(weapons_system.label(CombatSystems::WeaponsSystem).after(CombatSystems::AutoTargetingSystem))
            .with_system(health_system.label(CombatSystems::HealthSystem).after(CombatSystems::WeaponsSystem))
    }

    pub fn combat_startup_system(actors : Res<Actors>, map : Res<MapBounds>, mut commands : Commands) {
        let tpw = TeamPlayerWorld::new(actors, map);
        commands.insert_resource(tpw);
    }

    fn team_player_world_updater_system(mut team_player_world : ResMut<TeamPlayerWorld>, query : Query<(Entity, &Transform, &Collider, &TeamPlayer)>) {
        team_player_world.clear_trees();
        query.for_each(|(ent, tran, _, tp)| {
            //TODO: Fix Extents
            let quad = Quad::new(tran.translation.x, tran.translation.z, 0.5, 0.5);
            team_player_world.insert(*tp, ent, quad);
        })
    }

    pub fn manual_targeting_system(
        team_player_world : Res<TeamPlayerWorld>,
        identifiers : Res<Identifiers>,
        mut attack_command : EventReader<AttackCommand>,
        team_players : Query<&TeamPlayer>,
        trans : Query<&Transform>,
        mut query : Query<(&Transform, &TeamPlayer, &mut WeaponSet, &mut PathFinder, &mut MobileObject)>,
    ) {
        attack_command.iter().for_each(|f| {
            query.for_each_mut(|(tran, tp, mut wep, mut pf, mut mo)| {
                match identifiers.get_entity(f.target) {
                    Some(x) => {
                        match team_player_world.is_enemy(x, *tp, &team_players) {
                            Ok(y) => {
                                if y {
                                    wep.weapons[0].target = Target::ManualTarget(f.target);
                                    if let Ok(t) = trans.get(x) {
                                        let dir = Vec2::new(tran.translation.x - t.translation.x, tran.translation.z - t.translation.z).normalize() * wep.weapons[0].range * 0.99;
                                        if mathfu::D2::distance_magnitude((tran.translation.x, tran.translation.z), (t.translation.x, t.translation.z)) > dir.length_squared() {
                                            pf.start = tran.translation.xz();
                                            pf.end = t.translation.xz() + dir;
                                            mo.follow = true;
                                        }
                                        // pf.needs_path = true;
                                    }
                                }
                            },
                            Err(_) => {
                                wep.weapons[0].target = Target::None;
                            }
                        }
                    },
                    None => { }
                }
            })
        });
    }

    fn auto_targeting_system(
        team_player_world : Res<TeamPlayerWorld>,
        identifiers : Res<Identifiers>,
        transforms : Query<&Transform>,
        mut query : Query<(&Transform, &TeamPlayer, &mut WeaponSet)>
    ) {
        query.for_each_mut(|(tran, tp, mut wep)| {
            match wep.weapons[0].target {
                Target::AutoTarget(x) => {
                    match identifiers.get_entity(x) {
                        Some(e) => {
                            if match transforms.get(e) {
                                Ok(t) => {
                                    if !(mathfu::Dx::distance_between(vec![tran.translation.x, tran.translation.z], vec![t.translation.x, t.translation.z]) < wep.weapons[0].range) {
                                        true
                                    } else {
                                        false
                                    }
                                },
                                Err(_) => { true }
                            } {
                                wep.weapons[0].target = Target::None;
                            }
                        },
                        None => {
                            wep.weapons[0].target = Target::None;
                        }
                    }
                },
                Target::ManualTarget(x) => {
                    match identifiers.get_entity(x) {
                        Some(e) => {
                            if transforms.get(e).is_err() {
                                // println!("ERR!");
                                wep.weapons[0].target = Target::None;
                            }
                        },
                        None => {
                            wep.weapons[0].target = Target::None;
                        }
                    }
                },
                Target::None => {
                    for w in wep.weapons.clone().iter() {
                        let targets = team_player_world.search_targets(*tp, &tran.translation, &w);
                        for e in targets.iter() {
                            match transforms.get(*e) {
                                Ok(t) => {
                                    if mathfu::Dx::distance_between(vec![tran.translation.x, tran.translation.z], vec![t.translation.x, t.translation.z]) < wep.weapons[0].range {
                                        match identifiers.get_unique_id(*e) {
                                            Some(sf) => {
                                                wep.weapons[0].target = Target::AutoTarget(sf);
                                            },
                                            None => { }
                                        }
                                        break;
                                    }
                                },
                                Err(_) => { }
                            }
                        }
                    }
                }
            }
        })
    }

    fn weapons_look_at_system(
        identifiers : Res<Identifiers>,
        mut query : Query<(Entity, &WeaponSet)>,
        mut trans : Query<&mut Transform>,
    ) {
        let mut to_look : Vec<(Entity, Vec3)> = Vec::new();
        query.for_each_mut(|(ent, wep)| {
            for w in wep.weapons.iter() {
                if let Some(sf) = w.target.get_target() {
                    if let Some(e) = identifiers.get_entity(sf) {
                        if let Ok(t) = trans.get_mut(e) {
                            to_look.push((ent, t.translation));
                        }
                    }
                }
            }
        });
        for i in to_look {
            trans.get_mut(i.0).unwrap().look_at(i.1, Vec3::Y);
        }
    }

    fn weapons_system(
        time : Res<Time>,
        identifiers : Res<Identifiers>,
        mut query_iter : Query<(Entity, &Transform, &mut WeaponSet)>,
        trans : Query<&Transform>,
        mut healths : Query<&mut Health>
    ) {
        query_iter.for_each_mut(|(_, tran, mut wep)| {
            for w in wep.weapons.iter_mut() {
                if w.cooldown > 0.0 {
                    w.cooldown -= time.delta_seconds();
                }
                if w.cooldown > 0.0 {
                    continue;
                }
                if let Some(sf) = w.target.get_target() {
                    if let Some(e) = identifiers.get_entity(sf) {
                        if let (Ok(t), Ok(mut h)) = (trans.get(e), healths.get_mut(e)) {
                            if mathfu::D2::distance_magnitude((tran.translation.x, tran.translation.z), (t.translation.x, t.translation.z)) < w.range.powi(2) {
                                h.damage(w.damage, w.damage_types);
                                w.cooldown = w.fire_rate;
                            }
                        }
                    }
                }
            }
        });
    }

    fn health_system(mut dirties : ResMut<DirtyEntities>, query : Query<(Entity, &Health)>) {
        query.for_each(|(ent, hel)| {
            if hel.is_dead() {
                dirties.entities.push(ent);
            }
        });
    }
}