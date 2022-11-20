pub mod cleanup;
pub mod combat;
pub mod physics;
pub mod production;

pub use cleanup::*;
pub use combat::*;
pub use physics::*;
pub use production::*;

use bevy::prelude::PluginGroup;

pub struct CommonPlugins;

impl PluginGroup for CommonPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = bevy::app::PluginGroupBuilder::start::<CommonPlugins>();
        group
            .add(ProductionPlugin)
            .add(CombatPlugin)
    }
}