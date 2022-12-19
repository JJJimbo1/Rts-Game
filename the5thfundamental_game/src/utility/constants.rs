use std::path::PathBuf;

use bevy::prelude::Color;
use lazy_static::lazy_static;
use the5thfundamental_common::TeamPlayer;

////!Environment---!\\\\

// lazy_static! {
    // #[derive(Debug, Copy, Clone)]
    // pub static ref PROJECT_ROOT_DIRECTORY : String = "NO".to_string();
    // pub static ref PROJECT_ROOT_DIRECTORY : String = std::env::current_dir().unwrap_or(PathBuf::from("")).to_string_lossy().to_string();
    // #[derive(Debug, Copy, Clone)]
    // pub static ref ASSET_DIRECTORY : String = format!("{}", *PROJECT_ROOT_DIRECTORY);
// }

pub static CLEAR_COLOR : Color = Color::rgba_linear(0.0, 0.2, 0.7, 1.0);

////!---UI---!\\\\

pub static MENU_WIDTH : f32 = 480.0;
pub static MENU_HEIGHT : f32 = 1080.0;

pub static PRIMARY_MENU_MARGIN : f32 = 0.0;
pub static SECONDARY_MENU_MARGIN : f32 = PRIMARY_MENU_MARGIN + MENU_WIDTH;

pub static DARK_BACKGROUND_COLOR : Color = Color::rgba_linear(0.03, 0.03, 0.03, 0.9);
pub static LIGHT_BACKGROUND_COLOR : Color = Color::rgba_linear(0.7, 0.7, 0.7, 0.9);
pub static BLACK : Color = Color::rgba_linear(0.00, 0.00, 0.00, 1.0);
pub static GREEN : Color = Color::rgba_linear(0.0, 1.0, 0.0, 1.0);
pub static EMPTY_COLOR : Color = Color::rgba_linear(0.0, 0.0, 0.0, 0.0);

pub static LIGHT_SHADE_COLOR : Color = Color::rgba_linear(0.0, 0.0, 0.0, 0.25);
pub static MEDIUM_SHADE_COLOR : Color = Color::rgba_linear(0.0, 0.0, 0.0, 0.45);
pub static HARD_SHADE_COLOR : Color = Color::rgba_linear(0.0, 0.0, 0.0, 0.75);

pub static FONT_SIZE_SMALL : f32 = 20.0;
pub static FONT_SIZE_MEDIUM : f32 = 30.0;
pub static FONT_SIZE_LARGE : f32 = 40.0;
pub static FONT_SIZE_EXTRA_LARGE : f32 = 60.0;

pub static FONT_SIZE_HEADER_MUL : f32 = 2.0;

pub static TEXT_COLOR_NORMAL : Color = Color::rgba_linear(0.8, 0.8, 0.8, 1.0);
pub static TEXT_COLOR_UNUSED : Color = Color::rgba_linear(0.2, 0.2, 0.2, 1.0);
pub static TEXT_COLOR_HOVER : Color = Color::rgba_linear(0.5, 0.8, 0.3, 1.0);
pub static TEXT_COLOR_PRESS : Color = Color::rgba_linear(0.1, 0.4, 0.9, 1.0);

////!---Defaults---!\\\\

pub static PLAYER_ID : TeamPlayer = TeamPlayer { team : 1, player : 0};
pub static DEFAULT_MODEL : &str = "default_cube";

