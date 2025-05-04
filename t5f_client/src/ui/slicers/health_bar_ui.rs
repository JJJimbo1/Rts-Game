use bevy::sprite::{BorderRect, SliceScaleMode, TextureSlicer};

use super::SCALE;

pub const HEALTH_BAR_LEFT_MARGIN: f32 = 15.0;
pub const HEALTH_BAR_RIGHT_MARGIN: f32 = 15.0;
pub const HEALTH_BAR_TOP_MARGIN: f32 = 4.0;
pub const HEALTH_BAR_BOTTOM_MARGIN: f32 = 4.0;

pub struct HealthBarUI;

impl HealthBarUI {
    pub fn vertical_offset() -> f32 {
        HEALTH_BAR_TOP_MARGIN - 1.0
    }

    pub fn slicer() -> TextureSlicer {
        TextureSlicer {
            border: BorderRect {
                left: HEALTH_BAR_LEFT_MARGIN,
                right: HEALTH_BAR_RIGHT_MARGIN,
                top: HEALTH_BAR_TOP_MARGIN,
                bottom: HEALTH_BAR_BOTTOM_MARGIN,
            },
            center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: SCALE * 1.0
        }
    }
}