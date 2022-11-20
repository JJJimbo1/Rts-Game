use std::f32::{NEG_INFINITY, INFINITY,};

use bevy::{gltf::{Gltf}, input::mouse::MouseWheel, prelude::*, render::camera::Camera, math::Vec3Swizzles};
use bevy_ninepatch::*;
use bevy_rapier3d::{prelude::InteractionGroups, plugin::RapierContext};
use mathfu::D1;
use the5thfundamental_common::*;
use qloader::*;
use crate::{*, utility::assets::{ImageAsset, GltfAsset}};

pub const CLICK_BUFFER : usize = 8;

pub fn camera_setup_system_set(set : SystemSet) -> SystemSet {
    set.label(SystemSets::Camera)
        .with_system(create_camera)
        .with_system(create_selector)
        .with_system(building_placement_startup_system)
}

pub fn camera_system_set(set : SystemSet) -> SystemSet {
    set.label(SystemSets::Camera)
        .with_system(camera_control_system)
        .with_system(camera_raycast_system.after(camera_control_system))
        .with_system(building_placement_system.after(camera_raycast_system))
        .with_system(camera_raycast_response_system.after(building_placement_system))
        .with_system(show_selection_box.after(camera_raycast_response_system))
        .with_system(camera_select.after(show_selection_box))
        .with_system(camera_context_focus_system.after(camera_raycast_response_system))
        .with_system(selection_highlighter.after(camera_raycast_response_system))
        .with_system(command_system.after(camera_raycast_response_system))
}

#[derive(Debug)]
#[derive(Resource)]
pub struct CameraController {
    pub camera_root: Entity,
    pub camera: Entity,

    pub root_velocity: Vec3,
    pub rotation_velocity: f32,
    pub zoom_precentage: f32,
    pub zoom_velocity: f32,

    outside_window: bool,
    just_entered: bool,
    holding: bool,
}

