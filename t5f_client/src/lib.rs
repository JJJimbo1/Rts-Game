pub use bevy::prelude::*;
pub use bevy_ninepatch::*;
pub use chrono::Local;
pub use t5f_common::*;


pub mod assets;
pub mod ui;
pub mod resources;
pub mod states;
pub mod systems;
pub mod plugins;
pub mod utility;

pub use assets::*;
pub use ui::*;
pub use resources::*;
pub use states::*;
pub use systems::*;
pub use plugins::*;
pub use utility::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[derive(SystemSet)]
pub enum SystemSets {
    MainMenuUi,
    Camera,
}

#[derive(Debug, Clone)]
#[derive(Resource)]
pub struct FPSCounter{
    pub timer : Timer,
    pub frames : u32,
    pub frames_total : u64,
}