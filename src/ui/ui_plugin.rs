use bevy::{app::PluginGroupBuilder, prelude::*};
use crate::*;

pub struct ClientUIPlugins;

impl PluginGroup for ClientUIPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<ClientUIPlugins>()
            .add(ContextMenuPlugin)
            .add(DebugUIPlugin)
            .add(GamePlayUIPlugin)
            .add(HealthBarUIPlugin)
            .add(MainMenuPlugin)
    }
}
