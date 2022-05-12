pub mod main_menu_ui {

    use qloader::*;
    use crate::*;

    pub fn create_main_menu(settings : Res<MenuSettings>, textures : Res<QLoader<ImageAsset, AssetServer>>, fonts : Res<QLoader<FontAsset, AssetServer>>, mut commands : Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
        let main_menu = MainMenuUi::new(&settings, &textures, &fonts, &mut materials, &mut commands);
        commands.insert_resource(main_menu);
    }

    #[derive(Debug, Copy, Clone)]
    pub struct MainMenuUi {
        pub container : Entity,
        pub campaign : CampaignUi,
        pub skirmish : SkirmishUi,
    }

    impl MainMenuUi {
        pub fn new(
            settings : &MenuSettings,
            textures : &QLoader<ImageAsset, AssetServer>,
            fonts : &QLoader<FontAsset, AssetServer>,
            materials: &mut Assets<ColorMaterial>,
            commands : &mut Commands,
        ) -> Self {
            let font = fonts.get("Roboto-Black").unwrap().0.clone();
            let font_size = FONT_SIZE_LARGE * settings.font_size;

            let mut entity_commands = commands.spawn_bundle(NodeBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position : Rect {
                        left : Val::Px(PRIMARY_MENU_MARGIN),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(MENU_WIDTH), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color : UiColor(DARK_BACKGROUND_COLOR.into()),
                visibility : Visibility { is_visible : true},
                ..Default::default()
            });
            entity_commands.insert(DeleteOnStateChange);

            let container_entity = entity_commands.id();

            entity_commands.with_children(|parent| {
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position: Rect {
                            left : Val::Auto,
                            right : Val::Auto,
                            top : Val::Auto,
                            bottom : Val::Px(600.0),
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    ..Default::default()
                }).insert(MainMenuButtons::TopMenu(TopMenuButtons::Campaign))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Campaign",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_NORMAL,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });

                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            left : Val::Auto,
                            right : Val::Auto,
                            top : Val::Auto,
                            bottom : Val::Px(500.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    ..Default::default()
                }).insert(MainMenuButtons::TopMenu(TopMenuButtons::Skirmish))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Skirmish",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_NORMAL,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });

                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            left : Val::Auto,
                            right : Val::Auto,
                            top : Val::Auto,
                            bottom : Val::Px(400.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    ..Default::default()
                }).insert(MainMenuButtons::TopMenu(TopMenuButtons::Options))
                .insert(InactiveButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Options",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_UNUSED,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });

                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            left : Val::Auto,
                            right : Val::Auto,
                            top : Val::Auto,
                            bottom : Val::Px(300.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    ..Default::default()
                }).insert(MainMenuButtons::TopMenu(TopMenuButtons::Quit))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Quit",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_NORMAL,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            });

            Self {
                container : container_entity,
                campaign : CampaignUi::new(settings, textures, fonts, materials, commands),
                skirmish : SkirmishUi::new(settings, textures, fonts, materials, commands),
            }
        }
    }

    impl Menu for MainMenuUi {
        fn main_container(&self) -> Entity {
            self.container
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct CampaignUi {
        pub container : Entity,
        // pub map_select : (Entity, MapSelectionUi),
        // pub load : Entity,
        // pub back_button : Entity,
    }

    impl CampaignUi {
        pub fn new(
            settings : &MenuSettings,
            textures : &QLoader<ImageAsset, AssetServer>,
            fonts : &QLoader<FontAsset, AssetServer>,
            materials: &mut Assets<ColorMaterial>,
            commands : &mut Commands,
        ) -> Self {
            let font = fonts.get("Roboto-Black").unwrap().0.clone();
            let font_size = FONT_SIZE_LARGE * settings.font_size;

            let mut entity_commands = commands.spawn_bundle(NodeBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position : Rect {
                        left : Val::Px(SECONDARY_MENU_MARGIN),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(MENU_WIDTH), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color : UiColor(DARK_BACKGROUND_COLOR.into()),
                visibility : Visibility { is_visible : false},
                ..Default::default()
            });
            entity_commands.insert(DeleteOnStateChange);

            let container_entity = entity_commands.id();

            entity_commands.with_children(|parent| {
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position: Rect {
                            left : Val::Auto,
                            right : Val::Auto,
                            top : Val::Auto,
                            bottom : Val::Px(600.0),
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    visibility : Visibility { is_visible : false},
                    ..Default::default()
                }).insert(MainMenuButtons::Campaign(CampaignButtons::Continue))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Continue",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_NORMAL,
                            },
                            Default::default(),
                        ),
                        visibility : Visibility { is_visible : false},
                        ..Default::default()
                    });
                });

                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            left : Val::Auto,
                            right : Val::Auto,
                            top : Val::Auto,
                            bottom : Val::Px(500.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    visibility : Visibility { is_visible : false},
                    ..Default::default()
                }).insert(MainMenuButtons::Campaign(CampaignButtons::LevelSelect))
                //.insert(InactiveButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Level Select",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_NORMAL,
                            },
                            Default::default(),
                        ),
                        visibility : Visibility { is_visible : false},
                        ..Default::default()
                    });
                });

                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            bottom : Val::Px(400.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    visibility : Visibility { is_visible : false},
                    ..Default::default()
                }).insert(MainMenuButtons::Campaign(CampaignButtons::LoadGame))
                .insert(InactiveButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Load Game",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_UNUSED,
                            },
                            Default::default(),
                        ),
                        visibility : Visibility { is_visible : false},
                        ..Default::default()
                    });
                });

                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            bottom : Val::Px(300.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    visibility : Visibility { is_visible : false},
                    ..Default::default()
                }).insert(MainMenuButtons::Campaign(CampaignButtons::CustomGame))
                .insert(InactiveButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Custom Game",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_UNUSED,
                            },
                            Default::default(),
                        ),
                        visibility : Visibility { is_visible : false},
                        ..Default::default()
                    });
                });

                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            bottom : Val::Px(200.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    visibility : Visibility { is_visible : false},
                    ..Default::default()
                }).insert(MainMenuButtons::Campaign(CampaignButtons::Back))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Back",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_NORMAL,
                            },
                            Default::default(),
                        ),
                        visibility : Visibility { is_visible : false},
                        ..Default::default()
                    });
                });
            });

            Self {
                container : container_entity,
            }
        }
    }

    impl Menu for CampaignUi {
        fn main_container(&self) -> Entity {
            self.container
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct SkirmishUi {
        container : Entity,
    }

    impl SkirmishUi {
        pub fn new(
            settings : &MenuSettings,
            textures : &QLoader<ImageAsset, AssetServer>,
            fonts : &QLoader<FontAsset, AssetServer>,
            materials: &mut Assets<ColorMaterial>,
            commands : &mut Commands,
        ) -> Self {
            let font = fonts.get("Roboto-Black").unwrap().0.clone();
            let font_size = FONT_SIZE_LARGE * settings.font_size;

            let mut entity_commands = commands.spawn_bundle(NodeBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position : Rect {
                        left : Val::Px(SECONDARY_MENU_MARGIN),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(MENU_WIDTH), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color : UiColor(DARK_BACKGROUND_COLOR.into()),
                visibility : Visibility { is_visible : false},
                ..Default::default()
            });
            entity_commands.insert(DeleteOnStateChange);

            let container_entity = entity_commands.id();

            entity_commands.with_children(|parent| {
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position: Rect {
                            bottom : Val::Px(600.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    visibility : Visibility { is_visible : false},
                    ..Default::default()
                }).insert(MainMenuButtons::Skirmish(SkirmishButtons::Continue))
                .insert(InactiveButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Continue",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_UNUSED,
                            },
                            Default::default(),
                        ),
                        visibility : Visibility { is_visible : false},
                        ..Default::default()
                    });
                });

                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            bottom : Val::Px(500.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    visibility : Visibility { is_visible : false},
                    ..Default::default()
                }).insert(MainMenuButtons::Skirmish(SkirmishButtons::NewGame))
                .insert(InactiveButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "New Game",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_UNUSED,
                            },
                            Default::default(),
                        ),
                        visibility : Visibility { is_visible : false},
                        ..Default::default()
                    });
                });

                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            bottom : Val::Px(400.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    visibility : Visibility { is_visible : false},
                    ..Default::default()
                }).insert(MainMenuButtons::Skirmish(SkirmishButtons::LoadGame))
                .insert(InactiveButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Load Game",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_UNUSED,
                            },
                            Default::default(),
                        ),
                        visibility : Visibility { is_visible : false},
                        ..Default::default()
                    });
                });

                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            bottom : Val::Px(300.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(font_size * 10.0), Val::Px(font_size)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color : UiColor(EMPTY_COLOR.into()),
                    visibility : Visibility { is_visible : false},
                    ..Default::default()
                }).insert(MainMenuButtons::Skirmish(SkirmishButtons::Back))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Back",
                            TextStyle {
                                font: font.clone(),
                                font_size: font_size,
                                color: TEXT_COLOR_NORMAL,
                            },
                            Default::default(),
                        ),
                        visibility : Visibility { is_visible : false},
                        ..Default::default()
                    });
                });
            });

            Self {
                container : container_entity,
            }
        }
    }

    impl Menu for SkirmishUi {
        fn main_container(&self) -> Entity {
            self.container
        }
    }
}