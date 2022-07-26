use bevy::prelude::{Entity, Component};
use serde::{Deserialize, Serialize};

use crate::SerdeComponent;


#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Controller {
    pub follow : bool,
    pub max_forward_speed : f32,
    pub max_backwards_speed : f32,
    pub pursuant: Option<Entity>,
}

impl SerdeComponent for Controller {
    fn saved(&self) -> Option<Self> {
        if !self.follow {
            None
        } else {
            Some(*self)
        }
    }
}