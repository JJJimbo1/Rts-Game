pub mod developer;

use bevy_asset_loader::prelude::AssetCollection;
pub use developer::*;

use std::fmt::Display;
use bevy::{prelude::*, reflect::TypePath, asset::{AssetLoader, io::Reader}};
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]

pub enum MapType {
    Developer,
    // Direction,
    // Sandbox,
}

// impl From<MapType> for AssetType {
//     fn from(map_type: MapType) -> Self {
//         Self::Map(map_type)
//     }
// }

impl Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapType::Developer => write!(f, "Developer")
        }
    }
}

pub trait MapMarker { }

pub struct AnyMapMarker;

impl MapMarker for AnyMapMarker { }

#[derive(Debug, Clone)]
#[derive(Event)]
pub struct MapLoadEvent {
    pub map_serde: MapSerde,
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Resource)]
pub enum MapSerde {
    Developer(DeveloperSerde),
}

impl Default for MapSerde {
    fn default() -> Self {
        Self::Developer(default())
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Asset, TypePath)]
pub struct MapAsset {
    pub bounds: Option<MapBounds>,
    pub collider_string: Option<String>,
}

#[derive(Default)]
pub struct MapAssetLoader;

impl AssetLoader for MapAssetLoader {
    type Asset = MapAsset;
    type Settings = ();
    type Error = LoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = ron::de::from_bytes::<MapAsset>(&bytes).or_else(|_| bincode::deserialize(&bytes).map(|map_asset| map_asset))?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

#[derive(Debug, Default, Clone)]
#[derive(Resource)]
#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(path = "maps/developer.ron")]
    pub developer: Handle<MapAsset>
}

impl MapAssets {
    pub fn from_map_type(&self, map_type: MapType) -> &Handle<MapAsset> {
        match map_type {
            MapType::Developer => &self.developer
        }
    }

    pub fn from_serde_map(&self, serde_map: &MapSerde) -> &Handle<MapAsset> {
        match serde_map {
            MapSerde::Developer(_) => &self.developer
        }
    }
}

pub struct MapPlugin;

impl MapPlugin {
    pub fn load_map(
        mut load_events: EventReader<MapLoadEvent>,
        map_assets: Res<MapAssets>,
        maps: Res<Assets<MapAsset>>,
        // mut identifiers: ResMut<Identifiers>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in load_events.read() {
            println!("MAP LOADING");
            commands.insert_resource(event.map_serde.clone());

            let map_handle: Handle<MapAsset> = map_assets.from_serde_map(&event.map_serde).clone();
            let Some(map_asset) = maps.get(&map_handle) else { return; };
            let Some(bounds) = map_asset.bounds else { return; };
            commands.insert_resource(bounds);
            commands.insert_resource(GridMap(DS2Map::new()));
            commands.insert_resource(GridSpace::new());

            match event.map_serde {
                MapSerde::Developer(developer) => { commands.spawn(DeveloperBundle::from((developer, &map_asset.try_into().unwrap()))); }
            }
            println!("Map Loaded");
            status.map_loaded = true;
        }
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MapLoadEvent>()
            .add_systems(Update,
                Self::load_map.run_if(resource_exists::<MapAssets>)
                // Self::load_map
            )
        ;
    }
}