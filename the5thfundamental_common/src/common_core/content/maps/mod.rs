pub mod developer;

use bevy_rapier3d::prelude::Collider;
pub use developer::*;

use bevy::{prelude::*, reflect::TypeUuid, asset::{AssetLoader, LoadedAsset, AssetPath}};
use serde::{Serialize, Deserialize};

use crate::{AssetId, AssetType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]

pub enum MapType {
    Developer,
    // Direction,
    // Sandbox,
}

impl From<MapType> for AssetType {
    fn from(map_type: MapType) -> Self {
        Self::Map(map_type)
    }
}

impl AssetId for MapType {
    fn id(&self) -> Option<&'static str> {
        match self {
            Self::Developer => { Some("developer") },
            // Self::Direction => { "direction" },
            // Self::Sandbox => { "sandbox" },
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Resource)]
pub enum SerdeMap {
    Developer(SerdeDeveloper),
}

// impl AssetId for SerdeMap {
//     fn id(&self) -> Option<&'static str> {
//         match self {
//             Self::Developer(_) => { Some("developer") },
//             // Self::Direction => { "direction" },
//             // Self::Sandbox => { "sandbox" },
//         }
//     }
// }

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Resource)]
pub struct MapBounds(pub Vec2);

impl Default for MapBounds {
    fn default() -> Self {
        Self(Vec2::new(1000.0, 1000.0))
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(TypeUuid)]
#[uuid = "e6fdd9fa-16f4-4cea-afab-fdb5db3d0d80"]
pub struct Map {
    pub bounds: Option<MapBounds>,
    pub collider_string: Option<String>,
}

pub struct MapLoader;

impl AssetLoader for MapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            match ron::de::from_bytes::<Map>(bytes) {
                Ok(asset) => {

                },
                Err(e) => {
                    error!("{}", e);
                    error!("{:?}", &bytes[0..24]);
                }
            }
            let custom_asset = ron::de::from_bytes::<Map>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["t5fmap"]
    }
}


pub enum MapAsset {
    Developer
}

impl From<&SerdeMap> for MapAsset {
    fn from(value: &SerdeMap) -> Self {
        match value {
            SerdeMap::Developer(_) => MapAsset::Developer,
        }
    }
}

impl<'a> From<MapAsset> for AssetPath<'a> {
    fn from(value: MapAsset) -> Self {
        let path = match value {
            MapAsset::Developer => "developer.t5fmap"
        };
        format!("maps/{}", path).into()
    }
}
