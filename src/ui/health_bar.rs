use bevy::{ecs::component::{ComponentHook, Mutable, StorageType}, prelude::*, ui::widget::NodeImageMode};
use crate::*;

const SIZE: f32 = 16.0;
const HEALTH_PER_SEGMENT: f32 = 200.0;

const HEALTH_BAR_LEFT_MARGIN: f32 = 15.0;
const HEALTH_BAR_RIGHT_MARGIN: f32 = 15.0;
const HEALTH_BAR_TOP_MARGIN: f32 = 4.0;
const HEALTH_BAR_BOTTOM_MARGIN: f32 = 4.0;

#[derive(Debug, Clone, Copy)]
pub struct HealthBar {
    segments: u32,
    root: Entity,
    bar: Entity,
    _decor: Entity,
}

impl HealthBar {
    pub fn vertical_offset() -> f32 {
        HEALTH_BAR_TOP_MARGIN - 1.0
    }

    pub fn slicer() -> TextureSlicer {
        TextureSlicer {
            border: BorderRect {
                left: HEALTH_BAR_LEFT_MARGIN,
                right: HEALTH_BAR_RIGHT_MARGIN,
                top: HEALTH_BAR_TOP_MARGIN,
                bottom: HEALTH_BAR_BOTTOM_MARGIN,
            },
            center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: SCALE * 1.0
        }
    }

    pub fn offset(&self) -> Vec2 {
        Vec2::new(-(SIZE * self.segments as f32 + SIZE * 2.0) / 2.0, -(SIZE / 2.0) as f32)
    }

    pub fn adjust_bar_percent(&self, percent: f32, query: &mut Query<&mut Node>) {
        if let Ok(mut x) = query.get_mut(self.bar) {
            let clamped = percent.clamp(0.0, 1.0);
            let normalized = clamped.remap(0.0, 1.0, 0.0, self.segments as f32 * SIZE);
            x.width = Val::Px(normalized);
        }
    }
}

impl Component for HealthBar {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;

    fn on_remove() -> Option<ComponentHook> {
        Some(|mut world, health_bar| {
            let Some(health_bar) = world.get::<HealthBar>(health_bar.entity).cloned() else { return; };
            world.commands().entity(health_bar.root).despawn();
        })
    }
}

pub struct HealthBarUIPlugin;

impl HealthBarUIPlugin {
    pub fn spawn(
        trigger: Trigger<OnAdd, Health>,
        image_assets: Res<ImageAssets>,
        health: Query<&Health>,
        mut commands: Commands,
    ) {
        let Ok(health) = health.get(trigger.target()) else { return; };
        let segments = (health.max_health() / (HEALTH_PER_SEGMENT * health.dense())).ceil() as u32;

        let root = commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(segments as f32 * SIZE + SIZE * 2.0),
                height: Val::Px(SIZE),
                ..default()
            },
            BackgroundColor(EMPTY_COLOR),
        )).id();

        let bar = commands.spawn((
            ImageNode {
                image: image_assets.health_bar_green.clone().clone(),
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(HealthBar::vertical_offset()),
                left: Val::Px(SIZE),
                width: Val::Px(segments as f32 * SIZE),
                height: Val::Px(SIZE * 0.625),
                ..default()
            },
            ChildOf(root),
        )).id();

        let decor = commands.spawn((
            ImageNode {
                image: image_assets.health_bar.clone(),
                image_mode: NodeImageMode::Sliced(HealthBar::slicer()),
                ..default()
            },
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ChildOf(root),
        )).id();

        commands.entity(trigger.target()).insert(HealthBar {
            segments,
            root,
            bar,
            _decor: decor,
        });
    }

    pub fn update_health_bars(
        camera: Res<CameraController>,
        health_bars: Query<(&Transform, &Health, &HealthBar)>,
        mut nodes: Query<&mut Node>,
        mut visible_query: Query<(&mut Visibility, &InheritedVisibility)>,
        cameras: Query<(&Camera, &GlobalTransform)>,
    ) {
        let (camera, camera_transform) = cameras.get(camera.camera).unwrap();
        health_bars.iter().for_each(|(tran, hel, bar)| {
            if hel.is_full_health() {
                close(&mut visible_query, bar.root);
            } else {
                open(&mut visible_query, bar.root);
                match camera.world_to_viewport(camera_transform, tran.translation) {
                    Ok(point) => {
                        if let Ok(mut s) = nodes.get_mut(bar.root) {
                            let point = point + bar.offset();
                            s.left = Val::Px(point.x);
                            //TODO: find some way to get how far up the screen to put the health bar.
                            s.top = Val::Px(point.y - 50.0);
                        }
                    },
                    Err(_) => { },
                }
                bar.adjust_bar_percent(hel.health_percent(), &mut nodes);
            }
        });
    }
}

impl Plugin for HealthBarUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(Self::spawn)
            .add_systems(Update, (
                // Self::create_health_bars,
                Self::update_health_bars,
            ).run_if(resource_exists::<CameraController>))
        ;
    }
}