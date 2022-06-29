pub use economy::*;
mod economy {

    use bevy::prelude::Component;
    use serde::{Serialize, Deserialize};
    use crate::*;

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    #[derive(Component)]
    pub struct ResourceProvider {
        pub strength : f64,
    }
    
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct Economy {
        resources : f64,
        settings : EconomySettings,
    }

    impl Economy {
        pub fn resources(&self) -> f64 {
            self.resources
        }

        pub fn can_afford(&self, cost: f64) -> bool {
            self.resources() > cost
        }

        pub fn add_resources(&mut self, stat : (u32, f64)) {

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

        pub fn remove_resources(&mut self, amount : f64) -> bool {
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
                resources : DEFAULT_STARTING_MONEY,
                settings : EconomySettings::default(),
            }
        }
    }
}