use crate::*;

#[derive(Resource)]
pub struct ChosenSaveFile(pub String);


pub fn match_loading_state_on_enter_system_set() -> SystemSet {
    SystemSet::on_enter(GameState::MatchLoadingState)
        .with_system(match_loading_on_enter)
}

pub fn match_loading_state_on_update_system_set() -> SystemSet {
    SystemSet::on_update(GameState::MatchLoadingState)
        .with_system(match_loading_update)
}

pub fn match_loading_state_on_exit_system_set() -> SystemSet {
    SystemSet::on_exit(GameState::MatchLoadingState)
        .with_system(match_loading_on_exit)
}

pub fn match_loading_on_enter(
    file: Res<ChosenSaveFile>,
    mut load_event_write: EventWriter<LoadEvent>,
    level_assets: Res<LevelAssets>,
    asset_loader: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.insert_resource(Player(TeamPlayer::new(1, 0)));
    // load_event_write.send(LoadEvent(asset_loader.load(&file.0)));
    let level = asset_loader.load(file.0.clone());
    load_event_write.send(LoadEvent(level.clone()));
}

pub fn match_loading_update(
    mut loaded_event_reader: EventReader<LevelLoadedEvent>,
    mut state: ResMut<State<GameState>>,
) {
    for event in loaded_event_reader.iter() {
        match event {
            LevelLoadedEvent::Success => {
                state.overwrite_set(GameState::SingleplayerGame).unwrap()
            },
            LevelLoadedEvent::Failure => {
                error!("FAILLLLLLLLLLLLL");
            }
        }
    }
}

pub fn match_loading_on_exit(

) {

}
