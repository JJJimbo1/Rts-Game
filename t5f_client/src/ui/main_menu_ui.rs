use crate::*;

pub fn create_main_menu(
    settings : Res<MenuSettings>,
    mut font_assets: ResMut<FontAssets>,
    mut commands : Commands,
) {
    let main_menu = MainMenuUi::new(&settings, &mut font_assets, &mut commands);
    commands.insert_resource(main_menu);
}

#[derive(Debug, Copy, Clone)]
#[derive(Resource)]
pub struct MainMenuUi {
    pub container : Entity,
    pub campaign : CampaignUi,
    pub skirmish : SkirmishUi,
}

impl MainMenuUi {
    pub fn new(
        settings : &MenuSettings,
        font_assets: &mut FontAssets,
        // textures : &QLoader<ImageAsset, AssetServer>,
        // fonts : &QLoader<FontAsset, AssetServer>,
        // materials: &mut Assets<ColorMaterial>,
        commands : &mut Commands,
    ) -> Self {
        let font: Handle<Font> = font_assets.roboto.clone();
        let font_size = FONT_SIZE_LARGE * settings.font_size;

        let mut entity_commands = commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(PRIMARY_MENU_MARGIN),
                width: Val::Px(MENU_WIDTH),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color : DARK_BACKGROUND_COLOR.into(),
            visibility : Visibility::Visible,
            ..Default::default()
        });
        entity_commands.insert(DeleteOnStateChange);

        let container_entity = entity_commands.id();

        entity_commands.with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(600.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::TopMenu(TopMenuButtonsEvent::Campaign))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Campaign",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_NORMAL,
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(500.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::TopMenu(TopMenuButtonsEvent::Skirmish))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Skirmish",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_NORMAL,
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(400.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::TopMenu(TopMenuButtonsEvent::Options))
            .insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Options",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_UNUSED,
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(300.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::TopMenu(TopMenuButtonsEvent::Quit))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Quit",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_NORMAL,
                        },
                    ),
                    ..Default::default()
                });
            });
        });

        Self {
            container : container_entity,
            campaign : CampaignUi::new(settings, font_assets, commands),
            skirmish : SkirmishUi::new(settings, font_assets, commands),
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
        font_assets: &mut FontAssets,
        commands : &mut Commands,
    ) -> Self {
        let font: Handle<Font> = font_assets.roboto.clone();
        let font_size = FONT_SIZE_LARGE * settings.font_size;

        let mut entity_commands = commands.spawn(NodeBundle {
            style: Style {
                position_type : PositionType::Absolute,
                left: Val::Px(SECONDARY_MENU_MARGIN),
                width: Val::Px(MENU_WIDTH),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color : DARK_BACKGROUND_COLOR.into(),
            visibility : Visibility::Hidden,
            ..Default::default()
        });
        entity_commands.insert(DeleteOnStateChange);

        let container_entity = entity_commands.id();

        entity_commands.with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(600.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::Campaign(CampaignButtonsEvent::Continue))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Continue",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_NORMAL,
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(500.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::Campaign(CampaignButtonsEvent::LevelSelect))
            //.insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Level Select",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_NORMAL,
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(400.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::Campaign(CampaignButtonsEvent::LoadGame))
            .insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Load Game",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_UNUSED,
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(300.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::Campaign(CampaignButtonsEvent::CustomGame))
            .insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Custom Game",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_UNUSED,
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(200.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::Campaign(CampaignButtonsEvent::Back))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Back",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_NORMAL,
                        },
                    ),
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
        font_assets: &mut FontAssets,
        commands : &mut Commands,
    ) -> Self {
        let font: Handle<Font> = font_assets.roboto.clone();
        let font_size = FONT_SIZE_LARGE * settings.font_size;

        let mut entity_commands = commands.spawn(NodeBundle {
            style: Style {
                position_type : PositionType::Absolute,
                left : Val::Px(SECONDARY_MENU_MARGIN),
                width: Val::Px(MENU_WIDTH),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color : DARK_BACKGROUND_COLOR.into(),
            visibility : Visibility::Hidden,
            ..Default::default()
        });
        entity_commands.insert(DeleteOnStateChange);

        let container_entity = entity_commands.id();

        entity_commands.with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(600.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::Continue))
            .insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Continue",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_UNUSED,
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(500.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::NewGame))
            .insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "New Game",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_UNUSED,
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(400.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::LoadGame))
            .insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Load Game",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_UNUSED,
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(300.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color : EMPTY_COLOR.into(),
                ..Default::default()
            }).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::Back))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Back",
                        TextStyle {
                            font: font.clone(),
                            font_size: font_size,
                            color: TEXT_COLOR_NORMAL,
                        },
                    ),
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

// #[derive(Debug, Copy, Clone)]
// pub struct OptionsUi {
//     container : Entity,
// }

// impl OptionsUi {
//     pub fn new(
//         settings : &MenuSettings,
//         font_assets: &mut FontAssets,
//         commands : &mut Commands,
//     ) -> Self {
//         let font: Handle<Font> = font_assets.roboto.clone();
//         let font_size = FONT_SIZE_LARGE * settings.font_size;

