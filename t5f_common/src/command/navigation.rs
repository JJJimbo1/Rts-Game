use bevy::prelude::{Entity, Component};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Navigator {
    pub max_forward_speed : f32,
    pub max_backwards_speed : f32,
    pub pursue: Option<Entity>,
}