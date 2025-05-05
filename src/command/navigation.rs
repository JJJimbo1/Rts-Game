use bevy::prelude::{Entity, Component};
use serde::{Deserialize, Serialize};

use crate::Slim;

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Navigator {
    pub max_forward_speed : f32,
    pub max_backwards_speed : f32,
    pub pursue: Option<Entity>,
}

impl Slim for Navigator {
    fn slim(&self) -> Option<Self> {
        self.pursue.is_some().then_some(*self)
    }
}