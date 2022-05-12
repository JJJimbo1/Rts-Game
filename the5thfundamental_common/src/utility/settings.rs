pub use settings::*;
mod settings {
    use serde::{Serialize, Deserialize};
    use crate::{DEFAULT_DIM_SEVERITY, DEFAULT_ECOCORE_VALUE, DEFAULT_TIPPING_POINT};

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
}