use bevy::prelude::*;
use bevy_asset_loader::{prelude::{LoadingStateAppExt, LoadingState}, loading_state::config::ConfigureLoadingState};
use crate::*;


pub struct ClientLoadingPlugin;

impl Plugin for ClientLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Loading)
                .load_collection::<FontAssets>()
                .load_collection::<ImageAssets>()
            )
        ;
    }
}