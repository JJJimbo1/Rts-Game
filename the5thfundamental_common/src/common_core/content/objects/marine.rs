use bevy::{prelude::*, ecs::schedule::StateData};
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Default, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Marine;

// impl AssetId for Marine {
//     fn id(&self) -> Option<&'static str> {
//         ObjectType::from(*self).id()
//     }
// }

impl From<Marine> for ObjectType {
    fn from(_: Marine) -> Self {
        ObjectType::Marine
    }
}

impl From<Marine> for AssetType {
    fn from(_: Marine) -> Self {
        Self::Object(ObjectType::Marine)
    }
}


#[derive(Clone)]
#[derive(Bundle)]
pub struct MarineBundle {
    pub marine: Marine,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub snowflake: Snowflake,
    pub team_player: TeamPlayer,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl MarineBundle {
    pub fn with_spawn_data(mut self, spawn_data: SpawnData) -> Self {
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }
}

impl Default for MarineBundle {
    fn default() -> Self {
        Self {
            marine: Marine,
            object_type: Marine.into(),
            asset_type: Marine.into(),
            snowflake: Snowflake::new(),
            team_player: TeamPlayer::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

pub struct MarinePlugin<T: StateData> {
    state: T,
}

impl<T: StateData> MarinePlugin<T> {
    
}

impl<T: StateData> Plugin for MarinePlugin<T> {
    fn build(&self, app: &mut App) {
        
    }
}