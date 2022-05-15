pub mod game_state {

    use bevy_rapier3d::prelude::*;
    use bevy_pathfinding::{PathFinder, Path};
    use qloader::*;
    use the5thfundamental_common::*;
    use crate::*;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
    enum GameSystems {
        OnStart,
        CombatStartup,
    }

    pub fn singleplayer_game_on_enter() -> SystemSet {
        SystemSet::on_enter(GameState::SingleplayerGame)
            .with_system(singleplayer_game_state_on_enter.label(GameSystems::OnStart))
            .with_system(combat_startup_system.label(GameSystems::CombatStartup).after(GameSystems::OnStart))
            .with_system(create_debug_menu.after(GameSystems::OnStart))
            .with_system(create_gameplay_ui.after(GameSystems::OnStart))
            .with_system(load_save_file.after(GameSystems::OnStart))
            .with_system(create_context_menu)
            // .with_system(testing_start_up)
    }

    pub fn singleplayer_game_on_update() -> SystemSet {
        SystemSet::on_update(GameState::SingleplayerGame)
            .with_system(singleplayer_game_state_on_update)
            .with_system(debug_menu_update)
            .with_system(gameplay_ui_update)
            .with_system(health_bar_update_system)
            .with_system(health_bar_cleanup_system)
            .with_system(button_updater_system)
            .with_system(context_menu_update_system)
            .with_system(context_menu_event_writer_system)
            .with_system(context_menu_event_reader_system)
            .with_system(object_spawn_system)
            .with_system(clear_buffer_system)
    }

    pub fn singleplayer_game_on_exit() -> SystemSet {
        SystemSet::on_exit(GameState::SingleplayerGame)
            .with_system(singleplayer_game_state_on_exit)
            .with_system(cleanup_entities)
    }

    pub fn singleplayer_game_state_on_enter(
        // mut commands : Commands
    ) {
        // commands.spawn_bundle(UiCameraBundle::default());
    }

    pub fn singleplayer_game_state_on_update(
        input : Res<Input<KeyCode>>,
        idents : Res<Identifiers>,
        actors : Res<Actors>,
        save_map : Res<SaveMap>,
        query : Query<(Entity, &SaveObject, &Transform, &TeamPlayer, &Health, Option<&Velocity>, Option<&PathFinder>, Option<&Path>, Option<&Queues>, Option<&WeaponSet>)>

    ) {
        if input.just_pressed(KeyCode::F6) {
            let save = SaveFile::new(&idents, &actors, &save_map, &query);
            let _ = save_to_file::<SaveFile, &str>(&save, &format!("{}\\assets\\saves\\developer_level.ron", *PROJECT_ROOT_DIRECTORY));
        }
    }

    pub fn singleplayer_game_state_on_exit(

    ) {

    }
}