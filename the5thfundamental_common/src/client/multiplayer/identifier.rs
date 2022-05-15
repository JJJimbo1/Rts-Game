use bevy::ecs::entity::Entity;
use bimap::BiMap;

use crate::SnowFlake;

pub const SEPARATOR : &str = "||SS||";

#[derive(Debug, Default)]
pub struct Identifiers {
    identifiers : BiMap<SnowFlake, Entity>,
}

impl Identifiers {
    pub fn new() -> Self {
        Self {
            identifiers : BiMap::new(),
        }
    }

    pub fn insert(&mut self, left : SnowFlake, right : Entity) {
        self.identifiers.insert(left, right);
    }

    pub fn remove_by_unique_id(&mut self, left : SnowFlake) {
        self.identifiers.remove_by_left(&left);
    }

    pub fn remove_by_entity(&mut self, right : Entity) {
        self.identifiers.remove_by_right(&right);
    }

    pub fn get_entity(&self, left : SnowFlake) -> Option<Entity> {
        self.identifiers.get_by_left(&left).cloned()
    }

    pub fn get_unique_id(&self, right : Entity) -> Option<SnowFlake> {
        self.identifiers.get_by_right(&right).cloned()
    }
}