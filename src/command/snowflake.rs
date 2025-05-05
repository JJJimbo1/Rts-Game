use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Slim;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Component)]
pub struct Snowflake(pub Uuid);

impl Snowflake {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Slim for Snowflake {
    fn slim(&self) -> Option<Self> {
        Some(self.clone())
    }
}