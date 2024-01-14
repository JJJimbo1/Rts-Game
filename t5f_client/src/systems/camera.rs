use std::f32::{NEG_INFINITY, INFINITY,};

use bevy::{input::{mouse::{MouseWheel, MouseButtonInput}, ButtonState}, prelude::*, render::camera::Camera, math::Vec3Swizzles, window::PrimaryWindow};
// use bevy_ninepatch::*;
use bevy_rapier3d::{prelude::CollisionGroups, plugin::RapierContext};
use serde::{Serialize, Deserialize};
use t5f_common::*;
use t5f_utility::mathfu::*;
use crate::*;

pub static CLEAR_COLOR : Color = Color::rgba_linear(0.0, 0.2, 0.7, 1.0);
pub const CLICK_BUFFER : usize = 8;

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

#[derive(Serialize, Deserialize)]
#[derive(Resource)]
pub struct CameraSettings {
    pub offset_y: (f32, f32),
    pub offset_z: (f32, f32),
    pub curve_power: (f32, f32),

    //Settings relating to Rotation.
    pub max_rotation_speed: f32,
    pub rotation_acceleration: f32,
    pub rotation_deceleration: f32,
    pub slow_rotation_multiplier: f32,
    pub rotation_acceleration_curve: f32,

    //Setting relating to Scrolling.
    pub thresholds: (f32, f32),
    pub max_scroll_speed: f32,
    pub scroll_acceleration: f32,
    pub scroll_deceleration: f32,
    pub fast_decceleration_threshold: f32,
    pub fast_decceleration_strength: f32,
    pub slow_scroll_multiplier: f32,
    pub scroll_acceleration_curve: f32,
    pub use_scroll_button: bool,
    pub scroll_button_speed_multiplier: f32,
    pub post_action_stall: bool,

    //Settings relating to Zooming.
    pub max_zoom_speed: f32,
    pub zoom_acceleration: f32,
    pub zoom_deceleration: f32,
    pub slow_zoom_multiplier: f32,
    pub zoom_acceleration_curve: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub default_zoom: f32,
    pub zoom_base: f32,
    pub zoom_ratio: f32,
    pub zoom_curve_weight: f32,

    //Settings relating to Rotation, Scroll and Zoom.
    pub minimum_fps_for_deltatime : u16,
}

impl CameraSettings {
    pub fn default_direction_and_distance(&self) -> (Vec3, f32) {
        (Vec3::new(0.0, d1::normalize_from_01(self.default_zoom, self.min_zoom().y, self.max_zoom().y),
        d1::normalize_from_01(self.default_zoom, self.min_zoom().z, self.max_zoom().z)).normalize_or_zero(),
        d1::normalize_from_01(self.default_zoom, self.min_zoom, self.max_zoom))
    }

    pub fn min_zoom(&self) -> Vec3 {
        Vec3::new(0.0, self.offset_y.0 * self.min_zoom, self.offset_z.0 * self.min_zoom)
    }

    pub fn max_zoom(&self) -> Vec3 {
        Vec3::new(0.0, self.offset_y.1 * self.max_zoom, self.offset_z.1 * self.max_zoom)
    }
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            offset_y: (0.5, 1.0),
            offset_z: (0.65, 1.0),
            curve_power: (1.5, 1.5),

            max_rotation_speed: 5.0,
            rotation_acceleration: 3.,
            rotation_deceleration: 15.,
            slow_rotation_multiplier: 0.125,
            rotation_acceleration_curve: 2.,

            thresholds: (0.94, 0.9),
            max_scroll_speed: 30.,
            scroll_acceleration: 3.0,
            scroll_deceleration: 15.,
            fast_decceleration_threshold: 100.,
            fast_decceleration_strength: 1.0,
            slow_scroll_multiplier: 0.125,
            scroll_acceleration_curve: 2.,
            use_scroll_button: true,
            scroll_button_speed_multiplier: 0.5,
            post_action_stall: true,

            #[cfg(target_family = "wasm")]
            max_zoom_speed: 0.018,
            #[cfg(not(target_family = "wasm"))]
            max_zoom_speed: 1.8,
            zoom_acceleration: 3.0,
            zoom_deceleration: 15.0,
            slow_zoom_multiplier: 0.125,
            zoom_acceleration_curve: 2.0,
            min_zoom: 60.,
            max_zoom: 120.,
            default_zoom: 0.8,
            zoom_base: 10.0,
            zoom_ratio: 5.0,
            zoom_curve_weight: 0.25,

