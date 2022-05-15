// #![windows_subsystem = "windows"]

use std::{
    fs::{
        OpenOptions
    },
    io::Write,
    path::{
        PathBuf,
    },
};

use bevy::{diagnostic::DiagnosticsPlugin, prelude::*};
use bevy_ninepatch::*;
use bevy_pathfinding::{PathFindingPlugin, DefaultPather};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_rapier3d::plugin::{RapierPhysicsPlugin, NoUserData};
use chrono::Local;
use lazy_static::__Deref;
use the5thfundamental_common::*;
use simple_random::*;

mod ui;
mod resources;
mod settings;
mod states;
mod systems;
mod utility;

use ui::*;
use resources::*;
use settings::*;
use states::*;
use systems::*;
use utility::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameState {
    Loading,
    MainMenu,
    SingleplayerGame,
    MultiplayerGame,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum SystemSets {
    MainMenuUi,
    Camera,
}

pub struct FPSCounter{
    pub timer : Timer,
    pub frames : u32,
    pub frames_total : u64,
}

pub fn main() {
    // if (*PROJECT_ROOT_DIRECTORY).is_err() {
    //     log::error!("No root folder found. Game cannot start.");
    //     crash();
    //     return;
    // }
    // if !std::path::Path::new(ASSET_DIRECTORY.deref()).is_dir() {
    //     log::error!("No asset folder found. Game cannot start.");
    //     crash();
    //     return;
    // }
    println!("ROOT: {:?}", *PROJECT_ROOT_DIRECTORY);

    begin_log();
    play_game();
    end_log();
    crash();
}

fn begin_log() {
    let path = PathBuf::from(format!("{}\\log.txt", *PROJECT_ROOT_DIRECTORY));
    let mut _f = match OpenOptions::new()
        .write(true)
        .open(path.clone()) {
            Ok(mut x) => {
                match x.set_len(0) {
                    Ok(_) => { },
                    Err(e) => { log::error!("{}", e); }
                }
                match x.write(Local::now().format("Game started: %b %e, %Y, %H:%M:%S,\n================================\n").to_string().as_bytes()) {
                    Ok(_) => { },
                    Err(e) => { log::error!("{}", e); }
                }
            },
            Err(e) => { log::error!("{}", e); }
        };
}



fn play_game() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            title: "The 5th Fundamental".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(UiHit::<CLICK_BUFFER>{ hitting : [false; CLICK_BUFFER], holding : false, })
        .insert_resource(FPSCounter{
            timer : Timer::from_seconds(0.25, true),
            frames : 0,
            frames_total : 0,
        })

        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(PhysicsPlugin)
        .add_plugin(DiagnosticsPlugin::default())
        .add_plugin(NinePatchPlugin::<()>::default())
        .add_plugin(DebugLinesPlugin::with_depth_test(false))
        .add_plugin(PathFindingPlugin::<DefaultPather>::default())

        .add_event::<SelectionEvent>()

        .add_event::<TopMenuButtons>()
        .add_event::<CampaignButtons>()
        .add_event::<SkirmishButtons>()
        .add_event::<ContextMenuButtons>()

        .add_event::<MoveCommand>()
        .add_event::<AttackCommand>()

        //TODO: fix physics.
        .insert_resource(Random::<WichmannHill>::seeded(123.456))
        .insert_resource(Identifiers::default())
        .insert_resource(DirtyEntities::default())
        .insert_resource(InitRequests::default())
        // .insert_resource(PhysicsWorld::default())

        .add_system_set(loading_on_enter())
        .add_system_set(loading_on_update())
        .add_system_set(loading_on_exit())

        .add_system_set(main_menu_on_enter())
        .add_system_set(main_menu_on_update())
        .add_system_set(main_menu_on_exit())

        .add_system_set(singleplayer_game_on_enter())
        .add_system_set(camera_setup_system_set(SystemSet::on_enter(GameState::SingleplayerGame)))
        .add_system_set(singleplayer_game_on_update())
        .add_system_set(singleplayer_game_on_exit())

        .add_system(ui_hit_detection_system.label("ui_hit"))
        .add_system_set(camera_system_set(SystemSet::on_update(GameState::SingleplayerGame).after("ui_hit")))
        .add_system_set(misc_system_set(SystemSet::on_update(GameState::SingleplayerGame)))

        .add_system_set(combat_system_set(SystemSet::on_update(GameState::SingleplayerGame)))
        .add_system_set(command_system_set(SystemSet::on_update(GameState::SingleplayerGame)))
        .add_system_set(economy_system_set(SystemSet::on_update(GameState::SingleplayerGame)))
        .add_state(GameState::Loading)

        // .add_startup_system(setup)
        // .add_system(update)

    .run();
}

fn setup(
    mut commands : Commands,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform : Transform::from_translation(Vec3::new(0.0, 5.0, 5.0)).looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    });
}


fn update(
    mut debug : ResMut<DebugLines>,
) {
    debug.line_colored(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 100.0, 0.0), 0.0, Color::Rgba { red: 1.0, green: 0.2, blue: 0.2, alpha: 1.0 });
}

fn end_log() {
    let path = PathBuf::from(format!("{}\\log.txt", *PROJECT_ROOT_DIRECTORY));
    let mut _f = match OpenOptions::new()
        .append(true)
        .open(path.clone()) {
            Ok(mut x) => {
                match x.write(Local::now().format("\n================================\nGame ended: %b %e, %Y, %H:%M:%S").to_string().as_bytes()) {
                    Ok(_) => { },
                    Err(e) => { log::error!("{}", e); }
                }
            },
            Err(e) => { log::error!("{}", e); }
        };
}

fn crash() {
    let mut s = String::new();
    match std::io::stdin().read_line(&mut s) { _ => { } }
}