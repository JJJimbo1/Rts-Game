// #![windows_subsystem = "windows"]

use std::{
    fs::{
        OpenOptions
    },
    io::Write,
    path::{
        PathBuf,
    },
    process::Command,
};

use bevy::{diagnostic::DiagnosticsPlugin, prelude::*};
use bevy_ninepatch::*;
use bevy_pathfinding::{PathFindingPlugin, DefaultPather};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_rapier3d::{plugin::{RapierPhysicsPlugin, NoUserData}, prelude::RapierDebugRenderPlugin};
use chrono::Local;
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
    MatchLoadingState,
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

//TODO: Fix context menu.
pub fn main() {
    println!("ROOT: {:?}", *PROJECT_ROOT_DIRECTORY);

    begin_log();
    play_game();
    end_log();
    crash();
}

fn begin_log() {
    let path = PathBuf::from(format!("{}/log.txt", *PROJECT_ROOT_DIRECTORY));
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
            title: "untitled rts game".to_string(),
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

        .add_plugin(CommonPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(DebugPlugin)

        .add_plugin(NinePatchPlugin::<()>::default())
        .add_plugin(SavePlugin)

        .add_event::<SelectionEvent>()
        .add_event::<ActivationEvent>()

        .add_event::<TopMenuButtons>()
        .add_event::<CampaignButtons>()
        .add_event::<SkirmishButtons>()
        .add_event::<ContextMenuButtons>()

        .add_event::<ObjectSpawnEvent>()

        .add_event::<UnitCommand>()
        .add_event::<ObjectKilled>()
        // .add_event::<AttackCommand>()

        .insert_resource(Random::<WichmannHill>::seeded(123.456))
        .insert_resource(Identifiers::default())
        .insert_resource(DirtyEntities::default())
        // .insert_resource(InitRequests::default())
        .insert_resource(Manifest::default())
        // .insert_resource(PhysicsWorld::default())

        .add_system_set(game_loading_state_on_enter_system_set())
        .add_system_set(game_loading_state_on_update_system_set())
        .add_system_set(game_loading_state_on_exit_system_set())

        .add_system_set(main_menu_state_on_enter_system_set())
        .add_system_set(main_menu_state_on_update_system_set())
        .add_system_set(main_menu_state_on_exit_system_set())

        .add_system_set(match_loading_state_on_enter_system_set())
        .add_system_set(match_loading_state_on_update_system_set())
        .add_system_set(match_loading_state_on_exit_system_set())

        .add_system_set(singleplayer_game_state_on_enter_system_set())
        .add_system_set(singleplayer_game_state_on_update_system_set())
        .add_system_set(singleplayer_game_state_on_exit_system_set())

        .add_system_set(camera_setup_system_set(SystemSet::on_enter(GameState::SingleplayerGame)))
        .add_system(ui_hit_detection_system.label("ui_hit"))
        .add_system_set(camera_system_set(SystemSet::on_update(GameState::SingleplayerGame).after("ui_hit")))

        .add_state(GameState::Loading)

    .run();
}

fn end_log() {
    let path = PathBuf::from(format!("{}/log.txt", *PROJECT_ROOT_DIRECTORY));
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


#[test]
fn atest() {
    Command::new("D:/dev/rust/tools/gltf_to_collider").args([
        // "D:/dev/rust/projects/the5thfundamental_bevy/assets/colliders/crane_yard_collider.glb",
        "D:/dev/rust/projects/the5thfundamental_bevy/assets/colliders/resource_platform_collider.glb",
        // "D:/dev/rust/projects/the5thfundamental_bevy/assets/colliders/factory_collider.glb",
        // "D:/dev/rust/projects/the5thfundamental_bevy/assets/colliders/tank_collider.glb",
    ]).spawn().unwrap();
}


