use crate::*;

pub struct CustomGamePlugin;

impl CustomGamePlugin {
    pub fn spawn_camera(
        mut commands: Commands
    ) {
        commands.spawn(Camera3d::default()).insert(DeleteOnStateChange);
    }



    // pub fn main_menu_ui_button_event_reader(
    //     mut main_menu_button_event_reader : EventReader<TopMenuButtonsEvent>,
    //     mut campaign_button_event_reader : EventReader<CampaignButtonsEvent>,
    //     mut skirmish_button_event_reader : EventReader<SkirmishButtonsEvent>,
    //     mut state: ResMut<NextState<GameState>>,
    //     mut quit_app : EventWriter<AppExit>,
    //     main_menu : Res<MainMenuUi>,
    //     mut commands : Commands,
    //     mut visible_query: Query<(&mut Visibility, &InheritedVisibility)>,
    // ) {
    //     for event in main_menu_button_event_reader.read() {
    //         match *event {
    //             TopMenuButtonsEvent::Campaign => {
    //                 main_menu.skirmish.close(&mut visible_query);
    //                 main_menu.campaign.toggle(&mut visible_query);
    //             },
    //             TopMenuButtonsEvent::Skirmish => {
    //                 main_menu.campaign.close(&mut visible_query);
    //                 main_menu.skirmish.toggle(&mut visible_query);
    //             }
    //             TopMenuButtonsEvent::Quit => {
    //                 quit_app.send(AppExit);
    //             },
    //             _ => { }
    //         }
    //     }

    //     for event in campaign_button_event_reader.read() {
    //         match *event {
    //             CampaignButtonsEvent::Continue => {
    //                 commands.insert_resource(SaveFile::File("saves/developer_test.t5fsav".to_string()));
    //                 state.set(GameState::MatchLoadingState);
    //             },
    //             CampaignButtonsEvent::LevelSelect => {
    //                 commands.insert_resource(SaveFile::File("saves/developer_test.t5fsav".to_string()));
    //                 state.set(GameState::MatchLoadingState);
    //             }
    //             CampaignButtonsEvent::Back => {
    //                 main_menu.campaign.close(&mut visible_query);
    //             }
    //             _ => { }
    //         }
    //     }

    //     for event in skirmish_button_event_reader.read() {
    //         match *event {
    //             SkirmishButtonsEvent::Back => {
    //                 main_menu.skirmish.close(&mut visible_query);
    //             },
    //             SkirmishButtonsEvent::NewGame => {
    //                 state.set(GameState::CustomGame);
    //             }
    //             _ => { }
    //         }
    //     }
    // }
}

impl Plugin for CustomGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CustomGameUIPlugin)
            .add_systems(OnEnter(GameState::CustomGame), (
                Self::spawn_camera,
            ))
            // .add_systems(Update, (
            //     Self::main_menu_ui_button_event_reader.after(CustomGameUIPlugin::main_menu_ui_button_event_writer)
            // ).run_if(in_state(GameState::CustomGame)))
            .add_systems(OnExit(GameState::CustomGame), cleanup_entities)
        ;
    }
}