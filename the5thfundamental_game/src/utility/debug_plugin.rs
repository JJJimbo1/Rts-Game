use bevy::{prelude::*, diagnostic::DiagnosticsPlugin};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier3d::prelude::RapierDebugRenderPlugin;



#[derive(Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_plugin(RapierDebugRenderPlugin::default())
            // .add_plugin(DiagnosticsPlugin::default())
            .add_plugin(DebugLinesPlugin::with_depth_test(false))

        ;
    }
}
