pub mod loading_state;
pub mod main_menu_state;
pub mod singleplayer_game;
// pub mod multiplayer_state;
// pub mod error_state;


pub use self::loading_state::loading_state::*;
pub use self::main_menu_state::main_menu_state::*;
pub use self::singleplayer_game::game_state::*;
// pub use multiplayer_state::multiplayer_state::*;
// pub use error_state::*;

use bevy::prelude::*;
use the5thfundamental_common::DeleteOnStateChange;

pub fn cleanup_entities(
    entities : Query<Entity, With<DeleteOnStateChange>>,
    mut commands : Commands,
) {
    entities.for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}