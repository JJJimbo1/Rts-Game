pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;
pub mod ui;
pub mod assets;

pub use plugins::*;
pub use resources::*;
pub use states::*;
pub use systems::*;
pub use ui::*;
pub use assets::*;

use bevy::ecs::schedule::SystemSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[derive(SystemSet)]
pub enum SystemSets {
    MainMenuUi,
    Camera,
}