pub use assets::*;
pub mod assets {
    use bevy::prelude::*;
    use bevy_asset_loader::prelude::AssetCollection;
    use the5thfundamental_common::{AssetType, MapType, ObjectType};

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
        #[asset(path = "textures/ui/health_bar_start.png")]
        pub health_bar_start: Handle<Image>,
        #[asset(path = "textures/ui/health_bar_middle.png")]
        pub health_bar_middle: Handle<Image>,
        #[asset(path = "textures/ui/health_bar_end.png")]
        pub health_bar_end: Handle<Image>,
        #[asset(path = "textures/ui/health_bar_green.png")]
        pub health_bar_green: Handle<Image>,
        #[asset(path = "textures/ui/selection_box.png")]
        pub selection_box: Handle<Image>,
    }

    // impl<'a> From<ImageAsset> for AssetPath<'a> {
    //     fn from(value: ImageAsset) -> Self {
    //         let path = match value {
    //             ImageAsset::BlackBox => "black_box.png",
    //             ImageAsset::WhiteBox => "white_box.png",
    //             ImageAsset::WhiteDottedBox => "white_dotted_box.png",
    //             ImageAsset::GoldDottedBox => "gold_dotted_box.png",
    //             ImageAsset::HealthBarStart => "health_bar_start.png",
    //             ImageAsset::HealthBarMiddle => "health_bar_middle.png",
    //             ImageAsset::HealthBarEnd => "health_bar_end.png",
    //             ImageAsset::HealthBarGreen => "health_bar_green.png",
    //             ImageAsset::SelectionBox => "selection_box.png",
    //         };
    //         format!("textures/ui/{}", path).into()
    //     }
    // }

    #[derive(Debug, Default, Clone)]
    #[derive(Resource)]
    #[derive(AssetCollection)]
    pub struct GltfAssets {
        #[asset(path = "models/developer.glb#Scene0")]
        pub developer: Handle<Scene>,
        #[asset(path = "models/crane_yard.glb#Scene0")]
        pub crane_yard: Handle<Scene>,
        #[asset(path = "models/resource_node.glb#Scene0")]
        pub resource_node: Handle<Scene>,
        #[asset(path = "models/resource_platform_unclaimed.glb#Scene0")]
        pub resource_platform_unclaimed: Handle<Scene>,
        #[asset(path = "models/resource_platform_claimed.glb#Scene0")]
        pub resource_platform_claimed: Handle<Scene>,
        #[asset(path = "models/factory.glb#Scene0")]
        pub factory: Handle<Scene>,
        #[asset(path = "models/marine.glb#Scene0")]
        pub marine: Handle<Scene>,
        #[asset(path = "models/tank_base.glb#Scene0")]
        pub tank_base: Handle<Scene>,
        #[asset(path = "models/tank_gun.glb#Scene0")]
        pub tank_gun: Handle<Scene>

    }

    impl GltfAssets {
        pub fn get_scene(&self, asset_type: AssetType) -> Option<&Handle<Scene>> {
            match asset_type {
                AssetType::Map(map) => {
                    match map {
                        MapType::Developer => Some(&self.developer),
                        _ => None,
                    }
                },
                AssetType::Object(object) => {
                    match object {
                        ObjectType::CraneYard => Some(&self.crane_yard),
                        ObjectType::ResourceNode => Some(&self.resource_node),
                        ObjectType::ResourcePlatformUnclaimed => Some(&self.resource_platform_unclaimed),
                        ObjectType::ResourcePlatformClaimed => Some(&self.resource_platform_claimed),
                        ObjectType::Factory => Some(&self.factory),
                        ObjectType::Marine => Some(&self.marine),
                        ObjectType::TankBase => Some(&self.tank_base),
                        ObjectType::TankGun => Some(&self.tank_gun),
                        _ => None
                    }
                }
            }
        }
    }

    // impl From<AssetType> for GltfAsset {
    //     fn from(value: AssetType) -> Self {
    //         match value {
    //             AssetType::Map(map) => {
    //                 match map {
    //                     MapType::Developer => GltfAsset::Developer,
    //                 }
    //             },
    //             AssetType::Object(object) => {
    //                 match object {
    //                     ObjectType::CraneYard => GltfAsset::CraneYard,
    //                     ObjectType::ResourceNode => GltfAsset::ResourceNode,
    //                     ObjectType::ResourcePlatformClaimed => GltfAsset::ResourcePlatformClaimed,
    //                     ObjectType::ResourcePlatformUnclaimed => GltfAsset::ResourcePlatformUnclaimed,
    //                     ObjectType::Factory => GltfAsset::Factory,
    //                     ObjectType::MarineSquad => GltfAsset::MarineSquad,
    //                     ObjectType::Marine => GltfAsset::Marine,
    //                     ObjectType::TankBase => GltfAsset::TankBase,
    //                     ObjectType::TankGun => GltfAsset::TankGun,
    //                 }
    //             }
    //         }
    //     }
    // }

    // impl<'a> From<GltfAsset> for AssetPath<'a> {
    //     fn from(value: GltfAsset) -> Self {
    //         let path = match value {
    //             GltfAsset::Developer => "developer.glb#Scene0",
    //             GltfAsset::CraneYard => "crane_yard.glb#Scene0",
    //             GltfAsset::ResourceNode => "resource_node.glb#Scene0",
    //             GltfAsset::ResourcePlatformClaimed => "resource_platform_claimed.glb#Scene0",
    //             GltfAsset::ResourcePlatformUnclaimed => "resource_platform_unclaimed.glb#Scene0",
    //             GltfAsset::Factory => "factory.glb#Scene0",
    //             GltfAsset::MarineSquad => "marine_squad.glb#Scene0",
    //             GltfAsset::Marine => "marine.glb#Scene0",
    //             GltfAsset::TankBase => "tank_base.glb#Scene0",
    //             GltfAsset::TankGun => "tank_gun.glb#Scene0",
    //         };
    //         format!("models/{}", path).into()
    //     }
    // }
}