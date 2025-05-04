pub mod loading_state;
pub mod main_menu_state;
pub mod match_loading_state;
pub mod singleplayer_game;

pub use loading_state::*;
pub use main_menu_state::*;
pub use match_loading_state::*;
pub use singleplayer_game::*;

use bevy::prelude::*;
use t5f_common::{DeleteOnStateChange, DontDeleteOnStateChange};

pub fn cleanup_entities(
    entities : Query<Entity, (With<DeleteOnStateChange>, Without<DontDeleteOnStateChange>)>,
    mut commands : Commands,
) {
    entities.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}