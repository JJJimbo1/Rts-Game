pub use constants::*;
mod constants {
    use std::path::PathBuf;

    use bevy::prelude::Color;
    use lazy_static::lazy_static;
    use the5thfundamental_common::TeamPlayer;

    ////!Environment---!\\\\

    lazy_static! {
        #[derive(Debug, Copy, Clone)]
        // pub static ref PROJECT_ROOT_DIRECTORY : String = "NO".to_string();
        pub static ref PROJECT_ROOT_DIRECTORY : String = std::env::current_dir().unwrap_or(PathBuf::from("")).to_string_lossy().to_string();
        #[derive(Debug, Copy, Clone)]
        pub static ref ASSET_DIRECTORY : String = format!("{}", *PROJECT_ROOT_DIRECTORY);
    }

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
        use bevy::asset::AssetPath;
        use the5thfundamental_common::{AssetType, MapType, ObjectType};

        pub enum FontAsset {
            Roboto
        }

        impl<'a> From<FontAsset> for AssetPath<'a> {
            fn from(value: FontAsset) -> Self {
                let path = match value {
                    FontAsset::Roboto => "roboto/Roboto-Black.ttf"
                };
                format!("fonts/{}", path).into()
            }
        }

        pub enum ImageAsset {
            BlackBox,
            WhiteBox,
            WhiteDottedBox,
            GoldDottedBox,
            HealthBarStart,
            HealthBarMiddle,
            HealthBarEnd,
            HealthBarGreen,
            SelectionBox,
        }

        impl<'a> From<ImageAsset> for AssetPath<'a> {
            fn from(value: ImageAsset) -> Self {
                let path = match value {
                    ImageAsset::BlackBox => "black_box.png",
                    ImageAsset::WhiteBox => "white_box.png",
                    ImageAsset::WhiteDottedBox => "white_dotted_box.png",
                    ImageAsset::GoldDottedBox => "gold_dotted_box.png",
                    ImageAsset::HealthBarStart => "health_bar_start.png",
                    ImageAsset::HealthBarMiddle => "health_bar_middle.png",
                    ImageAsset::HealthBarEnd => "health_bar_end.png",
                    ImageAsset::HealthBarGreen => "health_bar_green.png",
                    ImageAsset::SelectionBox => "selection_box.png",
                };
                format!("textures/ui/{}", path).into()
            }
        }

        pub enum GltfAsset {


            Developer,

            CraneYard,
            ResourceNode,
            ResourcePlatformClaimed,
            ResourcePlatformUnclaimed,
            Factory,
            MarineSquad,
            Marine,
            TankBase,
            TankGun

        }

        impl From<AssetType> for GltfAsset {
            fn from(value: AssetType) -> Self {
                match value {
                    AssetType::Map(map) => {
                        match map {
                            MapType::Developer => GltfAsset::Developer,
                        }
                    },
                    AssetType::Object(object) => {
                        match object {
                            ObjectType::CraneYard => GltfAsset::CraneYard,
                            ObjectType::ResourceNode => GltfAsset::ResourceNode,
                            ObjectType::ResourcePlatformClaimed => GltfAsset::ResourcePlatformClaimed,
                            ObjectType::ResourcePlatformUnclaimed => GltfAsset::ResourcePlatformUnclaimed,
                            ObjectType::Factory => GltfAsset::Factory,
                            ObjectType::MarineSquad => GltfAsset::MarineSquad,
                            ObjectType::Marine => GltfAsset::Marine,
                            ObjectType::TankBase => GltfAsset::TankBase,
                            ObjectType::TankGun => GltfAsset::TankGun,
                        }
                    }
                }
            }
        }

        impl<'a> From<GltfAsset> for AssetPath<'a> {
            fn from(value: GltfAsset) -> Self {
                let path = match value {
                    GltfAsset::Developer => "developer.glb#Scene0",
                    GltfAsset::CraneYard => "crane_yard.glb#Scene0",
                    GltfAsset::ResourceNode => "resource_node.glb#Scene0",
                    GltfAsset::ResourcePlatformClaimed => "resource_platform_claimed.glb#Scene0",
                    GltfAsset::ResourcePlatformUnclaimed => "resource_platform_unclaimed.glb#Scene0",
                    GltfAsset::Factory => "factory.glb#Scene0",
                    GltfAsset::MarineSquad => "marine_squad.glb#Scene0",
                    GltfAsset::Marine => "marine.glb#Scene0",
                    GltfAsset::TankBase => "tank_base.glb#Scene0",
                    GltfAsset::TankGun => "tank_gun.glb#Scene0",
                };
                format!("models/{}", path).into()
            }
        }
    }
}