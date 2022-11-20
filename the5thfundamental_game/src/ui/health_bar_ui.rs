use bevy::{prelude::*, ecs::schedule::ShouldRun};
use qloader::*;
use crate::{*, utility::assets::ImageAsset};

const SIZE : u32 = 16;

#[derive(Component)]
pub struct HealthBar {
    container : Entity,
    segments : u32,
    green : Entity,
}

impl HealthBar {
    pub fn new(
        segments : u32,
        asset_server: &mut AssetServer,
        commands : &mut Commands
    ) -> Self{

        let start = asset_server.load(ImageAsset::HealthBarStart);
        let middle = asset_server.load(ImageAsset::HealthBarMiddle);
        let end = asset_server.load(ImageAsset::HealthBarEnd);
        let green = asset_server.load(ImageAsset::HealthBarGreen);

        // let start = textures.get("health_bar_start").unwrap();
        // let middle = textures.get("health_bar_middle").unwrap();
        // let end = textures.get("health_bar_end").unwrap();
        // let green = textures.get("health_bar_green").unwrap();

        let size = SIZE;
        let sizef = size as f32;
        // let mut green_entity = commands.spawn().id();

        let mut entity_commands = commands.spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                position_type : PositionType::Absolute,
                position : UiRect {
                    left : Val::Px(0.0),
                    bottom : Val::Px(-sizef * 2.0),
                    ..Default::default()
                },
                size: Size::new(Val::Px((segments * size + size * 2) as f32), Val::Px(sizef)),
                ..Default::default()
            },
            background_color : EMPTY_COLOR.into(),
            ..Default::default()
        });

        let main_container = entity_commands.id();
        // drop(entity_commands);
        let mut green_entity : Option<Entity> = None;

        entity_commands.with_children(|parent| {
            green_entity = Some(parent.spawn(ImageBundle {
                style : Style {
                    position_type : PositionType::Absolute,
                    position : UiRect {
                        left : Val::Px(sizef),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px((segments * size) as f32), Val::Px(sizef / 2.0)),
                    ..Default::default()
                },
                image : green.into(),
                ..Default::default()
            }).id());
        });

        entity_commands.with_children(|parent| {
            parent.spawn(ImageBundle {
                style : Style {
                    position_type : PositionType::Absolute,
                    size: Size::new(Val::Px(sizef), Val::Px(sizef)),
                    ..Default::default()
                },
                image : start.into(),
                ..Default::default()
            });
        });

        for s in 0..segments {
            entity_commands.with_children(|parent| {
                parent.spawn(ImageBundle {
                    style : Style {
                        position_type : PositionType::Absolute,
                        position : UiRect {
                            left : Val::Px((s * size + size) as f32),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(sizef), Val::Px(sizef)),
                        ..Default::default()
                    },
                    image : middle.clone().into(),
                    ..Default::default()
                });
            });
        }

        entity_commands.with_children(|parent| {
            parent.spawn(ImageBundle {
                style : Style {
                    position_type : PositionType::Absolute,
                    position : UiRect {
                        left : Val::Px((segments * size + size) as f32),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(sizef), Val::Px(sizef)),
                    ..Default::default()
                },
                image : end.into(),
                ..Default::default()
            });
        });

        Self {
            container : main_container,
            segments,
            green : green_entity.unwrap(),
        }
    }

    pub fn offset(&self) -> Vec2 {
        Vec2::new(-((SIZE * self.segments + SIZE * 2) as f32) / 2.0, -((SIZE / 2) as f32))
    }

    pub fn adjust_bar_percent(&self, percent : f32, query : &mut Query<&mut Style>) {
        let size = SIZE;

        if let Ok(mut x) = query.get_mut(self.green) {
            let clamped = mathfu::D1::clamp01(percent);
            let normalized = mathfu::D1::normalize_from_01(clamped, 0.0, (self.segments * size) as f32);
            x.size.width = Val::Px(normalized);
        }
    }
}

impl Menu for HealthBar {
    fn main_container(&self) -> Entity {
        self.container
    }
}

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::new()
            .with_run_criteria(should_update_health_bars)
            .with_system(create_health_bars)
            .with_system(update_health_bars.after(create_health_bars))
            .with_system(cleanup_health_bars.after(update_health_bars))
        );
    }
}

pub fn should_update_health_bars(
    camera_controller: Option<Res<CameraController>>,
) -> ShouldRun {
    camera_controller.is_some().into()
}

pub fn create_health_bars(
    mut asset_server: ResMut<AssetServer>,
    add_health_bars : Query<(Entity, &Health), Without<HealthBar>>,
    mut commands: Commands,
) {
    add_health_bars
        .iter()
        .map(|(entity, health)| (entity, (health.max_health() / 250.0).ceil() as u32))
        .for_each(|(entity, segments)| {
            let health_bar = HealthBar::new(segments, &mut asset_server, &mut commands);
            commands.entity(entity).insert(health_bar);
    });
}

pub fn update_health_bars(
    camera : Res<CameraController>,
    health_bars : Query<(&Transform, &Health, &HealthBar)>,
    mut styles : Query<&mut Style>,
    mut visibles : Query<&mut Visibility>,
    cameras : Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = cameras.get(camera.camera).unwrap();
    health_bars.for_each(|(tran, hel, bar)| {
        if hel.is_full_health() {
            bar.open(&mut visibles);
        } else {
            bar.open(&mut visibles);
            match camera.world_to_viewport(camera_transform, tran.translation) {
                Some(point) => {
                    if let Ok(mut s) = styles.get_mut(bar.main_container()) {
                        let point = point + bar.offset();
                        s.position.left = Val::Px(point.x);
                        //TODO: find some way to get how far up the screen to put the health bar.
                        s.position.bottom = Val::Px(point.y + 50.0);
                    }
                },
                None => { },
            }
            bar.adjust_bar_percent(hel.health_percent(), &mut styles);
        }
    });
}

pub fn cleanup_health_bars(
    mut object_killed_reader : EventReader<ObjectKilledEvent>,
    query : Query<&HealthBar>,
    mut commands : Commands
) {
    for event in object_killed_reader.iter() {
        commands.get_or_spawn(event.0).despawn_recursive();
        // commands.entity(*e).despawn_recursive();
        if let Some(x) = query.get(event.0).ok().and_then(|x| { commands.get_entity(x.main_container())}) {
            x.despawn_recursive();
        }
    }
}