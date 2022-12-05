use bevy::prelude::*;


pub mod components;
pub mod plugin;
pub mod systems;
pub mod traits;

pub use components::*;
pub use plugin::*;
pub use systems::*;
pub use traits::*;

use crate::pathing::pathing2d::*;
use bevy::prelude::*;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Resource)]
pub struct OGrid(pub GridMap);

impl PathingGridMap for OGrid {
    fn grid_map(&mut self) -> &mut GridMap {
        &mut self.0
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Resource)]
pub struct GridSpace {
    pub width : usize,
    pub length : usize,
    pub even_offset : Vec2,
    pub offset : Vec2,
    pub scale : Vec2,
}

impl GridSpace {
    pub fn new(width : usize, length : usize) -> Self {
        Self {
            width,
            length,
            even_offset : Vec2::new(if width % 2 == 0 { 0.5 } else { 0.0 }, if length % 2 == 0 { 0.5 } else { 0.0 }),
            ..Default::default()
        }
    }

    pub fn position_to_index(&self, position : Vec2) -> (isize, isize) {
        ((((position.x - self.offset.x - self.even_offset.x) / self.scale.x).round()).clamp(((self.width as isize) / -2) as f32, ((self.width as isize - 1) / 2 ) as f32) as isize,
        (((position.y - self.offset.y - self.even_offset.y) / self.scale.y).round()).clamp(((self.length as isize) / -2) as f32, ((self.length as isize - 1) / 2) as f32) as isize)
    }

    pub fn index_to_position(&self, index : (isize, isize)) -> Vec2 {
        Vec2::new(index.0.clamp(self.width as isize / -2, (self.width as isize - 1) / 2 ) as f32 * self.scale.x + self.offset.x + self.even_offset.x,
        index.1.clamp(self.length as isize / -2, (self.length as isize - 1) / 2) as f32 * self.scale.y + self.offset.y + self.even_offset.y)
    }
}

impl Default for GridSpace {
    fn default() -> Self {
        Self {
            width : 0,
            length : 0,
            offset : Vec2::default(),
            even_offset : Vec2::default(),
            scale : Vec2::new(1.0, 1.0),
        }
    }
}

impl PathingGridSpace for GridSpace {
    fn grid_space(&mut self) -> &mut GridSpace {
        self
    }
}

// pub struct DefaultPathingGridMap(pub GridMap);


#[derive(Debug, Clone, Copy)]
#[derive(Resource)]
pub struct DefaultPather {
    pub budget : u64,
    pub progress : u64,
}

impl DefaultPather {
    pub fn new(budget : u64) -> Self {
        Self {
            budget,
            progress : 0,
        }
    }
}

impl Default for DefaultPather {
    fn default() -> Self {
        Self {
            budget : 5000,
            progress : 0,
        }
    }
}

impl Pather for DefaultPather {
    fn reset(&mut self) {
        self.progress = 0;
    }

    fn run(&mut self) -> bool {
        self.progress < self.budget
    }

    fn complete(&mut self, path : &mut Path) {
        self.progress += path.0.len() as u64
    }
}