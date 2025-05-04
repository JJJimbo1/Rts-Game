use bevy::prelude::*;

use crate::ClientUIPlugin;

pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = bevy::app::PluginGroupBuilder::start::<ClientPlugins>();
        group
            .add(ClientUIPlugin)
    }
}