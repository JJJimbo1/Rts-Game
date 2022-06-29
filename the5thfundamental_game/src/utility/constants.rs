pub use constants::*;
mod constants {
    use std::path::PathBuf;

    use bevy::prelude::Color;
    use lazy_static::lazy_static;
    use the5thfundamental_common::TeamPlayer;

    ////!Environment---!\\\\

    lazy_static! {
        #[derive(Debug, Copy, Clone)]
        pub static ref PROJECT_ROOT_DIRECTORY : String = std::env::current_dir().unwrap().to_string_lossy().to_string();
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
}