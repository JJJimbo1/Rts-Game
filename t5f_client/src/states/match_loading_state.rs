use bevy::asset::LoadState;

use crate::*;

// #[derive(Resource)]
// pub struct ChosenSaveFile(pub String);

pub struct MatchLoadingStatePlugin;

impl MatchLoadingStatePlugin {
    pub fn load_level(
        mut save_file: ResMut<SaveFile>,
        asset_server: Res<AssetServer>,
    ) {
        if let Some(file) = save_file.file() {
            println!("FINGING LEVEL ASSET");
            let handle = asset_server.load::<SaveFile>(file);
            *save_file.as_mut() = SaveFile::Handle(handle);
        }
    }

    pub fn begin_loading(
        mut save_file: ResMut<SaveFile>,
        asset_server: Res<AssetServer>,
        save_file_assets: Res<Assets<SaveFile>>,
        mut load_event: EventWriter<LoadEvent>,
    ) {
        if let Some(handle) = save_file.handle() {
            match asset_server.load_state(&handle) {
                LoadState::NotLoaded => { println!("NOT LOADED"); },
                LoadState::Loading => { println!("LOADING"); },
                LoadState::Loaded => {
                    println!("LOADED");
                    if let Some(save) = save_file_assets.get(&handle) {
                        *save_file.as_mut() = save.clone();
                        load_event.send(LoadEvent::Load("".to_string()));
                    }
                },
                LoadState::Failed(_) => { println!("FAILED"); },
            }
        }
    }

    pub fn finish_loading(
        mut save_file: ResMut<SaveFile>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        if save_file.all_loaded() {
            save_file.reset();
            next_state.set(GameState::SingleplayerGame)
        }
        // for event in loaded_event_reader.read() {
        //     match event {
        //         LevelLoadedEvent::Success => {
        //             state.set(GameState::SingleplayerGame);
        //         },
        //         LevelLoadedEvent::Failure(reason) => {
        //             error!("Level failed to load with reason: {:?}", reason);
        //         }
        //     }
        // }
    }
}

impl Plugin for MatchLoadingStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MatchLoadingState),
                Self::load_level
            )
            .add_systems(Update,
                (Self::begin_loading,
                Self::finish_loading)
                    .run_if(in_state(GameState::MatchLoadingState))
            )
        ;
    }
}