use bevy::{ecs::entity::Entity, prelude::Resource};
use bimap::BiMap;

use crate::Snowflake;

pub const SEPARATOR : &str = "||SS||";

#[derive(Debug, Default)]
#[derive(Resource)]
pub struct Identifiers {
    identifiers : BiMap<Snowflake, Entity>,
}

impl Identifiers {
    pub fn new() -> Self {
        Self {
            identifiers : BiMap::new(),
        }
    }

    pub fn insert(&mut self, left : Snowflake, right : Entity) {
        self.identifiers.insert(left, right);
    }

    pub fn remove_by_unique_id(&mut self, left : Snowflake) {
        self.identifiers.remove_by_left(&left);
    }

    pub fn remove_by_entity(&mut self, right : Entity) {
        self.identifiers.remove_by_right(&right);
    }

    pub fn get_entity(&self, left : Snowflake) -> Option<Entity> {
        self.identifiers.get_by_left(&left).cloned()
    }

    pub fn get_unique_id(&self, right : Entity) -> Option<Snowflake> {
        self.identifiers.get_by_right(&right).cloned()
    }
}