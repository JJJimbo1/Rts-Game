use bevy::prelude::*;
use avian3d::prelude::PhysicsDebugPlugin;



#[derive(Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_plugins(PhysicsDebugPlugin::default())
            // .add_plugin(DiagnosticsPlugin::default())
            // .add_plugin(DebugLinesPlugin::with_depth_test(false))

        ;
    }
}
