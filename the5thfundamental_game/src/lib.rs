pub use bevy::prelude::*;
pub use bevy_ninepatch::*;
pub use chrono::Local;
pub use the5thfundamental_common::*;
pub use random::*;

pub mod ui;
pub mod resources;
pub mod settings;
pub mod states;
pub mod systems;
pub mod utility;

pub use ui::*;
pub use resources::*;
pub use settings::*;
pub use states::*;
pub use systems::*;
pub use utility::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameState {
    Loading,
    MainMenu,
    SingleplayerGame,
    MatchLoadingState,
    MultiplayerGame,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum SystemSets {
    MainMenuUi,
    Camera,
}

#[derive(Debug, Clone)]
#[derive(Resource)]
pub struct FPSCounter{
    pub timer : Timer,
    pub frames : u32,
    pub frames_total : u64,
}

pub fn play_game(asset_folder: String) {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(UiHit::<CLICK_BUFFER>{ hitting : [false; CLICK_BUFFER], holding : false, })
        .insert_resource(FPSCounter{
            timer : Timer::from_seconds(0.25, TimerMode::Repeating),
            frames : 0,
            frames_total : 0,
        })

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1920.0,
                height: 1080.0,
                title: "untitled rts game".to_string(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            },
            ..default()
        }).set(AssetPlugin {
                asset_folder,
                ..default()
            })
        )
        .add_plugins(CommonPlugins)
        .add_plugin(CommonLoadingPlugin { state: GameState::Loading})
        .add_plugin(PhysicsPlugin)
        .add_plugin(DebugPlugin)

        .add_plugin(NinePatchPlugin::<()>::default())
        .add_plugin(SavePlugin)
        .add_plugin(HealthBarPlugin)

        .add_event::<SelectionEvent>()
        .add_event::<ActivationEvent>()

        .add_event::<TopMenuButtonsEvent>()
        .add_event::<CampaignButtonsEvent>()
        .add_event::<SkirmishButtonsEvent>()
        .add_event::<ContextMenuButtonsEvent>()

        .add_event::<ObjectSpawnEvent>()
        .add_event::<UnitCommandEvent>()
        .add_event::<ObjectKilledEvent>()
        // .add_event::<AttackCommand>()

        .insert_resource(Random::<WichmannHill>::seeded(123.456))
        .insert_resource(Identifiers::default())
        // .insert_resource(DirtyEntities::default())
        // .insert_resource(InitRequests::default())
        .insert_resource(Manifest::default())
        // .insert_resource(PhysicsWorld::default())

        .add_system_set(game_loading_state_on_enter_system_set())
        .add_system_set(game_loading_state_on_update_system_set())
        .add_system_set(game_loading_state_on_exit_system_set())

        .add_system_set(main_menu_state_on_enter_system_set())
        .add_system_set(main_menu_state_on_update_system_set())
        .add_system_set(main_menu_state_on_exit_system_set())

        .add_system_set(match_loading_state_on_enter_system_set())
        .add_system_set(match_loading_state_on_update_system_set())
        .add_system_set(match_loading_state_on_exit_system_set())

        .add_system_set(singleplayer_game_state_on_enter_system_set())
        .add_system_set(singleplayer_game_state_on_update_system_set())
        .add_system_set(singleplayer_game_state_on_exit_system_set())

        .add_system_set(camera_setup_system_set(SystemSet::on_enter(GameState::SingleplayerGame)))
        .add_system(ui_hit_detection_system)
        .add_system_set(camera_system_set(SystemSet::on_update(GameState::SingleplayerGame).after(ui_hit_detection_system)))

        .add_state(GameState::Loading)

        // .add_startup_system(setup)
    .run();
}