pub mod combat;
pub mod command;
pub mod production;
pub mod multiplayer;
pub mod physics;
pub mod utility;
pub mod content;

pub use combat::*;
pub use command::*;
pub use production::*;
pub use multiplayer::*;
pub use physics::*;
pub use utility::*;
pub use content::*;

use bevy::ecs::schedule::SystemLabel;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CommonSystemSets {
    Combat,
    Command,
    Economy,
    Physics,
}

#[test]
fn atest() {
    println!("Yeet");
}