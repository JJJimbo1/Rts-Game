use crate::*;

pub struct SinglePlayerGamePlugin;

impl SinglePlayerGamePlugin {
    pub fn create_light(
        mut commands: Commands
    ) {
        commands.spawn((
            DirectionalLight::default(),
            Transform::from_xyz(800.0, 1800.0, 800.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        ));
    }

    pub fn save_game(
        input: Res<ButtonInput<KeyCode>>,
        mut save_events: ParamSet<(
            EventWriter<SaveEvent>,
            EventReader<SaveEvent>,
        )>,
        save_file: Res<SaveFile>,
    ) {
        if input.just_pressed(KeyCode::F6) {
            save_events.p0().write(SaveEvent::Save("saves/developer.ron".to_string()));
        }

        for event in save_events.p1().read().filter(|event| event.finishing()) {
            let Ok(root) = std::env::current_dir() else { return; };
            let path = &format!("{}/t5f_client/assets/{}", root.as_path().display(), event.file());
            println!("{}", path);

            //TODO: Replace this with whatever first party solution Bevy comes up with in the future
            let _ = save_to_file(&*save_file, path);
        }
    }
}

impl Plugin for SinglePlayerGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LocalPlayer(TeamPlayer::PLAYER_ID))
            .add_systems(OnEnter(GameState::SingleplayerGame), (
                Self::create_light,
            ))
            .add_systems(Update, (
                Self::save_game,
            ).run_if(in_state(GameState::SingleplayerGame)))
        ;
    }
}