
use bevy::prelude::Resource;
use t5f_common::TeamPlayer;

#[derive(Debug, Copy, Clone)]
#[derive(Resource)]
pub struct Player(pub TeamPlayer);