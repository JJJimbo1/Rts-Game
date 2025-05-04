use bevy::prelude::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Snowflake {
    uuid: Uuid,
    entity: Option<Entity>,
}

impl Snowflake {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            entity: None,
        }
    }

    pub fn new_with_entity(entity: Entity) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            entity: Some(entity),
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn entity(&self) -> Option<Entity> {
        self.entity
    }
}

impl From<(Uuid, Entity)> for Snowflake {
    fn from((uuid, entity): (Uuid, Entity)) -> Self {
        Self {
            uuid,
            entity: Some(entity),
        }
    }
}

impl From<Entity> for Snowflake {
    fn from(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            uuid: Uuid::new_v4(),
        }
    }
}

impl From<Uuid> for Snowflake {
    fn from(uuid: Uuid) -> Self {
        Self {
            entity: None,
            uuid,
        }
    }
}