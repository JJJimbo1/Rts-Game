use the5thfundamental_common::*;
use crate::*;

// #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
// enum GameSystems {
//     OnStart,
//     CombatStartup,
// }

pub fn singleplayer_game_state_on_enter_system_set() -> SystemSet {
    SystemSet::on_enter(GameState::SingleplayerGame)
        .with_system(singleplayer_game_on_enter)
        .with_system(combat_startup_system)
        .with_system(create_debug_menu)
        .with_system(create_gameplay_ui)
        // .with_system(load_save_file.after(GameSystems::OnStart))
        .with_system(create_context_menu)
        // .with_system(testing_start_up)
}

pub fn singleplayer_game_state_on_update_system_set() -> SystemSet {
    SystemSet::on_update(GameState::SingleplayerGame)
        .with_system(singleplayer_game_on_update)
        .with_system(debug_menu_update)
        .with_system(gameplay_ui_update)
        .with_system(health_bar_update_system)
        .with_system(health_bar_cleanup_system)
        .with_system(button_updater_system)
        .with_system(context_menu_update_system)
        .with_system(context_menu_event_writer_system)
        .with_system(context_menu_event_reader_system)

        .with_system(spawn_object)
        .with_system(client_object_spawn.after(spawn_object))
        .with_system(clear_buffer.after(spawn_object))

        .with_system(save_game)
}

pub fn singleplayer_game_state_on_exit_system_set() -> SystemSet {
    SystemSet::on_exit(GameState::SingleplayerGame)
        .with_system(singleplayer_game_state_on_exit)
        .with_system(cleanup_entities)
}

pub fn singleplayer_game_on_enter(
    mut commands : Commands
) {
    commands.spawn_bundle(DirectionalLightBundle  {
        directional_light : DirectionalLight {
            // shadows_enabled : true,
            ..Default::default()
        },

        transform: Transform::from_xyz(800.0, 1800.0, 800.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        ..Default::default()
    });
}

pub fn singleplayer_game_on_update(
    input : Res<Input<KeyCode>>,
    mut save_events: EventWriter<SaveEvent>,

) {
    if input.just_pressed(KeyCode::F6) {
        save_events.send(SaveEvent("/assets/saves/developer.ron".to_string()));
    }
}

pub fn singleplayer_game_state_on_exit(

) {

}