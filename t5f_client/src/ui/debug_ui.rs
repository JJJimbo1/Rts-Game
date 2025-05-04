use bevy::prelude::*;
use crate::*;

#[derive(Copy, Clone)]
#[derive(Component)]
pub struct DebugMenu;

#[derive(Debug, Clone)]
#[derive(Component)]
pub struct FPSCounter {
    pub timer: Timer,
    pub frames: u32,
}

impl Default for FPSCounter {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            frames: 0,
        }
    }
}

#[derive(Debug, Default, Clone)]
#[derive(Component)]
pub struct FrameCounter {
    pub frame: u64,
}

pub struct DebugUIPlugin;

impl DebugUIPlugin {
    pub fn spawn(
        settings : Res<MenuSettings>,
        font_assets: Res<FontAssets>,
        mut commands : Commands,
    ) {
        let font = font_assets.roboto.clone();
        let font_size = FONT_SIZE_SMALL * settings.font_size;

        commands.spawn((
            DebugMenu,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                width: Val::Px(200.0),
                height: Val::Px(400.0),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(DARK_BACKGROUND_COLOR),
            Visibility::Hidden,
        )).with_children(|parent| {
            parent.spawn((
                FPSCounter::default(),
                Text::new("FPS: 0"),
                TextFont {
                    font : font.clone(),
                    font_size,
                    ..default()
                },
                Node {
                    min_width: Val::Percent(100.0),
                    ..default()
                },
            ));
            parent.spawn((
                FrameCounter::default(),
                Text::new("FRAME: 0"),
                TextFont {
                    font : font.clone(),
                    font_size,
                    ..default()
                },
                Node {
                    min_width: Val::Percent(100.0),
                    ..default()
                },
            ));
        });
    }

    pub fn update(
        input: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
        mut debug_menu: ParamSet<(
            Query<Entity, With<DebugMenu>>,
            Query<(&mut Text, &mut FPSCounter)>,
            Query<(&mut Text, &mut FrameCounter)>,
        )>,
        mut visible_query: Query<(&mut Visibility, &InheritedVisibility)>,
    ) {
        if let Ok(menu) = debug_menu.p0().get_single() {
            if input.just_pressed(KeyCode::F3) {
                toggle(&mut visible_query, menu);
            }
        }

        if let Ok((mut text, mut counter)) = debug_menu.p1().get_single_mut() {
            counter.timer.tick(time.delta());
            counter.frames += 1;
            if counter.timer.finished() {
                text.0 = format!("FPS: {:.*}", 1, counter.frames as f32 / (counter.timer.elapsed_secs() + counter.timer.times_finished_this_tick() as f32 * counter.timer.duration().as_secs_f32()));
                counter.timer.reset();
                counter.frames = 0;
            }
        }

        if let Ok((mut text, mut counter)) = debug_menu.p2().get_single_mut() {
            counter.frame += 1;
            text.0 = format!("FRAME#: {}", counter.frame);
        }
    }
}

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::SingleplayerGame), Self::spawn)
            .add_systems(Update, Self::update.run_if(in_state(GameState::SingleplayerGame)));
    }
}