//         let mut entity_commands = commands.spawn(NodeBundle {
//             style: Style {
//                 position_type : PositionType::Absolute,
//                 left : Val::Px(SECONDARY_MENU_MARGIN),
//                 width: Val::Px(MENU_WIDTH),
//                 height: Val::Percent(100.0),
//                 justify_content: JustifyContent::Center,
//                 ..Default::default()
//             },
//             background_color : DARK_BACKGROUND_COLOR.into(),
//             visibility : Visibility::Hidden,
//             ..Default::default()
//         });
//         entity_commands.insert(DeleteOnStateChange);

//         let container_entity = entity_commands.id();

//         entity_commands.with_children(|parent| {
//             parent.spawn(ButtonBundle {
//                 style: Style {
//                     position_type : PositionType::Absolute,
//                     bottom : Val::Px(600.0),
//                     width: Val::Px(font_size * 10.0),
//                     height: Val::Px(font_size),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     ..Default::default()
//                 },
//                 background_color : EMPTY_COLOR.into(),
//                 ..Default::default()
//             }).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::Continue))
//             .insert(InactiveButton)
//             .with_children(|parent| {
//                 parent.spawn(TextBundle {
//                     text: Text::from_section(
//                         "Continue",
//                         TextStyle {
//                             font: font.clone(),
//                             font_size: font_size,
//                             color: TEXT_COLOR_UNUSED,
//                         },
//                     ),
//                     ..Default::default()
//                 });
//             });

//             parent.spawn(ButtonBundle {
//                 style: Style {
//                     position_type : PositionType::Absolute,
//                     bottom : Val::Px(500.0),
//                     width: Val::Px(font_size * 10.0),
//                     height: Val::Px(font_size),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     ..Default::default()
//                 },
//                 background_color : EMPTY_COLOR.into(),
//                 ..Default::default()
//             }).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::NewGame))
//             .insert(InactiveButton)
//             .with_children(|parent| {
//                 parent.spawn(TextBundle {
//                     text: Text::from_section(
//                         "New Game",
//                         TextStyle {
//                             font: font.clone(),
//                             font_size: font_size,
//                             color: TEXT_COLOR_UNUSED,
//                         },
//                     ),
//                     ..Default::default()
//                 });
//             });

//             parent.spawn(ButtonBundle {
//                 style: Style {
//                     position_type : PositionType::Absolute,
//                     bottom : Val::Px(400.0),
//                     width: Val::Px(font_size * 10.0),
//                     height: Val::Px(font_size),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     ..Default::default()
//                 },
//                 background_color : EMPTY_COLOR.into(),
//                 ..Default::default()
//             }).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::LoadGame))
//             .insert(InactiveButton)
//             .with_children(|parent| {
//                 parent.spawn(TextBundle {
//                     text: Text::from_section(
//                         "Load Game",
//                         TextStyle {
//                             font: font.clone(),
//                             font_size: font_size,
//                             color: TEXT_COLOR_UNUSED,
//                         },
//                     ),
//                     ..Default::default()
//                 });
//             });

//             parent.spawn(ButtonBundle {
//                 style: Style {
//                     position_type : PositionType::Absolute,
//                     bottom : Val::Px(300.0),
//                     width: Val::Px(font_size * 10.0),
//                     height: Val::Px(font_size),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     ..Default::default()
//                 },
//                 background_color : EMPTY_COLOR.into(),
//                 ..Default::default()
//             }).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::Back))
//             .with_children(|parent| {
//                 parent.spawn(TextBundle {
//                     text: Text::from_section(
//                         "Back",
//                         TextStyle {
//                             font: font.clone(),
//                             font_size: font_size,
//                             color: TEXT_COLOR_NORMAL,
//                         },
//                     ),
//                     ..Default::default()
//                 });
//             });
//         });

//         Self {
//             container : container_entity,
//         }
//     }
// }

// impl Menu for OptionsUi {
//     fn main_container(&self) -> Entity {
//         self.container
//     }
// }

pub fn main_menu_ui_button_event_writer_system(
    mut main_menu_button_events : EventWriter<TopMenuButtonsEvent>,
    mut campaign_button_events : EventWriter<CampaignButtonsEvent>,
    mut skirmish_button_events : EventWriter<SkirmishButtonsEvent>,
    interaction_query: Query<
    (&Interaction, &MainMenuButtons, &InheritedVisibility),
    (Changed<Interaction>, With<Button>)>
) {
    interaction_query.for_each(|(int, but, visible)| {
        if !visible.get() { return; }
        match *int {
            Interaction::Pressed => {
                match but {
                    MainMenuButtons::TopMenu(mmb) => {
                        main_menu_button_events.send(*mmb);
                    },
                    MainMenuButtons::Campaign(cb) => {
                        campaign_button_events.send(*cb);
                    }
                    MainMenuButtons::Skirmish(sb) => {
                        skirmish_button_events.send(*sb);
                    },

                }
            },
            Interaction::Hovered => { },
            Interaction::None => { }
        }
    })
}
