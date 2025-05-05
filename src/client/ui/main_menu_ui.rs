use crate::*;

#[derive(Debug, Copy, Clone)]
#[derive(Resource)]
pub struct MainMenuUi {
    pub container : Entity,
    pub campaign : CampaignUi,
    pub custom_game : CustomGameUi,
}

impl MainMenuUi {
    pub fn new(
        settings : &MenuSettings,
        font_assets: &mut FontAssets,
        commands : &mut Commands,
    ) -> Self {
        let font: Handle<Font> = font_assets.roboto.clone();
        let font_size = FONT_SIZE_LARGE * settings.font_size;

        let mut entity_commands = commands.spawn((
			Node {
                position_type: PositionType::Absolute,
                left: Val::Px(PRIMARY_MENU_MARGIN),
                width: Val::Px(MENU_WIDTH),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
			},
			BackgroundColor(DARK_BACKGROUND_COLOR),
			Visibility::Visible,
		));
        entity_commands.insert(DeleteOnStateChange);

        let container_entity = entity_commands.id();

        entity_commands.with_children(|parent| {
            parent.spawn((
				Button,
				Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(600.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::TopMenu(TopMenuButtonsEvent::Campaign))
            .with_children(|parent| {
                parent.spawn((
					Text::new("Campaign"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_NORMAL),
				));
            });

            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(500.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::TopMenu(TopMenuButtonsEvent::Skirmish))
            .with_children(|parent| {
                parent.spawn((
					Text::new("Skirmish"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_NORMAL),
				));
            });

            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(400.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::TopMenu(TopMenuButtonsEvent::Options))
            .insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn((
					Text::new("Options"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_UNUSED),
				));
            });

            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(300.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::TopMenu(TopMenuButtonsEvent::Quit))
            .with_children(|parent| {
                parent.spawn((
					Text::new("Quit"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_NORMAL),
				));
            });
        });

        Self {
            container : container_entity,
            campaign : CampaignUi::new(settings, font_assets, commands),
            custom_game : CustomGameUi::new(settings, font_assets, commands),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CampaignUi {
    pub container : Entity,
}

impl CampaignUi {
    pub fn new(
        settings : &MenuSettings,
        font_assets: &mut FontAssets,
        commands : &mut Commands,
    ) -> Self {
        let font: Handle<Font> = font_assets.roboto.clone();
        let font_size = FONT_SIZE_LARGE * settings.font_size;

        let mut entity_commands = commands.spawn((
			Node {
                position_type : PositionType::Absolute,
                left: Val::Px(SECONDARY_MENU_MARGIN),
                width: Val::Px(MENU_WIDTH),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
			},
			BackgroundColor(DARK_BACKGROUND_COLOR),
			Visibility::Hidden,
		));
        entity_commands.insert(DeleteOnStateChange);

        let container_entity = entity_commands.id();

        entity_commands.with_children(|parent| {
            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(600.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::Campaign(CampaignButtonsEvent::Continue))
            .with_children(|parent| {
                parent.spawn((
					Text::new("Continue"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_NORMAL),
				));
            });

            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(500.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::Campaign(CampaignButtonsEvent::LevelSelect))
            .with_children(|parent| {
                parent.spawn((
					Text::new("Level Select"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_NORMAL),
				));
            });

            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(400.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::Campaign(CampaignButtonsEvent::LoadGame))
            .insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn((
					Text::new("Load Game"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_UNUSED),
				));
            });

            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(300.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::Campaign(CampaignButtonsEvent::CustomGame))
            .insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn((
					Text::new("Custom Game"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_UNUSED),
				));
            });

            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(200.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::Campaign(CampaignButtonsEvent::Back))
            .with_children(|parent| {
                parent.spawn((
					Text::new("Back"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_NORMAL),
				));
            });
        });

        Self {
            container : container_entity,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CustomGameUi {
    pub container : Entity,
}

impl CustomGameUi {
    pub fn new(
        settings : &MenuSettings,
        font_assets: &mut FontAssets,
        commands : &mut Commands,
    ) -> Self {
        let font: Handle<Font> = font_assets.roboto.clone();
        let font_size = FONT_SIZE_LARGE * settings.font_size;

        let mut entity_commands = commands.spawn((
			Node {
                position_type : PositionType::Absolute,
                left : Val::Px(SECONDARY_MENU_MARGIN),
                width: Val::Px(MENU_WIDTH),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
			},
			BackgroundColor(DARK_BACKGROUND_COLOR),
			Visibility::Hidden,
		));
        entity_commands.insert(DeleteOnStateChange);

        let container_entity = entity_commands.id();

        entity_commands.with_children(|parent| {
            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(600.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::Continue))
            .insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn((
					Text::new("Continue"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_UNUSED),
				));
            });

            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(500.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::NewGame))
            .with_children(|parent| {
                parent.spawn((
					Text::new("New Game"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_NORMAL),
				));
            });

            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(400.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::LoadGame))
            .insert(InactiveButton)
            .with_children(|parent| {
                parent.spawn((
					Text::new("Load Game"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_UNUSED),
				));
            });

            parent.spawn((
				Button,
				Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(300.0),
                    width: Val::Px(font_size * 10.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
				},
				BackgroundColor(EMPTY_COLOR),
			)).insert(MainMenuButtons::Skirmish(SkirmishButtonsEvent::Back))
            .with_children(|parent| {
                parent.spawn((
					Text::new("Back"),
					TextFont {
						font: font.clone(),
						font_size,
						..default()
					},
					TextColor(TEXT_COLOR_NORMAL),
				));
            });
        });

        Self {
            container : container_entity,
        }
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



pub struct MainMenuUIPlugin;

impl MainMenuUIPlugin {
    pub fn create_main_menu_ui(
        settings : Res<MenuSettings>,
        mut font_assets: ResMut<FontAssets>,
        mut commands : Commands,
    ) {
        let main_menu = MainMenuUi::new(&settings, &mut font_assets, &mut commands);
        commands.insert_resource(main_menu);
    }

    pub fn main_menu_ui_button_event_writer(
        mut main_menu_button_events : EventWriter<TopMenuButtonsEvent>,
        mut campaign_button_events : EventWriter<CampaignButtonsEvent>,
        mut skirmish_button_events : EventWriter<SkirmishButtonsEvent>,
        interaction_query: Query<
        (&Interaction, &MainMenuButtons, &InheritedVisibility),
        (Changed<Interaction>, With<Button>)>
    ) {
        interaction_query.iter().for_each(|(int, but, visible)| {
            if !visible.get() { return; }
            match *int {
                Interaction::Pressed => {
                    match but {
                        MainMenuButtons::TopMenu(mmb) => {
                            main_menu_button_events.write(*mmb);
                        },
                        MainMenuButtons::Campaign(cb) => {
                            campaign_button_events.write(*cb);
                        }
                        MainMenuButtons::Skirmish(sb) => {
                            skirmish_button_events.write(*sb);
                        },

                    }
                },
                Interaction::Hovered => { },
                Interaction::None => { }
            }
        })
    }
}

impl Plugin for MainMenuUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), Self::create_main_menu_ui)
            .add_systems(Update, (
                Self::main_menu_ui_button_event_writer.after(ClientUIPlugin::button_updater_system),
            ).run_if(in_state(GameState::MainMenu)))
        ;
    }
}