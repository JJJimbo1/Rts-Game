use bevy::prelude::*;
use crate::pathing::pathing2d::GridMap;
use crate::{Path, GridSpace};

pub trait PathingGridMap {
    fn grid_map(&mut self) -> &mut GridMap;
}

pub trait PathingGridSpace {
    fn grid_space(&mut self) -> &mut GridSpace;
}

pub trait Pather : Sized + Clone {
    fn reset(&mut self) { }
    fn run(&mut self) -> bool { true }
    fn complete(&mut self, _path : &mut Path) { }
}

pub trait PathFinder: Component {
    fn start(&self) -> Vec2;
    fn end(&self) -> Vec2;
}