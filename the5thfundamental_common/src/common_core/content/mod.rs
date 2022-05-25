pub mod maps;
pub mod objects;

pub use maps::*;
pub use objects::*;

use bevy::prelude::Component;




pub trait AssetId {
    fn id(&self) -> &'static str;
}

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub enum AssetType {
    Map(MapType),
    Object(ObjectType),
}

impl AssetId for AssetType {
    fn id(&self) -> &'static str {
        match self {
            AssetType::Map(map) => map.id(),
            AssetType::Object(object) => object.id(),
        }
    }
}