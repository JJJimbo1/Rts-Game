use bevy::{utils::Uuid, prelude::*,};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Snowflake {
    entity: Option<Entity>,
    uuid: Option<Uuid>,
}

impl Snowflake {
    pub fn new() -> Self {
        Self {
            entity: None,
            uuid: Some(Uuid::new_v4()),
        }
    }

    pub fn new_with_entity(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            uuid: Some(Uuid::new_v4()),
        }
    }

    pub fn entity(&self) -> Option<Entity> {
        self.entity
    }

    pub fn uuid(&self) -> Option<Uuid> {
        self.uuid
    }
}

impl From<(Entity, Uuid)> for Snowflake {
    fn from((entity, uuid): (Entity, Uuid)) -> Self {
        Self {
            entity: Some(entity),
            uuid: Some(uuid),
        }
    }
}

impl From<Entity> for Snowflake {
    fn from(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            uuid: None,
        }
    }
}

impl From<Uuid> for Snowflake {
    fn from(uuid: Uuid) -> Self {
        Self {
            entity: None,
            uuid: Some(uuid),
        }
    }
}