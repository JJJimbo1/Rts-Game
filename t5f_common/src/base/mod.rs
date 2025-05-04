pub mod level;
pub mod map;
pub mod object;
pub mod client;
pub mod error;

use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};
pub use level::*;
pub use map::*;
pub use object::*;
pub use client::*;
pub use error::*;

use bevy::prelude::*;

use crate::{DiskPlugin, GameState};

pub static BASE_LABEL: &'static str = "base";

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


pub struct BasePlugins;

impl PluginGroup for BasePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = bevy::app::PluginGroupBuilder::start::<BasePlugins>();
        let group = group
            .add(BaseLoadingPlugin)
            .add(LevelPlugin)
            .add(MapPlugin)
            .add(ObjectPlugin)
            .add(DiskPlugin)
            .add(BaseClientPlugin);

        group
    }
}

