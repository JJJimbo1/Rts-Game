pub mod states;

pub use states::*;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::*;

#[derive(Debug, Default, Clone, Resource, AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/roboto/Roboto-Black.ttf")]
    pub roboto: Handle<Font>,
}

#[derive(Debug, Default, Clone, Resource, AssetCollection)]
pub struct ImageAssets {
    pub menu_button: Handle<Image>,
    #[asset(path = "textures/ui/black_box.png")]
    pub black_box: Handle<Image>,
    #[asset(path = "textures/ui/white_box.png")]
    pub white_box: Handle<Image>,
    #[asset(path = "textures/ui/white_dotted_box.png")]
    pub white_dotted_box: Handle<Image>,
    #[asset(path = "textures/ui/gold_dotted_box.png")]
    pub gold_dotted_box: Handle<Image>,
    #[asset(path = "textures/ui/health_bar.png")]
    pub health_bar: Handle<Image>,
    #[asset(path = "textures/ui/health_bar_green.png")]
    pub health_bar_green: Handle<Image>,
    #[asset(path = "textures/ui/selection_box.png")]
    pub selection_box: Handle<Image>,
}

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