pub mod buffer;
pub mod constants;
pub mod saveload;
pub mod settings;

pub use buffer::*;
pub use constants::*;
pub use saveload::*;
pub use settings::*;

use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, Component)]
pub struct DeleteOnStateChange;