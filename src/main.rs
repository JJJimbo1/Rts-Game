use clap::*;
use bevy::{image::ImageSamplerDescriptor, prelude::*, window::PresentMode};
use t5f::*;

// #![windows_subsystem = "windows"]

#[cfg(not(target_family = "wasm"))]
const ASSET_PATH: &str = "assets";
#[cfg(target_family = "wasm")]
const ASSET_PATH: &str = "the5thfundamental_game/assets";

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Args {
    #[command(subcommand)]
    mode: Mode
}

#[derive(Subcommand, Debug, Clone)]
enum Mode {
    #[clap(short_flag('c'))]
    Client,
    #[clap(short_flag('s'))]
    Server,
    #[clap(short_flag('a'))]
    Asset {
        path: String,
    }
}

pub fn main() {
    match Args::try_parse() {
        Ok(arg) => {
            match arg.mode {
                Mode::Client => client(),
                Mode::Server => server(),
                Mode::Asset {path, } => asset(path),
            };
        },
        Err(_) => client(),
    };
    // if Args::parse().server { server(); } else { client(); }
}

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

            LoadingStatePlugin,
            MatchLoadingStatePlugin,
            SinglePlayerGamePlugin,

            CameraPlugin,

        ))

        .add_event::<ContextMenuButtonsEvent>()

        .init_state::<GameState>()

    .run();
}

pub fn server() {
    App::new()

    .add_plugins((MinimalPlugins, ServerPlugin))
    .run();
}

pub fn asset(path: String) {
    let trimesh = extract_trimesh(format!("{}/assets/{}", std::env::current_dir().unwrap().to_str().unwrap(), path)).unwrap();
    let code = encode(trimesh).unwrap();
    println!("{}", code);
}