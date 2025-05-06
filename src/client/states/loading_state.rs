use bevy::prelude::*;
use crate::*;

pub struct LoadingStatePlugin;

impl LoadingStatePlugin {
    pub fn load(
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut state: ResMut<NextState<GameState>>,
        mut commands: Commands
    ) {
        commands.insert_resource(CameraSettings::default());
        commands.insert_resource(MenuSettings { font_size: 1.0 });
        commands.insert_resource(ButtonMaterials {
            normal: materials.add(TEXT_COLOR_NORMAL),
            hovered: materials.add(TEXT_COLOR_HOVER),
            pressed: materials.add(TEXT_COLOR_PRESS),
        });
        commands.insert_resource(CameraRaycast::default());
        commands.insert_resource(ContextFocus(None));
        state.set(GameState::MainMenu);
    }
}

impl Plugin for LoadingStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Loading), Self::load)
        ;
    }
}