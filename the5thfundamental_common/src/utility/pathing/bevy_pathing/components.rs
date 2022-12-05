use bevy::prelude::*;

#[derive(Debug, Default, Clone)]
#[derive(Component)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Path(pub Vec<Vec2>);