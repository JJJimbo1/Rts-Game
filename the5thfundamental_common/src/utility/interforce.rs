use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct InterForce {
    pub force : Vec3,
    pub max_speed : f32,
    pub acceleration : f32,
}

impl InterForce {
    pub fn apply_force(&mut self, direction : Vec3, acceleration_scale : f32) {
        let target_xz = direction * self.max_speed;
        let difference = target_xz - self.force;
        self.force += difference.clamp_length_max((self.acceleration * acceleration_scale).min(difference.length()));
        // println!("{}", self.force);
    }

    pub fn set_force(&mut self, force : Vec3) {
        self.force = force;
    }
}
