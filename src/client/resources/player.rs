
use bevy::prelude::Resource;
use crate::*;

#[derive(Debug, Copy, Clone)]
#[derive(Resource)]
pub struct Player(pub TeamPlayer);