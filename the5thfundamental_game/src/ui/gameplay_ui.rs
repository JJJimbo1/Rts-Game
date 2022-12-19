use bevy::prelude::*;
use crate::{*, utility::assets::FontAssets};

pub fn create_gameplay_ui(
    settings : Res<MenuSettings>,
    mut font_assets: Res<FontAssets>,
    mut commands : Commands,
) {
    let gameplay_ui = GameplayUi::new(&settings, &font_assets, &mut commands);
    commands.insert_resource(gameplay_ui);
}

#[derive(Copy, Clone)]
#[derive(Resource)]
pub struct GameplayUi {
    container : Entity,
    resources : Entity,
}

impl GameplayUi {
    pub fn new(
        settings : &MenuSettings,
        font_assets: &FontAssets,
        commands : &mut Commands,
    ) -> Self {
        let font = font_assets.roboto.clone();
        let font_size = FONT_SIZE_LARGE * settings.font_size;

        let mut entity_commands = commands.spawn(NodeBundle {
            style: Style {
                position_type : PositionType::Absolute,
                position : UiRect {
                    right : Val::Px(0.0),
                    bottom : Val::Px(0.0),
                    ..Default::default()
                },
                size: Size::new(Val::Px(600.0), Val::Px(200.0)),
                ..Default::default()
            },
            background_color : DARK_BACKGROUND_COLOR.into(),
            visibility : Visibility { is_visible : true},
            ..Default::default()
        });

        let container = entity_commands.id();
        let mut resources = None;

        entity_commands.with_children(|parent| {
            resources = Some(parent.spawn(TextBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position: UiRect {
                        top : Val::Px(10.0),
                        left: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    "Resources: 0",
                    TextStyle {
                        font : font.clone(),
                        font_size,
                        color: Color::WHITE,
                    },
                ),
                visibility : Visibility { is_visible : true},
                ..Default::default()
            }).id());
        });

        Self {
            container,
            resources : resources.unwrap(),
        }
    }
}

pub fn gameplay_ui_update(
    menu : Res<GameplayUi>,
    player : Res<Player>,
    actors : Res<Actors>,
    mut texts : Query<&mut Text>,
) {

    if let (Ok(mut text), Some(actor)) = (texts.get_mut(menu.resources), actors.actors.get(&player.0)) {
        text.sections[0].value = format!("Resources: {}", actor.economy.resources().round());
    }
}

impl Menu for GameplayUi {
    fn main_container(&self) -> Entity {
        self.container
    }
}