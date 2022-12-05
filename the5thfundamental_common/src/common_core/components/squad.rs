
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::*;


#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Squad {
    pub max_members: u8,
    pub current_members: u8,
    #[serde(skip)]
    pub members: Vec<(ObjectType, Option<Entity>)>,
}

impl SerdeComponent for Squad {
    fn saved(&self) -> Option<Self> {
        if self.current_members == self.max_members {
            None
        } else {
            Some(self.clone())
        }
    }
}

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct PrefabSquad {
    pub members: Vec<ObjectType>,
}

impl From<PrefabSquad> for Squad {
    fn from(prefab_squad: PrefabSquad) -> Self {
        Self {
            max_members: prefab_squad.members.len() as u8,
            current_members: prefab_squad.members.len() as u8,
            members: prefab_squad.members.iter().map(|object_type| (*object_type, None)).collect(),
        }
    }
}