use std::path::Path;

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use qloader::*;



#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_system(bound_system)

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
    bounded_query.for_each_mut(|(mut tran, lob)| {
            tran.translation.x = tran.translation.x.clamp(lob.x.x, lob.x.y);
            tran.translation.y = tran.translation.y.clamp(lob.y.x, lob.y.y);
            tran.translation.z = tran.translation.z.clamp(lob.z.x, lob.z.y);
    });
}

#[derive(Debug, Clone)]
pub struct ColliderAsset {
    vertices: Vec<Vec3>,
    indices: Vec<[u32; 3]>,
}

impl QLoad<()> for ColliderAsset {
    const PATHTYPE : PathType = PathType::Absolute;
    fn extensions() -> Vec<&'static str> {
        vec!["brcol"]
    }
    fn load<S : AsRef<Path>>(_path : S) -> Result<Self, QLoaderError> {
        if let Ok(bytes) = std::fs::read(_path) {
            if let Ok((v, i)) = bincode::deserialize::<(Vec<Vec3>, Vec<[u32; 3]>)>(&bytes) {
                return Ok(Self {
                    vertices: v,
                    indices: i,
                });
            }
        }
        Err(QLoaderError::ParseError)
    }
}

impl From<ColliderAsset> for Collider {
    fn from(collider_asset : ColliderAsset) -> Self {
        Self::trimesh(collider_asset.vertices, collider_asset.indices)
    }
}