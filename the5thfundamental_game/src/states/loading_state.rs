pub mod loading_state {
    use std::path::Path;

    use bevy::{gltf::Gltf, prelude::*};
    // use log::info;
    use the5thfundamental_common::*;
    use crate::*;


    pub fn game_loading_state_on_enter_system_set() -> SystemSet {
        SystemSet::on_enter(GameState::Loading)
            .with_system(game_loading_on_enter)
            // .with_system(load_object_prefabs)
            // .with_system(load_map_prefabs)
    }

    pub fn game_loading_state_on_update_system_set() -> SystemSet {
        SystemSet::on_update(GameState::Loading)
            .with_system(game_loading_update)
    }

    pub fn game_loading_state_on_exit_system_set() -> SystemSet {
        SystemSet::on_exit(GameState::Loading)
            .with_system(game_loading_on_exit)
            .with_system(cleanup_entities)
    }

    pub fn game_loading_on_enter(
        mut materials : ResMut<Assets<ColorMaterial>>,
        mut commands : Commands
    ) {
        info!("LOADING!!!");
        commands.insert_resource(CameraSettings::default());
        commands.insert_resource(MenuSettings { font_size : 1.0 });
        commands.insert_resource(ButtonMaterials {
            normal: materials.add(TEXT_COLOR_NORMAL.into()),
            hovered: materials.add(TEXT_COLOR_HOVER.into()),
            pressed: materials.add(TEXT_COLOR_PRESS.into()),
        });
        commands.insert_resource(CameraRaycast::default());
        commands.insert_resource(ContextFocus(None));
    }

    pub fn game_loading_update(
        mut state: ResMut<State<GameState>>
    ) {
        info!("UPDATING!!!");
        match state.overwrite_set(GameState::MainMenu) { _ => { } }
        // for event in content_load_event.iter() {
        //     match event {
        //         ContentLoadEvent::Success => {
        //         },
        //         ContentLoadEvent::Failure => {
        //             error!("WHATTTTTTTTTTTTTTT");
        //         }
        //     }
        // }
    }

    pub fn game_loading_on_exit(

    ) {

    }
}