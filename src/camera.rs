use std::f32::{NEG_INFINITY, INFINITY,};
use bevy::{input::{mouse::{MouseButtonInput, MouseWheel}, ButtonState}, math::Vec3Swizzles, prelude::*, render::camera::Camera, ui::widget::NodeImageMode, window::PrimaryWindow};
use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use serde::{Serialize, Deserialize};
use crate::*;

pub static CLEAR_COLOR: Color = Color::linear_rgba(0.0, 0.2, 0.7, 1.0);
pub const CLICK_BUFFER: usize = 1;

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

    //* Camera rotation settings.
    pub max_rotation_speed: f32,
    pub rotation_acceleration: f32,
    pub rotation_deceleration: f32,
    pub slow_rotation_multiplier: f32,
    pub rotation_acceleration_curve: f32,

    //* Camera scrolling settings.
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

    //* Camera zooming settings.
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

    //* Misc settings.
    pub minimum_fps_for_deltatime: u16,
}

impl CameraSettings {
    pub fn default_direction_and_distance(&self) -> (Vec3, f32) {
        (Vec3::new(0.0, self.default_zoom.remap(0.0, 1.0, self.min_zoom().y, self.max_zoom().y),
        self.default_zoom.remap(0.0, 1.0, self.min_zoom().z, self.max_zoom().z)).normalize_or_zero(),
        self.default_zoom.remap(0.0, 1.0, self.min_zoom, self.max_zoom))
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
            min_zoom: 50.,
            max_zoom: 150.,
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
    pub entity: Entity,
    pub point: Vec3,
    pub len: f32,
}

#[derive(Debug, Default, Copy, Clone)]
#[derive(Resource)]
pub struct CameraRaycast {
    pub last_valid_cast: Option<RayCastResult>,
    pub current_cast: Option<RayCastResult>,
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
    selection_box_entity: Entity,
    mouse_start_pos: Vec2,
    mouse_end_pos: Vec2,
    minimum_distance: f32,
    status: BoxSelectStatus,
}

impl CameraSelector {
    pub fn showing(&self) -> bool {
        self.mouse_start_pos.distance(self.mouse_end_pos) >= self.minimum_distance
    }
}

#[derive(Debug, Clone)]
pub enum PlacementStatus {
    Idle,
    Began(PrePlacementInfo),
    Placing(PlacementInfo),
    Rotating(PlacementInfo),
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
pub struct CurrentPlacement<const U: usize> {
    pub status: PlacementStatus,
    pub placing: [bool; U],
}

impl<const U: usize> CurrentPlacement<U> {
    pub fn new() -> Self {
        Self {
            status: PlacementStatus::Idle,
            placing: [false; U],
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
        let (direction, distance) = settings.default_direction_and_distance();

        let z = direction.z * distance;
        let y = direction.y * distance;

        let mut transform = Transform::from_xyz(0.0, y, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);

        let root_entity = commands.spawn((
            Transform::default(),
            GlobalTransform::default(),
            LocalBounds {
                x: Vec2::new(-map.0.x / 2.0, map.0.x / 2.0),
                y: Vec2::new(NEG_INFINITY, INFINITY),
                z: Vec2::new(-map.0.y / 2.0, map.0.y / 2.0)
            },
        )).id();

        let camera_entity = commands.spawn((Camera3d::default(), transform)).id();

        commands.entity(root_entity).add_child(camera_entity);

        commands.insert_resource(CameraController {
            camera_root: root_entity,
            camera: camera_entity,

            root_velocity: Vec3::default(),
            rotation_velocity: 0.0,
            zoom_precentage: settings.default_zoom,
            zoom_velocity: 0.0,

            outside_window: false,
            just_entered: false,
            holding: false,
        });
    }

    pub fn create_selector(
        image_assets: Res<ImageAssets>,
        mut commands: Commands
    ) {
        let entity = commands.spawn((ImageNode {
            image: image_assets.selection_box.clone(),
            image_mode: NodeImageMode::Sliced(SelectBoxUI::slicer()),
            ..default()
        }, Node {
            position_type: PositionType::Absolute,
            ..default()
        })).id();

        commands.insert_resource(CameraSelector {
            selection_box_entity: entity,
            mouse_start_pos: Vec2::ZERO,
            mouse_end_pos: Vec2::ZERO,
            //TODO: Add setting for this
            minimum_distance: 40.0,
            status: BoxSelectStatus::Idle,
        });

        commands.insert_resource(CurrentPlacement::<CLICK_BUFFER> {
            status: PlacementStatus::Idle,
            placing: [false; CLICK_BUFFER],
        });
    }

    //TODO: Abstract out key bindings.
    pub fn camera_control_system(
        settings: Res<CameraSettings>,
        mut camera_controller: ResMut<CameraController>,
        mut scroll_event_reader: EventReader<MouseWheel>,
        mouse_buttons: Res<ButtonInput<MouseButton>>,
        key_input: Res<ButtonInput<KeyCode>>,
        window: Query<&Window, With<PrimaryWindow>>,
        time: Res<Time>,
        mut trans: Query<&mut Transform>,
    ) {
        let window = window.single().unwrap();
        let half_size = Vec2::new(window.width() / 2.0, window.height() / 2.0);
        if mouse_buttons.pressed(MouseButton::Left) {
            camera_controller.holding = true;
        }

        let mouse_pos = window.cursor_position().map_or(Vec2::default(), |pos|
            if !camera_controller.outside_window && (!camera_controller.just_entered || !settings.post_action_stall) && !camera_controller.holding { pos - half_size } else { Vec2::default() }
        );

        let threshholds: (f32, f32) = (
            settings.thresholds.0.remap(0.0, 1.0, 0., half_size.x),
            settings.thresholds.1.remap(0.0, 1.0, 0., half_size.y),
        );

        let height = trans.get_mut(camera_controller.camera).map_or(1.0, |x|{
            x.translation.length().remap_clamped(settings.min_zoom().length(), settings.max_zoom().length(),
            settings.zoom_base, settings.zoom_base * settings.zoom_ratio)
        });

        let slow = if key_input.pressed(KeyCode::KeyC) {
            (settings.slow_rotation_multiplier, settings.slow_scroll_multiplier, settings.slow_zoom_multiplier)
        } else {
            (1., 1., 1.)
        };

        let mouse_dir = Vec3::new(mouse_pos.x, 0.0, mouse_pos.y).normalize_or_zero();

        let mags: (f32,f32) = (
            mouse_pos.x.abs().remap_clamped(threshholds.0, half_size.x, 0.0, 1.0).powf_signum(settings.scroll_acceleration_curve),
            mouse_pos.y.abs().remap_clamped(threshholds.1, half_size.y, 0.0, 1.0).powf_signum(settings.scroll_acceleration_curve),
        );

        let hor = {
            let mut h = 0.0;
            if key_input.pressed(KeyCode::KeyD) { h += 1.0; }
            if key_input.pressed(KeyCode::KeyA) { h -= 1.0; }
            h * settings.scroll_button_speed_multiplier
        };

        let vert = {
            let mut v = 0.0;
            if key_input.pressed(KeyCode::KeyW) { v -= 1.0; }
            if key_input.pressed(KeyCode::KeyS) { v += 1.0; }
            v * settings.scroll_button_speed_multiplier
        };

        let scroll_dir_x = mouse_dir.x * mags.0.max(mags.1);
        let scroll_dir_z = mouse_dir.z * mags.0.max(mags.1);

        let (x, z) = (
            ((scroll_dir_x + hor) * slow.1 * settings.max_scroll_speed * height).clamp(-settings.max_scroll_speed * height, settings.max_scroll_speed * height),
            ((scroll_dir_z + vert) * slow.1 * settings.max_scroll_speed * height).clamp(-settings.max_scroll_speed * height, settings.max_scroll_speed * height),
        );

        let delta = (time.delta_secs()).clamp(0.0, 1.0 / settings.minimum_fps_for_deltatime as f32);

        if let Ok(mut tran) = trans.get_mut(camera_controller.camera_root) {
            //* Rotation
            let mut dir = 0.;
            if key_input.pressed(KeyCode::KeyQ) {
                camera_controller.rotation_velocity = camera_controller.rotation_velocity.clamp(0., INFINITY);
                dir += 1.0;
            }

            if key_input.pressed(KeyCode::KeyE) {
                camera_controller.rotation_velocity = camera_controller.rotation_velocity.clamp(NEG_INFINITY, 0.);
                dir -= 1.0;
            }

            if dir != 0. {
                camera_controller.rotation_velocity = camera_controller.rotation_velocity.lerp(dir * delta * settings.max_rotation_speed * slow.0, (settings.rotation_acceleration * delta).clamp(0.0, 1.0));
            } else {
                camera_controller.rotation_velocity = camera_controller.rotation_velocity.lerp(0.0, (settings.rotation_deceleration * delta).clamp(0.0, 1.0));
            }
            tran.rotate(Quat::from_rotation_y(camera_controller.rotation_velocity));

            if key_input.just_pressed(KeyCode::Backquote) {
                tran.rotation = Quat::default();
                camera_controller.rotation_velocity = 0.0;
            }

            //* Scrolling

            let x_acceleration = {
                delta * if !x.is_normal() { settings.scroll_deceleration }
                else if x.signum() == camera_controller.root_velocity.x.signum() {
                    if x.abs() > camera_controller.root_velocity.x.abs() { settings.scroll_acceleration } else { settings.scroll_deceleration }
                } else {
                    if camera_controller.root_velocity.x.abs() < settings.fast_decceleration_threshold { settings.scroll_acceleration } else { settings.scroll_deceleration * settings.fast_decceleration_strength }
                }
            }.clamp(0.0, 1.0);

            let z_acceleration = {
                delta * if !z.is_normal() { settings.scroll_deceleration }
                else if z.signum() == camera_controller.root_velocity.z.signum() {
                    if z.abs() > camera_controller.root_velocity.z.abs() { settings.scroll_acceleration } else { settings.scroll_deceleration }
                } else {
                    if camera_controller.root_velocity.z.abs() < settings.fast_decceleration_threshold { settings.scroll_acceleration } else { settings.scroll_deceleration * settings.fast_decceleration_strength }
                }
            }.clamp(0.0, 1.0);

            camera_controller.root_velocity.x = camera_controller.root_velocity.x.lerp(x, x_acceleration).deadzone(0.001);
            camera_controller.root_velocity.z = camera_controller.root_velocity.z.lerp(z, z_acceleration).deadzone(0.001);

            let movement = tran.rotation * camera_controller.root_velocity * delta;
            tran.translation += movement;
        }

        if let Ok(mut tran) = trans.get_mut(camera_controller.camera) {
            //* Zooming
            let mut zoom_add = 0.0;
            for ev in scroll_event_reader.read() {
                zoom_add = (0.01 * -ev.y.signum()) / delta;
            }

            if zoom_add > 0.0 {
                zoom_add = zoom_add.clamp(0.0, INFINITY);
            } else if zoom_add < 0. {
                zoom_add = zoom_add.clamp(NEG_INFINITY, 0.0);
            }

            if zoom_add != 0.0 {
                camera_controller.zoom_velocity = camera_controller.zoom_velocity.lerp(zoom_add * height * settings.max_zoom_speed * slow.2, (settings.zoom_acceleration * delta).clamp(0.0, 1.0));
            } else {
                camera_controller.zoom_velocity = camera_controller.zoom_velocity.lerp(0., (settings.zoom_deceleration * delta).clamp(0.0, 1.0));
            }

            camera_controller.zoom_precentage = (camera_controller.zoom_precentage + camera_controller.zoom_velocity * delta).clamp(0.0, 1.0);

            let direction = Vec3::new(0.0, camera_controller.zoom_precentage.remap(0.0, 1.0, settings.min_zoom().y, settings.max_zoom().y),
                camera_controller.zoom_precentage.remap(0.0, 1.0, settings.min_zoom().z, settings.max_zoom().z)).normalize_or_zero();
            let distance = camera_controller.zoom_precentage.remap(0.0, 1.0, settings.min_zoom, settings.max_zoom);
            tran.translation = direction * distance;

            if key_input.just_pressed(KeyCode::Backquote) {
                let (direction, distance) = settings.default_direction_and_distance();
                tran.translation = direction * distance;
                camera_controller.zoom_velocity = 0.0;
                camera_controller.zoom_precentage = settings.default_zoom;
            }
            tran.look_at(Vec3::ZERO, Vec3::Y);
        }

        let mouse_pos = match window.cursor_position() {
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

        if (mouse_pos.x.abs() < threshholds.0.abs() && mouse_pos.y.abs() < threshholds.1.abs()) || !settings.post_action_stall {
            camera_controller.just_entered = false;
            camera_controller.holding = false;
        }
    }

    pub fn ui_hit_detection_system(
        mut ui_hit: ResMut<UiHit<CLICK_BUFFER>>,
        mut input: EventReader<MouseButtonInput>,
        interaction_query: Query<
            (&Interaction, &InheritedVisibility),
            (Changed<Interaction>, With<BlocksRaycast>),
        >,
    ) {
        *ui_hit.hitting.last_mut().unwrap() = false;
        for b in 1..ui_hit.hitting.len() {
            ui_hit.hitting[b-1] = ui_hit.hitting[b]
        }

        interaction_query.iter().for_each(|(interaction, visibility)| {
            if visibility.get() && *interaction == Interaction::Pressed {
                for b in 0..ui_hit.hitting.len() {
                    ui_hit.hitting[b] = true;
                }
                ui_hit.holding = true;
            }
        });
        for event in input.read() {
            if event.state == ButtonState::Released && event.button == MouseButton::Left && ui_hit.holding {
                ui_hit.holding = false;
                for b in 0..ui_hit.hitting.len() {
                    ui_hit.hitting[b] = true;
                }
            }
        }
    }

    pub fn camera_raycast_system(
        controller: Res<CameraController>,
        ui_hit: Res<UiHit<CLICK_BUFFER>>,
        context: SpatialQuery,
        mut cast: ResMut<CameraRaycast>,
        windows: Query<&Window, With<PrimaryWindow>>,
        cameras: Query<(&GlobalTransform, &Camera)>,
    ) {
        cast.current_cast = None;
        if ui_hit.hit() { return; }

        if let Ok((gl_transform, camera)) = cameras.get(controller.camera) {
            let Ok(window) = windows.single() else { return; };
            let Some(cursor) = window.cursor_position() else { return; };
            let Ok(ray) = camera.viewport_to_world(gl_transform, cursor) else { return; };
            if let Some(hit) = context.cast_ray(ray.origin, ray.direction.into(), f32::MAX, true, &SpatialQueryFilter::default()) {
                let point = ray.origin + ray.direction * hit.distance;
                let cam_cast = RayCastResult { entity: hit.entity, point, len: hit.distance};
                cast.last_valid_cast = Some(cam_cast);
                cast.current_cast = Some(cam_cast);
            }
        }
    }

    pub fn camera_raycast_response_system(
        mut selection_events: EventWriter<SelectionEvent>,
        mut selector: ResMut<CameraSelector>,
        ui_hit: Res<UiHit<CLICK_BUFFER>>,
        placement: Res<CurrentPlacement<CLICK_BUFFER>>,
        mouse_input: Res<ButtonInput<MouseButton>>,
        window: Query<&Window, With<PrimaryWindow>>,
    ) {
        if ui_hit.hit()
        || placement.placing()
        { return; }

        match selector.status {
            BoxSelectStatus::Idle => {
                if mouse_input.just_pressed(MouseButton::Left) {
                    selector.mouse_start_pos = window.single().ok().and_then(|w| w.cursor_position()).unwrap_or(Vec2::new(0.0, 0.0));
                    selector.mouse_end_pos = selector.mouse_start_pos;
                    selector.status = BoxSelectStatus::Dragging;
                }
            },
            BoxSelectStatus::Dragging => {
                if let Some(p) = window.single().ok().and_then(|w| w.cursor_position()) { selector.mouse_end_pos = p; }
                if mouse_input.just_released(MouseButton::Left) {
                    selector.status = BoxSelectStatus::Idle;
                    if selector.showing() {
                        selection_events.write(SelectionEvent::Box(selector.mouse_start_pos, selector.mouse_end_pos));
                    } else {
                        selection_events.write(SelectionEvent::Single);
                    }
                }
            },
        }
    }

    pub fn show_selection_box(
        selector: Res<CameraSelector>,
        placement: Res<CurrentPlacement<CLICK_BUFFER>>,

        mut nodes: Query<&mut Node>,
        mut visible_query: Query<(&mut Visibility, &InheritedVisibility)>,
    ) {
        if placement.placing() { return; };
        match selector.status {
            BoxSelectStatus::Idle => {
                close(&mut visible_query, selector.selection_box_entity);
            },
            BoxSelectStatus::Dragging => {
                if let Ok(mut style) = nodes.get_mut(selector.selection_box_entity) {
                    let extents = (
                        selector.mouse_start_pos.x.min(selector.mouse_end_pos.x),
                        selector.mouse_start_pos.x.max(selector.mouse_end_pos.x),
                        selector.mouse_start_pos.y.min(selector.mouse_end_pos.y),
                        selector.mouse_start_pos.y.max(selector.mouse_end_pos.y),
                    );

                    style.left = Val::Px(extents.0);
                    style.top = Val::Px(extents.2);
                    style.width = Val::Px(extents.1 - extents.0);
                    style.height = Val::Px(extents.3 - extents.2);
                }

                if selector.showing() {
                    open(&mut visible_query, selector.selection_box_entity);
                } else {
                    close(&mut visible_query, selector.selection_box_entity);
                }
            }
        }
    }

    pub fn camera_select(
        mut selection_event: EventReader<SelectionEvent>,
        mut command_events: EventWriter<CommandEvent>,
        camera: Res<CameraController>,
        cast: Res<CameraRaycast>,
        player: Res<LocalPlayer>,

        key_input: Res<ButtonInput<KeyCode>>,

        mut objects: Query<(Entity, &GlobalTransform, &mut Selectable, &TeamPlayer)>,
        cameras: Query<(&GlobalTransform, &Camera)>,
    ) {
        for event in selection_event.read() {
            let add_to_selection = key_input.pressed(KeyCode::ShiftLeft) || key_input.pressed(KeyCode::ShiftRight);
            let mut empty = true;
            objects.iter().for_each(|(_, _, sel, _)| {
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
                        let clear = objects.get_mut(ent).unwrap().2.context == SelectableType::Clear;
                        if clear && !add_to_selection {
                            objects.iter_mut().for_each(|(_, _, mut selectable, team_player)| {
                                if *team_player == player.0 {
                                    selectable.selected = false;
                                }
                            });
                            continue;
                        }
                        if !empty && add_to_selection {
                            let (_, _, mut sel, tp) = objects.get_mut(ent).unwrap();
                            if sel.context == SelectableType::MultiSelect {
                                if *tp == player.0 {
                                    sel.selected = true;
                                }
                            }
                        } else {
                            objects.iter_mut().for_each(|(_, _, mut selectable, team_player)| {
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
                            objects: vec![ent],
                            command: CommandType::Activate,
                        };
                        command_events.write(command_event);
                    } else {
                        if !add_to_selection {
                            objects.iter_mut().for_each(|(_, _, mut selectable, team_player)| {
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
                            objects.iter_mut().for_each(|(_, _, mut selectable, team_player)| {
                                if *team_player == player.0 {
                                    selectable.selected = false;
                                }
                            });
                        }

                        let mut ents = Vec::new();

                        objects.iter().for_each(|(ent, gl_tran, _, tp)| {
                            if *tp == player.0 {
                                if let Ok(center) = camera.world_to_viewport(cam_tran, gl_tran.translation()) {
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
                                SelectableType::Single => {
                                    if ents.len() == 1 {
                                        sel.selected = true;
                                    }
                                },
                                SelectableType::MultiSelect => {
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
        mut focus: ResMut<ContextFocus>,
        selectables: Query<(Entity, &Selectable)>
    ) {
        let selects = selectables.iter().filter_map(|(e, s)| if s.selected && s.context == SelectableType::Single { Some(e) } else { None}).collect::<Vec<Entity>>();
        if selects.len() == 1 {
            focus.0 = selects.first().cloned();
        } else {
            focus.0 = None;
        }
    }

    pub fn selection_highlighter(
        cast: Res<CameraRaycast>,
        mut gizmos: Gizmos,
        query: Query<(&GlobalTransform, &Selectable)>,
    ) {
        if let Some((glt, _)) = cast.current_cast
            .and_then(|c| query.get(c.entity)
                .map_or(None, |x| Some(x))
        ) {
            let tr = Transform::from(*glt);
            gizmos.line(
                tr.translation,
                tr.translation + Vec3::Y * 25.0,
                Color::srgba(0.1, 0.35, 0.45, 1.0),
            );
        }

        query.iter().for_each(|(glt, sel)| {
            let tr = Transform::from(*glt);
            if sel.selected {
                gizmos.line(
                    tr.translation,
                    tr.translation + Vec3::Y * 5.0,
                    Color::srgba(0.1, 0.35, 0.45, 1.0),
                );
            }
        });
    }

    pub fn command_system(
        mut unit_commands: EventWriter<CommandEvent>,
        player: Res<LocalPlayer>,
        cast: Res<CameraRaycast>,
        current_placement: Res<CurrentPlacement<CLICK_BUFFER>>,
        input: Res<ButtonInput<MouseButton>>,

        units: Query<(Entity, &Selectable), With<PathFinder>>,
        team_players: Query<&TeamPlayer>,
        combat_world: Res<CombatWorld>,
    ) {
        if current_placement.placing() { return; }
        if input.just_released(MouseButton::Right) {
            if let Some(ray_cast) = cast.current_cast {
                if combat_world.is_enemy(ray_cast.entity, player.0, &team_players)
                    .map_or(false, |t| t) {
                    let command = CommandEvent{
                        player: player.0,
                        objects: units.iter().filter_map(|(id, sel)| if sel.selected { Some(id) } else { None }).collect(),
                        command: CommandType::Attack(ray_cast.entity),
                    };
                    unit_commands.write(command);
                } else {
                    unit_commands.write(CommandEvent {
                        player: player.0,
                        objects: units.iter().filter_map(|(id, sel)| if sel.selected { Some(id) } else { None }).collect(),
                        command: CommandType::Move(ray_cast.point.xz()),
                    });
                }
            }
        }
    }

    pub fn building_placement_system(
        mut command_events: EventWriter<CommandEvent>,
        mut current_placement: ResMut<CurrentPlacement::<CLICK_BUFFER>>,
        input: Res<ButtonInput<MouseButton>>,
        cast: Res<CameraRaycast>,
        team_players: Query<&TeamPlayer>,
        mut queueses: Query<&mut Queues>,
        mut trans: Query<&mut Transform>,
        mut visibles: Query<&mut Visibility>,
        mut commands: Commands,
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
                let ghost = commands.spawn((Transform::default(), Visibility::default())).id();

                if let Ok(teamplayer) = team_players.get(info.constructor) {
                    let command_event = CommandEvent {
                        player: *teamplayer,
                        objects: vec![ghost],
                        command: CommandType::Build(BuildStatus::Begin(info.data.object.clone())),
                    };
                    command_events.write(command_event);
                }
                current_placement.status = PlacementStatus::Placing((info, ghost).into());
            },
            PlacementStatus::Placing(info) => {
                if let (Ok(mut v), Ok(mut t)) = (visibles.get_mut(info.ghost), trans.get_mut(info.ghost)) {
                    if let Some(cc) = cast.current_cast {
                        *t = Transform::from_xyz(cc.point.x, cc.point.y, cc.point.z);
                        *v = Visibility::Visible;
                    } else {
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
                            objects: vec![info.ghost],
                            command: CommandType::Build(BuildStatus::Finish(*tran)),
                        };
                        command_events.write(command_event);
                    }
                    current_placement.status = PlacementStatus::Completed(info);
                }
            },
            // TODO: Not yet implemented
            PlacementStatus::Canceled(info) => {
                commands.entity(info.ghost).despawn();
                current_placement.status = PlacementStatus::Idle;
            }
            PlacementStatus::Completed(info) => {
                if let Ok(mut x) = queueses.get_mut(info.constructor) {
                    x.queues.get_mut(&info.queue).unwrap().remove_from_buffer(&info.data);
                }
                commands.entity(info.ghost).despawn();
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