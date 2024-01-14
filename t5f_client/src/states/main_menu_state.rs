use bevy::app::AppExit;
use crate::*;

// #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
// enum MainMenuSystems {
//     Update,
//     ButtonUpdater,
//     MainMenuEventUpdater,
//     // MainMenuEventReader,
// }



pub struct MainUIPlugin;

impl MainUIPlugin {
    pub fn spawn_camera(
        mut commands: Commands
    ) {
        commands.spawn(Camera3dBundle::default()).insert(DeleteOnStateChange);
    }

    pub fn handle_buttons(
        mut main_menu_button_event_reader : EventReader<TopMenuButtonsEvent>,
        mut campaign_button_event_reader : EventReader<CampaignButtonsEvent>,
        mut skirmish_button_event_reader : EventReader<SkirmishButtonsEvent>,
        mut state: ResMut<NextState<GameState>>,
        mut quit_app : EventWriter<AppExit>,
        main_menu : Res<MainMenuUi>,
        mut commands : Commands,
        mut visible_query: Query<(&mut Visibility, &InheritedVisibility)>,
    ) {
        for event in main_menu_button_event_reader.read() {
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

        for event in campaign_button_event_reader.read() {
            match *event {
                CampaignButtonsEvent::Continue => {
                    commands.insert_resource(SaveFile::File("saves/developer_test.t5fsav".to_string()));
                    state.set(GameState::MatchLoadingState);
                },
                CampaignButtonsEvent::LevelSelect => {
                    commands.insert_resource(SaveFile::File("levels/developer.t5flvl".to_string()));
                    state.set(GameState::MatchLoadingState);
                }
                CampaignButtonsEvent::Back => {
                    main_menu.campaign.close(&mut visible_query);
                }
                _ => { }
            }
        }

        for event in skirmish_button_event_reader.read() {
            match *event {
                SkirmishButtonsEvent::Back => {
                    main_menu.skirmish.close(&mut visible_query);
                },
                _ => { }
            }
        }
    }
}

impl Plugin for MainUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), (
                Self::spawn_camera,
                create_main_menu,
            ))
            .add_systems(Update, (
                main_menu_ui_button_event_writer_system.after(button_updater_system),
                Self::handle_buttons.after(main_menu_ui_button_event_writer_system)
            ).run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_entities)
        ;
    }
}