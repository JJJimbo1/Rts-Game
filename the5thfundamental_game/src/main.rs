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

use bevy::prelude::*;
use chrono::Local;
use the5thfundamental_game::*;

pub fn main() {
    // console_log::init();
    // log::info!("IT WOIRLKSFASDFASDFSSs!!!!");
    // println!("ROOT: {:?}", *PROJECT_ROOT_DIRECTORY);

    // begin_log();
    #[cfg(target_family = "wasm")]
    play_game("the5thfundamental_game/assets".to_owned());
    #[cfg(not(target_family = "wasm"))]
    play_game("assets".to_owned());
    // end_log();
    // crash();
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

#[allow(unused)]
pub fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera3dBundle{
        ..default()
    });
}
