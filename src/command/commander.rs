use bevy::{prelude::Resource, platform::collections::HashMap};
use serde::{Serialize, Deserialize};


use crate::{TeamPlayer, EconomySettings, DEFAULT_STARTING_MONEY};


#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Resource)]
pub struct Commanders {
    pub commanders: HashMap<TeamPlayer, Commander>,
}

impl Commanders {
    pub fn new() -> Self {
        Self {
            commanders: HashMap::new(),
        }
    }

    pub fn reset_ratings(&mut self) {
        for a in self.commanders.values_mut() {
            a.rating.reset();
        }
    }
}


#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct Commander {
    pub commander_type: CommanderType,
    pub rating: Rating,
    pub economy: Economy,
}

impl Commander {
    pub fn new(actor_type: CommanderType) -> Self {
        Self {
            commander_type: actor_type,
            rating: Rating::default(),
            economy: Economy::default(),
        }
    }

    pub fn new_player() -> Self {
        Self {
            commander_type: CommanderType::Player,
            rating: Rating::default(),
            economy: Economy::default(),
        }
    }

    pub fn new_ai(difficulty: AIDifficulty, settings: AISettings) -> Self {
        Self {
            commander_type: CommanderType::AI { difficulty, settings },
            rating: Rating::default(),
            economy: Economy::default(),
        }
    }

}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum CommanderType {
    AI{difficulty: AIDifficulty, settings: AISettings},
    Player,
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum AIDifficulty {
    Easy,
    Normal,
    Hard,
    // Brutal, WurtziteBoronNitride, Custom
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct AISettings {
    dynamic: Option<(f32, f32)>,
}

#[derive(Debug, Default, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct Rating {
    pub economy_score: f64,
    pub production_score: f64,
    pub power_score: f64,
}

impl Rating {
    pub fn reset(&mut self) {
        self.economy_score = 0.0;
        self.production_score = 0.0;
        self.power_score = 0.0;
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct Economy {
    resources: f64,
    settings: EconomySettings,
}

impl Economy {
    pub fn resources(&self) -> f64 {
        self.resources
    }

    pub fn can_afford(&self, cost: f64) -> bool {
        self.resources() > cost
    }

    pub fn add_resources(&mut self, stat: (u32, f64)) {

        let amount = {
            if stat.0 <= self.settings.tipping_point {
                stat.1 + self.settings.ecocore_value() * stat.0 as f64
            } else {
                let avg = stat.1 / stat.0 as f64;
                avg * self.settings.tipping_point as f64 + self.settings.ecocore_value() * stat.0 as f64 + ((stat.0 - self.settings.tipping_point) as f64 * avg).powf(self.settings.dim_rate) + self.settings.ecocore_value() * stat.0 as f64
            }
        };

        self.resources += amount.abs();

    }

    pub fn remove_resources(&mut self, amount: f64) -> bool {
        if self.resources > amount {
            self.resources -= amount;
            return true;
        }
        return false;
    }
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            resources: DEFAULT_STARTING_MONEY,
            settings: EconomySettings::default(),
        }
    }
}
