pub mod levels;
pub mod maps;
pub mod objects;
pub mod error;
pub mod loading;

use std::fmt::Display;

pub use levels::*;
pub use maps::*;
pub use objects::*;
pub use error::*;
pub use loading::*;

use bevy::{prelude::{Component, Entity}, utils::HashSet, math::Vec2};

use crate::TeamPlayer;




// pub trait AssetId {
//     fn id(&self) -> Option<&'static str>;
// }

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub enum AssetType {
    Map(MapType),
    Object(ObjectType),
}

impl Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Map(map) => map.fmt(f),
            Self::Object(object) => object.fmt(f)
        }
    }
}

// impl AssetId for AssetType {
//     fn id(&self) -> Option<&'static str> {
//         match self {
//             AssetType::Map(map) => map.id(),
//             AssetType::Object(object) => object.id(),
//         }
//     }
// }


#[derive(Debug, Clone, Copy)]
pub struct ActivationEvent {
    pub entity: Entity,
    pub player: TeamPlayer,
}

#[derive(Debug, Clone)]
pub struct UnitCommandEvent {
    pub units: HashSet<Entity>,
    pub command_type: UnitCommandType,
}

#[derive(Debug, Copy, Clone)]
pub enum UnitCommandType {
    Move(Vec2),
    Attack(Entity),
}
