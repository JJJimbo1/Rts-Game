pub use systems::*;
mod systems {
    use bevy::{ecs::entity::Entities, prelude::*};

    use mathfu::D1;

    use crate::*;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
    pub enum PhysicsSystems {
        TorqueSystem,
        VelocitySystem,
        PhysicsWorldSystem,
    }

    #[derive(Default)]
    pub struct PhysicsPlugin;

    pub fn physics_system_set(set : SystemSet) -> SystemSet {
        set.label(CommonSystemSets::Physics)
            .with_system(torque_system.system().label(PhysicsSystems::TorqueSystem))
            .with_system(velocity_system.system().label(PhysicsSystems::VelocitySystem).after(PhysicsSystems::TorqueSystem))
            .with_system(physics_world_system.system().label(PhysicsSystems::PhysicsWorldSystem).after(PhysicsSystems::VelocitySystem))
    }

    fn torque_system(time : Res<Time>, mut query : Query<(&mut Transform, &Torque)>) {
        query.for_each_mut(|(mut tran, tor)| {
            if tor.x.is_normal() {
                tran.rotation *= Quat::from_rotation_x(tor.x);
            }
            if tor.y.is_normal() {
                tran.rotation *= Quat::from_rotation_y(tor.y);
            }
            if tor.z.is_normal() {
                tran.rotation *= Quat::from_rotation_z(tor.z);
            }
        });
    }

    fn velocity_system(
        time : Res<Time>,
        // bounds : Res<PhysicsBounds>,
        mut bounded_query : Query<(&mut Transform, &Velocity, &LocalBounds), Without<Immobile>>,
        mut unbounded_query : Query<(&mut Transform, &Velocity), (Without<LocalBounds>, Without<Immobile>)>,
    ) {
        bounded_query.for_each_mut(|(mut tran, vel, lob)| {
                let movement = if vel.local { tran.rotation.mul_vec3(Vec3::new(vel.x, vel.y, vel.z)) } else { Vec3::new(vel.x, vel.y, vel.z) };
                tran.translation += movement * time.delta_seconds();
                tran.translation.x = D1::clamp(tran.translation.x, lob.x.x, lob.x.y);
                tran.translation.y = D1::clamp(tran.translation.y, lob.y.x, lob.y.y);
                tran.translation.z = D1::clamp(tran.translation.z, lob.z.x, lob.z.y);
        });
        unbounded_query.for_each_mut(|(mut tran, vel)| {
            let movement = if vel.local { tran.rotation.mul_vec3(Vec3::new(vel.x, vel.y, vel.z)) } else { Vec3::new(vel.x, vel.y, vel.z) };
            tran.translation += movement * time.delta_seconds();
        });
        // match *bounds {
        //     Some(x) => {
        //     },
        //     None => {
        //         query.for_each_mut(|(mut tran, vel, unb, lob)| {
        //             match lob {
        //                 Some(lb) => {
        //                     let movement = if vel.local { tran.rotation.mul_vec3(Vec3::new(vel.x, vel.y, vel.z)) } else { Vec3::new(vel.x, vel.y, vel.z) };
        //                     tran.translation += movement * time.delta_seconds();
        //                     tran.translation.x = D1::clamp(tran.translation.x, lb.x.x, lb.x.y);
        //                     tran.translation.y = D1::clamp(tran.translation.y, lb.y.x, lb.y.y);
        //                     tran.translation.z = D1::clamp(tran.translation.z, lb.z.x, lb.z.y);
        //                 },
        //                 None => {
        //                     let movement = if vel.local { tran.rotation.mul_vec3(Vec3::new(vel.x, vel.y, vel.z)) } else { Vec3::new(vel.x, vel.y, vel.z) };
        //                     tran.translation += movement * time.delta_seconds();
        //                 }
        //             }
        //         });
        //     }
        // }
    }

    fn physics_world_system(mut physics_world : ResMut<PhysicsWorld>, team_players : Query<&TeamPlayer>, query : Query<(Entity, &Transform, &TeamPlayer)>) {
        physics_world.populate(team_players, query);
    }
}