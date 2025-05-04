use serde::{Serialize, Deserialize};

use bevy::prelude::*;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Reference {
    pub references: Vec<(Option<Transform>, Option<Entity>)>,
}

impl Reference {
    pub fn push(&mut self, transform: Transform, entity: Option<Entity>) {
        self.references.push((Some(transform), entity));
    }
}