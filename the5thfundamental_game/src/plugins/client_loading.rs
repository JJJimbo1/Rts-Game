use bevy::{prelude::*, ecs::schedule::StateData};
use bevy_asset_loader::prelude::{LoadingStateAppExt, LoadingState};
use crate::*;


pub struct ClientLoadingPlugin<S: StateData> {
    pub loading_state: S,
    pub next_state: S,
}

impl<S: StateData + Clone> Plugin for ClientLoadingPlugin<S> {
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