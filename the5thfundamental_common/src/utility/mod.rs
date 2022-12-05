pub mod buffer;
pub mod constants;
pub mod decode_collider;
pub mod manifest;
pub mod mathfu;
pub mod random;
pub mod saveload;
pub mod serde_structs;
pub mod pathing;
pub mod zipqueue;

pub use buffer::*;
pub use constants::*;
pub use decode_collider::*;
pub use manifest::*;
pub use mathfu::*;
pub use random::*;
pub use saveload::*;
pub use serde_structs::*;
pub use pathing::*;
pub use zipqueue::*;

use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, Component)]
pub struct DeleteOnStateChange;
#[derive(Debug, Clone, Copy, Component)]
pub struct DontDeleteOnStateChange;