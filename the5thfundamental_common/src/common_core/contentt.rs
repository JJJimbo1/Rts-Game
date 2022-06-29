
use std::{path::Path, time::Duration};

use bevy::{math::Vec2, prelude::Entity};
use bevy::prelude::Component;
use serde::{
    Serialize, Deserialize,
};
use qloader::*;
use ronfile::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UIInfo {
    pub display_name : String,
    pub description : String,
    pub thumbnail : String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub ui_info : UIInfo,
    pub save_state : SaveFile,
}

impl QLoad<()> for Level {
    const PATHTYPE : PathType = PathType::Absolute;
    fn extensions() -> Vec<&'static str> {
        vec!["ron"]
    }
    fn load<S : AsRef<Path>>(_path : S) -> Result<Self, QLoaderError> {
        match RonFile::load::<Self, S>(_path) {
            Ok(x) => {
                Ok(x)
            },
            Err(_e) => {
                Err(QLoaderError::ParseError)
            }
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct MapBounds(pub Vec2);

impl Default for MapBounds {
    fn default() -> Self {
        Self(Vec2::new(1000.0, 1000.0))
    }
}

// #[derive(Debug, Clone)]
// #[derive(Serialize, Deserialize)]
// pub struct Constructor {
//     pub buildings : Vec<String>,
//     pub builder : bool,
// }

// #[derive(Debug, Clone)]
// #[derive(Serialize, Deserialize)]
// pub struct Trainer {
//     pub trainies : Vec<String>,
//     pub spawn_point : (f32, f32, f32),
//     pub end_point : (f32, f32, f32),
// }

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct EconomicObject {
    //Money
    pub resource_gen : f64,
    pub resource_drain : f64,
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct PowerObject {
    pub power_gen : u32,
    pub power_drain : u32,
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct MobileObject {
    pub follow : bool,
    pub max_forward_speed : f32,
    pub max_backwards_speed : f32,
    pub pursuant: Option<Entity>,
}

impl SerdeComponent for MobileObject {
    fn saved(&self) -> Option<Self> {
        if !self.follow {
            None
        } else {
            Some(*self)
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct StackData {
    // pub id : String,
    pub object_type : ObjectType,
    pub time_to_build : Duration,
    pub cost : u128,
    pub buffered : bool,
}