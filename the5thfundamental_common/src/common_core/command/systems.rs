pub use systems::*;
mod systems {
    use bevy::{math::Vec3Swizzles, prelude::*, tasks::ComputeTaskPool};
    use bevy_prototype_debug_lines::DebugLines;
    use bevy_pathfinding::*;
    use bevy_rapier3d::prelude::Velocity;
    use crate::{ActorType, Actors, CommonSystemSets, Identifiers, MoveCommand, Queues, ResourceProvider, TeamPlayer, WeaponSet, MobileObject};
    use simple_random::*;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
    pub enum CommandSystems {
        ScoreCalculatorSystem,
        AIUpdaterSystem,
        PathFindingSystem,
        PathFollowingSystem,
    }

    pub fn command_system_set(set : SystemSet) -> SystemSet {
        set.label(CommonSystemSets::Command)
            .with_system(score_calculator_system.label(CommandSystems::ScoreCalculatorSystem))
            .with_system(ai_updater_system.label(CommandSystems::AIUpdaterSystem).after(CommandSystems::ScoreCalculatorSystem))
            .with_system(set_follow_system.label(CommandSystems::PathFindingSystem).after(CommandSystems::AIUpdaterSystem))
            .with_system(path_following_system.label(CommandSystems::PathFollowingSystem).after(PathFindingSystems::PathFindingSystem))
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

    pub fn ai_updater_system(
        actors : ResMut<Actors>,
        idents : Res<Identifiers>,
        mut queues : Query<&mut Queues>,
    ) {
        for a in actors.actors.iter() {
            match a.1.actor_type {
                ActorType::AI { difficulty: _, settings: _ } => {
                    for b in a.1.buildings.clone().iter() {
                        if let Some(e) = idents.get_entity(*b) {
                            if let Ok(mut q) = queues.get_mut(e) {
                                if let Some(uq) = q.unit_queue.as_mut() {
                                    if uq.spine().len() > 1 {
                                        continue;
                                    }
                                    for o in uq.ordered().iter() {
                                        if uq.is_empty() {
                                            uq.data_mut().set_timer(o.time_to_build.as_secs_f64());
                                        }
                                        uq.raise_stack(o.clone(), 1);
                                    }
                                }
                            }
                        }
                    }
                }
                ActorType::Player => {

                },
            }
        }
    }

    pub fn set_follow_system(
        idents : Res<Identifiers>,
        mut rand : ResMut<Random>,
        mut mobile_object : Query<(&Transform, &mut PathFinder, &mut MobileObject)>,
        mut move_commands : EventReader<MoveCommand>,
    ) {
        move_commands.iter().for_each(|c| {
            let spread = (c.units.len() as f32).sqrt() * 2.0;
            for id in c.units.iter() {
                if let Some((trans, mut pf, mut mo)) = idents.get_entity(*id).and_then(|e| mobile_object.get_mut(e).ok()) {
                    pf.start = trans.translation.xz();
                    pf.end = c.position + Vec2::new(rand.range(-spread, spread), rand.range(-spread, spread));
                    mo.follow = true;
                }
            }
        });
    }

    pub fn path_following_system(
        // pool : Res<ComputeTaskPool>,
        mut debug : ResMut<DebugLines>,
        mut followers : Query<(&mut Path, &mut Transform, &mut Velocity, &mut MobileObject)>,
    ) {
        followers.for_each_mut(|(mut path, mut tran, mut vel, mut mobility)| {
            if !mobility.follow { return; }
            if let Some(x) = path.0.as_ref().and_then(|f| f.first().cloned()) {
                let y = tran.translation.y;
                tran.look_at(x.extend(y).xzy(), Vec3::Y);
                if tran.rotation.is_nan() { tran.rotation = Quat::IDENTITY}
                let distance = mathfu::D1::more_than_value_or_zero(mathfu::D2::distance((tran.translation.x, tran.translation.z), (x.x, x.y)), 0.5) * 2.0;
                let dir = Vec2::new(x.x - tran.translation.x, x.y - tran.translation.z).normalize();
                if path.0.as_ref().and_then(|f| f.get(1).cloned()).is_some() {
                    if distance > 0.5 {
                        vel.linvel.x =  dir.x * mobility.max_forward_speed.abs();
                        vel.linvel.z =  dir.y * mobility.max_forward_speed.abs();
                    } else {
                        path.0.as_mut().unwrap().remove(0);
                    }
                } else {
                    if distance > 0.1 {
                        vel.linvel.x =  dir.x * mathfu::D1::clamp(distance, -mobility.max_backwards_speed, mobility.max_forward_speed).abs();
                        vel.linvel.z =  dir.y * mathfu::D1::clamp(distance, -mobility.max_backwards_speed, mobility.max_forward_speed).abs();
                    } else {
                        vel.linvel.x = 0.0;
                        vel.linvel.z = 0.0;
                        path.0.as_mut().unwrap().remove(0);
                        mobility.follow = false;
                    }
                }

            } else {
                vel.linvel.x = 0.0;
                vel.linvel.z = 0.0;
            }
        });

        followers.for_each_mut(|(path, tran, _, _)| {
            if let Some(p) = &path.0 {
                if let Some(x) = p.first() {
                    // println!("{}", x);
                    debug.line_colored(tran.translation.xz().extend(1.0).xzy(), x.extend(1.0).xzy(), 0.0, Color::Rgba{ red : 1.0, green : 0.2, blue : 0.2, alpha : 1.0});
                }
                for i in 1..p.len() {
                    let previous = p[i - 1];
                    let current = p[i];
    
                    debug.line_colored(previous.extend(1.0).xzy(), current.extend(1.0).xzy(), 0.0, Color::Rgba{ red : 1.0, green : 0.2, blue : 0.2, alpha : 1.0});
                }
            }
        });


    }
}