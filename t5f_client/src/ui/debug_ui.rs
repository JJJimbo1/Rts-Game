use bevy::prelude::*;
use crate::*;

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
        font_assets: &FontAssets,
        commands : &mut Commands,
    ) -> Self {
        let font = font_assets.roboto.clone();

        let font_size = FONT_SIZE_SMALL * settings.font_size;

        let mut entity_commands = commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                width: Val::Px(200.0),
                height: Val::Px(400.0),
                ..Default::default()
            },
            background_color : DARK_BACKGROUND_COLOR.into(),
            visibility : Visibility::Hidden,
            ..Default::default()
        });

        let container_entity = entity_commands.id();
        let mut fps_counter_entity = None;

        let entity_commands = entity_commands.with_children(|parent| {
            fps_counter_entity = Some(parent.spawn(TextBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    top : Val::Px(10.0),
                    left: Val::Px(10.0),
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
                visibility : Visibility::Inherited,
                ..Default::default()
            }).id());
        });

        let mut frame_number_entity = None;

        entity_commands.with_children(|parent| {
            // text
            frame_number_entity = Some(parent.spawn(TextBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    top : Val::Px(font_size + 20.0),
                    left: Val::Px(10.0),
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
                visibility : Visibility::Inherited,
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

pub struct DebugUIPlugin;

impl DebugUIPlugin {
    pub fn create_debug_ui(
        settings : Res<MenuSettings>,
        font_assets: Res<FontAssets>,
        mut commands : Commands,
    ) {
        let debug_menu = DebugMenu::new(&settings, &font_assets, &mut commands);
        commands.insert_resource(debug_menu);
    }

    pub fn debug_menu_update(
        menu : Res<DebugMenu>,
        input : Res<Input<KeyCode>>,
        time : Res<Time>,
        mut fps_counter : ResMut<FPSCounter>,
        mut visible_query: Query<(&mut Visibility, &InheritedVisibility)>,
        mut texts : Query<&mut Text>,
    ) {

        fps_counter.timer.tick(time.delta());
        fps_counter.frames += 1;
        fps_counter.frames_total += 1;
        if input.just_pressed(KeyCode::F3) {
            menu.toggle(&mut visible_query);
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
}

impl Menu for DebugMenu {
    fn main_container(&self) -> Entity {
        self.container
    }
}

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::SingleplayerGame), Self::create_debug_ui)
            .add_systems(Update, Self::debug_menu_update.run_if(in_state(GameState::SingleplayerGame)));
    }
}