
use bevy::prelude::Resource;
use the5thfundamental_common::TeamPlayer;

#[derive(Debug, Copy, Clone)]
#[derive(Resource)]
pub struct Player(pub TeamPlayer);