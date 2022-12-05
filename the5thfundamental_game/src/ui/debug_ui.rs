use bevy::prelude::*;
use crate::{*, utility::assets::FontAsset};

pub fn create_debug_menu(
    settings : Res<MenuSettings>,
    mut asset_server: ResMut<AssetServer>,
    mut commands : Commands,
) {
    let debug_menu = DebugMenu::new(&settings, &mut asset_server, &mut commands);
    commands.insert_resource(debug_menu);
}

#[derive(Copy, Clone)]
#[derive(Resource)]
pub struct DebugMenu {
    container : Entity,
    fps_counter : Entity,
    frame_number : Entity,
}

impl DebugMenu {
    pub fn new(
        settings : &MenuSettings,
        asset_server: &mut AssetServer,
        commands : &mut Commands,
    ) -> Self {
        let font = asset_server.load(FontAsset::Roboto);

        let font_size = FONT_SIZE_SMALL * settings.font_size;

        let mut entity_commands = commands.spawn(NodeBundle {
            style: Style {
                position_type : PositionType::Absolute,
                position: UiRect {
                    top : Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                size: Size::new(Val::Px(200.0), Val::Px(400.0)),
                ..Default::default()
            },
            background_color : DARK_BACKGROUND_COLOR.into(),
            visibility : Visibility { is_visible : false},
            ..Default::default()
        });

        let container_entity = entity_commands.id();
        let mut fps_counter_entity = None;

        let entity_commands = entity_commands.with_children(|parent| {
            fps_counter_entity = Some(parent.spawn(TextBundle {
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
                    "FPS : 0",
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

        let mut frame_number_entity = None;

        entity_commands.with_children(|parent| {
            // text
            frame_number_entity = Some(parent.spawn(TextBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position: UiRect {
                        top : Val::Px(font_size + 20.0),
                        left: Val::Px(10.0),
                        // top: Val::Px(0.0),
                        ..Default::default()
                    },
                    //margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::from_section(
                    "FRAME# : 0",
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
            container : container_entity,
            fps_counter : fps_counter_entity.unwrap(),
            frame_number : frame_number_entity.unwrap(),
        }
    }
}

pub fn debug_menu_update(
    menu : Res<DebugMenu>,
    input : Res<Input<KeyCode>>,
    time : Res<Time>,
    mut fps_counter : ResMut<FPSCounter>,
    mut visibles : Query<&mut Visibility>,
    mut texts : Query<&mut Text>,
) {

    fps_counter.timer.tick(time.delta());
    fps_counter.frames += 1;
    fps_counter.frames_total += 1;
    if input.just_pressed(KeyCode::F3) {
        menu.toggle(&mut visibles);
    }

    if let Ok(mut text) = texts.get_mut(menu.fps_counter) {
        if fps_counter.timer.finished() {
            text.sections[0].value = format!("FPS: {:.*}", 1, fps_counter.frames as f32 / (fps_counter.timer.elapsed_secs() + fps_counter.timer.times_finished_this_tick() as f32 * fps_counter.timer.duration().as_secs_f32()));
            fps_counter.timer.reset();
            fps_counter.frames = 0;
        }
    }
    if let Ok(mut text) = texts.get_mut(menu.frame_number) {
        text.sections[0].value = format!("FRAME#: {}", fps_counter.frames_total);
    }
}

impl Menu for DebugMenu {
    fn main_container(&self) -> Entity {
        self.container
    }
}