pub mod controller;
pub mod health;
pub mod queue;
pub mod resource;
pub mod select;
pub mod teamplayer;
pub mod weapon;

pub use controller::*;
pub use health::*;
pub use queue::*;
pub use resource::*;
pub use select::*;
pub use teamplayer::*;
pub use weapon::*;

use bevy::prelude::{Component, Vec2};
use bevy_pathfinding::PathFinder;

#[derive(Debug, Default, Clone, Copy)]
#[derive(Component)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GroundPathFinder {
    pub start: Vec2,
    pub end: Vec2,
}

impl PathFinder for GroundPathFinder {
    fn start(&self) -> Vec2 {
        self.start
    }
    fn end(&self) -> Vec2 {
        self.end
    }
}