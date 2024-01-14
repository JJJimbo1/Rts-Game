use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Snowflake(pub Uuid);

impl Snowflake {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}