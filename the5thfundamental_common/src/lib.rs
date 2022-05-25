pub mod client;
pub mod common_core;
pub mod utility;

pub use client::*;
pub use common_core::*;
pub use utility::*;

use bevy::ecs::schedule::SystemLabel;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CommonSystemSets {
    Combat,
    Command,
    Economy,
    Physics,
}