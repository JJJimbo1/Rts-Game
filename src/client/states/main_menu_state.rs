use bevy::app::AppExit;
use crate::*;

pub struct MainMenuPlugin;

impl MainMenuPlugin {
    pub fn spawn_camera(
        mut commands: Commands
    ) {
        commands.spawn(Camera3d::default()).insert(DeleteOnStateChange);
    }

    pub fn main_menu_ui_button_event_reader(
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
                    close(&mut visible_query, main_menu.custom_game.container);
                    toggle(&mut visible_query, main_menu.campaign.container);
                },
                TopMenuButtonsEvent::Skirmish => {
                    close(&mut visible_query, main_menu.campaign.container);
                    toggle(&mut visible_query, main_menu.custom_game.container);
                }
                TopMenuButtonsEvent::Quit => {
                    quit_app.write(AppExit::Success);
                },
                _ => { }
            }
        }

        for event in campaign_button_event_reader.read() {
            match *event {
                CampaignButtonsEvent::Continue => {
                    commands.insert_resource(SaveFile::File("saves/developer.ron".to_string()));
                    state.set(GameState::MatchLoadingState);
                },
                CampaignButtonsEvent::LevelSelect => {
                    commands.insert_resource(SaveFile::File("levels/developer.ron".to_string()));
                    state.set(GameState::MatchLoadingState);
                }
                CampaignButtonsEvent::Back => {
                    close(&mut visible_query, main_menu.campaign.container);
                }
                _ => { }
            }
        }

        for event in skirmish_button_event_reader.read() {
            match *event {
                SkirmishButtonsEvent::Back => {
                    close(&mut visible_query, main_menu.custom_game.container);
                },
                SkirmishButtonsEvent::NewGame => {
                    state.set(GameState::CustomGame);
                }
                _ => { }
            }
        }
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), (
                Self::spawn_camera,
            ))
            .add_systems(Update, (
                Self::main_menu_ui_button_event_reader.after(MainMenuUIPlugin::main_menu_ui_button_event_writer)
            ).run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_entities)
        ;
    }
}