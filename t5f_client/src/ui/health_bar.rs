use bevy::{ecs::component::{ComponentHooks, StorageType}, prelude::*, ui::widget::NodeImageMode};
use t5f_utility::mathfu::d1;
use crate::*;

const SIZE: f32 = 16.0;
const HEALTH_PER_SEGMENT: f32 = 200.0;

#[derive(Debug, Clone, Copy)]
pub struct HealthBar {
    segments: u32,
    root: Entity,
    bar: Entity,
    decor: Entity,
}

impl HealthBar {
    pub fn new(
        segments: u32,
        image_assets: &ImageAssets,
        commands: &mut Commands
    ) -> Self{
        let root;
        let mut bar = Entity::PLACEHOLDER;
        let mut decor = Entity::PLACEHOLDER;

        root = commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(segments as f32 * SIZE + SIZE * 2.0),
                height: Val::Px(SIZE),
                ..default()
            },
            BackgroundColor(EMPTY_COLOR),
        )).with_children(|parent| {

            bar = parent.spawn((
                ImageNode {
                    image: image_assets.health_bar_green.clone(),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(HealthBarUI::vertical_offset()),
                    left : Val::Px(SIZE),
                    width: Val::Px(segments as f32 * SIZE),
                    height: Val::Px(SIZE * 0.625),
                    ..default()
                },
            )).id();

            decor = parent.spawn((
                ImageNode {
                    image: image_assets.health_bar.clone(),
                    image_mode: NodeImageMode::Sliced(HealthBarUI::slicer()),
                    ..default()
                },
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                }
            )).id();
        }).id();

        Self {
            segments,
            root,
            bar,
            decor,
        }
    }

    pub fn offset(&self) -> Vec2 {
        Vec2::new(-(SIZE * self.segments as f32 + SIZE * 2.0) / 2.0, -(SIZE / 2.0) as f32)
    }

    pub fn adjust_bar_percent(&self, percent : f32, query : &mut Query<&mut Node>) {
        if let Ok(mut x) = query.get_mut(self.bar) {
            let clamped = percent.clamp(0.0, 1.0);
            let normalized = d1::normalize_from_01(clamped, 0.0, self.segments as f32 * SIZE);
            x.width = Val::Px(normalized);
        }
    }
}

impl Component for HealthBar {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_remove(|mut world, health_bar_entity, _| {
            let Some(health_bar) = world.get::<HealthBar>(health_bar_entity).cloned() else { return; };
            world.commands().entity(health_bar.root).despawn_recursive();
        });
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
        let Ok(health) = health.get(trigger.entity()) else { return; };
        let segments = (health.max_health() / (HEALTH_PER_SEGMENT * health.dense())).ceil() as u32;

        let mut bar: Entity = Entity::PLACEHOLDER;
        let mut decor: Entity = Entity::PLACEHOLDER;

        let root = commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(segments as f32 * SIZE + SIZE * 2.0),
                height: Val::Px(SIZE),
                ..default()
            },
            BackgroundColor(EMPTY_COLOR),
        )).with_children(|parent| {

            bar = parent.spawn((
                ImageNode {
                    image: image_assets.health_bar_green.clone().clone(),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(HealthBarUI::vertical_offset()),
                    left : Val::Px(SIZE),
                    width: Val::Px(segments as f32 * SIZE),
                    height: Val::Px(SIZE * 0.625),
                    ..default()
                },
            )).id();

            decor = parent.spawn((
                ImageNode {
                    image: image_assets.health_bar.clone(),
                    // TODO: Uncomment when this is fixed.
                    // image_mode: NodeImageMode::Sliced(HealthBarUI::slicer()),
                    ..default()
                },
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                }
            )).id();

        }).id();

        commands.entity(trigger.entity()).insert(HealthBar {
            segments,
            root,
            bar,
            decor,
        });
    }

    // pub fn create_health_bars(
    //     image_assets: Res<ImageAssets>,
    //     add_health_bars : Query<(Entity, &Health), Without<HealthBar>>,
    //     mut commands: Commands,
    // ) {
    //     add_health_bars
    //         .iter()
    //         .map(|(entity, health)| (entity, (health.max_health() / 250.0).ceil() as u32))
    //         .for_each(|(entity, segments)| {
    //             let health_bar = HealthBar::new(segments, &image_assets, &mut commands);
    //             commands.entity(entity).insert(health_bar);
    //     });
    // }

    pub fn update_health_bars(
        camera : Res<CameraController>,
        health_bars : Query<(&Transform, &Health, &HealthBar)>,
        mut nodes : Query<&mut Node>,
        mut visible_query: Query<(&mut Visibility, &InheritedVisibility)>,
        cameras : Query<(&Camera, &GlobalTransform)>,
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