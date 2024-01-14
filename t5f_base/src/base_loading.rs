use bevy::prelude::*;
use bevy_asset_loader::{prelude::LoadingStateAppExt, loading_state::{LoadingState, config::ConfigureLoadingState}};
use crate::*;


pub struct BaseLoadingPlugin;

impl Plugin for BaseLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<LevelAsset>()
            .init_asset::<MapAsset>()
            .init_asset::<ObjectAsset>()

            .init_asset_loader::<LevelLoader>()
            .init_asset_loader::<MapAssetLoader>()
            .init_asset_loader::<ObjectAssetLoader>()

            .add_loading_state(LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Loading)
                .load_collection::<MapAssets>()
                .load_collection::<ObjectAssets>()
                .init_resource::<ObjectPrefabs>()
            )
        ;
    }
}
