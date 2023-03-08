use bevy::{prelude::*, asset::{LoadState, HandleId}, utils::HashMap, ecs::schedule::{StateData, }};
use bevy_asset_loader::prelude::{AssetCollection, LoadingStateAppExt, LoadingState};
use crate::*;


pub struct ClientLoadingPlugin<T: StateData> {
    pub loading_state: T,
    pub next_state: T,
}

impl<T: StateData + Clone> Plugin for ClientLoadingPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(LoadingState::new(self.loading_state.clone())
                .with_collection::<FontAssets>()
                .with_collection::<ImageAssets>()
                .with_collection::<GltfAssets>()
                .continue_to_state(self.next_state.clone())
            )
        ;
    }
}