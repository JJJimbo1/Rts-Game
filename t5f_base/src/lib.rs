pub mod levels;
pub mod maps;
pub mod objects;
pub mod persistence;
pub mod client;
pub mod base_loading;
pub mod error;

pub use levels::*;
pub use maps::*;
pub use objects::*;
pub use persistence::*;
pub use client::*;
pub use base_loading::*;
pub use error::*;
use t5f_common::GameState;

use std::fmt::Display;
use bevy::prelude::*;

pub static MOD_LABEL: &'static str = "base";

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub enum AssetType {
    Map(MapType),
    Object(ObjectType),
}

impl Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Map(map) => map.fmt(f),
            Self::Object(object) => object.fmt(f)
        }
    }
}

pub struct BasePlugins;

impl PluginGroup for BasePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = bevy::app::PluginGroupBuilder::start::<BasePlugins>();
        let group = group
            .add(BaseLoadingPlugin)
            .add(LevelPlugin)
            .add(MapPlugin)
            .add(ObjectPlugin)
            .add(PersistencePlugin);

        // #[cfg(client)]
        let group = group.add(BaseClientPlugin);

        group
    }
}