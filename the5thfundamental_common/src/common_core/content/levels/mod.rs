use bevy::{prelude::*, asset::{AssetLoader, LoadedAsset}, reflect::TypeUuid};
use bevy_asset_loader::prelude::AssetCollection;
use serde::{Serialize, Deserialize};
use crate::SaveState;





















#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(TypeUuid)]
#[uuid = "215a1291-b752-4d19-97fd-6827d6cbaee0"]
pub struct LevelAsset {
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
            let custom_asset = ron::de::from_bytes::<LevelAsset>(bytes).or_else(|_| bincode::deserialize(bytes))?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["t5flvl"]
    }
}

#[derive(Debug, Default, Clone)]
#[derive(Resource)]
#[derive(AssetCollection)]
pub struct LevelAssets {
    #[asset(path = "levels/developer.t5flvl")]
    pub developer: Handle<LevelAsset>
}

// impl LevelAssets {
//     pub fn from_map_type(&self, map_type: Level) -> &Handle<MapAsset> {
//         match map_type {
//             MapType::Developer => &self.developer
//         }
//     }

//     pub fn from_serde_map(&self, serde_map: &SerdeMap) -> &Handle<MapAsset> {
//         match serde_map {
//             SerdeMap::Developer(_) => &self.developer
//         }
//     }
// }
