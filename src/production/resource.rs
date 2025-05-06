use bevy::prelude::Component;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct EconomicObject {
    //Money
    pub resource_gen: f64,
    pub resource_drain: f64,
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct PowerObject {
    pub power_gen: u32,
    pub power_drain: u32,
}