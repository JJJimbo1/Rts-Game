use bevy::prelude::*;

use super::SCALE;

pub const WHITE_BOX_LEFT_MARGIN: f32 = 2.0;
pub const WHITE_BOX_RIGHT_MARGIN: f32 = 2.0;
pub const WHITE_BOX_TOP_MARGIN: f32 = 2.0;
pub const WHITE_BOX_BOTTOM_MARGIN: f32 = 2.0;

#[derive(Debug, Clone, Copy)]
pub struct WhiteBoxUI;

impl WhiteBoxUI {
    pub fn slicer() -> TextureSlicer {
        TextureSlicer {
            border: BorderRect {
                left: WHITE_BOX_LEFT_MARGIN,
                right: WHITE_BOX_RIGHT_MARGIN,
                top: WHITE_BOX_TOP_MARGIN,
                bottom: WHITE_BOX_BOTTOM_MARGIN,
            },
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: SCALE
        }
    }
}