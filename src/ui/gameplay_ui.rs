use bevy::prelude::*;
use crate::*;

#[derive(Copy, Clone)]
#[derive(Resource)]
pub struct GameplayUi {
    _container: Entity,
    resources: Entity,
}

impl GameplayUi {
    pub fn new(
        settings: &MenuSettings,
        font_assets: &FontAssets,
        commands: &mut Commands,
    ) -> Self {
        let font = font_assets.roboto.clone();
        let font_size = FONT_SIZE_LARGE * settings.font_size;

        let mut entity_commands = commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                width: Val::Px(600.0),
                height: Val::Px(200.0),
                ..default()
            },
            BackgroundColor(DARK_BACKGROUND_COLOR),
            Visibility::Visible,
        ));

        let container = entity_commands.id();
        let mut resources = None;

        entity_commands.with_children(|parent| {
            resources = Some(parent.spawn((
                Text::new("$ 0"),
                TextFont {
                    font: font.clone(),
                    font_size,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
                },
            )).id());
        });

        Self {
            _container: container,
            resources: resources.unwrap(),
        }
    }
}

pub struct GamePlayUIPlugin;

impl GamePlayUIPlugin {
    pub fn create_gameplay_ui(
        settings: Res<MenuSettings>,
        font_assets: Res<FontAssets>,
        mut commands: Commands,
    ) {
        let gameplay_ui = GameplayUi::new(&settings, &font_assets, &mut commands);
        commands.insert_resource(gameplay_ui);
    }

    pub fn gameplay_ui_update(
        menu: Res<GameplayUi>,
        player: Res<LocalPlayer>,
        actors: Res<Commanders>,
        mut texts: Query<&mut Text>,
    ) {

        if let (Ok(mut text), Some(actor)) = (texts.get_mut(menu.resources), actors.commanders.get(&player.0)) {
            text.0 = format!("$ {}", actor.economy.resources().round());
        }
    }
}

impl Plugin for GamePlayUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::SingleplayerGame), Self::create_gameplay_ui)
            .add_systems(Update, Self::gameplay_ui_update.run_if(in_state(GameState::SingleplayerGame)));
    }
}