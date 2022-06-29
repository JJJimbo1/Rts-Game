use bevy::{math::{Vec2, Vec3, Mat4}, prelude::{GlobalTransform, Entity}, render::camera::Camera, window::Windows};

// use crate::Snowflake;

#[derive(Debug, Copy, Clone)]
pub struct RayCastResult {
    pub entity : Entity,
    pub point : Vec3,
    pub len : f32,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct CameraRaycast {
    pub last_valid_cast : Option<RayCastResult>,
    pub current_cast : Option<RayCastResult>,
}

pub fn ray(cursor_pos_screen: Vec2,
    windows: &Windows,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> (Vec3, Vec3) {
    let camera_position = camera_transform.compute_matrix();
    let window = windows.get_primary().unwrap();
    let screen_size = Vec2::from([window.width() as f32, window.height() as f32]);
    let projection_matrix = camera.projection_matrix;

    let cursor_ndc = (cursor_pos_screen / screen_size) * 2.0 - Vec2::from([1.0, 1.0]);
    let cursor_pos_ndc_near: Vec3 = cursor_ndc.extend(-1.0);
    let cursor_pos_ndc_far: Vec3 = cursor_ndc.extend(1.0);

    let ndc_to_world: Mat4 = camera_position * projection_matrix.inverse();
    let cursor_pos_near: Vec3 = ndc_to_world.project_point3(cursor_pos_ndc_near);
    let cursor_pos_far: Vec3 = ndc_to_world.project_point3(cursor_pos_ndc_far);
    let ray_direction = cursor_pos_far - cursor_pos_near;
    (cursor_pos_near, ray_direction)
}