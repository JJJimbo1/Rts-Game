pub use bevy::prelude::*;
pub use chrono::Local;
pub use t5f_common::*;

pub mod client;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;
pub mod ui;
pub mod assets;

pub use client::*;
pub use plugins::*;
pub use resources::*;
pub use states::*;
pub use systems::*;
pub use ui::*;
pub use assets::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[derive(SystemSet)]
pub enum SystemSets {
    MainMenuUi,
    Camera,
}