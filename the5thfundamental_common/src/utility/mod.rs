pub mod buffer;
pub mod constants;
pub mod interforce;
pub mod saveload;
pub mod settings;
pub mod serde_structs;

pub use buffer::*;
pub use constants::*;
pub use interforce::*;
pub use saveload::*;
pub use settings::*;
pub use serde_structs::*;

use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, Component)]
pub struct DeleteOnStateChange;