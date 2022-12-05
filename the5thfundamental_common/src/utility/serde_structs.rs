use serde::{Serialize, Deserialize};
use bevy::{prelude::Transform, math::{Vec3, Quat}};
use bevy_rapier3d::prelude::Velocity;
use approx::*;

use crate::{GroundPathFinder, pathing::Path};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SerdeTransform {
    translation: (f32, f32, f32),
    rotation: (f32, f32, f32, f32),
    scale: (f32, f32, f32),
}

impl From<Transform> for SerdeTransform {
    fn from(tran: Transform) -> Self {
        let position = (tran.translation.x, tran.translation.y, tran.translation.z);
        let rotation = {
            let angle = tran.rotation.to_axis_angle();
            (angle.0.x, angle.0.y, angle.0.z, angle.1)
        };
        let scale = (tran.scale.x, tran.scale.y, tran.scale.z);
        Self {
            translation: position,
            rotation,
            scale,
        }
    }
}

impl From<SerdeTransform> for Transform {
    fn from(stran: SerdeTransform) -> Self {
        let translation = Vec3::new(stran.translation.0, stran.translation.1, stran.translation.2);
        let rotation = Quat::from_axis_angle(Vec3::new(stran.rotation.0, stran.rotation.1, stran.rotation.2), stran.rotation.3);
        let scale = Vec3::new(stran.scale.0, stran.scale.1, stran.scale.2);
        Transform::default()
            .with_translation(translation)
            .with_rotation(rotation)
            .with_scale(scale)
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct SerdeVelocity{
    lin: (f32, f32, f32),
    ang: (f32, f32, f32),
}

impl SerdeComponent for SerdeVelocity {
    fn saved(&self) -> Option<Self> {
        if abs_diff_eq!(self.lin.0, 0.0, epsilon = f32::EPSILON)
        && abs_diff_eq!(self.lin.1, 0.0, epsilon = f32::EPSILON)
        && abs_diff_eq!(self.lin.2, 0.0, epsilon = f32::EPSILON)
        && abs_diff_eq!(self.ang.0, 0.0, epsilon = f32::EPSILON)
        && abs_diff_eq!(self.ang.1, 0.0, epsilon = f32::EPSILON)
        && abs_diff_eq!(self.ang.2, 0.0, epsilon = f32::EPSILON) {
            None
        } else {
            Some(*self)
        }
    }
}

impl From<Velocity> for SerdeVelocity {
    fn from(vel: Velocity) -> Self {
        Self {
            lin: (vel.linvel.x, vel.linvel.y, vel.linvel.z),
            ang: (vel.angvel.x, vel.angvel.y, vel.angvel.z),
        }
    }
}

impl From<SerdeVelocity> for Velocity {
    fn from(vel: SerdeVelocity) -> Self {
        Self {
            linvel: Vec3::new(vel.lin.0, vel.lin.1, vel.lin.2),
            angvel: Vec3::new(vel.ang.0, vel.ang.1, vel.ang.2),
        }
    }
}

pub trait SerdeComponent: Sized {
    fn saved(&self) -> Option<Self>;
}



impl SerdeComponent for GroundPathFinder {
    fn saved(&self) -> Option<Self> {
        if abs_diff_eq!(self.start.x, 0.0, epsilon = f32::EPSILON)
        && abs_diff_eq!(self.start.y, 0.0, epsilon = f32::EPSILON)
        && abs_diff_eq!(self.end.x, 0.0, epsilon = f32::EPSILON)
        && abs_diff_eq!(self.end.y, 0.0, epsilon = f32::EPSILON) {
            None
        } else {
            Some(self.clone())
        }
    }
}

impl SerdeComponent for Path {
    fn saved(&self) -> Option<Self> {
        if self.0.len() == 0 {
            None
        } else {
            Some(self.clone())
        }
    }
}
