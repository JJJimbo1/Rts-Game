use bevy::prelude::Entity;






#[derive(Debug, Clone, Copy)]
pub struct ObjectKilledEvent(pub Entity);

pub struct CleanupPlugin;

impl CleanupPlugin {

}