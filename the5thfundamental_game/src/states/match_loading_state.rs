use crate::*;

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
    mut commands: Commands,
) {
    commands.insert_resource(Player(TeamPlayer::new(1, 0)));
    load_event_write.send(LoadEvent(file.0.clone()));
}

pub fn match_loading_update(
    mut loaded_event_reader: EventReader<SaveLoaded>,
    mut state: ResMut<State<GameState>>,
) {
    for _ in loaded_event_reader.iter() {
        state.overwrite_set(GameState::SingleplayerGame).unwrap()
    }
}

pub fn match_loading_on_exit(

) {

}
