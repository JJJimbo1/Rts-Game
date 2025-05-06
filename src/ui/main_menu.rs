use bevy::prelude::*;
use crate::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct MainMenu;

#[derive(Debug, Clone, Copy, Component)]
pub struct CampaignMenu;

#[derive(Debug, Clone, Copy, Component)]
pub struct CustomGameMenu;

pub struct MainMenuPlugin;

impl MainMenuPlugin {
    pub fn spawn(
        settings: Res<MenuSettings>,
        font_assets: Res<FontAssets>,
        mut commands: Commands,
    ) {
        let font: Handle<Font> = font_assets.roboto.clone();
        let font_size = FONT_SIZE_LARGE * settings.font_size;

        commands.spawn(Camera3d::default());

        let main_menu = commands.spawn((
            MainMenu,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(PRIMARY_MENU_MARGIN),
                width: Val::Px(MENU_WIDTH),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(DARK_BACKGROUND_COLOR),
        )).id();

        commands.spawn((
            Button,
            Text::new("Campaign"),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(TEXT_COLOR_NORMAL),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(600.0),
                ..default()
            },
            ChildOf(main_menu),
        )).observe(|
            _: Trigger<Pointer<Click>>,
            campaign_menu: Query<Entity, With<CampaignMenu>>,
            custom_game_menu: Query<Entity, With<CustomGameMenu>>,
            mut visible_query: Query<(&mut Visibility, &InheritedVisibility)>,
        | {
            let Ok(campaign_menu) = campaign_menu.single() else { return; };
            let Ok(custom_game_menu) = custom_game_menu.single() else { return; };

            close(&mut visible_query, custom_game_menu);
            toggle(&mut visible_query, campaign_menu);
        });

        commands.spawn((
            Button,
            Text::new("Custom Game"),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(TEXT_COLOR_NORMAL),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(500.0),
                ..default()
            },
            ChildOf(main_menu),
        )).observe(|
            _: Trigger<Pointer<Click>>,
            campaign_menu: Query<Entity, With<CampaignMenu>>,
            custom_game_menu: Query<Entity, With<CustomGameMenu>>,
            mut visible_query: Query<(&mut Visibility, &InheritedVisibility)>,
        | {
            let Ok(campaign_menu) = campaign_menu.single() else { return; };
            let Ok(custom_game_menu) = custom_game_menu.single() else { return; };

            close(&mut visible_query, campaign_menu);
            toggle(&mut visible_query, custom_game_menu);
        });

        commands.spawn((
            Button,
            Text::new("Options"),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(TEXT_COLOR_NORMAL),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(400.0),
                ..default()
            },
            ChildOf(main_menu),
        ));

        commands.spawn((
            Button,
            Text::new("Quit"),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(TEXT_COLOR_NORMAL),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(300.0),
                ..default()
            },
            ChildOf(main_menu),
        )).observe(|
            _: Trigger<Pointer<Click>>,
            mut quit_app: EventWriter<AppExit>,
        | {
            quit_app.write(AppExit::Success);
        });

        let campaign_menu = commands.spawn((
            CampaignMenu,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(SECONDARY_MENU_MARGIN),
                width: Val::Px(MENU_WIDTH),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(DARK_BACKGROUND_COLOR),
            Visibility::Hidden,
            ChildOf(main_menu),
        )).id();

        commands.spawn((
            Button,
            Text::new("Continue"),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(TEXT_COLOR_NORMAL),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(600.0),
                ..default()
            },
            ChildOf(campaign_menu),
        )).observe(|
            _: Trigger<Pointer<Click>>,
            mut state: ResMut<NextState<GameState>>,
            mut commands: Commands,
        | {
            commands.insert_resource(SaveFile::File("saves/developer.ron".to_string()));
            state.set(GameState::MatchLoadingState);
        });

        commands.spawn((
            Button,
            Text::new("Missions"),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(TEXT_COLOR_NORMAL),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(500.0),
                ..default()
            },
            ChildOf(campaign_menu),
        )).observe(|
            _: Trigger<Pointer<Click>>,
            mut state: ResMut<NextState<GameState>>,
            mut commands: Commands,
        | {
            commands.insert_resource(SaveFile::File("levels/developer.ron".to_string()));
            state.set(GameState::MatchLoadingState);
        });

        commands.spawn((
            Button,
            Text::new("Load Game"),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(TEXT_COLOR_NORMAL),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(400.0),
                ..default()
            },
            ChildOf(campaign_menu),
        ));

        let custom_game_menu = commands.spawn((
            CustomGameMenu,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(SECONDARY_MENU_MARGIN),
                width: Val::Px(MENU_WIDTH),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(DARK_BACKGROUND_COLOR),
            Visibility::Hidden,
            ChildOf(main_menu),
        )).id();

        commands.spawn((
            Button,
            Text::new("New Game"),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(TEXT_COLOR_NORMAL),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(600.0),
                ..default()
            },
            ChildOf(custom_game_menu),
        )).observe(|
            _: Trigger<Pointer<Click>>,
            mut state: ResMut<NextState<GameState>>,
        | {
            state.set(GameState::CustomGame);
        });

        commands.spawn((
            Button,
            Text::new("Load Game"),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(TEXT_COLOR_NORMAL),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(500.0),
                ..default()
            },
            ChildOf(custom_game_menu),
        ));
    }

    pub fn despawn(
        camera: Query<Entity, With<Camera>>,
        main_menu: Query<Entity, With<MainMenu>>,
        mut commands: Commands,
    ) {
        let Ok(camera) = camera.single() else { return; };
        let Ok(main_menu) = main_menu.single() else { return; };
        commands.entity(camera).despawn();
        commands.entity(main_menu).despawn();
    }

    pub fn pressed(
        trigger: Trigger<Pointer<Pressed>>,
        mut text_color: Query<&mut TextColor, With<Button>>,
    ) {
        let Ok(mut text_color) = text_color.get_mut(trigger.target()) else { return; };
        text_color.0 = TEXT_COLOR_PRESS;
    }

    pub fn released(
        trigger: Trigger<Pointer<Released>>,
        mut text_color: Query<&mut TextColor, With<Button>>,
    ) {
        let Ok(mut text_color) = text_color.get_mut(trigger.target()) else { return; };
        text_color.0 = TEXT_COLOR_HOVER;
    }

    pub fn over(
        trigger: Trigger<Pointer<Over>>,
        mut text_color: Query<&mut TextColor, With<Button>>,
    ) {
        let Ok(mut text_color) = text_color.get_mut(trigger.target()) else { return; };
        text_color.0 = TEXT_COLOR_HOVER;
    }

    pub fn out(
        trigger: Trigger<Pointer<Out>>,
        mut text_color: Query<&mut TextColor, With<Button>>,
    ) {
        let Ok(mut text_color) = text_color.get_mut(trigger.target()) else { return; };
        text_color.0 = TEXT_COLOR_NORMAL;
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), Self::spawn)
            .add_systems(OnExit(GameState::MainMenu), Self::despawn)
            .add_observer(Self::pressed)
            .add_observer(Self::released)
            .add_observer(Self::over)
            .add_observer(Self::out)
        ;
    }
}