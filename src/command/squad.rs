
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{ObjectType, Slim};

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Squad {
    pub buffer: f32,
    pub members: Vec<(ObjectType, Option<Entity>)>,
}

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct AssetSquad {
    pub buffer: f32,
    pub members: Vec<ObjectType>,
}

impl From<AssetSquad> for Squad {
    fn from(prefab_squad: AssetSquad) -> Self {
        Self {
            buffer: prefab_squad.buffer,
            members: prefab_squad.members.iter().map(|object_type| (object_type.clone(), None)).collect(),
        }
    }
}

impl Slim for Squad {
    fn slim(&self) -> Option<Self> {
        None
    }
}