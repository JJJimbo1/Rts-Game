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
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(ProductionPlugin)
            .add(CombatPlugin)
        ;
    }
}