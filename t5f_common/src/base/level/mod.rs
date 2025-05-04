use bevy::{prelude::*, asset::{AssetLoader, io::Reader}, reflect::TypePath};
use serde::{Serialize, Deserialize};
use crate::*;

pub struct AnyLevelMarker;
pub trait LevelMarker { }
impl LevelMarker for AnyLevelMarker { }

#[derive(Debug, Clone)]
#[derive(Event)]
pub struct LoadLevels;


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Asset, TypePath)]
pub struct LevelAsset;

#[derive(Default)]
pub struct LevelLoader;

impl AssetLoader for LevelLoader {
    type Asset = LevelAsset;
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
        let asset = ron::de::from_bytes::<LevelAsset>(&bytes).or_else(|_| bincode::deserialize(&bytes))?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}


pub struct LevelPlugin;

impl LevelPlugin {
    pub fn load_level(
        mut load_events: EventReader<LoadLevels>,
        mut status: ResMut<LoadingStatus>,
    ) {
        for _ in load_events.read() {
            status.level_loaded = true;
        }
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LoadLevels>()
            .add_systems(Update,
                Self::load_level
            )
        ;
    }
}