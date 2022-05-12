#[allow(dead_code)]
pub mod main_menu_state {

    use std::collections::HashMap;

    use bevy::app::AppExit;
    // use bevy_prototype_debug_lines::LineShader;
    use bevy_pathfinding::d2::*;
    use the5thfundamental_common::{Level, Map, NetMode, SaveFile, TeamPlayer, load_from_file, ServerCommands, ServerRequests};
    use qloader::*;
    use crate::*;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
    enum MainMenuSystems {
        Update,
        ButtonUpdater,
        MainMenuEventUpdater,
        MainMenuEventReader,
    }

    pub fn main_menu_on_enter() -> SystemSet {
        SystemSet::on_enter(GameState::MainMenu)
            .with_system(main_menu_state_on_enter)
            .with_system(create_main_menu)
            .with_system(spawn_health_bar)
    }

    pub fn main_menu_on_update() -> SystemSet {
        SystemSet::on_update(GameState::MainMenu)
            .with_system(main_menu_state_update.label(MainMenuSystems::Update))
            .with_system(button_updater_system.label(MainMenuSystems::ButtonUpdater).after(MainMenuSystems::Update))
            .with_system(main_menu_button_event_writer_system.label(MainMenuSystems::MainMenuEventUpdater).after(MainMenuSystems::ButtonUpdater))
            .with_system(main_menu_event_reader_system.label(MainMenuSystems::MainMenuEventReader).after(MainMenuSystems::MainMenuEventUpdater))
            // .with_system(health_bar_update_system.system())
            // .with_system(health_bar_cleanup_system.system())
    }

    pub fn main_menu_on_exit() -> SystemSet {
        SystemSet::on_exit(GameState::MainMenu)
            .with_system(main_menu_state_on_exit)
            .with_system(cleanup_entities)
    }

    pub fn main_menu_state_on_enter(/*save_file : ResMut<SaveFile>, maps : Res<QLoader<Map, ()>>, */mut commands : Commands) {
        commands.spawn_bundle(UiCameraBundle::default());
    }

    pub fn main_menu_state_update(
        // mut state: ResMut<State<GameState>>, 
        // mut random : ResMut<Random>,
        // query : Query<&HealthBar>,
        // mut styles : Query<&mut Style>
    ) {
        // query.for_each_mut(|hel| {
        //     hel.adjust_bar_percent(random.cycle(), &mut styles);
        // });
    }

    pub fn main_menu_state_on_exit(
        save_file : Res<SaveFile>,
        maps : Res<QLoader<Map, ()>>,
        mut commands : Commands,
    ) {
        if setup_save_file(&save_file, &maps, &mut commands).is_err() { panic!(); }
    }

    fn spawn_health_bar(textures : Res<QLoader<ImageAsset, AssetServer>>, mut materials : ResMut<Assets<ColorMaterial>>, mut commands : Commands) {
        let health_bar = HealthBar::new(50, textures.deref(), &mut materials, &mut commands);
        commands.spawn().insert(health_bar);
    }

    fn main_menu_event_reader_system(
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
                    let x = load_from_file::<SaveFile, &str>(&format!("{}\\assets\\saves\\developer_level.ron", *PROJECT_ROOT_DIRECTORY));
                    commands.insert_resource(x.unwrap());
                    state.overwrite_replace(GameState::SingleplayerGame).unwrap();
                },
                CampaignButtons::LevelSelect => {
                    let x = load_from_file::<Level, &str>(&format!("{}\\assets\\levels\\developer_level.ron", *PROJECT_ROOT_DIRECTORY));
                    commands.insert_resource(x.unwrap().save_state);
                    state.overwrite_replace(GameState::SingleplayerGame).unwrap();
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

    fn setup_save_file(
        save_file : &SaveFile,
        maps : &QLoader<Map, ()>,
        commands : &mut Commands
    ) -> Result<(), ()> {
        if let Some(x) = maps.get(&save_file.map) {
            commands.insert_resource(SaveMap(save_file.map.clone()));
            commands.insert_resource(x.clone());
            commands.insert_resource({
                // TODO: Map analyzation.
                GridMap::new(x.bounds.0 as usize, x.bounds.1 as usize)
                    .with_cells(|x, z| GridCell::new(x, z, false ))
                    .precomputed()
                });
            commands.insert_resource(Player(TeamPlayer::new(1, 0)));
        } else {
            return Err(());
        }
        commands.insert_resource(save_file.actors.clone());

        Ok(())
    }
}