use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierDebugRenderPlugin;



#[derive(Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_plugins(RapierDebugRenderPlugin::default())
            // .add_plugin(DiagnosticsPlugin::default())
            // .add_plugin(DebugLinesPlugin::with_depth_test(false))

        ;
    }
}