            minimum_fps_for_deltatime: 20,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BoxSelectStatus {
    Idle,
    Dragging,
}

#[derive(Debug, Copy, Clone)]
pub struct RayCastResult {
    pub entity : Entity,
    pub point : Vec3,
    pub len : f32,
}

#[derive(Debug, Default, Copy, Clone)]
#[derive(Resource)]
pub struct CameraRaycast {
    pub last_valid_cast : Option<RayCastResult>,
    pub current_cast : Option<RayCastResult>,
}

#[derive(Debug, Clone, Copy)]
#[derive(Event)]
pub enum SelectionEvent {
    Single,
    Box(Vec2, Vec2),
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

#[derive(Debug, Clone)]
pub enum PlacementStatus {
    Idle,
    Began(PrePlacementInfo),
    Placing(PlacementInfo),
    Rotating(PlacementInfo),
    // Stopped(Entity),
    Canceled(PlacementInfo),
    Completed(PlacementInfo),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrePlacementInfo {
    pub constructor: Entity,
    pub queue: ActiveQueue,
    pub data: StackData,
}

#[derive(Debug, Clone)]
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

pub struct CameraPlugin;

impl CameraPlugin {
    pub fn create_camera(
        settings: Res<CameraSettings>,
        map: Res<MapBounds>,
        mut commands: Commands
    ) {
        println!("CREATING CAMERA");
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

    pub fn create_selector(
        image_assets : Res<ImageAssets>,
        // textures : Res<QLoader<ImageAsset, AssetServer>>,
        // mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
        mut commands : Commands
    ) {
        let select = image_assets.selection_box.clone();
        let entity = commands.spawn(ImageBundle {
            style: Style {
                position_type : PositionType::Absolute,
                width: Val::Px(0.0),
                height: Val::Px(0.0),
                ..Default::default()
            },
            image: UiImage::new(select),
            visibility: Visibility::Inherited,
            ..Default::default()
        }).id();

        //TODO: Use this when Ninepatch gets patched.
        // let nine_patch: Handle<NinePatchBuilder> = nine_patches.add(NinePatchBuilder::by_margins(2, 2, 2, 2));
        // let entity = commands.spawn((NinePatchBundle {
        //     style: Style {
        //         position_type : PositionType::Absolute,
        //         width: Val::Px(0.0),
        //         // height: Val::Percent(0.0),
        //         height: Val::Px(0.0),
        //         // size: Size::new(Val::Px(0.0), Val::Percent(0.0)),
        //         // justify_content: JustifyContent::Center,
        //         ..Default::default()
        //     },
        //     nine_patch_data : NinePatchData {
        //         nine_patch,
        //         texture : select,
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // }, Visibility::Inherited)).id();

        let container_entity = entity;
        commands.insert_resource(CameraSelector {
            selection_box_entity : container_entity,
            mouse_start_pos : Vec2::ZERO,
            mouse_end_pos : Vec2::ZERO,
            //TODO: Add setting for this
            minimum_distance: 40.0,
            status: BoxSelectStatus::Idle,
        });

        commands.insert_resource(CurrentPlacement::<CLICK_BUFFER> {
            status : PlacementStatus::Idle,
            placing : [false; CLICK_BUFFER],
        });
    }

    //TODO: Abstract out key bindings.
    pub fn camera_control_system(
        settings : Res<CameraSettings>,
        mut camera_controller : ResMut<CameraController>,
        mut scroll_event_reader: EventReader<MouseWheel>,
        mouse_buttons : Res<Input<MouseButton>>,
        key_input : Res<Input<KeyCode>>,
        window : Query<&Window, With<PrimaryWindow>>,
        time : Res<Time>,
        mut trans : Query<&mut Transform>,
    ) {
        let window = window.get_single().unwrap();
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
            d1::normalize_from_01(settings.thresholds.0, 0., half_size.x),
            d1::normalize_from_01(settings.thresholds.1, 0., half_size.y),
        );

        let height = trans.get_mut(camera_controller.camera).map_or(1.0, |x|{
            d1::normalize_from_to(x.translation.distance(Vec3::default()), settings.min_zoom().length(), settings.max_zoom().length(),
            settings.zoom_base, settings.zoom_base * settings.zoom_ratio).clamp(settings.zoom_base, settings.zoom_base * settings.zoom_ratio)
        });

        let slow = if key_input.pressed(KeyCode::C) {
            (settings.slow_rotation_multiplier, settings.slow_scroll_multiplier, settings.slow_zoom_multiplier)
        } else {
            (1., 1., 1.)
        };

        let mouse_dir = Vec3::new(adjusted_mouse_pos.x, 0.0, -adjusted_mouse_pos.y).normalize_or_zero();

        let mags : (f32,f32) = (
            d1::powf_sign(d1::normalize_to_01(adjusted_mouse_pos.x.abs(), threshholds.0, half_size.x).clamp(0.0, 1.0), settings.scroll_acceleration_curve),
            d1::powf_sign(d1::normalize_to_01(adjusted_mouse_pos.y.abs(), threshholds.1, half_size.y).clamp(0.0, 1.0), settings.scroll_acceleration_curve),
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
            ((scroll_dir_x + hor) * slow.1 * settings.max_scroll_speed * height).clamp(-settings.max_scroll_speed * height, settings.max_scroll_speed * height),
            ((scroll_dir_z + vert) * slow.1 * settings.max_scroll_speed * height).clamp(-settings.max_scroll_speed * height, settings.max_scroll_speed * height),
        );

        let delta = (time.delta_seconds()).clamp(0.0, 1.0 / settings.minimum_fps_for_deltatime as f32);

        if let Ok(mut tran) = trans.get_mut(camera_controller.camera_root) {
            //*Rotation
            let mut dir = 0.;
            if key_input.pressed(KeyCode::Q) {
                camera_controller.rotation_velocity = camera_controller.rotation_velocity.clamp(0., INFINITY);
                dir += 0.01;
            }

            if key_input.pressed(KeyCode::E) {
                camera_controller.rotation_velocity = camera_controller.rotation_velocity.clamp(NEG_INFINITY, 0.);
                dir -= 0.01;
            }

            if dir != 0. {
                camera_controller.rotation_velocity = d1::lerp(camera_controller.rotation_velocity, dir * settings.max_rotation_speed * slow.0, (settings.rotation_acceleration * delta).clamp(0.0, 1.0));
            } else {
                camera_controller.rotation_velocity = d1::lerp(camera_controller.rotation_velocity, 0.0, (settings.rotation_deceleration * delta).clamp(0.0, 1.0));
            }
            tran.rotate(Quat::from_rotation_y(camera_controller.rotation_velocity));

            //*Scrolling
            if x.is_normal() {
                if x.signum() == camera_controller.root_velocity.x.signum() {
                    if d1::farther_from_zero(x, camera_controller.root_velocity.x) {
                        camera_controller.root_velocity.x = d1::more_than_or_zero_pog(
                            d1::lerp(camera_controller.root_velocity.x, x, (settings.scroll_acceleration * delta).clamp(0.0,1.0))
                        );
                    } else {
                        camera_controller.root_velocity.x = d1::more_than_or_zero_pog(
                            d1::lerp(camera_controller.root_velocity.x, x, (settings.scroll_deceleration * delta).clamp(0.0, 1.0))
                        );
                    }
                } else {
                    if camera_controller.root_velocity.x.abs() < settings.fast_decceleration_threshold {
                        camera_controller.root_velocity.x = d1::more_than_or_zero_pog(
                            d1::lerp(camera_controller.root_velocity.x, x, (settings.scroll_acceleration * delta).clamp(0.0, 1.0))
                        );
                    } else {
                        camera_controller.root_velocity.x = d1::more_than_or_zero_pog(
                            d1::lerp(camera_controller.root_velocity.x, x, (settings.scroll_deceleration * settings.fast_decceleration_strength * delta).clamp(0.0, 1.0))
                        );
                    }
                }
            } else {
                camera_controller.root_velocity.x = d1::more_than_or_zero_pog(
                    d1::lerp(camera_controller.root_velocity.x, 0.0, (settings.scroll_deceleration * delta).clamp(0.0, 1.0))
                );
            }

            if z.is_normal() {
                if z.signum() == camera_controller.root_velocity.z.signum() {
                    if d1::farther_from_zero(z, camera_controller.root_velocity.z) {
                        camera_controller.root_velocity.z = d1::more_than_or_zero_pog(
                            d1::lerp(camera_controller.root_velocity.z, z, (settings.scroll_acceleration * delta).clamp(0.0, 1.0))
                        );
                    } else {
                        camera_controller.root_velocity.z = d1::more_than_or_zero_pog(
                            d1::lerp(camera_controller.root_velocity.z, z, (settings.scroll_deceleration * delta).clamp(0.0, 1.0))
                        );
                    }
                } else {
                    if camera_controller.root_velocity.z.abs() < settings.fast_decceleration_threshold {
                        camera_controller.root_velocity.z = d1::more_than_or_zero_pog(
                            d1::lerp(camera_controller.root_velocity.z, z, (settings.scroll_acceleration * delta).clamp(0.0, 1.0))
                        );
                    } else {
                        camera_controller.root_velocity.z = d1::more_than_or_zero_pog(
                            d1::lerp(camera_controller.root_velocity.z, z, (settings.scroll_deceleration * settings.fast_decceleration_strength * delta).clamp(0.0, 1.0))
                        );
                    }
                }
            } else {
                camera_controller.root_velocity.z = d1::more_than_or_zero_pog(
                    d1::lerp(camera_controller.root_velocity.z, 0.0, (settings.scroll_deceleration * delta).clamp(0.0, 1.0))
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
            for ev in scroll_event_reader.read() {
                zoom_add = -ev.y;
            }

            if zoom_add > 0. {
                zoom_add = zoom_add.clamp(0.0, INFINITY);
            } else if zoom_add < 0. {
                zoom_add = zoom_add.clamp(NEG_INFINITY, 0.0);
            }

            if zoom_add != 0. {
                camera_controller.zoom_velocity = d1::lerp(camera_controller.zoom_velocity, zoom_add * height * settings.max_zoom_speed * slow.2, (settings.zoom_acceleration * delta).clamp(0.0, 1.0));
            } else {
                camera_controller.zoom_velocity = d1::lerp(camera_controller.zoom_velocity, 0., (settings.zoom_deceleration * delta).clamp(0.0, 1.0));
            }

            camera_controller.zoom_precentage = (camera_controller.zoom_precentage + camera_controller.zoom_velocity * delta).clamp(0.0, 1.0);

            let direction = Vec3::new(0.0, d1::normalize_from_01(camera_controller.zoom_precentage, settings.min_zoom().y, settings.max_zoom().y),
                d1::normalize_from_01(camera_controller.zoom_precentage, settings.min_zoom().z, settings.max_zoom().z)).normalize_or_zero();
            let distance = d1::normalize_from_01(camera_controller.zoom_precentage, settings.min_zoom, settings.max_zoom);
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

    pub fn ui_hit_detection_system(
        mut ui_hit : ResMut<UiHit<CLICK_BUFFER>>,
        mut input : EventReader<MouseButtonInput>,
        interaction_query: Query<
            (&Interaction, &InheritedVisibility),
            (Changed<Interaction>, With<BlocksRaycast>),
        >,
    ) {
        *ui_hit.hitting.last_mut().unwrap() = false;
        for b in 1..ui_hit.hitting.len() {
            ui_hit.hitting[b-1] = ui_hit.hitting[b]
        }

        interaction_query.for_each(|(interaction, visibility)| {
            if visibility.get() {
                match interaction {
                    Interaction::Pressed => {
                        for b in 0..ui_hit.hitting.len() {
                            ui_hit.hitting[b] = true;
                        }
                        ui_hit.holding = true;
                    },
                    _ => { }
                }
            }
        });
        for event in input.read() {
            match event.state {
                ButtonState::Released => {
                    if event.button == MouseButton::Left {
                        if ui_hit.holding {
                            ui_hit.holding = false;
                            for b in 0..ui_hit.hitting.len() {
                                ui_hit.hitting[b] = true;
                            }
                        }
                    }
                },
                _ => { }
            }
        }
    }

    pub fn camera_raycast_system(
        controller : Res<CameraController>,
        ui_hit : Res<UiHit<CLICK_BUFFER>>,
        context : Res<RapierContext>,
        mut cast : ResMut<CameraRaycast>,
        windows : Query<&Window, With<PrimaryWindow>>,
        cameras : Query<(&GlobalTransform, &Camera)>,
    ) {
        cast.current_cast = None;
        if ui_hit.hit() { return; }

        if let Ok((gl_transform, camera)) = cameras.get(controller.camera) {
            let Ok(window) = windows.get_single() else { return; };
            let Some(cursor) = window.cursor_position() else { return; };
            let Some(ray) = camera.viewport_to_world(gl_transform, cursor) else { return; };
            if let Some((entity, len)) = context.cast_ray(ray.origin, ray.direction, f32::MAX, true, CollisionGroups::default().into()) {
                let point = ray.origin + ray.direction * len;
                let cam_cast = RayCastResult { entity, point, len};
                cast.last_valid_cast = Some(cam_cast);
                cast.current_cast = Some(cam_cast);
            }
        }
    }

    pub fn camera_raycast_response_system(
        mut selection_events : EventWriter<SelectionEvent>,
        mut selector : ResMut<CameraSelector>,
        ui_hit : Res<UiHit<CLICK_BUFFER>>,
        placement : Res<CurrentPlacement<CLICK_BUFFER>>,
        mouse_input : Res<Input<MouseButton>>,
        window : Query<&Window, With<PrimaryWindow>>,
    ) {
        if ui_hit.hit()
        || placement.placing()
        { return; }

        match selector.status {
            BoxSelectStatus::Idle => {
                if mouse_input.just_pressed(MouseButton::Left) {
                    selector.mouse_start_pos = window.get_single().ok().and_then(|w| w.cursor_position()).unwrap_or(Vec2::new(0.0, 0.0));
                    selector.mouse_end_pos = selector.mouse_start_pos;
                    selector.status = BoxSelectStatus::Dragging;
                }
            },
            BoxSelectStatus::Dragging => {
                if let Some(p) = window.get_single().ok().and_then(|w| w.cursor_position()) { selector.mouse_end_pos = p; }
                // selector.mouse_end_pos = window.get_single().ok().and_then(|w| w.cursor_position()).unwrap_or(Vec2::new(0.0, 0.0));
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

    pub fn show_selection_box(
        selector : Res<CameraSelector>,
        placement : Res<CurrentPlacement<CLICK_BUFFER>>,

        mut styles : Query<&mut Style>,
        mut visible_query: Query<(&mut Visibility, &InheritedVisibility)>,
    ) {
        if placement.placing() { return; };
        match selector.status {
            BoxSelectStatus::Idle => {
                selector.close(&mut visible_query);
            },
            BoxSelectStatus::Dragging => {
                if let Ok(mut style) = styles.get_mut(selector.selection_box_entity) {
                    let extents = (selector.mouse_start_pos.x.min(selector.mouse_end_pos.x), selector.mouse_start_pos.x.max(selector.mouse_end_pos.x),
                        selector.mouse_start_pos.y.min(selector.mouse_end_pos.y), selector.mouse_start_pos.y.max(selector.mouse_end_pos.y));

                    style.left = Val::Px(extents.0);
                    style.top = Val::Px(extents.2);
                    style.width = Val::Px(extents.1 - extents.0);
                    style.height = Val::Px(extents.3 - extents.2);
                }

                if selector.showing() {
                    selector.open(&mut visible_query);
                } else {
                    selector.close(&mut visible_query);
                }
            }
        }
    }

    pub fn camera_select(
        mut selection_event : EventReader<SelectionEvent>,
        mut command_events : EventWriter<CommandEvent>,
        camera : Res<CameraController>,
        cast : Res<CameraRaycast>,
        player : Res<Player>,

        key_input : Res<Input<KeyCode>>,

        mut objects : Query<(Entity, &GlobalTransform, &mut Selectable, &TeamPlayer)>,
        cameras : Query<(&GlobalTransform, &Camera)>,
    ) {
        for event in selection_event.read() {
            let add_to_selection = key_input.pressed(KeyCode::ShiftLeft) || key_input.pressed(KeyCode::ShiftRight);
            let mut empty = true;
            objects.for_each(|(_, _, sel, _)| {
                if sel.selected {
                    empty = false;
                }
            });

            match event {
                SelectionEvent::Single => {
                    let entity = cast.current_cast
                        .and_then(|cast| objects.get_mut(cast.entity)
                        .ok()
                        .and_then(|(ent, _, _, _,)| Some(ent))
                    );
                    if let Some(ent) = entity {
                        let clear = objects.get_mut(ent).unwrap().2.context == SelectableContext::Clear;
                        if clear && !add_to_selection {
                            objects.for_each_mut(|(_, _, mut selectable, team_player)| {
                                if *team_player == player.0 {
                                    selectable.selected = false;
                                }
                            });
                            continue;
                        }
                        if !empty && add_to_selection {
                            let (_, _, mut sel, tp) = objects.get_mut(ent).unwrap();
                            if sel.context == SelectableContext::MultiSelect {
                                if *tp == player.0 {
                                    sel.selected = true;
                                }
                            }
                        } else {
                            objects.for_each_mut(|(_, _, mut selectable, team_player)| {
                                if *team_player == player.0 {
                                    selectable.selected = false;
                                }
                            });
                            let (_, _, mut sel, tp) = objects.get_mut(ent).unwrap();
                            if *tp == player.0 {
                                sel.selected = true;
                            }
                        }
                        let command_event = CommandEvent {
                            player: player.0,
                            object: Some(CommandObject::Structure(ent)),
                            command: CommandType::Activate,
                        };
                        command_events.send(command_event);
                        // command_events.send(ActivationEvent { entity: ent, player: player.0 });
                    } else {
                        if !add_to_selection {
                            objects.for_each_mut(|(_, _, mut selectable, team_player)| {
                                if *team_player == player.0 {
                                    selectable.selected = false;
                                }
                            });
                        }
                    }
                },
                SelectionEvent::Box(min, max) => {
                    let (min, max) = (min.min(*max), max.max(*min));
                    if let Ok((cam_tran, camera)) = cameras.get(camera.camera) {

                        if !add_to_selection {
                            objects.for_each_mut(|(_, _, mut selectable, team_player)| {
                                if *team_player == player.0 {
                                    selectable.selected = false;
                                }
                            });
                        }

                        let mut ents = Vec::new();

                        objects.for_each(|(ent, gl_tran, _, tp)| {
                            if *tp == player.0 {
                                if let Some(center) = camera.world_to_viewport(cam_tran, gl_tran.translation()) {
                                    if center.x > min.x && center.x < max.x
                                    && center.y > min.y && center.y < max.y {
                                        ents.push(ent);
                                    }
                                }
                            }
                        });

                        for e in ents.iter() {
                            let (_, _, mut sel, _) = objects.get_mut(*e).unwrap();
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
        mut gizmos: Gizmos,
        query : Query<(&GlobalTransform, &Selectable)>,
    ) {
        if let Some((glt, _)) = cast.current_cast
            .and_then(|c| query.get(c.entity)
                .map_or(None, |x| Some(x))
        ) {
            let tr = Transform::from(*glt);
            gizmos.line(
                tr.translation,
                tr.translation + Vec3::Y * 25.0,
                Color::rgba(0.1, 0.35, 0.45, 1.0),
            );
        }

        query.for_each(|(glt, sel)| {
            let tr = Transform::from(*glt);
            if sel.selected {
                gizmos.line(
                    tr.translation,
                    tr.translation + Vec3::Y * 5.0,
                    Color::rgba(0.1, 0.35, 0.45, 1.0),
                );
            }
        });
    }

    pub fn command_system(
        mut unit_commands : EventWriter<CommandEvent>,
        player : Res<Player>,
        cast : Res<CameraRaycast>,
        current_placement : Res<CurrentPlacement<CLICK_BUFFER>>,
        input : Res<Input<MouseButton>>,

        units : Query<(Entity, &Selectable), With<t5f_common::PathFinder>>,
        team_players : Query<&TeamPlayer>,
        teamplayer_world : Res<TeamPlayerWorld>,
    ) {
        if current_placement.placing() { return; }
        if input.just_released(MouseButton::Right) {
            if let Some(ray_cast) = cast.current_cast {
                // if idents.get_entity(ray_cast.entity)
                if teamplayer_world.is_enemy(ray_cast.entity, player.0, &team_players)
                    .map_or(false, |t| t) {
                    let command = CommandEvent{
                        player: player.0,
                        object : Some(CommandObject::Units(units.iter().filter_map(|(id, sel)| if sel.selected { Some(id) } else { None }).collect())),
                        command: CommandType::Attack(ray_cast.entity),
                    };
                    unit_commands.send(command);
                } else {
                    unit_commands.send(CommandEvent {
                        player: player.0,
                        object : Some(CommandObject::Units(units.iter().filter_map(|(id, sel)| if sel.selected { Some(id) } else { None }).collect())),
                        command: CommandType::Move(ray_cast.point.xz()),
                    });
                }
            }
        }
    }

    pub fn building_placement_system(
        mut command_events: EventWriter<CommandEvent>,
        mut current_placement : ResMut<CurrentPlacement::<CLICK_BUFFER>>,
        input : Res<Input<MouseButton>>,
        cast : Res<CameraRaycast>,
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
        match current_placement.status.clone() {
            PlacementStatus::Began(info) => {
                let ghost = commands.spawn(SpatialBundle {
                    visibility: Visibility::Inherited,
                    ..default()
                }).id();

                if let Ok(teamplayer) = team_players.get(info.constructor) {
                    let command_event = CommandEvent {
                        player: *teamplayer,
                        object: Some(CommandObject::Structure(ghost)),
                        command: CommandType::Build(BuildStatus::Begin(info.data.object.clone())),
                    };
                    command_events.send(command_event);
                }
                current_placement.status = PlacementStatus::Placing((info, ghost).into());
            },
            PlacementStatus::Placing(info) => {
                if let (Ok(mut v), Ok(mut t)) = (visibles.get_mut(info.ghost), trans.get_mut(info.ghost)) {
                    if let Some(cc) = cast.current_cast {
                        *t = Transform::from_xyz(cc.point.x, cc.point.y, cc.point.z);
                        *v = Visibility::Visible;
                    } else {
                        // *t = Transform::from_xyz(0.0, -10000000000.0, 0.0);
                        *v = Visibility::Hidden;
                    }
                }
                if down {
                    current_placement.status = PlacementStatus::Rotating(info.clone());
                }
                if trans.get(info.constructor).is_err() {
                    current_placement.status = PlacementStatus::Canceled(info.clone());
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
                        let command_event = CommandEvent {
                            player: *teamplayer,
                            object: Some(CommandObject::Structure(info.ghost)),
                            command: CommandType::Build(BuildStatus::Finish(*tran)),
                        };
                        command_events.send(command_event);
                    }
                    current_placement.status = PlacementStatus::Completed(info);
                }
            },
            // PlacementStatus::Stopped(e) => {

            // },
            // TODO: Not yet implemented
            PlacementStatus::Canceled(info) => {
                commands.entity(info.ghost).despawn();
                current_placement.status = PlacementStatus::Idle;
            }
            PlacementStatus::Completed(info) => {

                // command_events.send(ObjectSpawnEvent(spawn_data.clone(), PhantomData));
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
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SelectionEvent>()
            .insert_resource(CurrentPlacement::<CLICK_BUFFER>::new())
            .add_systems(OnEnter(GameState::SingleplayerGame), (
                Self::create_camera,
                Self::create_selector,
            ))
            .add_systems(Update, (
                Self::camera_control_system,
                Self::ui_hit_detection_system.after(Self::camera_control_system),
                Self::camera_raycast_system.after(Self::ui_hit_detection_system),
                Self::building_placement_system.after(Self::camera_raycast_system),
                Self::camera_raycast_response_system.after(Self::building_placement_system),
                Self::show_selection_box.after(Self::camera_raycast_response_system),
                Self::camera_select.after(Self::show_selection_box),
                Self::camera_context_focus_system.after(Self::camera_raycast_response_system),
                Self::selection_highlighter.after(Self::camera_raycast_response_system),
                Self::command_system.after(Self::camera_raycast_response_system),
            ).run_if(in_state(GameState::SingleplayerGame)))
        ;
    }
}