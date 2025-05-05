use bevy::{image::ImageSamplerDescriptor, prelude::*, window::PresentMode};
use bevy_asset_loader::loading_state::{LoadingStateAppExt, LoadingState};
use crate::*;

// #![windows_subsystem = "windows"]

#[cfg(not(target_family = "wasm"))]
const ASSET_PATH: &str = "assets";
#[cfg(target_family = "wasm")]
const ASSET_PATH: &str = "the5thfundamental_game/assets";

pub fn client() {

    println!();

    App::new()
        // .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(UiHit::<CLICK_BUFFER>{ hitting : [false; CLICK_BUFFER], holding : false, })
        .insert_resource(Random::<WichmannHill>::seeded(123.456))
        .insert_resource(Identifiers::default())

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920.0, 1080.0).into(),
                title: "untitled rts game".to_string(),
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

        .add_loading_state(LoadingState::new(GameState::AssetLoading)
            .continue_to_state(GameState::Loading)
        )

        .add_plugins((

            CommonPlugins,
            BasePlugins,
            ClientLoadingPlugin,
            ClientUIPlugins,
            ClientPlugin,

            DebugPlugin,
        ))
        .add_plugins((

            LoadingStatePlugin,
            MainMenuPlugin,
            // CustomGamePlugin,
            MatchLoadingStatePlugin,
            SinglePlayerGamePlugin,

            CameraPlugin,

        ))

        .add_event::<TopMenuButtonsEvent>()
        .add_event::<CampaignButtonsEvent>()
        .add_event::<SkirmishButtonsEvent>()
        .add_event::<ContextMenuButtonsEvent>()

        .init_state::<GameState>()

    .run();
}

fn _crash() {
    let mut s = String::new();
    match std::io::stdin().read_line(&mut s) { _ => { } }
}
