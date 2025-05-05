use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;

#[derive(Debug, Default, Clone)]
#[derive(Resource)]
#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/roboto/Roboto-Black.ttf")]
    pub roboto: Handle<Font>,
}

#[derive(Debug, Default, Clone)]
#[derive(Resource)]
#[derive(AssetCollection)]
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