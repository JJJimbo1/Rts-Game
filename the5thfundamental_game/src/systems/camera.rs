use std::f32::{NEG_INFINITY, INFINITY,};

use bevy::{gltf::{Gltf, GltfMesh}, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*, render::camera::Camera};
use bevy_ninepatch::*;
use bevy_pathfinding::PathFinder;
use the5thfundamental_common::*;
use qloader::*;
use crate::*;

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
        .with_system(camera_context_focus_system.label(CameraSystems::CameraContextFocusSystem).after(CameraSystems::CameraRaycastResponseSystem))
        .with_system(selection_highlighter.label(CameraSystems::SelectionHighlighterSystem).after(CameraSystems::CameraRaycastResponseSystem))
        .with_system(command_system.label(CameraSystems::CommandSystem).after(CameraSystems::CameraRaycastResponseSystem))
}

pub struct CameraController {
    pub camera_root : Entity,
    pub camera_pivot : Entity,
    pub camera : Entity,

    outside_window : bool,
    just_entered : bool,
    holding : bool,
}

pub fn create_camera(camera_settings : Res<CameraSettings>, map : Res<Map>, mut commands : Commands) {
    let direction = Vec3::new(0.0, camera_settings.offset.1, camera_settings.offset.0).normalize_or_zero();

    let distance = mathfu::D1::normalize_from_01(mathfu::D1::clamp01(camera_settings.default_zoom), camera_settings.min_zoom, camera_settings.max_zoom);

    let min_z = direction.z * camera_settings.min_zoom;
    let min_y = direction.y * camera_settings.min_zoom;
    let max_z = direction.z * camera_settings.max_zoom;
    let max_y = direction.y * camera_settings.max_zoom;
    let z = direction.z * distance;
    let y = direction.y * distance;

    let mut transform = Transform::from_xyz(2.2, y, z);
    transform.look_at(Vec3::ZERO, Vec3::Y);

    let root_entity = commands.spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Velocity::new(0., 0., 0., true))
        .insert(Torque::default())
        .insert(LocalBounds {
            x : Vec2::new(-map.bounds.0 / 2.0, map.bounds.0 / 2.0),
            y : Vec2::new(NEG_INFINITY, INFINITY),
            z : Vec2::new(-map.bounds.1 / 2.0, map.bounds.1 / 2.0)
        }).id();

    let pivot_entity = commands.spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Torque::default())
        .insert(Parent(root_entity)).id();

    let camera_entity = commands.spawn_bundle(PerspectiveCameraBundle {
        transform,
        ..Default::default()
    })
        .insert(Velocity::new(0., 0., 0., true))
        .insert(LocalBounds {
            x : Vec2::new(0.0, 0.0),
            y : Vec2::new(min_y, max_y),
            z : Vec2::new(min_z, max_z),
        })
        .insert(Parent(pivot_entity)).id();

    commands.insert_resource(CameraController {
        camera_root : root_entity,
        camera_pivot : pivot_entity,
        camera : camera_entity,

        outside_window : false,
        just_entered : false,
        holding : false,
    });
}

