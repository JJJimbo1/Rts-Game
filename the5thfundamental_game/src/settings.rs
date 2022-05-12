pub use settings::*;
mod settings {
    use serde::{Serialize, Deserialize};

    #[derive(Deserialize)]
    pub struct CameraSettings {
        pub offset : (f32, f32),

        //Settings relating to Rotation.
        pub max_rotation_speed : f32,
        pub rotation_acceleration : f32,
        pub rotation_deceleration : f32,
        pub slow_rotation_multiplier : f32,
        pub rotation_acceleration_curve : f32,

        //Setting relating to Scrolling.
        pub thresholds : (f32, f32),
        pub max_scroll_speed : f32,
        pub scroll_acceleration : f32,
        pub scroll_deceleration : f32,
        pub fast_decceleration_threshold : f32,
        pub fast_decceleration_strength : f32,
        pub slow_scroll_multiplier : f32,
        pub scroll_acceleration_curve : f32,
        pub use_scroll_button : bool,
        pub scroll_button_speed_multiplier : f32,
        pub post_action_stall : bool,

        //Settings relating to Zooming.
        pub max_zoom_speed : f32,
        pub zoom_acceleration : f32,
        pub zoom_deceleration : f32,
        pub slow_zoom_multiplier : f32,
        pub zoom_acceleration_curve : f32,
        pub min_zoom : f32,
        pub max_zoom : f32,
        pub default_zoom : f32,
        pub zoom_base : f32,
        pub zoom_ratio : f32,
        pub zoom_curve_weight : f32,

        //Settings relating to Rotation, Scroll and Zoom.
        pub minimum_fps_for_deltatime : u16,
    }

    impl Default for CameraSettings {
        fn default() -> Self {
            Self {
                offset : (3., 5.),

                max_rotation_speed : 0.8,
                rotation_acceleration : 3.,
                rotation_deceleration : 10.,
                slow_rotation_multiplier : 0.125,
                rotation_acceleration_curve : 2.,

                thresholds : (0.8, 0.7),
                max_scroll_speed : 30.,
                scroll_acceleration : 1.2,
                scroll_deceleration : 15.,
                fast_decceleration_threshold : 100.,
                fast_decceleration_strength : 0.5,
                slow_scroll_multiplier : 0.125,
                scroll_acceleration_curve : 2.,
                use_scroll_button : true,
                scroll_button_speed_multiplier : 0.5,
                post_action_stall : true,

                max_zoom_speed : 15.,
                zoom_acceleration : 8.,
                zoom_deceleration : 10.,
                slow_zoom_multiplier : 0.125,
                zoom_acceleration_curve : 2.,
                min_zoom : 20.,
                max_zoom : 120.,
                default_zoom : 0.8,
                zoom_base : 10.,
                zoom_ratio : 2.,
                zoom_curve_weight : 1.,

                minimum_fps_for_deltatime : 20,
            }
        }
    }

    pub struct MenuSettings {
        pub font_size : f32,
    }

    impl Default for MenuSettings {
        fn default() -> Self {
            MenuSettings {
                font_size : 1.0,
            }
        }
    }
}