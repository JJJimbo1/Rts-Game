pub mod actor;
pub mod commands;
pub mod select;

pub use actor::*;
pub use commands::*;
pub use select::*;

use bevy::{math::Vec2, prelude::Component};
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