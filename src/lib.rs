pub mod base;
pub mod camera;
pub mod client;
pub mod combat;
pub mod command;
pub mod disk;
pub mod net;
pub mod production;
pub mod physics;
pub mod ui;
pub mod utility;

pub use base::*;
pub use camera::*;
pub use client::*;
pub use combat::*;
pub use command::*;
pub use disk::*;
pub use net::*;
pub use production::*;
pub use physics::*;
pub use ui::*;
pub use utility::*;

use bevy::{app::PluginGroupBuilder, prelude::*};

#[derive(Debug, Copy, Clone, Resource)]
pub struct LocalPlayer(pub TeamPlayer);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Loading,
    MainMenu,
    CustomGame,
    MatchLoadingState,
    SingleplayerGame,
    MultiplayerGame,
}

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<GamePlugins>()
            .add(CommandPlugin)
            .add(ProductionPlugin)
            .add(CombatPlugin)
            .add(PhysicsPlugin)
            .add(SaveLoadPlugin)
    }
}