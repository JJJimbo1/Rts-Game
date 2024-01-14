use std::time::Duration;
use serde::{Serialize, Deserialize};

pub static DEFAULT_COST : u128 = 100;
pub static DEFAULT_BUILD_TIME : Duration = Duration::from_secs(1);
pub static DEFAULT_POWER_GEN : u32 = 0;
pub static DEFAULT_POWER_DRAIN : u32 = 0;

pub static DEFAULT_STARTING_MONEY : f64 = 5000.0;
pub static DEFAULT_TIPPING_POINT : u32 = 4;
pub static DEFAULT_DIM_SEVERITY : f64 = 0.9;
pub static DEFAULT_ECOCORE_VALUE : f64 = 4.0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EconomySettings {
    pub tipping_point : u32,
    pub dim_rate : f64,
    pub economy_core : bool,
    pub eco_value : f64,
}

impl EconomySettings {
    pub fn ecocore_value(&self) -> f64 {
        if self.economy_core {
            self.eco_value
        } else {
            0.0
        }
    }
}

impl Default for EconomySettings {
    fn default() -> Self {
        Self {
            tipping_point : DEFAULT_TIPPING_POINT,
            dim_rate : DEFAULT_DIM_SEVERITY,
            economy_core : false,
            eco_value : DEFAULT_ECOCORE_VALUE
        }
    }
}