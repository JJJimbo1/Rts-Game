use bevy::prelude::*;

use super::SCALE;

pub const SELECT_BOX_LEFT_MARGIN: f32 = 2.0;
pub const SELECT_BOX_RIGHT_MARGIN: f32 = 2.0;
pub const SELECT_BOX_TOP_MARGIN: f32 = 2.0;
pub const SELECT_BOX_BOTTOM_MARGIN: f32 = 2.0;

#[derive(Debug, Clone, Copy)]
pub struct SelectBoxUI;

impl SelectBoxUI {
    pub fn slicer() -> TextureSlicer {
        TextureSlicer {
            border: BorderRect {
                left: SELECT_BOX_LEFT_MARGIN,
                right: SELECT_BOX_RIGHT_MARGIN,
                top: SELECT_BOX_TOP_MARGIN,
                bottom: SELECT_BOX_BOTTOM_MARGIN,
            },
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: SCALE
        }
    }
}