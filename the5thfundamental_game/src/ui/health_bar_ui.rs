use bevy::prelude::*;
use qloader::*;
use crate::*;

const SIZE : u32 = 16;

#[derive(Component)]
pub struct HealthBar {
    container : Entity,
    segments : u32,
    green : Entity,
}

impl HealthBar {
    pub fn new(segments : u32, textures : &QLoader<ImageAsset, AssetServer>, materials: &mut Assets<ColorMaterial>, commands : &mut Commands) -> Self {
        let start = textures.get("health_bar_start").unwrap();
        let middle = textures.get("health_bar_middle").unwrap();
        let end = textures.get("health_bar_end").unwrap();
        let green = textures.get("health_bar_green").unwrap();

        let size = SIZE;
        let sizef = size as f32;
        // let mut green_entity = commands.spawn().id();

        let mut entity_commands = commands.spawn_bundle(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                position_type : PositionType::Absolute,
                position : Rect {
                    left : Val::Px(0.0),
                    bottom : Val::Px(-sizef * 2.0),
                    ..Default::default()
                },
                size: Size::new(Val::Px((segments * size + size * 2) as f32), Val::Px(sizef)),
                ..Default::default()
            },
            color : UiColor(EMPTY_COLOR.into()),
            ..Default::default()
        });

        let main_container = entity_commands.id();
        // drop(entity_commands);
        let mut green_entity : Option<Entity> = None;

        entity_commands.with_children(|parent| {
            green_entity = Some(parent.spawn_bundle(ImageBundle {
                style : Style {
                    position_type : PositionType::Absolute,
                    position : Rect {
                        left : Val::Px(sizef),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px((segments * size) as f32), Val::Px(sizef / 2.0)),
                    ..Default::default()
                },
                image : UiImage(green.0.clone()),
                ..Default::default()
            }).id());
        });

        entity_commands.with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style : Style {
                    position_type : PositionType::Absolute,
                    size: Size::new(Val::Px(sizef), Val::Px(sizef)),
                    ..Default::default()
                },
                image : UiImage(start.0.clone()),
                ..Default::default()
            });
        });

        for s in 0..segments {
            entity_commands.with_children(|parent| {
                parent.spawn_bundle(ImageBundle {
                    style : Style {
                        position_type : PositionType::Absolute,
                        position : Rect {
                            left : Val::Px((s * size + size) as f32),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(sizef), Val::Px(sizef)),
                        ..Default::default()
                    },
                    image : UiImage(middle.0.clone()),
                    ..Default::default()
                });
            });
        }

        entity_commands.with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style : Style {
                    position_type : PositionType::Absolute,
                    position : Rect {
                        left : Val::Px((segments * size + size) as f32),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(sizef), Val::Px(sizef)),
                    ..Default::default()
                },
                image : UiImage(end.0.clone()),
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
        Vec2::new(-((SIZE * self.segments + SIZE * 2) as f32) / 2.0,
        -((SIZE / 2) as f32))
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

pub fn health_bar_update_system(
    textures : Res<QLoader<ImageAsset, AssetServer>>,
    mut materials : ResMut<Assets<ColorMaterial>>,
    windows : Res<Windows>,
    images : Res<Assets<Image>>,
    camera : Res<CameraController>,

    mut commands : Commands,

    add_health_bars : Query<(Entity, &Health), Without<HealthBar>>,
    health_bars : Query<(&Transform, &Health, &HealthBar)>,
    mut styles : Query<&mut Style>,
    mut visibles : Query<&mut Visibility>,
    children : Query<&Children>,
    cameras : Query<(&Camera, &GlobalTransform)>,
) {
    let mut ents_to_add : Vec<(Entity, u32)> = Vec::new();

    add_health_bars.for_each(|(ent, hel)| {
        ents_to_add.push((ent, (hel.max_health() / 250.0).ceil() as u32));
    });

    // println!("{}", ents_to_add.len());
    for (e, s) in ents_to_add.iter() {
        let health_bar = HealthBar::new(*s, &textures, &mut materials, &mut commands);
        commands.entity(*e).insert(health_bar);
    }

    let cam = cameras.get(camera.camera).unwrap();
    health_bars.for_each(|(tran, hel, bar)| {
        if hel.is_full_health() {
            bar.close(&mut visibles, &children);
        } else {
            bar.open(&mut visibles, &children);
            match cam.0.world_to_screen(&windows, &images, cam.1, tran.translation) {
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

pub fn health_bar_cleanup_system(
    mut object_killed_reader : EventReader<ObjectKilledEvent>,
    query : Query<&HealthBar>,
    mut commands : Commands
) {
    for event in object_killed_reader.iter() {
        // println!("deleting{:?}", e);
        commands.get_or_spawn(event.0).despawn_recursive();
        // commands.entity(*e).despawn_recursive();
        if let Ok(x) = query.get(event.0) {
            commands.get_or_spawn(x.main_container()).despawn_recursive();
        }
    }
}