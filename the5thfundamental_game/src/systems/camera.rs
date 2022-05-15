use std::f32::{NEG_INFINITY, INFINITY,};

use bevy::{gltf::{Gltf, GltfMesh}, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::camera::Camera, utils::tracing::Event};
use bevy_ninepatch::*;
use bevy_pathfinding::PathFinder;
use bevy_rapier3d::{prelude::{Velocity, InteractionGroups, RigidBody}, plugin::RapierContext};
use mathfu::D1;
use the5thfundamental_common::*;
use qloader::*;
use crate::*;

pub const CLICK_BUFFER : usize = 8;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CameraSetupSystems {
    CreateCamera,
    CreateSelector,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CameraSystems {
    CameraControlSystem,
    CameraRaycastSystem,
    CameraRaycastResponseSystem,
    CameraContextFocusSystem,
    SelectionHighlighterSystem,
    CommandSystem,
    BuildingPlacementSystem,
}

//TODO: Fix camera movement

pub fn camera_setup_system_set(set : SystemSet) -> SystemSet {
    set.label(SystemSets::Camera)
        .with_system(create_camera.label(CameraSetupSystems::CreateCamera))
        .with_system(create_selector.label(CameraSetupSystems::CreateSelector).after(CameraSetupSystems::CreateCamera))
        .with_system(building_placement_startup_system)
}

pub fn camera_system_set(set : SystemSet) -> SystemSet {
    set.label(SystemSets::Camera)
        .with_system(camera_control_system.label(CameraSystems::CameraControlSystem))
        .with_system(camera_raycast_system.label(CameraSystems::CameraRaycastSystem).after(CameraSystems::CameraControlSystem))
        .with_system(building_placement_system.label(CameraSystems::BuildingPlacementSystem).after(CameraSystems::CameraRaycastSystem))
        .with_system(camera_raycast_response_system.label(CameraSystems::CameraRaycastResponseSystem).after(CameraSystems::BuildingPlacementSystem))
        .with_system(show_selection_box.after(camera_raycast_response_system))
        .with_system(camera_select.after(show_selection_box))
        .with_system(camera_context_focus_system.label(CameraSystems::CameraContextFocusSystem).after(CameraSystems::CameraRaycastResponseSystem))
        .with_system(selection_highlighter.label(CameraSystems::SelectionHighlighterSystem).after(CameraSystems::CameraRaycastResponseSystem))
        .with_system(command_system.label(CameraSystems::CommandSystem).after(CameraSystems::CameraRaycastResponseSystem))
}

pub struct CameraController {
    pub camera_root: Entity,
    // pub camera_pivot: Entity,
    pub camera: Entity,

    pub root_velocity: Vec3,
    pub rotation_velocity: f32,
    pub zoom_precentage: f32,
    pub zoom_velocity: f32,
    // pub root_velocity: InterForce,

    outside_window: bool,
    just_entered: bool,
    holding: bool,
}

pub fn create_camera(
    settings: Res<CameraSettings>,
    map: Res<Map>,
    mut commands: Commands
) {
    let (direction, distance) = settings.default_direction_and_distance();

    // let min_z = direction.z * settings.min_zoom;
    // let min_y = direction.y * settings.min_zoom;
    // let max_z = direction.z * settings.max_zoom;
    // let max_y = direction.y * settings.max_zoom;
    let z = direction.z * distance;
    let y = direction.y * distance;

    let mut transform = Transform::from_xyz(2.2, y, z);
    transform.look_at(Vec3::ZERO, Vec3::Y);

    let root_entity = commands.spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        // .insert(Velocity { linvel : Vec3::new(0.0, 0.0, 0.0), angvel : Vec3::new(0.0, 0.0, 0.0)})
        // .insert(RigidBody::KinematicVelocityBased)
        // .insert(Torque::default())
        .insert(LocalBounds {
            x : Vec2::new(-map.bounds.0 / 2.0, map.bounds.0 / 2.0),
            y : Vec2::new(NEG_INFINITY, INFINITY),
            z : Vec2::new(-map.bounds.1 / 2.0, map.bounds.1 / 2.0)
            // x : Vec2::new(-20.0, 20.0),
            // y : Vec2::new(NEG_INFINITY, INFINITY),
            // z : Vec2::new(-20.0, 20.0)
        }).id();

    // let pivot_entity = commands.spawn()
    //     .insert(Transform::default())
    //     .insert(GlobalTransform::default())
    //     .insert(Velocity { linvel : Vec3::new(0.0, 0.0, 0.0), angvel : Vec3::new(0.0, 0.0, 0.0)})
    //     .insert(Parent(root_entity)).id();

    let camera_entity = commands.spawn_bundle(PerspectiveCameraBundle {
        transform,
        ..Default::default()
    })
    // .insert(Velocity { linvel : Vec3::new(0.0, 0.0, 0.0), angvel : Vec3::new(0.0, 0.0, 0.0)})
        // .insert(LocalBounds {
        //     x : Vec2::new(0.0, 0.0),
        //     y : Vec2::new(min_y, max_y),
        //     z : Vec2::new(min_z, max_z),
        // })
        .insert(Parent(root_entity))
        .id();
        
    commands.insert_resource(CameraController {
        camera_root : root_entity,
        // camera_pivot : pivot_entity,
        camera : camera_entity,

        // root_velocity: InterForce { force: Vec3::default(), max_speed: 1.0, acceleration: settings.scroll_acceleration},
        root_velocity: Vec3::default(),
        rotation_velocity: 0.0,
        zoom_precentage: settings.default_zoom,
        zoom_velocity: 0.0,

        outside_window : false,
        just_entered : false,
        holding : false,
    });
}

//TODO: Abstract out key bindings.
pub fn camera_control_system(
    settings : Res<CameraSettings>,
    mut controller : ResMut<CameraController>,
    mut scroll_evr: EventReader<MouseWheel>,
    mouse_buttons : Res<Input<MouseButton>>,
    key_input : Res<Input<KeyCode>>,
    windows : Res<Windows>,
    time : Res<Time>,
    mut trans : Query<&mut Transform>,
) {
    let window = windows.get_primary().unwrap();
    let half_size = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    if mouse_buttons.pressed(MouseButton::Left) {
        controller.holding = true;
    }

    let real_mouse_pos = match window.cursor_position() {
        Some(pos) => {
            if controller.outside_window {
                controller.just_entered = true;
            }
            controller.outside_window = false;
            pos - half_size
        },
        None => {
            controller.outside_window = true;
            Vec2::default()
        }
    };

    let adjusted_mouse_pos = window.cursor_position().map_or(Vec2::default(), |pos|
        if !controller.outside_window && (!controller.just_entered || !settings.post_action_stall) && !controller.holding { pos - half_size } else { Vec2::default() });

    let threshholds : (f32, f32) = (
        mathfu::D1::normalize_from_01(settings.thresholds.0, 0., half_size.x),
        mathfu::D1::normalize_from_01(settings.thresholds.1, 0., half_size.y),
    );

    let height = trans.get_mut(controller.camera).map_or(1.0, |x|{

        // println!("{}", x.translation.y);
        // println!("{}", settings.min_zoom);
        // println!("{}", settings.max_zoom);
        // println!("{}", settings.zoom_base);
        // println!("{}", settings.zoom_base * settings.zoom_ratio);
        // println!("{}", mathfu::D1::normalize_from_to(x.translation.distance(Vec3::default()), settings.min_zoom().length(), settings.max_zoom().length(), settings.zoom_base, settings.zoom_base * settings.zoom_ratio));
        mathfu::D1::clamp(mathfu::D1::normalize_from_to(x.translation.distance(Vec3::default()), settings.min_zoom().length(), settings.max_zoom().length(),
        settings.zoom_base, settings.zoom_base * settings.zoom_ratio), settings.zoom_base, settings.zoom_base * settings.zoom_ratio)
    });

    let slow = if key_input.pressed(KeyCode::C) {
        (settings.slow_rotation_multiplier, settings.slow_scroll_multiplier, settings.slow_zoom_multiplier)
    } else {
        (1., 1., 1.)
    };

    let mouse_dir = Vec3::new(adjusted_mouse_pos.x, 0.0, -adjusted_mouse_pos.y).normalize_or_zero();

    let mags : (f32,f32) = (
        mathfu::D1::powf_sign(mathfu::D1::clamp01(mathfu::D1::normalize_to_01(
        adjusted_mouse_pos.x.abs(), threshholds.0, half_size.x)), settings.scroll_acceleration_curve),
        mathfu::D1::powf_sign(mathfu::D1::clamp01(mathfu::D1::normalize_to_01(
        adjusted_mouse_pos.y.abs(), threshholds.1, half_size.y)), settings.scroll_acceleration_curve),
    );

    let hor = {
        let mut h = 0.0;
        if key_input.pressed(KeyCode::D) { h += 1.0; }
        if key_input.pressed(KeyCode::A) { h -= 1.0; }
        h * settings.scroll_button_speed_multiplier
    };

    let vert = {
        let mut v = 0.0;
        if key_input.pressed(KeyCode::W) { v -= 1.0; }
        if key_input.pressed(KeyCode::S) { v += 1.0; }
        v * settings.scroll_button_speed_multiplier
    };

    let scroll_dir_x = mouse_dir.x * mags.0.max(mags.1);
    let scroll_dir_z = mouse_dir.z * mags.0.max(mags.1);

    let (x, z) = (
        mathfu::D1::clamp((scroll_dir_x + hor) * slow.1 * settings.max_scroll_speed * height,
        -settings.max_scroll_speed * height, settings.max_scroll_speed * height),
        mathfu::D1::clamp((scroll_dir_z + vert) * slow.1 * settings.max_scroll_speed * height,
        -settings.max_scroll_speed * height, settings.max_scroll_speed * height),
    );

    let delta = mathfu::D1::clamp(time.delta_seconds(), 0.0, 1.0 / settings.minimum_fps_for_deltatime as f32);

    // controller.root_velocity.apply_force(Vec3::new(x, 0.0, z), delta);

    
    if let Ok(mut tran) = trans.get_mut(controller.camera_root) {
        //*Rotation
        let mut dir = 0.;
        if key_input.pressed(KeyCode::Q) {
            controller.rotation_velocity = mathfu::D1::clamp(controller.rotation_velocity, 0., INFINITY);
            dir += 0.01;
        }

        if key_input.pressed(KeyCode::E) {
            controller.rotation_velocity = mathfu::D1::clamp(controller.rotation_velocity, NEG_INFINITY, 0.);
            dir -= 0.01;
        }

        if dir != 0. {
            controller.rotation_velocity = mathfu::D1::lerp(controller.rotation_velocity, dir * settings.max_rotation_speed * slow.0, mathfu::D1::clamp01(settings.rotation_acceleration * delta));
        } else {
            controller.rotation_velocity = mathfu::D1::lerp(controller.rotation_velocity, 0.0, mathfu::D1::clamp01(settings.rotation_deceleration * delta));
        }
        tran.rotate(Quat::from_rotation_y(controller.rotation_velocity));

        //*Scrolling
        if x.is_normal() {
            if mathfu::D1::same_sign(x, controller.root_velocity.x) {
                if mathfu::D1::farther_from_zero(x, controller.root_velocity.x) {
                    controller.root_velocity.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(controller.root_velocity.x, x, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    controller.root_velocity.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(controller.root_velocity.x, x, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
                    );
                }
            } else {
                if controller.root_velocity.x.abs() < settings.fast_decceleration_threshold {
                    controller.root_velocity.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(controller.root_velocity.x, x, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    controller.root_velocity.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(controller.root_velocity.x, x, mathfu::D1::clamp01(settings.scroll_deceleration * settings.fast_decceleration_strength * delta))
                    );
                }
            }
        } else {
            controller.root_velocity.x = mathfu::D1::more_than_or_zero_pog(
                mathfu::D1::lerp(controller.root_velocity.x, 0.0, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
            );
        }

        if z.is_normal() {
            if mathfu::D1::same_sign(z, controller.root_velocity.z) {
                if mathfu::D1::farther_from_zero(z, controller.root_velocity.z) {
                    controller.root_velocity.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(controller.root_velocity.z, z, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    controller.root_velocity.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(controller.root_velocity.z, z, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
                    );
                }
            } else {
                if controller.root_velocity.z.abs() < settings.fast_decceleration_threshold {
                    controller.root_velocity.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(controller.root_velocity.z, z, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    controller.root_velocity.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(controller.root_velocity.z, z, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
                    );
                }
            }
        } else {
            controller.root_velocity.z = mathfu::D1::more_than_or_zero_pog(
                mathfu::D1::lerp(controller.root_velocity.z, 0.0, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
            );
        }

        let movement = tran.rotation * controller.root_velocity * delta;
        tran.translation += movement;

        if key_input.just_pressed(KeyCode::Grave) {
            tran.rotation = Quat::default();
        }
    }

    if let Ok(mut tran) = trans.get_mut(controller.camera) {
        let mut zoom_add = 0.0;
        for ev in scroll_evr.iter() {
            zoom_add = -ev.y;
        }

        let min_zoom = settings.min_zoom();
        let max_zoom = settings.max_zoom();

        if zoom_add > 0. {
            zoom_add = mathfu::D1::clamp(zoom_add, 0., INFINITY);
        } else if zoom_add < 0. {
            zoom_add = mathfu::D1::clamp(zoom_add, NEG_INFINITY, 0.);
        }

        if zoom_add != 0. {
            controller.zoom_velocity = mathfu::D1::lerp(controller.zoom_velocity, zoom_add * height * settings.max_zoom_speed * slow.2, mathfu::D1::clamp01(settings.zoom_acceleration * delta));
        } else {
            controller.zoom_velocity = mathfu::D1::lerp(controller.zoom_velocity, 0., mathfu::D1::clamp01(settings.zoom_deceleration * delta));
        }

        controller.zoom_precentage = D1::clamp01(controller.zoom_precentage + controller.zoom_velocity * delta);

        let zoom_y = D1::normalize_from_01(controller.zoom_precentage, min_zoom.y, max_zoom.y);
        let zoom_z = D1::normalize_from_01(controller.zoom_precentage, min_zoom.z, max_zoom.z);

        tran.translation.y = zoom_y;
        tran.translation.z = zoom_z;

        if key_input.just_pressed(KeyCode::Grave) {
            let (direction, distance) = settings.default_direction_and_distance();
            tran.translation = direction * distance;
            controller.zoom_velocity = 0.0;
            controller.zoom_precentage = settings.default_zoom;
        }
        tran.look_at(Vec3::ZERO, Vec3::Y);
    }

    if (real_mouse_pos.x.abs() < threshholds.0.abs() && real_mouse_pos.y.abs() < threshholds.1.abs()) || !settings.post_action_stall {
        controller.just_entered = false;
        controller.holding = false;
    }
}

pub fn camera_raycast_system(
    controller : Res<CameraController>,
    windows : Res<Windows>,
    ui_hit : Res<UiHit<CLICK_BUFFER>>,
    context : Res<RapierContext>,
    identifiers : Res<Identifiers>,
    mut cast : ResMut<CameraRaycast>,

    cameras : Query<(&GlobalTransform, &Camera)>,
) {
    cast.current_cast = None;
    if ui_hit.hit() { return; }

    if let Ok((gl_transform, camera)) = cameras.get(controller.camera) {
        if let Some(cursor) = windows.get_primary().and_then(|w| w.cursor_position()) {
            let (origin, direction) = ray(cursor, &windows, camera, gl_transform);
            if let Some((entity, len)) = context.cast_ray(origin, direction, f32::MAX, true, InteractionGroups::all(), None) {
                let point = origin + direction * len;
                if let Some(cam_cast) = identifiers.get_unique_id(entity).map(|id| RayCastResult { id, point, len}) {
                    cast.last_valid_cast = Some(cam_cast);
                    cast.current_cast = Some(cam_cast);
                }
            }
        }

    }
}

pub enum BoxSelectStatus {
    Idle,
    Dragging,
}

pub struct CameraSelector {
    selection_box_entity : Entity,
    mouse_start_pos : Vec2,
    mouse_end_pos : Vec2,
    minimum_distance : f32,
    // box_selecting : bool,
    status: BoxSelectStatus,

    // add_to_selection : bool,
    // to_clear : bool,
}

impl CameraSelector {
    pub fn showing(&self) -> bool {
        self.mouse_start_pos.distance(self.mouse_end_pos) >= self.minimum_distance
    }
}

impl Menu for CameraSelector {
    fn main_container(&self) -> Entity {
        self.selection_box_entity
    }
}

pub fn camera_raycast_response_system(
    mut selector : ResMut<CameraSelector>,
    ui_hit : Res<UiHit<CLICK_BUFFER>>,
    placement : Res<CurrentPlacement<CLICK_BUFFER>>,
    windows : Res<Windows>,
    mouse_input : Res<Input<MouseButton>>,
    mut selection_events : EventWriter<SelectionEvent>,
) {
    if ui_hit.hit()
    || placement.placing()
    { return; }

    match selector.status {
        BoxSelectStatus::Idle => {
            if mouse_input.just_pressed(MouseButton::Left) {
                selector.mouse_start_pos = windows.get_primary().and_then(|w| w.cursor_position()).unwrap_or(Vec2::new(0.0, 0.0));
                selector.mouse_end_pos = selector.mouse_start_pos;
                selector.status = BoxSelectStatus::Dragging;
            }
        },
        BoxSelectStatus::Dragging => {
            selector.mouse_end_pos = windows.get_primary().and_then(|w| w.cursor_position()).unwrap_or(Vec2::new(0.0, 0.0));
            if mouse_input.just_released(MouseButton::Left) {
                selector.status = BoxSelectStatus::Idle;
                if selector.showing() {
                    selection_events.send(SelectionEvent::Box(selector.mouse_start_pos, selector.mouse_end_pos))
                } else {
                    selection_events.send(SelectionEvent::Single);
                }
            }
        },
    }
}

pub fn create_selector(
    textures : Res<QLoader<ImageAsset, AssetServer>>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    mut commands : Commands
) {
    let select = textures.get("selection_box").unwrap();
    let nine_patch = nine_patches.add(NinePatchBuilder::by_margins(2, 2, 2, 2));
    let mut entity_commands = commands.spawn_bundle(NinePatchBundle {
        style: Style {
            position_type : PositionType::Absolute,
            size: Size::new(Val::Px(0.0), Val::Percent(0.0)),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        nine_patch_data : NinePatchData {
            nine_patch,
            texture : select.0.clone(),
            ..Default::default()
        },
        ..Default::default()
    });
    entity_commands.insert(Visibility { is_visible : false});

    let container_entity = entity_commands.id();
    commands.insert_resource(CameraSelector {
        selection_box_entity : container_entity,
        mouse_start_pos : Vec2::ZERO,
        mouse_end_pos : Vec2::ZERO,
        ///TODO: Add setting for this
        minimum_distance: 40.0,
        status: BoxSelectStatus::Idle,
    });

    commands.insert_resource(CurrentPlacement::<CLICK_BUFFER> {
        status : PlacementStatus::Idle,
        constructor : None,
        data : None,
        ins_data : None,
        entity : None,
        placing : [false; CLICK_BUFFER],
    });
}

pub fn show_selection_box(
    selector : ResMut<CameraSelector>,
    placement : Res<CurrentPlacement<CLICK_BUFFER>>,

    mut styles : Query<&mut Style>,
    mut visibles : Query<&mut Visibility>,
    children : Query<&Children>,
) {
    if placement.placing() { return; };
    match selector.status {
        BoxSelectStatus::Idle => {
            selector.close(&mut visibles, &children);
        },
        BoxSelectStatus::Dragging => {
            if let Ok(mut style) = styles.get_mut(selector.selection_box_entity) {
                let extents = (selector.mouse_start_pos.x.min(selector.mouse_end_pos.x), selector.mouse_start_pos.x.max(selector.mouse_end_pos.x),
                    selector.mouse_start_pos.y.min(selector.mouse_end_pos.y), selector.mouse_start_pos.y.max(selector.mouse_end_pos.y));

                style.position.left = Val::Px(extents.0);
                style.position.bottom = Val::Px(extents.2);
                style.size.width = Val::Px(extents.1 - extents.0);
                style.size.height = Val::Px(extents.3 - extents.2);
            }

            if selector.showing() {
                selector.open(&mut visibles, &children);
            } else {
                selector.close(&mut visibles, &children);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SelectionEvent {
    Single,
    Box(Vec2, Vec2),
}

pub fn camera_select(
    mut selection_event : EventReader<SelectionEvent>,
    camera : Res<CameraController>,
    cast : Res<CameraRaycast>,
    identifiers : Res<Identifiers>,
    player : Res<Player>,

    windows : Res<Windows>,
    images : Res<Assets<Image>>,

    key_input : Res<Input<KeyCode>>,

    mut units : Query<(Entity, &GlobalTransform, &mut Selectable, &TeamPlayer)>,
    // teamplayers : Query<&TeamPlayer>,
    cameras : Query<(&GlobalTransform, &Camera)>,
) {
    for event in selection_event.iter() {

        let add_to_selection = key_input.pressed(KeyCode::LShift) || key_input.pressed(KeyCode::RShift);
        let mut empty = true;
        units.for_each(|(_, _, sel, _)| {
            if sel.selected {
                empty = false;
            }
        });

        match event {
            SelectionEvent::Single => {
                let entity = cast.current_cast
                    .and_then(|c| identifiers.get_entity(c.id))
                    .and_then(|e| units.get_mut(e)
                        .map_or(None, |(ent, _, _, _,)| Some(ent))
                );
                if let Some(ent) = entity {
                    let clear = units.get_mut(ent).unwrap().2.context == SelectableContext::Clear;
                    if clear && !add_to_selection {
                        units.for_each_mut(|(_, _, mut selectable, team_player)| {
                            if *team_player == player.0 {
                                selectable.selected = false;
                            }
                        });
                        continue;
                    }
                    if !empty && add_to_selection {
                        let (_, _, mut sel, tp) = units.get_mut(ent).unwrap();
                        if sel.context == SelectableContext::MultiSelect {
                            if *tp == player.0 {
                                sel.selected = true;
                            }
                        }
                    } else {
                        units.for_each_mut(|(_, _, mut selectable, team_player)| {
                            if *team_player == player.0 {
                                selectable.selected = false;
                            }
                        });
                        let (_, _, mut sel, tp) = units.get_mut(ent).unwrap();
                        if *tp == player.0 {
                            sel.selected = true;
                        }
                    }
                } else {
                    if !add_to_selection {
                        units.for_each_mut(|(_, _, mut selectable, team_player)| {
                            if *team_player == player.0 {
                                selectable.selected = false;
                            }
                        });
                    }
                }
            },
            SelectionEvent::Box(min, max) => {
                if let Ok((cam_tran, camera)) = cameras.get(camera.camera) {

                    if !add_to_selection {
                        units.for_each_mut(|(_, _, mut selectable, team_player)| {
                            if *team_player == player.0 {
                                selectable.selected = false;
                            }
                        });
                    }

                    let mut ents = Vec::new();

                    units.for_each(|(ent, gl_tran, _, tp)| {
                        if *tp == player.0 {
                            if let Some(center) = camera.world_to_screen(&windows, &images, cam_tran, gl_tran.translation) {
                                if center.x > min.x && center.x < max.x
                                && center.y < min.y && center.y > max.y {
                                    ents.push(ent);
                                }
                            }
                        }
                    });

                    for e in ents.iter() {
                        let (_, _, mut sel, _) = units.get_mut(*e).unwrap();
                        match sel.context {
                            SelectableContext::Single => {
                                if ents.len() == 1 {
                                    sel.selected = true;
                                }
                            },
                            SelectableContext::MultiSelect => {
                                sel.selected = true;
                            },
                            _ => { }
                        }
                    }
                }
            }
        }
    }


}

pub fn camera_context_focus_system(
    mut focus : ResMut<ContextFocus>,
    selectables : Query<(Entity, &Selectable)>
) {
    let selects = selectables.iter().filter_map(|(e, s)| if s.selected && s.context == SelectableContext::Single { Some(e) } else { None}).collect::<Vec<Entity>>();
    if selects.len() == 1 {
        focus.0 = selects.first().cloned();
    } else {
        focus.0 = None;
    }
}

pub fn selection_highlighter(
    cast : Res<CameraRaycast>,
    idents : Res<Identifiers>,
    mut debug_lines : ResMut<DebugLines>,

    query : Query<(&GlobalTransform, &Selectable)>,
) {
    if let Some((glt, _)) = cast.current_cast
        .and_then(|c| idents.get_entity(c.id))
        .and_then(|e| query.get(e)
            .map_or(None, |x| Some(x))
    ) {
        debug_lines.line_colored(
            glt.translation,
            glt.translation + Vec3::Y * 10.0,
            0.0,
            Color::rgba(0.1, 0.35, 0.45, 1.0),
        );
    }

    query.for_each(|(glt, sel)| {
        if sel.selected {
            debug_lines.line_colored(
                glt.translation,
                glt.translation + Vec3::Y * 3.0,
                0.0,
                Color::rgba(0.1, 0.35, 0.45, 1.0),
            );
        }
    });
}

pub fn command_system(
    player : Res<Player>,
    cast : Res<CameraRaycast>,
    idents : Res<Identifiers>,
    current_placement : Res<CurrentPlacement<CLICK_BUFFER>>,
    input : Res<Input<MouseButton>>,

    units : Query<(&SnowFlake, &Selectable), With<PathFinder>>,
    team_players : Query<&TeamPlayer>,
    teamplayer_world : Res<TeamPlayerWorld>,
    mut move_commands : EventWriter<MoveCommand>,
    mut attack_commands : EventWriter<AttackCommand>,
) {
    if current_placement.placing() { return; }
    if input.just_released(MouseButton::Right) {
        if let Some(ray_cast) = cast.current_cast {
            if idents.get_entity(ray_cast.id)
                .map_or(false, |e| teamplayer_world.is_enemy(e, player.0, &team_players)
                    .map_or(false, |t| t)) {
                attack_commands.send(AttackCommand{
                    target : ray_cast.id,
                    units : units.iter().filter_map(|(id, sel) |if sel.selected { Some(*id) } else { None }).collect::<Vec<SnowFlake>>(),
                });
            } else {
                move_commands.send(MoveCommand {
                    position : Vec2::new(ray_cast.point.x, ray_cast.point.z),
                    units : units.iter().filter_map(|(id, sel)| if sel.selected { Some(*id) } else { None }).collect::<Vec<SnowFlake>>(),
                });
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PlacementStatus {
    Idle,
    Began,
    Placing(Entity),
    Rotating(Entity),
    // Stopped(Entity),
    Canceled(Entity),
    Completed(Entity),
}

impl Default for PlacementStatus {
    fn default() -> Self {
        PlacementStatus::Idle
    }
}

#[derive(Debug, Clone)]
pub struct CurrentPlacement<const U : usize> {
    pub status : PlacementStatus,
    pub constructor : Option<Entity>,
    pub data : Option<StackData>,
    pub ins_data : Option<InstantiationData>,
    pub entity : Option<Entity>,
    pub placing : [bool; U],
}

impl<const U : usize> CurrentPlacement<U> {
    pub fn new() -> Self {
        Self {
            status : PlacementStatus::Idle,
            constructor : None,
            data : None,
            ins_data : None,
            entity : None,
            placing : [false; U],
        }
    }

    pub fn placing(&self) -> bool {
        self.placing.first().map_or(false, |f| *f) || self.status != PlacementStatus::Idle
    }
}

pub fn building_placement_startup_system(mut commands : Commands) {
    commands.insert_resource(CurrentPlacement::<CLICK_BUFFER>::new());
}

pub fn building_placement_system(
    gltf_assets : Res<QLoader<GltfAsset, AssetServer>>,
    gltfs : Res<Assets<Gltf>>,
    gltf_meshes : Res<Assets<GltfMesh>>,
    cast : Res<CameraRaycast>,
    input : Res<Input<MouseButton>>,

    mut current_placement : ResMut<CurrentPlacement::<CLICK_BUFFER>>,

    team_players : Query<&TeamPlayer>,

    mut trans : Query<&mut Transform>,
    mut visibles : Query<&mut Visibility>,

    mut commands : Commands,
) {
    for i in 1..current_placement.placing.len() {
        current_placement.placing[i-1] = current_placement.placing[i];
    }
    let y = current_placement.status != PlacementStatus::Idle;
    if let Some(x) = current_placement.placing.last_mut() {
        *x = y;
    }
    let down = input.just_pressed(MouseButton::Left);
    let up = input.just_released(MouseButton::Left);
    match current_placement.status {
        PlacementStatus::Began => {
            let mut entity_builder = commands.spawn();

            if let Some(x) = &current_placement.data {
                let (mesh, material) = {
                    let m1 = gltf_assets.get(&x.id).clone();
                    let m2 = gltfs.get(m1.unwrap().0.clone());
                    let m3 = gltf_meshes.get(m2.unwrap().meshes[0].clone());
                    let m4 = m3.unwrap().primitives[0].clone();
                    (m4.mesh, m4.material.unwrap())
                };

                if let Some(lvc) = cast.last_valid_cast {
                    entity_builder.insert_bundle(PbrBundle {
                        mesh,
                        material,
                        transform : Transform::from_xyz(lvc.point.x, lvc.point.y, lvc.point.z),
                        ..Default::default()
                    });
                } else {
                    entity_builder.insert_bundle(PbrBundle {
                        mesh,
                        material,
                        transform : Transform::default(),
                        ..Default::default()
                    });
                }
            }
            entity_builder.insert(Visibility { is_visible: false});
            let e = entity_builder.id();
            current_placement.status = PlacementStatus::Placing(e);
            current_placement.entity = Some(e);
        },
        PlacementStatus::Placing(e) => {
            if let Ok(mut v) = visibles.get_mut(e) {
                if let (Some(cc), Ok(mut t)) = (cast.current_cast, trans.get_mut(e)) {
                    *t = Transform::from_xyz(cc.point.x, cc.point.y, cc.point.z);
                    v.is_visible = true;
                } else {
                    v.is_visible = false;
                }
            }
            if down {
                current_placement.status = PlacementStatus::Rotating(e);
            }
            if current_placement.constructor.map_or(false, |c| trans.get(c).is_err()) {
                current_placement.status = PlacementStatus::Canceled(e);
            }
        },
        PlacementStatus::Rotating(e) => {
            if let Some((constructor, entity)) = current_placement.constructor.zip(current_placement.entity) {
                if trans.get(constructor).is_err() {
                    current_placement.status = PlacementStatus::Canceled(e);
                    return;
                }
                if let Ok(mut tran) = trans.get_mut(entity) {
                    match cast.current_cast {
                        Some(raycast) => {
                            if (raycast.point.x - tran.translation.x).abs() > 0.01 || (raycast.point.z - tran.translation.z).abs() > 0.01 {
                                tran.look_at(Vec3::new(raycast.point.x, 0.0, raycast.point.z), Vec3::new(0.0, 1.0, 0.0));
                            }
                        },
                        None => {
                            // TODO: This should be able to rotate even when cursor is off map.
                        }
                    }
                    if up {
                        if let Ok(teamplayer) = team_players.get(constructor) {
                            current_placement.ins_data = Some(InstantiationData{transform : tran.clone(), spawn_point : None, end_point : None, team_player : *teamplayer, multiplayer : false, had_identifier : false,});
                            current_placement.status = PlacementStatus::Completed(e);
                        }
                    }
                }
            }
        },
        // PlacementStatus::Stopped(e) => {

        // },
        PlacementStatus::Canceled(e) => {
            commands.entity(e).despawn();
            current_placement.status = PlacementStatus::Idle;
        }
        _ => { }
    }
}