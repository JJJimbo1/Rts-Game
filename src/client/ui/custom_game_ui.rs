use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct Ticker;

#[derive(Debug, Copy, Clone)]
#[derive(Resource)]
pub struct CustomGameUi {
    pub container: Entity,
    pub ticker: Entity,
}

impl CustomGameUi {
    pub fn new(
        settings : &MenuSettings,
        font_assets: &mut FontAssets,
        // textures : &QLoader<ImageAsset, AssetServer>,
        // fonts : &QLoader<FontAsset, AssetServer>,
        // materials: &mut Assets<ColorMaterial>,
        commands : &mut Commands,
    ) -> Self {
        let font: Handle<Font> = font_assets.roboto.clone();
        let font_size = FONT_SIZE_LARGE * settings.font_size * 2.0;

        let mut entity_commands = commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(500.0),
                height: Val::Px(500.0),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(DARK_BACKGROUND_COLOR),
            Visibility::Visible,
            DeleteOnStateChange
        ));

        let container_entity = entity_commands.id();
        let mut ticker = None;

        entity_commands.with_children(|parent| {
            ticker = Some(parent.spawn((
                Text::new("0"),
                TextFont {
                    font: font.clone(),
                    font_size,
                    ..default()
                },
                TextColor(TEXT_COLOR_NORMAL),
            )).id());

            parent.spawn((
                Button,
                Node {
                    position_type : PositionType::Absolute,
                    bottom : Val::Px(50.0),
                    width: Val::Px(font_size * 4.0),
                    height: Val::Px(font_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BackgroundColor(EMPTY_COLOR),
                Ticker,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Tick"),
                    TextFont {
                        font: font.clone(),
                        font_size: font_size,
                        ..default()
                    },
                    TextColor(TEXT_COLOR_NORMAL),
                ));
            });

        });

        Self {
            container: container_entity,
            ticker: ticker.unwrap(),
        }
    }
}

pub struct CustomGameUIPlugin;

impl CustomGameUIPlugin {
    pub fn create_custom_game_ui(
        settings : Res<MenuSettings>,
        mut font_assets: ResMut<FontAssets>,
        mut commands : Commands,
    ) {
        let custom_game_ui = CustomGameUi::new(&settings, &mut font_assets, &mut commands);
        commands.insert_resource(custom_game_ui);
    }

    /*
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
    */

    pub fn send_uptick(
        mut events: EventWriter<ClientRequest>,
        interaction_query: Query<
        (&Interaction, &InheritedVisibility),
        (Changed<Interaction>, With<Ticker>, With<Button>)>
    ) {
        interaction_query.iter().for_each(|(int, visible)| {
            if !visible.get() { return; }
            match *int {
                Interaction::Pressed => {
                    events.write(ClientRequest::default());
                },
                Interaction::Hovered => { },
                Interaction::None => { }
            }
        })
    }

    pub fn uptick(
        mut events: EventReader<ServerCommand>,
        ui: Res<CustomGameUi>,
        mut texts: Query<&mut Text>,
    ) {
        for _event in events.read() {
            let Ok(mut ticker) = texts.get_mut(ui.ticker) else { return; };
            let value: i32 = (*ticker).0.parse().unwrap();
            (*ticker).0 = (value + 1).to_string();
        }
    }
}

impl Plugin for CustomGameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::CustomGame), Self::create_custom_game_ui)
            .add_systems(Update, ((
                Self::send_uptick,
                Self::uptick,
            )).run_if(in_state(GameState::CustomGame)))
        ;
    }
}