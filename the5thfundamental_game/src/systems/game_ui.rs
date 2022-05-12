use bevy::{input::mouse::MouseButtonInput, prelude::*, reflect::List, ui::FocusPolicy};
use qloader::QLoader;
use crate::*;

#[derive(Component)]
pub struct BlocksRaycast;

pub fn ui_hit_detection_system<const U : usize>(
    mut counter : bevy::ecs::prelude::Local<[bool; U]>,
    mut ui_hit : ResMut<UiHit>,

    mut input : EventReader<MouseButtonInput>,

    interaction_query: Query<
        (&Interaction, &Visibility),
        (Changed<Interaction>, With<BlocksRaycast>),
    >,
) {
    if U == 0 { panic!("Length must not be 0"); }

    counter[U - 1] = false;
    for b in 1..counter.len() {
        counter[b-1] = counter[b]
    }
    
    interaction_query.for_each(|(int, vis)| {
        if vis.is_visible {
            match int {
                Interaction::Clicked => {
                    ui_hit.hit = true;
                    ui_hit.holding = true;
                    for b in 0..counter.len() {
                        counter[b] = true;
                    }
                },
                // Interaction::None => {
                //     ui_hit.hit = true;
                //     ui_hit.holding = true;
                //     for b in 0..counter.len() {
                //         counter[b] = true;
                //     }
                // }
                _ => { }
            }
        }
    });
    for event in input.iter() {
        match event.state {
            bevy::input::ElementState::Released => {
                if event.button == MouseButton::Left {
                    if ui_hit.holding {
                        ui_hit.hit = true;
                        ui_hit.holding = false;
                        for b in 0..counter.len() {
                            counter[b] = true;
                        }
                    }
                }
            },
            _ => { }
        }
    }
    ui_hit.hit = counter[0] || ui_hit.holding;
    // println!("{}, {}", counter[0], ui_hit.holding);
}

// pub struct Testing {
//     entity : Entity,
// }

// pub fn testing_start_up(
//     settings : Res<MenuSettings>,
//     textures : Res<QLoader<ImageAsset, AssetServer>>,
//     fonts : Res<QLoader<FontAsset, AssetServer>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut commands : Commands,
// ) {

//     let mut entity = None;
//     let mut entity_commands = commands.spawn_bundle(NodeBundle {
//         style: Style {
//             position_type : PositionType::Absolute,
//             position : Rect {
//                 left : Val::Px(PRIMARY_MENU_MARGIN),
//                 ..Default::default()
//             },
//             size: Size::new(Val::Px(500.0), Val::Percent(100.0)),
//             justify_content: JustifyContent::Center,
//             ..Default::default()
//         },
//         color : UiColor(DARK_BACKGROUND_COLOR.into()),
//         visibility : Visibility { is_visible : true},
//         ..Default::default()
//     }).insert(Interaction::default()).with_children(|parent| {
//         entity = Some(parent.spawn_bundle(NodeBundle {
//             style: Style {
//                 position_type : PositionType::Absolute,
//                 position : Rect {
//                     // left : Val::Px(100.0),
//                     // right : Val::Px(100.0),
//                     ..Default::default()
//                 },
//                 size: Size::new(Val::Px(100.0), Val::Px(100.0)),
//                 // justify_content: JustifyContent::Center,
//                 ..Default::default()
//             },
//             color : UiColor(LIGHT_BACKGROUND_COLOR.into()),
//             visibility : Visibility { is_visible : true},
//             ..Default::default()
//         }).insert(FocusPolicy::Pass).id());
//     });

//     // let entity = entity_commands2.id();

//     commands.insert_resource(Testing{ entity : entity.unwrap()});
// }

// pub fn testing_system(
//     testing : Res<Testing>,
//     mut styles : Query<&mut Style>,
//     windows : Res<Windows>,
//     // parents : Query<&Parent>,
//     mut children : Query<&mut Children>,
//     interaction_query: Query<
//         &Interaction,
//         Changed<Interaction>,
//     >,
//     mut commands : Commands,
// ) {
//     interaction_query.for_each(|int| {
//         println!("{:?}", int);
//     });
//     // let style = style_set.q0().get(testing.entity).unwrap();
//     // let size = get_absolute_size_recursive(testing.entity, &parents, style_set.q0(), (1.0, 1.0));
//     if let Ok(mut style) = styles.get_mut(testing.entity) {
//         if let Some(win) = windows.get_primary() {
//             let w = match style.size.width {
//                 Val::Undefined | Val::Auto => 0.0,
//                 Val::Px(x) => { x }
//                 Val::Percent(x) => { win.width() * x / 100.0 }
//             };
//             let h = match style.size.height {
//                 Val::Undefined | Val::Auto => 0.0,
//                 Val::Px(x) => { x }
//                 Val::Percent(x) => { win.height() * x / 100.0 }
//             };
//             if let Some(pos) = win.cursor_position() {
//                 style.position.left = Val::Px(pos.x - w / 2.0);
//                 style.position.bottom = Val::Px(pos.y - h / 2.0);
//             }
//         }
//     }
//     commands.entity(testing.entity).remove::<Parent>();
//     // if let Ok(c) = children.get_mut(testing.entity) {
//     //     for i in 0..c.len() {
//     //         if c[i] == testing.entity {
//     //             c.deref().
//     //         }
//     //     }
//     // }
    
// }

// fn get_absolute_size_recursive(
//     entity : Entity,
//     parent_query : &Query<&Parent>,
//     style_query : &Query<&Style>,
//     window : &Window,
//     (width, height) : (f32, f32),
// ) -> (f32, f32) {
//     // let (w, h) = if let Ok(style) = style_query.get(entity) {
//     //     let w = match style.size.width {
//     //         Val::Undefined | Val::Auto => 0.0,
//     //         Val::Px(_) => { 1.0 },
//     //         Val::Percent(x) => { x / 100.0 }
//     //     };
//     //     let h = match style.size.height {
//     //         Val::Undefined | Val::Auto => 0.0,
//     //         Val::Px(_) => { 1.0 }
//     //         Val::Percent(x) => { x / 100.0 }
//     //     };
//     //     (w, h)
//     //     // *width *= w;
//     //     // *height *= h;
//     // } else {
//     //     (1.0, 1.0)
//     // };

//     // if let Ok(p) = parent_query.get(entity) {
//     //     println!("What");
//     //     let s = get_absolute_size_recursive(p.0, parent_query, style_query, (w, h));
//     //     (w * s.0, h * s.1)
//     // } else {
//     //     (w, h)
//     // }
//     match (style_query.get(entity), parent_query.get(entity)) {
//         (Ok(style), Ok(parent)) => {
//             let w = match style.size.width {
//                 Val::Undefined | Val::Auto => 0.0,
//                 Val::Px(_) => { 1.0 },
//                 Val::Percent(x) => { x / 100.0 }
//             };
//             let h = match style.size.height {
//                 Val::Undefined | Val::Auto => 0.0,
//                 Val::Px(_) => { 1.0 }
//                 Val::Percent(x) => { x / 100.0 }
//             };
//         }
//         (Ok(style), Err(_)) => { }
//         (Err(_), Ok(parent)) => { }
//         (Err(_), Err(_)) => { }
//     }
// }