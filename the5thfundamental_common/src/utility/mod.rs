pub mod buffer;
pub mod constants;
pub mod decode_collider;
pub mod interforce;
pub mod manifest;
pub mod saveload;
pub mod settings;
pub mod serde_structs;

pub use buffer::*;
pub use constants::*;
pub use decode_collider::*;
pub use interforce::*;
pub use manifest::*;
pub use saveload::*;
pub use settings::*;
pub use serde_structs::*;

use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, Component)]
pub struct DeleteOnStateChange;
#[derive(Debug, Clone, Copy, Component)]
pub struct DontDeleteOnStateChange;