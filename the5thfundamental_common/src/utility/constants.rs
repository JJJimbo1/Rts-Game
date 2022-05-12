pub use constants::*;
mod constants {
    use std::time::Duration;

    pub const SMALL_BUFFER_SIZE : usize = 24;
    pub const MEDIUM_BUFFER_SIZE : usize = 128;
    pub const LARGE_BUFFER_SIZE : usize = 512;

    pub static DEFAULT_COST : u128 = 100;
    pub static DEFAULT_BUILD_TIME : Duration = Duration::from_secs(1);
    pub static DEFAULT_POWER_GEN : u32 = 0;
    pub static DEFAULT_POWER_DRAIN : u32 = 0;

    pub static DEFAULT_STARTING_MONEY : f64 = 5000.0;
    pub static DEFAULT_TIPPING_POINT : u32 = 4;
    pub static DEFAULT_DIM_SEVERITY : f64 = 0.9;
    pub static DEFAULT_ECOCORE_VALUE : f64 = 4.0;
}