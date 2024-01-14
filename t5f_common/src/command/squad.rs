
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
// use t5f_utility::mathfu::d1::*;


#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Squad {
    pub buffer: f32,
    pub members: Vec<(String, Option<Entity>)>,
}

impl Squad {
    // pub fn members(&self, health_percent: f32) -> usize {
    //     if health_percent > self.buffer { return self.members.len(); }
    //     let x = lerp(0.0, self.buffer, health_percent);

    // }
}

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct AssetSquad {
    pub buffer: f32,
    pub members: Vec<String>,
}

impl From<AssetSquad> for Squad {
    fn from(prefab_squad: AssetSquad) -> Self {
        Self {
            buffer: prefab_squad.buffer,
            members: prefab_squad.members.iter().map(|object_type| (object_type.clone(), None)).collect(),
        }
    }
}