//TODO: Abstract out key bindings.
pub fn camera_control_system(
    time : Res<Time>,
    mut controller : ResMut<CameraController>,
    settings : Res<CameraSettings>,
    windows : Res<Windows>,
    mut trans : Query<&mut Transform>,
    mut velocities : Query<&mut Velocity>,
    mut torques : Query<&mut Torque>,
    mouse_buttons : Res<Input<MouseButton>>,
    key_input : Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
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

    let height = trans.get_mut(controller.camera).map_or(1.0, |x|
        mathfu::D1::clamp(mathfu::D1::normalize_from_to(x.translation.y, settings.min_zoom, settings.max_zoom, settings.zoom_base, settings.zoom_base * settings.zoom_ratio), 0.01, 100.));

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

    let mut zoom= 0.0;
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                zoom = -ev.y;
            }
            MouseScrollUnit::Pixel => {
                zoom = -ev.y
            }
        }
    }
    let delta = mathfu::D1::clamp(time.delta_seconds(), 0.0, 1.0 / settings.minimum_fps_for_deltatime as f32);

    if let (Ok(mut tran), Ok(mut vel), Ok(mut tor)) = (trans.get_mut(controller.camera_root), velocities.get_mut(controller.camera_root), torques.get_mut(controller.camera_root)) {
        //*Rotation
        let mut dir = 0.;
        if key_input.pressed(KeyCode::Q) {
            tor.y = mathfu::D1::clamp(tor.y, 0., INFINITY);
            dir += 0.01;
        }

        if key_input.pressed(KeyCode::E) {
            tor.y = mathfu::D1::clamp(tor.y, NEG_INFINITY, 0.);
            dir -= 0.01;
        }

        if dir != 0. {
            tor.y = mathfu::D1::lerp(tor.y, dir * settings.max_rotation_speed * slow.0, mathfu::D1::clamp01(settings.rotation_acceleration * delta));
        } else {
            tor.y = mathfu::D1::lerp(tor.y, 0.0, mathfu::D1::clamp01(settings.rotation_deceleration * delta));
        }

        //*Scrolling
        if x.is_normal() {
            if mathfu::D1::same_sign(x, vel.x) {
                if mathfu::D1::farther_from_zero(x, vel.x) {
                    vel.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(vel.x, x, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    vel.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(vel.x, x, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
                    );
                }
            } else {
                if vel.x.abs() < settings.fast_decceleration_threshold {
                    vel.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(vel.x, x, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    vel.x = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(vel.x, x, mathfu::D1::clamp01(settings.scroll_deceleration * settings.fast_decceleration_strength * delta))
                    );
                }
            }
        } else {
            vel.x = mathfu::D1::more_than_or_zero_pog(
                mathfu::D1::lerp(vel.x, 0.0, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
            );
        }

        if z.is_normal() {
            if mathfu::D1::same_sign(z, vel.z) {
                if mathfu::D1::farther_from_zero(z, vel.z) {
                    vel.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(vel.z, z, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    vel.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(vel.z, z, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
                    );
                }
            } else {
                if vel.z.abs() < settings.fast_decceleration_threshold {
                    vel.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(vel.z, z, mathfu::D1::clamp01(settings.scroll_acceleration * delta))
                    );
                } else {
                    vel.z = mathfu::D1::more_than_or_zero_pog(
                        mathfu::D1::lerp(vel.z, z, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
                    );
                }
            }
        } else {
            vel.z = mathfu::D1::more_than_or_zero_pog(
                mathfu::D1::lerp(vel.z, 0.0, mathfu::D1::clamp01(settings.scroll_deceleration * delta))
            );
        }

        if key_input.pressed(KeyCode::Grave) {
            tran.rotation = Quat::default();
            tor.y = 0.;
        }
    }

    match (trans.get_mut(controller.camera), velocities.get_mut(controller.camera)) {
        //*Zooming
        (Ok(mut tran), Ok(mut vel)) => {

            if zoom > 0. {
                vel.z = mathfu::D1::clamp(vel.z, 0., INFINITY);
            } else if zoom < 0. {
                vel.z = mathfu::D1::clamp(vel.z, NEG_INFINITY, 0.);
            }

            if zoom != 0. {
                vel.z = mathfu::D1::lerp(vel.z, zoom * height * settings.max_zoom_speed * slow.2, mathfu::D1::clamp01(settings.zoom_acceleration * delta));
            } else {
                vel.z = mathfu::D1::lerp(vel.z, 0., mathfu::D1::clamp01(settings.zoom_deceleration * delta));
            }

            if key_input.pressed(KeyCode::Grave) {
                let dz = mathfu::D1::normalize_from_01(mathfu::D1::clamp01(settings.default_zoom), settings.min_zoom, settings.max_zoom);
                vel.z = 0.;
                let dir = Vec3::new(0.0, settings.offset.1, settings.offset.0).normalize();
                let y = dir.y * dz;
                let z = dir.z * dz;
                tran.translation = Vec3::new(0.0, y, z);
            }
            tran.look_at(Vec3::ZERO, Vec3::Y);
        },
        _ => {

        }
    };

    if (real_mouse_pos.x.abs() < threshholds.0.abs() && real_mouse_pos.y.abs() < threshholds.1.abs()) || !settings.post_action_stall {
        controller.just_entered = false;
        controller.holding = false;
    }
}

pub struct UiHit {
    pub hit : bool,
    pub holding : bool,
}

pub fn camera_raycast_system(
    windows : Res<Windows>,
    controller : Res<CameraController>,
    ui_hit : Res<UiHit>,
    mut cast : ResMut<CameraRaycast>,


    cameras : Query<(&GlobalTransform, &Camera)>,
    colliders : Query<(&Transform, &Collider)>,
) {
    cast.current_cast = None;
    if ui_hit.hit { return; }

    if let Some(ray_cast_result) = cameras.get(controller.camera)
        .map_or(None, |x| Some(x))
        .and_then(|(gt, c)| windows.get_primary()
            .and_then(|w| w.cursor_position())
            .and_then(|p| ray(p, &windows, c, gt)))
        .and_then(|ray| PhysicsWorld::ray_cast(&colliders, ray)) {
        cast.last_valid_cast = Some(ray_cast_result);
        cast.current_cast = Some(ray_cast_result);
    }
}

pub struct CameraSelector {
    selection_box_entity : Entity,
    mouse_start_pos : Vec2,
    mouse_end_pos : Vec2,
    box_selecting : bool,
    add_to_selection : bool,
    to_clear : bool,
}

impl CameraSelector {
    fn box_select(&self,
        physics_world : &PhysicsWorld,
        player_id : TeamPlayer,
        cam : &Camera,
        windows : &Windows,
        global_tran : &GlobalTransform,
        selectables : &mut Query<&mut Selectable>,
    ) {

        let ents = physics_world.box_cast(
            (self.mouse_start_pos.x.min(self.mouse_end_pos.x),
            self.mouse_start_pos.x.max(self.mouse_end_pos.x),
            self.mouse_start_pos.y.min(self.mouse_end_pos.y),
            self.mouse_start_pos.y.max(self.mouse_end_pos.y)),
            player_id, cam, windows, global_tran);

        if !self.add_to_selection || (self.to_clear && ents.len() > 0) {
            selectables.for_each_mut(|mut sel| {
                sel.selected = false;
            });
        }

        let mut empty = true;
        selectables.for_each_mut(|sel| {
            if sel.selected {
                empty = false;
            }
        });

        for e in ents.iter() {
            let mut context : Option<SelectableContext> = None;
            match selectables.get_mut(*e) {
                Ok(x) => {
                    context = Some(x.context);
                },
                _ => { }
            }
            match context {
                Some(x) => {
                    match x {
                        SelectableContext::Single => {
                            if (!self.add_to_selection || empty) && ents.len() == 1 {
                                selectables.for_each_mut(|mut sel| {
                                    sel.selected = false;
                                });
                                match selectables.get_mut(*e) {
                                    Ok(mut s) => {
                                        s.selected = true;
                                    },
                                    _ => { }
                                }
                            }
                        },
                        SelectableContext::MultiSelect => {
                            match selectables.get_mut(*e) {
                                Ok(mut s) => {
                                    s.selected = true;
                                },
                                _ => { }
                            }
                        },
                        SelectableContext::Clear => {

                        }
                    }
                },
                None => { }
            }
        }

    }

    fn show_or_hide_box(&self, visibles : &mut Query<&mut Visibility>, children : &Query<&Children>, styles : &mut Query<&mut Style>) -> bool {

        match styles.get_mut(self.selection_box_entity) {
            Ok(mut x) => {
                let extents = (self.mouse_start_pos.x.min(self.mouse_end_pos.x),
                self.mouse_start_pos.x.max(self.mouse_end_pos.x),
                self.mouse_start_pos.y.min(self.mouse_end_pos.y),
                self.mouse_start_pos.y.max(self.mouse_end_pos.y));

                x.position.left = Val::Px(extents.0);
                x.position.bottom = Val::Px(extents.2);
                x.size.width = Val::Px(extents.1 - extents.0);
                x.size.height = Val::Px(extents.3 - extents.2);
            },
            _ => { }
        }

        if mathfu::D2::distance(
            (self.mouse_start_pos.x, self.mouse_start_pos.y),
            (self.mouse_end_pos.x, self.mouse_end_pos.y)) >= 40.0 && self.box_selecting {
            self.open(visibles, children);
            return false;

        } else {
            self.close(visibles, children);
            return true;
        }
    }
}

impl Menu for CameraSelector {
    fn main_container(&self) -> Entity {
        self.selection_box_entity
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
        box_selecting : false,
        add_to_selection : false,
        to_clear : false,
    });

    commands.insert_resource(CurrentPlacement {
        status : PlacementStatus::Idle,
        constructor : None,
        data : None,
        ins_data : None,
        entity : None,
        // placed : [false; 8],
        placing : false,
    });
}

pub fn camera_raycast_response_system(
    camera : Res<CameraController>,
    mut selector : ResMut<CameraSelector>,
    placement : Res<CurrentPlacement>,
    physics_world : Res<PhysicsWorld>,
    cast : Res<CameraRaycast>,
    idents : Res<Identifiers>,

    windows : Res<Windows>,
    key_input : Res<Input<KeyCode>>,
    mouse_input : Res<Input<MouseButton>>,

    cameras : Query<(&GlobalTransform, &Camera)>,
    mut visibles : Query<&mut Visibility>,
    children : Query<&Children>,
    mut styles : Query<&mut Style>,
    mut selectables : Query<&mut Selectable>,
    team_players : Query<&TeamPlayer>,


) {
    if placement.placing { return; }
    // if placement.placed.first().map_or(false, |f| *f) { return; }
    if placement.status != PlacementStatus::Idle {
        return;
    }
    selector.add_to_selection = key_input.pressed(KeyCode::LShift) || key_input.pressed(KeyCode::RShift);
    selector.to_clear = false;
    let mouse_pos = windows.get_primary().unwrap().cursor_position().unwrap_or(Vec2::new(0.0, 0.0));
    let left_down = mouse_input.pressed(MouseButton::Left);
    let left_up = mouse_input.just_released(MouseButton::Left);
    let result = cameras.get(camera.camera).unwrap();

    if !selector.box_selecting && left_down {
        selector.mouse_start_pos = mouse_pos;
        selector.box_selecting = true;
    } else if selector.box_selecting {
        selector.mouse_end_pos = mouse_pos;
        if !left_down {
            selector.box_selecting = false;
            if mathfu::D2::distance(
            (selector.mouse_start_pos.x, selector.mouse_start_pos.y),
            (selector.mouse_end_pos.x, selector.mouse_end_pos.y)) >= 40.0 {
                selectables.for_each_mut(|sel| {
                    if sel.selected && sel.context == SelectableContext::Single {
                        selector.to_clear = true;
                    }
                });
                if selector.show_or_hide_box(&mut visibles, &children, &mut styles) {
                    selector.box_select(&physics_world, PLAYER_ID, &result.1, &windows, &result.0, &mut selectables);
                }
                return;
            }
        }
        selector.show_or_hide_box(&mut visibles, &children, &mut styles);

        let mut empty = true;
        selectables.for_each_mut(|sel| {
            if sel.selected == true {
                empty = false;
            }
        });

        if !left_up { return; }
        if let Some(rc) = cast.current_cast {
            if let Some(e) = idents.get_entity(rc.id) {
                let mut context : Option<(SelectableContext, TeamPlayer)> = None;
                if let (Ok(s), Ok(tp)) = (selectables.get_mut(e), team_players.get(e)) {
                    if *tp != PLAYER_ID {
                        return;
                    }
                    context = Some((s.context, *tp));
                }
                if let Some(ctp) = context {
                    match ctp.0 {
                        SelectableContext::Single => {
                            if !selector.add_to_selection || empty {
                                selectables.for_each_mut(|mut sel| {
                                    sel.selected = false;
                                });
                                if let Ok(mut s) = selectables.get_mut(e) {
                                    s.selected = true;
                                }
                            }
                        },
                        SelectableContext::MultiSelect => {
                            if selector.add_to_selection {
                                let mut to_clear = false;
                                selectables.for_each_mut(|sel| {
                                    match sel.context {
                                        SelectableContext::Single => {
                                            to_clear = true;
                                        },
                                        _ => { }
                                    }
                                });
                                if to_clear {
                                    selectables.for_each_mut(|mut sel| {
                                        sel.selected = false;
                                    });
                                }
                                if let Ok(mut s) = selectables.get_mut(e) {
                                    s.selected = true;
                                }
                            } else {
                                selectables.for_each_mut(|mut sel| {
                                    sel.selected = false;
                                });
                                if let Ok(mut s) = selectables.get_mut(e) {
                                    s.selected = true;
                                }
                            }
                        },
                        SelectableContext::Clear => {
                            if !selector.add_to_selection {
                                selectables.for_each_mut(|mut sel| {
                                    sel.selected = false;
                                });
                            }
                        }
                    }
                } else {
                    if !selector.add_to_selection {
                        selectables.for_each_mut(|mut sel| {
                            sel.selected = false;
                        });
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

    query : Query<(&Transform, &Collider, &Selectable)>,
    single : Query<(&Transform, &Collider)>,
) {
    PhysicsWorld::highlight_selected(&query, &mut debug_lines);
    if let Some(x) = cast.current_cast {
        if let Some(e) = idents.get_entity(x.id) {
            PhysicsWorld::highlight_single(e, &single, &mut debug_lines);
        }
    }
}

pub fn command_system(
    player : Res<Player>,
    cast : Res<CameraRaycast>,
    idents : Res<Identifiers>,
    current_placement : Res<CurrentPlacement>,
    // mut rand : ResMut<Random>,
    input : Res<Input<MouseButton>>,

    units : Query<(&SnowFlake, &Selectable), With<PathFinder>>,
    team_players : Query<&TeamPlayer>,
    teamplayer_world : Res<TeamPlayerWorld>,
    mut move_commands : EventWriter<MoveCommand>,
    mut attack_commands : EventWriter<AttackCommand>,
) {
    if current_placement.placing { return; }
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
    Stopped(Entity),
    Canceled(Entity),
    Completed(Entity),
}

impl Default for PlacementStatus {
    fn default() -> Self {
        PlacementStatus::Idle
    }
}

#[derive(Debug, Clone)]
pub struct CurrentPlacement {
    pub status : PlacementStatus,
    pub constructor : Option<Entity>,
    pub data : Option<StackData>,
    pub ins_data : Option<InstantiationData>,
    pub entity : Option<Entity>,
    // pub placed : [bool; U],
    pub placing : bool,
}

impl CurrentPlacement {
    pub fn new() -> Self {
        Self {
            status : PlacementStatus::Idle,
            constructor : None,
            data : None,
            ins_data : None,
            entity : None,
            // placed : [false; U],
            placing : false,
        }
    }
}

pub fn building_placement_startup_system(mut commands : Commands) {
    commands.insert_resource(CurrentPlacement::new());
}

pub fn building_placement_system(
    gltf_assets : Res<QLoader<GltfAsset, AssetServer>>,
    gltfs : Res<Assets<Gltf>>,
    gltf_meshes : Res<Assets<GltfMesh>>,
    cast : Res<CameraRaycast>,
    input : Res<Input<MouseButton>>,

    mut current_placement : ResMut<CurrentPlacement>,

    team_players : Query<&TeamPlayer>,

    mut trans : Query<&mut Transform>,
    mut visibles : Query<&mut Visibility>,

    mut commands : Commands,
) {
    // for i in 1..current_placement.placed.len() {
    //     current_placement.placed[i-1] = current_placement.placed[i];
    // }
    current_placement.placing = current_placement.status != PlacementStatus::Idle;
    // let y = current_placement.status != PlacementStatus::Idle;
    // if let Some(x) = current_placement.placed.last_mut() {
    //     *x = y;
    // }
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
                }
            }
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
        PlacementStatus::Stopped(e) => {

        },
        PlacementStatus::Canceled(e) => {
            commands.entity(e).despawn();
            current_placement.status = PlacementStatus::Idle;
        }
        _ => { }
    }
}