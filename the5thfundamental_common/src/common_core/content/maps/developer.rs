use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Developer;

impl AssetId for Developer {
    fn id(&self) -> Option<&'static str> {
        MapType::from(*self).id()
    }
}

impl From<Developer> for MapType {
    fn from(_: Developer) -> Self {
        Self::Developer
    }
}

impl From<Developer> for AssetType {
    fn from(_: Developer) -> Self {
        Self::Map(MapType::Developer)
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct DeveloperBundle {
    pub developer: Developer,
    pub map_type: MapType,
    pub asset_type: AssetType,
    pub snowflake: Snowflake,
    pub collider: Collider,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

// impl DeveloperBundle {
//     pub fn with_spawn_data(mut self, spawn_data: MapSpawnEventData) -> Self {
//         self
//     }
// }

impl From<DeveloperPrefab> for DeveloperBundle {
    fn from(prefab: DeveloperPrefab) -> Self {
        Self {
            developer: Developer,
            map_type: Developer.into(),
            asset_type: Developer.into(),
            snowflake: Snowflake::new(),
            collider: prefab.real_collider.clone().unwrap(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(SerdeDeveloper, &DeveloperPrefab)> for DeveloperBundle {
    fn from((save, prefab): (SerdeDeveloper, &DeveloperPrefab)) -> Self {
        Self {
            developer: Developer,
            map_type: Developer.into(),
            asset_type: Developer.into(),
            snowflake: Snowflake::new(),
            collider: prefab.real_collider.clone().unwrap(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct DeveloperPrefab {
    pub bounds: MapBounds,
    pub collider_string: String,
    #[serde(skip)]
    pub real_collider: Option<Collider>,
    // pub stack: (ActiveQueue, StackData),
    // pub health: Health,
    // pub queues: PrefabQueues,
    // #[serde(skip)]
    // pub real_queues: Option<Queues>,
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct SerdeDeveloper {
    // snowflake: Snowflake,
}

// impl<'a> From<SerdeDeveloperQuery<'a>> for SerdeDeveloper {
//     fn from(object: SerdeDevelopoerQuery) -> Self {
//         Self {
//             // health: object.0.saved(),
//             // queues: object.1.saved(),
//             // team_player: *object.2,
//             // transform: (*object.3).into(),
//         }
//     }
// }