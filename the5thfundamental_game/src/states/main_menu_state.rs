use bevy::render::camera::Viewport;
use bevy::app::AppExit;
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
    mut commands: Commands
) {
    commands.spawn(Camera3dBundle::default()).insert(DeleteOnStateChange);
}

pub fn main_menu_update(
    mut main_menu_button_event_reader : EventReader<TopMenuButtonsEvent>,
    mut campaign_button_event_reader : EventReader<CampaignButtonsEvent>,
    mut skirmish_button_event_reader : EventReader<SkirmishButtonsEvent>,
    mut state: ResMut<State<GameState>>,
    mut quit_app : EventWriter<AppExit>,
    main_menu : Res<MainMenuUi>,
    mut commands : Commands,
    mut visible_query : Query<&mut Visibility>,
) {
    for event in main_menu_button_event_reader.iter() {
        match *event {
            TopMenuButtonsEvent::Campaign => {
                main_menu.skirmish.close(&mut visible_query);
                main_menu.campaign.toggle(&mut visible_query);
            },
            TopMenuButtonsEvent::Skirmish => {
                main_menu.campaign.close(&mut visible_query);
                main_menu.skirmish.toggle(&mut visible_query);
            }
            TopMenuButtonsEvent::Quit => {
                quit_app.send(AppExit);
            },
            _ => { }
        }
    }

    for event in campaign_button_event_reader.iter() {
        match *event {
            CampaignButtonsEvent::Continue => {
                // commands.insert_resource(ChosenSaveFile("/assets/saves/developer.ron".to_string()));
                // state.overwrite_replace(GameState::MatchLoadingState).unwrap();
            },
            CampaignButtonsEvent::LevelSelect => {
                commands.insert_resource(ChosenSaveFile("levels/developer.t5flvl".to_string()));
                state.overwrite_replace(GameState::MatchLoadingState).unwrap();
            }
            CampaignButtonsEvent::Back => {
                main_menu.campaign.close(&mut visible_query);
            }
            _ => { }
        }
        println!("Bruh");
    }

    for event in skirmish_button_event_reader.iter() {
        match *event {
            SkirmishButtonsEvent::Back => {
                main_menu.skirmish.close(&mut visible_query);
            },
            _ => { }
        }
    }
}

pub fn main_menu_on_exit(

) {

}