use bevy::prelude::*;
use t5f_utility::mathfu::d1;
use crate::*;

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
        image_assets: &ImageAssets,
        commands : &mut Commands
    ) -> Self{

        let start = image_assets.health_bar_start.clone();
        let middle = image_assets.health_bar_middle.clone();
        let end = image_assets.health_bar_end.clone();
        let green = image_assets.health_bar_green.clone();

        let size = SIZE;
        let sizef = size as f32;

        let mut entity_commands = commands.spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                position_type : PositionType::Absolute,
                left : Val::Px(0.0),
                bottom : Val::Px(-sizef * 2.0),
                width: Val::Px((segments * size + size * 2) as f32),
                height: Val::Px(sizef),
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
                    left : Val::Px(sizef),
                    width: Val::Px((segments * size) as f32),
                    height: Val::Px(sizef / 2.0),
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
                    width: Val::Px(sizef),
                    height: Val::Px(sizef),
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
                        left : Val::Px((s * size + size) as f32),
                        width: Val::Px(sizef),
                        height: Val::Px(sizef),
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
                    left : Val::Px((segments * size + size) as f32),
                    width: Val::Px(sizef),
                    height: Val::Px(sizef),
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
            let clamped = percent.clamp(0.0, 1.0);
            let normalized = d1::normalize_from_01(clamped, 0.0, (self.segments * size) as f32);
            x.width = Val::Px(normalized);
        }
    }
}

impl Menu for HealthBar {
    fn main_container(&self) -> Entity {
        self.container
    }
}

pub struct HealthBarUIPlugin;

impl HealthBarUIPlugin {
    pub fn create_health_bars(
        image_assets: Res<ImageAssets>,
        add_health_bars : Query<(Entity, &Health), Without<HealthBar>>,
        mut commands: Commands,
    ) {
        add_health_bars
            .iter()
            .map(|(entity, health)| (entity, (health.max_health() / 250.0).ceil() as u32))
            .for_each(|(entity, segments)| {
                let health_bar = HealthBar::new(segments, &image_assets, &mut commands);
                commands.entity(entity).insert(health_bar);
        });
    }

    pub fn update_health_bars(
        camera : Res<CameraController>,
        health_bars : Query<(&Transform, &Health, &HealthBar)>,
        mut styles : Query<&mut Style>,
        mut visible_query: Query<(&mut Visibility, &InheritedVisibility)>,
        cameras : Query<(&Camera, &GlobalTransform)>,
    ) {
        let (camera, camera_transform) = cameras.get(camera.camera).unwrap();
        health_bars.for_each(|(tran, hel, bar)| {
            if hel.is_full_health() {
                bar.open(&mut visible_query);
            } else {
                bar.open(&mut visible_query);
                match camera.world_to_viewport(camera_transform, tran.translation) {
                    Some(point) => {
                        if let Ok(mut s) = styles.get_mut(bar.main_container()) {
                            let point = point + bar.offset();
                            s.left = Val::Px(point.x);
                            //TODO: find some way to get how far up the screen to put the health bar.
                            s.top = Val::Px(point.y - 50.0);
                        }
                    },
                    None => { },
                }
                bar.adjust_bar_percent(hel.health_percent(), &mut styles);
            }
        });
    }

    pub fn cleanup_health_bars(
        mut object_killed_reader: EventReader<ObjectKilledEvent>,
        query : Query<&HealthBar>,
        mut commands : Commands
    ) {
        for event in object_killed_reader.read() {
            if let Some(x) = query.get(event.0).ok().and_then(|x| { commands.get_entity(x.main_container())}) {
                x.despawn_recursive();
            }
        }
    }
}

impl Plugin for HealthBarUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                Self::create_health_bars,
                Self::update_health_bars.after(Self::create_health_bars),
                Self::cleanup_health_bars.after(CombatSystems),
            ).run_if(resource_exists::<CameraController>()))
        ;
    }
}