use t5f_common::*;
use crate::*;




pub struct SinglePlayerGamePlugin;

impl SinglePlayerGamePlugin {
    pub fn create_light(
        mut commands : Commands
    ) {
        commands.spawn(DirectionalLightBundle  {
            directional_light : DirectionalLight {
                // shadows_enabled : true,
                ..Default::default()
            },

            transform: Transform::from_xyz(800.0, 1800.0, 800.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        });
    }

    pub fn save_game(
        input : Res<Input<KeyCode>>,
        mut save_events: EventWriter<SaveEvent>,
    ) {
        if input.just_pressed(KeyCode::F6) {
            save_events.send(SaveEvent::Save("saves/developer.t5flvl".to_string()));
        }
    }
}

impl Plugin for SinglePlayerGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Player(TeamPlayer::PLAYER_ID))
            .add_systems(OnEnter(GameState::SingleplayerGame), (
                Self::create_light,
            ))
            .add_systems(Update, (
                Self::save_game,
            ).run_if(in_state(GameState::SingleplayerGame)))
            .add_systems(OnExit(GameState::SingleplayerGame), cleanup_entities)
        ;
    }
}