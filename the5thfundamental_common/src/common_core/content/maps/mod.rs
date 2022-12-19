pub mod developer;

use bevy_asset_loader::prelude::AssetCollection;
use bevy_rapier3d::prelude::Collider;
pub use developer::*;

use std::fmt::Display;
use bevy::{prelude::*, reflect::TypeUuid, asset::{AssetLoader, LoadedAsset, AssetPath}};
use serde::{Serialize, Deserialize};

use crate::AssetType;

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

impl Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapType::Developer => write!(f, "Developer")
        }
    }
}

// impl AssetId for MapType {
//     fn id(&self) -> Option<&'static str> {
//         match self {
//             Self::Developer => { Some("developer") },
//             // Self::Direction => { "direction" },
//             // Self::Sandbox => { "sandbox" },
//         }
//     }
// }

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Resource)]
pub enum SerdeMap {
    Developer(SerdeDeveloper),
}

impl Default for SerdeMap {
    fn default() -> Self {
        Self::Developer(default())
    }
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
pub struct MapAsset {
    pub bounds: Option<MapBounds>,
    pub collider_string: Option<String>,
}

pub struct MapAssetLoader;

impl AssetLoader for MapAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<MapAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["t5fmap"]
    }
}

#[derive(Debug, Default, Clone)]
#[derive(Resource)]
#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(path = "maps/developer.t5fmap")]
    pub developer: Handle<MapAsset>
}

impl MapAssets {
    pub fn from_map_type(&self, map_type: MapType) -> &Handle<MapAsset> {
        match map_type {
            MapType::Developer => &self.developer
        }
    }

    pub fn from_serde_map(&self, serde_map: &SerdeMap) -> &Handle<MapAsset> {
        match serde_map {
            SerdeMap::Developer(_) => &self.developer
        }
    }
}
