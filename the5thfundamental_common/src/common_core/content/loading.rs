use bevy::{prelude::*, ecs::schedule::{StateData, }};
use bevy_asset_loader::prelude::{LoadingStateAppExt, LoadingState};
use crate::*;


pub struct CommonLoadingPlugin<T: StateData> {
    pub loading_state: T,
    pub next_state: T,
}

impl<T: StateData + Clone> Plugin for CommonLoadingPlugin<T> {
    fn build(&self, app: &mut App) {
        app

            .add_asset::<LevelAsset>()
            .add_asset::<MapAsset>()
            .add_asset::<ObjectAsset>()

            .add_asset_loader(LevelLoader)
            .add_asset_loader(MapAssetLoader)
            .add_asset_loader(ObjectAssetLoader)

            .add_loading_state(LoadingState::new(self.loading_state.clone())
                .with_collection::<LevelAssets>()
                .with_collection::<MapAssets>()
                .with_collection::<ObjectAssets>()
                .init_resource::<ObjectPrefabs>()
                .continue_to_state(self.next_state.clone())
            )
        ;
    }
}
