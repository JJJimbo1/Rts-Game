pub mod frame_data_ui {

    use bevy::prelude::*;

    use qloader::*;
    use crate::*;

    pub fn create_debug_menu(settings : Res<MenuSettings>, textures : Res<QLoader<ImageAsset, AssetServer>>, fonts : Res<QLoader<FontAsset, AssetServer>>, mut commands : Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
        let debug_menu = DebugMenu::new(settings, textures, fonts, &mut commands, materials);
        commands.insert_resource(debug_menu);
    }

    #[derive(Copy, Clone)]
    pub struct DebugMenu {
        container : Entity,
        fps_counter : Entity,
        frame_number : Entity,
    }

    impl DebugMenu {
        pub fn new(
            settings : Res<MenuSettings>,
            _textures : Res<QLoader<ImageAsset, AssetServer>>,
            fonts : Res<QLoader<FontAsset, AssetServer>>,
            commands : &mut Commands,
            mut materials: ResMut<Assets<ColorMaterial>>
        ) -> Self {
            let font = fonts.get("square").unwrap().0.clone();

            let font_size = FONT_SIZE_SMALL * settings.font_size;

            let mut entity_commands = commands.spawn_bundle(NodeBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position: Rect {
                        top : Val::Px(10.0),
                        left: Val::Px(10.0),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(200.0), Val::Px(400.0)),
                    ..Default::default()
                },
                color : UiColor(DARK_BACKGROUND_COLOR.into()),
                visibility : Visibility { is_visible : false},
                ..Default::default()
            });

            let container_entity = entity_commands.id();
            let mut fps_counter_entity = None;

            let entity_commands = entity_commands.with_children(|parent| {
                fps_counter_entity = Some(parent.spawn_bundle(TextBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position: Rect {
                            top : Val::Px(10.0),
                            left: Val::Px(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "FPS : 0",
                        TextStyle {
                            font : font.clone(),
                            font_size,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    visibility : Visibility { is_visible : false},
                    ..Default::default()
                }).id());
            });

            let mut frame_number_entity = None;

            entity_commands.with_children(|parent| {
                // text
                frame_number_entity = Some(parent.spawn_bundle(TextBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position: Rect {
                            top : Val::Px(font_size + 20.0),
                            left: Val::Px(10.0),
                            // top: Val::Px(0.0),
                            ..Default::default()
                        },
                        //margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "FRAME# : 0",
                        TextStyle {
                            font : font.clone(),
                            font_size,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    visibility : Visibility { is_visible : false},
                    ..Default::default()
                }).id());
            });


            Self {
                container : container_entity,
                fps_counter : fps_counter_entity.unwrap(),
                frame_number : frame_number_entity.unwrap(),
            }
        }

        // pub fn update(&self, world : &mut World) {
        //     let storage = world.read_storage::<HiddenPropagate>();
        //     match storage.get(self.container) {
        //         Some(_) => { },
        //         None => {
        //             let mut ui_text_storage = world.write_storage::<UiText>();

        //             match ui_text_storage.get_mut(self.fps) {
        //                 Some(x) => {
        //                     if world.read_resource::<Time>().frame_number() % 20 == 0 {
        //                         let fps = world.read_resource::<FpsCounter>().sampled_fps();
        //                         x.text = format!("FPS: {:.*}", 1, fps);
        //                     }
        //                 },
        //                 None => {
        //                     log::warn!("Ui_text_storage does not contain an fps counter entity... Somehow.");
        //                 }
        //             }

        //             match ui_text_storage.get_mut(self.frame_number) {
        //                 Some(x) => {
        //                     x.text = world.read_resource::<Time>().frame_number().to_string();
        //                 },
        //                 None => {
        //                     log::warn!("Ui_text_storage does not contain an frame number counter entity... Somehow.");
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    pub fn debug_menu_update(
        menu : Res<DebugMenu>,
        input : Res<Input<KeyCode>>,
        time : Res<Time>,
        mut fps_counter : ResMut<FPSCounter>,
        children : Query<&Children>,
        mut visibles : Query<&mut Visibility>,
        mut texts : Query<&mut Text>,
    ) {

        fps_counter.timer.tick(time.delta());
        fps_counter.frames += 1;
        fps_counter.frames_total += 1;
        if input.just_pressed(KeyCode::F3) {
            menu.toggle(&mut visibles, &children);
        }

        if let Ok(mut text) = texts.get_mut(menu.fps_counter) {
            if fps_counter.timer.finished() {
                text.sections[0].value = format!("FPS: {:.*}", 1, fps_counter.frames as f32 / (fps_counter.timer.elapsed_secs() + fps_counter.timer.times_finished() as f32 * fps_counter.timer.duration().as_secs_f32()));
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
}