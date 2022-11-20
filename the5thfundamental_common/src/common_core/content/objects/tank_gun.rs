use bevy::prelude::*;
use bevy_pathfinding::Path;
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct TankGun;

impl AssetId for TankGun {
    fn id(&self) -> Option<&'static str> {
        ObjectType::from(*self).id()
    }
}

impl From<TankGun> for ObjectType {
    fn from(_: TankGun) -> Self {
        ObjectType::TankGun
    }
}

impl From<TankGun> for AssetType {
    fn from(_: TankGun) -> Self {
        Self::Object(ObjectType::TankGun)
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct TankGunBundle {
    pub tank_gun: TankGun,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub snowflake: Snowflake,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl TankGunBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.transform = spawn_data.transform;
        self
    }
}

impl Default for TankGunBundle {
    fn default() -> Self {
        Self {
            tank_gun: TankGun,
            object_type: TankGun.into(),
            asset_type: TankGun.into(),
            snowflake: Snowflake::new(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

pub fn tank_gun_spawn(
    prefabs: Res<ObjectPrefabs>,
    mut resource_nodes: Query<(Entity, &Transform), Added<TankGun>>,
    mut commands: Commands,
) {
    resource_nodes.for_each_mut(|(entity, transform)| {
        
        println!("Tank gun");
        // for i in 0..tank.0.len() {
        //     let tank_turret_offset = Vec3::new(0.0, 1.81797, -0.28511);
        //     let spawn_data = ObjectSpawnEventData {
        //         snowflake: Snowflake::new(),
        //         object_type: ObjectType::TankGun,
        //         teamplayer: TeamPlayer::default(),
        //         transform: tank_turret_offset,
        //     };
        //     let platform = ResourcePlatformUnclaimed(Some((entity, i)));
        //     commands.spawn(ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_platform(platform).with_spawn_data(spawn_data));
        // }
    });
}