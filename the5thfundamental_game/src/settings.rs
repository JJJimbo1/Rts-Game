pub use settings::*;
mod settings {
    use bevy::{math::Vec3, prelude::Resource};
    use mathfu::D1;
    use serde::{Serialize, Deserialize};

    #[derive(Deserialize)]
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
            (Vec3::new(0.0, D1::normalize_from_01(self.default_zoom, self.min_zoom().y, self.max_zoom().y),
            D1::normalize_from_01(self.default_zoom, self.min_zoom().z, self.max_zoom().z)).normalize_or_zero(),
            D1::normalize_from_01(self.default_zoom, self.min_zoom, self.max_zoom))
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
                offset_y: (2.0, 4.0),
                offset_z: (3.0, 4.0),
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

                max_zoom_speed: 1.2,
                zoom_acceleration: 9.0,
                zoom_deceleration: 15.0,
                slow_zoom_multiplier: 0.125,
                zoom_acceleration_curve: 2.0,
                min_zoom: 20.,
                max_zoom: 30.,
                default_zoom: 0.8,
                zoom_base: 10.0,
                zoom_ratio: 5.0,
                zoom_curve_weight: 0.25,

                minimum_fps_for_deltatime: 20,
            }
        }
    }

    #[derive(Resource)]
    pub struct MenuSettings {
        pub font_size: f32,
    }

    impl Default for MenuSettings {
        fn default() -> Self {
            MenuSettings {
                font_size: 1.0,
            }
        }
    }
}