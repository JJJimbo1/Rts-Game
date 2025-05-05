
use avian3d::{prelude::LinearVelocity, PhysicsPlugins};
use bevy::prelude::*;

use crate::Slim;


impl Slim for LinearVelocity {
    fn slim(&self) -> Option<Self> {
        Some(self.clone())
    }
}

#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PhysicsPlugins::new(PostUpdate))
            .add_systems(PostUpdate, bound_system)
        ;
    }
}

#[derive(Component)]
pub struct LocalBounds {
    pub x : Vec2,
    pub y : Vec2,
    pub z : Vec2,
}

fn bound_system(
    mut bounded_query : Query<(&mut Transform, &LocalBounds)>,
) {
    bounded_query.iter_mut().for_each(|(mut tran, lob)| {
        tran.translation.x = tran.translation.x.clamp(lob.x.x, lob.x.y);
        tran.translation.y = tran.translation.y.clamp(lob.y.x, lob.y.y);
        tran.translation.z = tran.translation.z.clamp(lob.z.x, lob.z.y);
    });
}
