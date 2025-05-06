use bevy::{image::ImageSamplerDescriptor, prelude::*, window::PresentMode};
use crate::*;

// #![windows_subsystem = "windows"]

#[cfg(not(target_family = "wasm"))]
const ASSET_PATH: &str = "assets";
#[cfg(target_family = "wasm")]
const ASSET_PATH: &str = "the5thfundamental_game/assets";

pub fn client() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(UiHit::<CLICK_BUFFER>{ hitting: [false; CLICK_BUFFER], holding: false, })
        .insert_resource(Random::<WichmannHill>::seeded(123.456))

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920.0, 1080.0).into(),
                title: "T5f".to_string(),
                present_mode: PresentMode::Immediate,
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }).set(AssetPlugin {
                file_path: ASSET_PATH.to_string(),
                ..default()
            })
        )

        .add_plugins((
            GamePlugins,
            BasePlugins,
            ClientLoadingPlugin,
            ClientUIPlugins,
            ClientPlugin,
        // ))
        // .add_plugins((

            LoadingStatePlugin,
            MatchLoadingStatePlugin,
            SinglePlayerGamePlugin,

            CameraPlugin,

        ))

        .add_event::<ContextMenuButtonsEvent>()

        .init_state::<GameState>()

    .run();
}