pub mod base;
pub mod client;
pub mod client_m;
pub mod combat;
pub mod command;
pub mod disk;
pub mod net;
pub mod server_m;
pub mod production;
pub mod physics;
pub mod utility;
pub mod mainmenu;

pub use base::*;
pub use client::*;
pub use client_m::*;
pub use combat::*;
pub use command::*;
pub use disk::*;
pub use net::*;
pub use server_m::*;
pub use production::*;
pub use physics::*;
pub use utility::*;
pub use mainmenu::*;

use std::marker::PhantomData;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Loading,
    MainMenu,
    CustomGame,
    MatchLoadingState,
    // MatchLoadingState(String),
    SingleplayerGame,
    MultiplayerGame,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct DeleteOnStateChange;
#[derive(Debug, Clone, Copy, Component)]
pub struct DontDeleteOnStateChange;



#[derive(Debug, Clone, Copy, Component)]
pub struct OptOut<T: OptOutSytem>(PhantomData<T>);

#[derive(Debug, Clone, Copy, Component)]
pub struct Navigation;

impl OptOutSytem for Navigation { }

#[derive(Debug, Clone, Copy, Component)]
pub struct Targeting;

impl OptOutSytem for Targeting { }

pub trait OptOutSytem { }



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