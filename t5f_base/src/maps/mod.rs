pub mod developer;

use bevy_asset_loader::prelude::AssetCollection;
use t5f_common::{GridMap, DS2Map, GridSpace, MapBounds};
pub use developer::*;

use std::{fmt::Display, marker::PhantomData};
use bevy::{prelude::*, reflect::{TypeUuid, TypePath}, asset::{AssetLoader, io::Reader, AsyncReadExt}};
use serde::{Serialize, Deserialize};

use crate::{AssetType, LoadingStatus, LoaderError};

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

pub trait MapMarker { }

pub struct AnyMapMarker;

impl MapMarker for AnyMapMarker { }

#[derive(Debug, Clone)]
#[derive(Event)]
pub struct MapLoadEvent<M: MapMarker>(pub MapLoadEventData, pub PhantomData<M>);

impl<M: MapMarker> MapLoadEvent<M> {
    // pub fn spawn_data(&self) -> &SpawnData {
    //     &self.0.spawn_data
    // }

    // pub fn serde_data(&self) -> &Option<SerdeData> {
    //     &self.0.serde_data
    // }
}

#[derive(Debug, Clone)]
#[derive(Event)]
pub struct MapLoadEventData {
    pub map: MapSerde,
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
#[derive(Asset, TypePath, TypeUuid)]
#[uuid = "e6fdd9fa-16f4-4cea-afab-fdb5db3d0d80"]
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

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<MapAsset, LoaderError>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let asset = ron::de::from_bytes::<MapAsset>(&bytes).or_else(|_| bincode::deserialize(&bytes))?;
            Ok(asset)
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

    pub fn from_serde_map(&self, serde_map: &MapSerde) -> &Handle<MapAsset> {
        match serde_map {
            MapSerde::Developer(_) => &self.developer
        }
    }
}

pub struct MapPlugin;

impl MapPlugin {
    pub fn load_map(
        mut load_events: EventReader<MapLoadEvent<AnyMapMarker>>,
        map_assets: Res<MapAssets>,
        maps: Res<Assets<MapAsset>>,
        // mut identifiers: ResMut<Identifiers>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in load_events.read() {
            let map_serde = &event.0.map;
            commands.insert_resource(map_serde.clone());

            let map_handle: Handle<MapAsset> = map_assets.from_serde_map(map_serde).clone();
            let Some(map_asset) = maps.get(&map_handle) else { return; };
            let Some(bounds) = map_asset.bounds else { return; };
            commands.insert_resource(bounds);
            commands.insert_resource(GridMap(DS2Map::new()));
            commands.insert_resource(GridSpace::new());

            match map_serde {
                MapSerde::Developer(developer) => { let map: DeveloperPrefab = map_asset.try_into().unwrap(); commands.spawn(DeveloperBundle::from((*developer, &map))); }
            }
            println!("Map Loaded");
            status.map_loaded = true;
        }
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MapLoadEvent<AnyMapMarker>>()
            .add_systems(Update,
                Self::load_map.run_if(resource_exists::<MapAssets>())
            )
        ;
    }
}