#[allow(dead_code)]
pub mod main_menu_state {

    use std::collections::HashMap;

    use bevy::app::AppExit;
    // use bevy_prototype_debug_lines::LineShader;
    use bevy_pathfinding::d2::*;
    use the5thfundamental_common::{Level, MapBounds, NetMode, SaveFile, TeamPlayer, load_from_file, ServerCommands, ServerRequests};
    use qloader::*;
    use crate::*;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
    enum MainMenuSystems {
        Update,
        ButtonUpdater,
        MainMenuEventUpdater,
        // MainMenuEventReader,
    }

    pub fn main_menu_state_on_enter_system_set() -> SystemSet {
        SystemSet::on_enter(GameState::MainMenu)
            .with_system(main_menu_on_enter)
            .with_system(create_main_menu)
            // .with_system(spawn_health_bar)
    }

    pub fn main_menu_state_on_update_system_set() -> SystemSet {
        SystemSet::on_update(GameState::MainMenu)
            .with_system(button_updater_system.label(MainMenuSystems::ButtonUpdater))
            .with_system(main_menu_button_event_writer_system.label(MainMenuSystems::MainMenuEventUpdater).after(MainMenuSystems::ButtonUpdater))
            .with_system(main_menu_update.after(MainMenuSystems::MainMenuEventUpdater))
    }

    pub fn main_menu_state_on_exit_system_set() -> SystemSet {
        SystemSet::on_exit(GameState::MainMenu)
            .with_system(main_menu_on_exit)
            .with_system(cleanup_entities)
    }

    pub fn main_menu_on_enter(
        mut commands : Commands
    ) {
        commands.spawn_bundle(UiCameraBundle::default());
    }

    pub fn main_menu_update(
        mut main_menu_button_event_reader : EventReader<TopMenuButtons>,
        mut campaign_button_event_reader : EventReader<CampaignButtons>,
        mut skirmish_button_event_reader : EventReader<SkirmishButtons>,
        mut state: ResMut<State<GameState>>,
        mut quit_app : EventWriter<AppExit>,
        main_menu : Res<MainMenuUi>,
        mut commands : Commands,
        mut visible_query : Query<&mut Visibility>,
        children_query : Query<&Children>,
    ) {
        for event in main_menu_button_event_reader.iter() {
            match *event {
                TopMenuButtons::Campaign => {
                    main_menu.skirmish.close(&mut visible_query, &children_query);
                    main_menu.campaign.toggle(&mut visible_query, &children_query);
                },
                TopMenuButtons::Skirmish => {
                    main_menu.campaign.close(&mut visible_query, &children_query);
                    main_menu.skirmish.toggle(&mut visible_query, &children_query);
                }
                TopMenuButtons::Quit => {
                    quit_app.send(AppExit);
                },
                _ => { }
            }
        }

        for event in campaign_button_event_reader.iter() {
            match *event {
                CampaignButtons::Continue => {
                    commands.insert_resource(ChosenSaveFile("/assets/saves/developer.ron".to_string()));
                    state.overwrite_replace(GameState::MatchLoadingState).unwrap();
                },
                CampaignButtons::LevelSelect => {
                    commands.insert_resource(ChosenSaveFile("/assets/levels/developer.ron".to_string()));
                    state.overwrite_replace(GameState::MatchLoadingState).unwrap();
                }
                CampaignButtons::Back => {
                    main_menu.campaign.close(&mut visible_query, &children_query);
                }
                _ => { }
            }
        }

        for event in skirmish_button_event_reader.iter() {
            match *event {
                SkirmishButtons::Back => {
                    main_menu.skirmish.close(&mut visible_query, &children_query);
                },
                _ => { }
            }
        }
    }

    pub fn main_menu_on_exit(

    ) {

    }

    // fn setup_save_file(
    //     save_file : &SaveFile,
    //     maps : &QLoader<Map, ()>,
    //     commands : &mut Commands
    // ) -> Result<(), ()> {
    //     if let Some(x) = maps.get(&save_file.map) {
    //         commands.insert_resource(SaveMap(save_file.map.clone()));
    //         commands.insert_resource(x.clone());
    //         commands.insert_resource({
    //             // TODO: Map analyzation.
    //             GridMap::new(x.bounds.0 as usize, x.bounds.1 as usize)
    //                 .with_cells(|x, z| GridCell::new(x, z, false ))
    //                 .precomputed()
    //             });
    //         commands.insert_resource(Player(TeamPlayer::new(1, 0)));
    //     } else {
    //         return Err(());
    //     }
    //     commands.insert_resource(save_file.actors.clone());

    //     Ok(())
    // }
}