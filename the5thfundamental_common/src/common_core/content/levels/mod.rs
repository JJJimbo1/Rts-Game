use std::path::Path;

use bevy::{asset::{AssetLoader, LoadedAsset}, reflect::TypeUuid};
use serde::{Serialize, Deserialize};
use crate::SaveState;





















#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(TypeUuid)]
#[uuid = "215a1291-b752-4d19-97fd-6827d6cbaee0"]
pub struct Level {
    // pub ui_info : UIInfo,
    pub save_state : SaveState,
}

pub struct LevelLoader;

impl AssetLoader for LevelLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<Level>(bytes).or_else(|_| bincode::deserialize(bytes))?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["t5flvl"]
    }
}