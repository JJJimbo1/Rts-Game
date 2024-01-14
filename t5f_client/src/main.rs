use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingStateAppExt, LoadingState};
use t5f_base::BasePlugins;
// #![windows_subsystem = "windows"]
use t5f_common::*;
use t5f_client::*;
use t5f_utility::random::{Random, WichmannHill};

#[cfg(not(target_family = "wasm"))]
const ASSET_PATH: &str = "assets";
#[cfg(target_family = "wasm")]
const ASSET_PATH: &str = "the5thfundamental_game/assets";

pub fn main() {

    App::new()
        .add_state::<GameState>()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(UiHit::<CLICK_BUFFER>{ hitting : [false; CLICK_BUFFER], holding : false, })
        .insert_resource(FPSCounter{
            timer : Timer::from_seconds(0.25, TimerMode::Repeating),
            frames : 0,
            frames_total : 0,
        })

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920.0, 1080.0).into(),
                title: "untitled rts game".to_string(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
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

            DebugPlugin,
            NinePatchPlugin::<()>::default(),

        )).add_plugins((


            LoadingStatePlugin,
            MatchLoadingStatePlugin,
            SinglePlayerGamePlugin,

            CameraPlugin,

        ))

        .add_event::<TopMenuButtonsEvent>()
        .add_event::<CampaignButtonsEvent>()
        .add_event::<SkirmishButtonsEvent>()
        .add_event::<ContextMenuButtonsEvent>()

        .insert_resource(Random::<WichmannHill>::seeded(123.456))
        .insert_resource(Identifiers::default())
    .run();
}

fn _crash() {
    let mut s = String::new();
    match std::io::stdin().read_line(&mut s) { _ => { } }
}
