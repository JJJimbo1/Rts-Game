use bevy::prelude::Component;
use serde::{Serialize, Deserialize};
use crate::*;

const MIN_VALUE : f32 = -9.0;
const MAX_VALUE : f32 = 0.9;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct Health {
    max_health : f32,
    health : f32,
    resistance : f32,
    resistances : DamageTypes,
}

impl Health {
    pub fn new(max : f32, resistances : DamageTypes, resistance : f32) -> Self {
        Self {
            max_health : max,
            health : max,
            resistance,
            resistances,
        }
    }

    pub fn max_health(&self) -> f32 {
        self.max_health
    }

    pub fn health(&self) -> f32 {
        self.health
    }

    pub fn health_percent(&self) -> f32 {
        self.health / self.max_health
    }

    pub fn is_full_health(&self) -> bool {
        self.health_percent() > 0.9999
    }

    pub fn is_not_full_health(&self) -> bool {
        !self.is_full_health()
    }

    pub fn damage(&mut self, damage : f32, dmg_types : DamageTypes) {
        self.health -= (damage * dmg_types.kinetic - (damage * dmg_types.kinetic * (self.resistances.kinetic + self.resistance).clamp(MIN_VALUE, MAX_VALUE)))
            + (damage * dmg_types.fire - (damage * dmg_types.fire * (self.resistances.fire + self.resistance).clamp(MIN_VALUE, MAX_VALUE)))
            + (damage * dmg_types.energy - (damage * dmg_types.energy * (self.resistances.energy + self.resistance).clamp(MIN_VALUE, MAX_VALUE)))
            + (damage * dmg_types.sonic - (damage * dmg_types.sonic * (self.resistances.sonic + self.resistance).clamp(MIN_VALUE, MAX_VALUE)))
            + (damage * dmg_types.explosive - (damage * dmg_types.explosive * (self.resistances.explosive + self.resistance).clamp(MIN_VALUE, MAX_VALUE)))
            + (damage * dmg_types.shock - (damage * dmg_types.shock * (self.resistances.shock + self.resistance).clamp(MIN_VALUE, MAX_VALUE)))
            + (damage * dmg_types.radioactivity - (damage * dmg_types.radioactivity * (self.resistances.radioactivity + self.resistance).clamp(MIN_VALUE, MAX_VALUE)))
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }
}