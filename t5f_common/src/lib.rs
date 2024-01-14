pub mod combat;
pub mod command;
pub mod production;
pub mod identifier;
pub mod physics;
pub mod saveload;

pub use combat::*;
pub use command::*;
pub use production::*;
pub use identifier::*;
pub use physics::*;
pub use saveload::*;

use bevy::{ecs::{schedule::{SystemSet, States}, component::Component}, app::PluginGroup};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Loading,
    MainMenu,
    MatchLoadingState,
    // MatchLoadingState(String),
    SingleplayerGame,
    MultiplayerGame,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct DeleteOnStateChange;
#[derive(Debug, Clone, Copy, Component)]
pub struct DontDeleteOnStateChange;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum CommonSystemSets {
    Combat,
    Command,
    Economy,
    Physics,
}

pub struct CommonPlugins;

impl PluginGroup for CommonPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = bevy::app::PluginGroupBuilder::start::<CommonPlugins>();
        group
            .add(CommandPlugin)
            .add(ProductionPlugin)
            .add(CombatPlugin)
            .add(PhysicsPlugin)
            .add(SaveLoadPlugin)
    }
}