pub fn create_camera(
    settings: Res<CameraSettings>,
    map: Res<MapBounds>,
    mut commands: Commands
) {
    let (direction, distance) = settings.default_direction_and_distance();

    let z = direction.z * distance;
    let y = direction.y * distance;

    let mut transform = Transform::from_xyz(0.0, y, z);
    transform.look_at(Vec3::ZERO, Vec3::Y);

    let root_entity = commands.spawn((
        Transform::default(),
        GlobalTransform::default(),
        LocalBounds {
            x : Vec2::new(-map.0.x / 2.0, map.0.x / 2.0),
            y : Vec2::new(NEG_INFINITY, INFINITY),
            z : Vec2::new(-map.0.y / 2.0, map.0.y / 2.0)
        },
    )).id();

    let camera_entity = commands.spawn(Camera3dBundle {
        transform,
        ..Default::default()
    }).id();

    commands.entity(root_entity).add_child(camera_entity);

    commands.insert_resource(CameraController {
        camera_root : root_entity,
        camera : camera_entity,

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
    mut camera_controller : ResMut<CameraController>,
    mut scroll_event_reader: EventReader<MouseWheel>,
    mouse_buttons : Res<Input<MouseButton>>,
    key_input : Res<Input<KeyCode>>,
    windows : Res<Windows>,
    time : Res<Time>,
    mut trans : Query<&mut Transform>,
) {
    if windows.get_primary().is_none() { return; }
    let window = windows.get_primary().unwrap();
    let half_size = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    if mouse_buttons.pressed(MouseButton::Left) {
        camera_controller.holding = true;
    }

    let real_mouse_pos = match window.cursor_position() {
        Some(pos) => {
            if camera_controller.outside_window {
                camera_controller.just_entered = true;
            }
            camera_controller.outside_window = false;
            pos - half_size
        },
        None => {
            camera_controller.outside_window = true;
            Vec2::default()
        }
    };

    let adjusted_mouse_pos = window.cursor_position().map_or(Vec2::default(), |pos|
        if !camera_controller.outside_window && (!camera_controller.just_entered || !settings.post_action_stall) && !camera_controller.holding { pos - half_size } else { Vec2::default() });

    let threshholds : (f32, f32) = (
        mathfu::D1::normalize_from_01(settings.thresholds.0, 0., half_size.x),
        mathfu::D1::normalize_from_01(settings.thresholds.1, 0., half_size.y),
    );

    let height = trans.get_mut(camera_controller.camera).map_or(1.0, |x|{
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
    // let (x, z) = (
    //     mathfu::D1::clamp(hor * slow.1 * settings.max_scroll_speed * height,
    //     -settings.max_scroll_speed * height, settings.max_scroll_speed * height),
    //     mathfu::D1::clamp(vert * slow.1 * settings.max_scroll_speed * height,
    //     -settings.max_scroll_speed * height, settings.max_scroll_speed * height),
    // );

    let delta = mathfu::D1::clamp(time.delta_seconds(), 0.0, 1.0 / settings.minimum_fps_for_deltatime as f32);

    if let Ok(mut tran) = trans.get_mut(camera_controller.camera_root) {
        //*Rotation
        let mut dir = 0.;
        if key_input.pressed(KeyCode::Q) {
            camera_controller.rotation_velocity = mathfu::D1::clamp(camera_controller.rotation_velocity, 0., INFINITY);
            dir += 0.01;
        }

        if key_input.pressed(KeyCode::E) {
            camera_controller.rotation_velocity = mathfu::D1::clamp(camera_controller.rotation_velocity, NEG_INFINITY, 0.);
            dir -= 0.01;
        }

        if dir != 0. {
            camera_controller.rotation_velocity = mathfu::D1::lerp(camera_controller.rotation_velocity, dir * settings.max_rotation_speed * slow.0, mathfu::D1::clamp01(settings.rotation_acceleration * delta));
        } else {
            camera_controller.rotation_velocity = mathfu::D1::lerp(camera_controller.rotation_velocity, 0.0, mathfu::D1::clamp01(settings.rotation_deceleration * delta));
        }
        tran.rotate(Quat::from_rotation_y(camera_controller.rotation_velocity));

        //*Scrolling
        if x.is_normal() {
            if mathfu::D1::same_sign(x, camera_controller.root_velocity.x) {
                if mathfu::D1::farther_from_zero(x, camera_controller.root_velocity.x) {
                    camera_controller.root_velocity.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(camera_controller.root_velocity.x, x, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    camera_controller.root_velocity.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(camera_controller.root_velocity.x, x, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
                    );
                }
            } else {
                if camera_controller.root_velocity.x.abs() < settings.fast_decceleration_threshold {
                    camera_controller.root_velocity.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(camera_controller.root_velocity.x, x, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    camera_controller.root_velocity.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(camera_controller.root_velocity.x, x, mathfu::D1::clamp01(settings.scroll_deceleration * settings.fast_decceleration_strength * delta))
                    );
                }
            }
        } else {
            camera_controller.root_velocity.x = mathfu::D1::more_than_or_zero_pog(
                mathfu::D1::lerp(camera_controller.root_velocity.x, 0.0, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
            );
        }

        if z.is_normal() {
            if mathfu::D1::same_sign(z, camera_controller.root_velocity.z) {
                if mathfu::D1::farther_from_zero(z, camera_controller.root_velocity.z) {
                    camera_controller.root_velocity.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(camera_controller.root_velocity.z, z, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    camera_controller.root_velocity.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(camera_controller.root_velocity.z, z, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
                    );
                }
            } else {
                if camera_controller.root_velocity.z.abs() < settings.fast_decceleration_threshold {
                    camera_controller.root_velocity.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(camera_controller.root_velocity.z, z, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    camera_controller.root_velocity.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(camera_controller.root_velocity.z, z, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
                    );
                }
            }
        } else {
            camera_controller.root_velocity.z = mathfu::D1::more_than_or_zero_pog(
                mathfu::D1::lerp(camera_controller.root_velocity.z, 0.0, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
            );
        }

        let movement = tran.rotation * camera_controller.root_velocity * delta;
        tran.translation += movement;

        if key_input.just_pressed(KeyCode::Grave) {
            tran.rotation = Quat::default();
        }
    }

    if let Ok(mut tran) = trans.get_mut(camera_controller.camera) {
        let mut zoom_add = 0.0;
        for ev in scroll_event_reader.iter() {
            zoom_add = -ev.y;
        }

        if zoom_add > 0. {
            zoom_add = mathfu::D1::clamp(zoom_add, 0., INFINITY);
        } else if zoom_add < 0. {
            zoom_add = mathfu::D1::clamp(zoom_add, NEG_INFINITY, 0.);
        }

        if zoom_add != 0. {
            camera_controller.zoom_velocity = mathfu::D1::lerp(camera_controller.zoom_velocity, zoom_add * height * settings.max_zoom_speed * slow.2, mathfu::D1::clamp01(settings.zoom_acceleration * delta));
        } else {
            camera_controller.zoom_velocity = mathfu::D1::lerp(camera_controller.zoom_velocity, 0., mathfu::D1::clamp01(settings.zoom_deceleration * delta));
        }

        camera_controller.zoom_precentage = D1::clamp01(camera_controller.zoom_precentage + camera_controller.zoom_velocity * delta);

        let direction = Vec3::new(0.0, D1::normalize_from_01(camera_controller.zoom_precentage, settings.min_zoom().y, settings.max_zoom().y),
            D1::normalize_from_01(camera_controller.zoom_precentage, settings.min_zoom().z, settings.max_zoom().z)).normalize_or_zero();
        let distance = D1::normalize_from_01(camera_controller.zoom_precentage, settings.min_zoom, settings.max_zoom);
        tran.translation = direction * distance;

        if key_input.just_pressed(KeyCode::Grave) {
            let (direction, distance) = settings.default_direction_and_distance();
            tran.translation = direction * distance;
            camera_controller.zoom_velocity = 0.0;
            camera_controller.zoom_precentage = settings.default_zoom;
        }
        tran.look_at(Vec3::ZERO, Vec3::Y);
    }

    if (real_mouse_pos.x.abs() < threshholds.0.abs() && real_mouse_pos.y.abs() < threshholds.1.abs()) || !settings.post_action_stall {
        camera_controller.just_entered = false;
        camera_controller.holding = false;
    }
}

pub fn camera_raycast_system(
    controller : Res<CameraController>,
    windows : Res<Windows>,
    ui_hit : Res<UiHit<CLICK_BUFFER>>,
    context : Res<RapierContext>,
    mut cast : ResMut<CameraRaycast>,
    cameras : Query<(&GlobalTransform, &Camera)>,
) {
    cast.current_cast = None;
    if ui_hit.hit() { return; }

    if let Ok((gl_transform, camera)) = cameras.get(controller.camera) {
        if let Some(cursor) = windows.get_primary().and_then(|w| w.cursor_position()) {
            let (origin, direction) = ray(cursor, &windows, camera, gl_transform);
            if let Some((entity, len)) = context.cast_ray(origin, direction, f32::MAX, true, InteractionGroups::all().into()) {
                let point = origin + direction * len;
                let cam_cast = RayCastResult { entity, point, len};
                cast.last_valid_cast = Some(cam_cast);
                cast.current_cast = Some(cam_cast);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BoxSelectStatus {
    Idle,
    Dragging,
}

#[derive(Debug, Clone, Copy)]
#[derive(Resource)]
pub struct CameraSelector {
    selection_box_entity : Entity,
    mouse_start_pos : Vec2,
    mouse_end_pos : Vec2,
    minimum_distance : f32,
    status: BoxSelectStatus,
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
    mut asset_server : ResMut<AssetServer>,
    // textures : Res<QLoader<ImageAsset, AssetServer>>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    mut commands : Commands
) {
    let select = asset_server.load(ImageAsset::SelectionBox);
    let nine_patch = nine_patches.add(NinePatchBuilder::by_margins(2, 2, 2, 2));
    let mut entity_commands = commands.spawn(NinePatchBundle {
        style: Style {
            position_type : PositionType::Absolute,
            size: Size::new(Val::Px(0.0), Val::Percent(0.0)),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        nine_patch_data : NinePatchData {
            nine_patch,
            texture : select,
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
            selector.close(&mut visibles);
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
                selector.open(&mut visibles);
            } else {
                selector.close(&mut visibles);
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
    mut activation_events : EventWriter<ActivationEvent>,
    camera : Res<CameraController>,
    cast : Res<CameraRaycast>,
    player : Res<Player>,

    key_input : Res<Input<KeyCode>>,

    mut units : Query<(Entity, &GlobalTransform, &mut Selectable, &TeamPlayer)>,
    cameras : Query<(&GlobalTransform, &Camera)>,
) {
    for event in selection_event.iter() {
        // println!("send selection event of {:?}", event);

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
                    .and_then(|cast| units.get_mut(cast.entity)
                    .ok()
                    .and_then(|(ent, _, _, _,)| Some(ent))
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
                    println!("activate");
                    activation_events.send(ActivationEvent { entity: ent, player: player.0 });
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
                            if let Some(center) = camera.world_to_viewport(cam_tran, gl_tran.translation()) {
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
    // mut debug_lines : ResMut<DebugLines>,

    query : Query<(&GlobalTransform, &Selectable)>,
) {
    if let Some((glt, _)) = cast.current_cast
        .and_then(|c| query.get(c.entity)
            .map_or(None, |x| Some(x))
    ) {
        // debug_lines.line_colored(
        //     glt.translation,
        //     glt.translation + Vec3::Y * 10.0,
        //     0.0,
        //     Color::rgba(0.1, 0.35, 0.45, 1.0),
        // );
    }

    query.for_each(|(glt, sel)| {
        if sel.selected {
            // debug_lines.line_colored(
            //     glt.translation,
            //     glt.translation + Vec3::Y * 3.0,
            //     0.0,
            //     Color::rgba(0.1, 0.35, 0.45, 1.0),
            // );
        }
    });
}

pub fn command_system(
    player : Res<Player>,
    cast : Res<CameraRaycast>,
    current_placement : Res<CurrentPlacement<CLICK_BUFFER>>,
    input : Res<Input<MouseButton>>,

    units : Query<(Entity, &Selectable), With<GroundPathFinder>>,
    team_players : Query<&TeamPlayer>,
    teamplayer_world : Res<TeamPlayerWorld>,
    mut unit_commands : EventWriter<UnitCommandEvent>,
) {
    if current_placement.placing() { return; }
    if input.just_released(MouseButton::Right) {
        if let Some(ray_cast) = cast.current_cast {
            // if idents.get_entity(ray_cast.entity)
                if teamplayer_world.is_enemy(ray_cast.entity, player.0, &team_players)
                    .map_or(false, |t| t) {
                let command = UnitCommandEvent{
                    units : units.iter().filter_map(|(id, sel)| if sel.selected { Some(id) } else { None }).collect(),
                    command_type: UnitCommandType::Attack(ray_cast.entity),
                };
                unit_commands.send(command);
            } else {
                unit_commands.send(UnitCommandEvent {
                    units : units.iter().filter_map(|(id, sel)| if sel.selected { Some(id) } else { None }).collect(),
                    command_type: UnitCommandType::Move(ray_cast.point.xz()),
                });
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlacementStatus {
    Idle,
    Began(PrePlacementInfo),
    Placing(PlacementInfo),
    Rotating(PlacementInfo),
    // Stopped(Entity),
    Canceled(PlacementInfo),
    Completed(PlacementInfo, ObjectSpawnEventData),
}

impl Default for PlacementStatus {
    fn default() -> Self {
        PlacementStatus::Idle
    }
}

#[derive(Debug, Clone)]
#[derive(Resource)]
pub struct CurrentPlacement<const U : usize> {
    pub status : PlacementStatus,
    pub placing : [bool; U],
}

impl<const U : usize> CurrentPlacement<U> {
    pub fn new() -> Self {
        Self {
            status : PlacementStatus::Idle,
            placing : [false; U],
        }
    }

    pub fn placing(&self) -> bool {
        self.placing.first().map_or(false, |f| *f) || if let PlacementStatus::Idle = self.status { false } else { true }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrePlacementInfo {
    pub constructor: Entity,
    pub queue: ActiveQueue,
    pub data: StackData,
}

#[derive(Debug, Clone, Copy)]
pub struct PlacementInfo {
    pub constructor: Entity,
    pub ghost: Entity,
    pub queue: ActiveQueue,
    pub data: StackData,
}

impl From<(PrePlacementInfo, Entity)> for PlacementInfo {
    fn from((ppi, e): (PrePlacementInfo, Entity)) -> Self {
        Self {
            constructor: ppi.constructor,
            ghost: e,
            queue: ppi.queue,
            data: ppi.data,
        }
    }
}

pub fn building_placement_startup_system(mut commands : Commands) {
    commands.insert_resource(CurrentPlacement::<CLICK_BUFFER>::new());
}

pub fn building_placement_system(
    mut spawn_event_writer: EventWriter<ObjectSpawnEvent>,
    mut asset_server : ResMut<AssetServer>,
    
    cast : Res<CameraRaycast>,
    input : Res<Input<MouseButton>>,

    mut current_placement : ResMut<CurrentPlacement::<CLICK_BUFFER>>,

    team_players : Query<&TeamPlayer>,
    mut queueses: Query<&mut Queues>,

    mut trans : Query<&mut Transform>,
    mut visibles : Query<&mut Visibility>,

    mut commands : Commands,
) {
    for i in 1..current_placement.placing.len() {
        current_placement.placing[i-1] = current_placement.placing[i];
    }
    let y = if let PlacementStatus::Idle = current_placement.status { false } else { true };
    if let Some(x) = current_placement.placing.last_mut() {
        *x = y;
    }
    let down = input.just_pressed(MouseButton::Left);
    let up = input.just_released(MouseButton::Left);
    //Todo: Fix visibility. Bug with Bevy?
    match current_placement.status {
        PlacementStatus::Began(info) => {
            let mut entity_builder = commands.spawn(SpatialBundle {
                visibility: Visibility { is_visible: false },
                ..default()
            });

            let gltf = asset_server.load(GltfAsset::from(AssetType::from(info.data.object_type)));
            entity_builder.with_children(|parent| {
                parent.spawn(SceneBundle{
                    scene: gltf,
                    ..default()
                });
            });
            let ghost = entity_builder.id();
            current_placement.status = PlacementStatus::Placing((info, ghost).into());
        },
        PlacementStatus::Placing(info) => {
            if let (Ok(mut v), Ok(mut t)) = (visibles.get_mut(info.ghost), trans.get_mut(info.ghost)) {
                if let Some(cc) = cast.current_cast {
                    *t = Transform::from_xyz(cc.point.x, cc.point.y, cc.point.z);
                    v.is_visible = true;
                } else {
                    // *t = Transform::from_xyz(0.0, -10000000000.0, 0.0);
                    v.is_visible = false;
                }
            }
            if down {
                println!("down");
                current_placement.status = PlacementStatus::Rotating(info);
            }
            if trans.get(info.constructor).is_err() {
                current_placement.status = PlacementStatus::Canceled(info);
            }
        },
        PlacementStatus::Rotating(info) => {
            if trans.get(info.constructor).is_err() {
                current_placement.status = PlacementStatus::Canceled(info);
                return;
            }
            let mut tran = trans.get_mut(info.ghost).unwrap();
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
                if let Ok(teamplayer) = team_players.get(info.constructor) {
                    let spawn_data = ObjectSpawnEventData {
                        object_type: info.data.object_type,
                        snowflake: Snowflake::new(),
                        transform : tran.clone(),
                        teamplayer : *teamplayer
                    };
                    current_placement.status = PlacementStatus::Completed(info, spawn_data.clone());
                }
            }
        },
        // PlacementStatus::Stopped(e) => {

        // },
        PlacementStatus::Canceled(info) => {
            commands.entity(info.ghost).despawn();
            current_placement.status = PlacementStatus::Idle;
        }
        PlacementStatus::Completed(info, spawn_data) => {
            spawn_event_writer.send(ObjectSpawnEvent(spawn_data.clone()));
            // requests.request(ObjectType::Building, current_placement.data.clone().unwrap().id.clone(), current_placement.spawn_data.clone().unwrap(), Some(e));
            if let Ok(mut x) = queueses.get_mut(info.constructor) {
                x.queues.get_mut(&info.queue).unwrap().remove_from_buffer(&info.data);
            }
            commands.entity(info.ghost).despawn_recursive();
            current_placement.status = PlacementStatus::Idle;
        },
        _ => { }
    }
}