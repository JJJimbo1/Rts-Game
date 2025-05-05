use bevy::{ecs::entity::Entity, prelude::Resource};
use bimap::BiMap;

use crate::snowflake::Snowflake;

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

    pub fn insert(&mut self, uuid : Snowflake, entity : Entity) {
        self.identifiers.insert(uuid, entity);
    }

    pub fn remove_by_uuid(&mut self, uuid : Snowflake) {
        self.identifiers.remove_by_left(&uuid);
    }

    pub fn remove_by_entity(&mut self, entity : Entity) {
        self.identifiers.remove_by_right(&entity);
    }

    pub fn get_entity(&self, uuid : Snowflake) -> Option<Entity> {
        self.identifiers.get_by_left(&uuid).cloned()
    }

    pub fn get_uuid(&self, entity : Entity) -> Option<Snowflake> {
        self.identifiers.get_by_right(&entity).cloned()
    }
}