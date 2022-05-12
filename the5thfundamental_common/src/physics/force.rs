pub use force::*;
mod force {
    use bevy::prelude::*;
    use serde::{
        Serialize, Deserialize
    };

    #[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
    #[derive(Component)]
    pub struct Velocity {
        pub x : f32,
        pub y : f32,
        pub z : f32,
        pub local : bool,
    }

    impl Velocity {
        pub fn new(x : f32, y : f32, z : f32, local : bool) -> Velocity {
            Velocity {
                x,
                y,
                z,
                local,
            }
        }

        pub fn is_zero(&self) -> bool {
            self.x.abs() < 0.0001 && self.y.abs() < 0.0001 && self.z.abs() < 0.0001
        }
    }

    #[derive(Component)]
    pub struct LocalBounds {
        pub x : Vec2,
        pub y : Vec2,
        pub z : Vec2,
    }

    #[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
    #[derive(Component)]
    pub struct Immobile;


    // #[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
    // #[derive(Component)]
    // pub struct SpeedLimit {
    //     pub positive : f32,
    //     pub negative : f32,
    // }

    #[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
    #[derive(Component)]
    pub struct Torque {
        pub x : f32,
        pub y : f32,
        pub z : f32,
    }

    #[allow(dead_code)]
    impl Torque {
        pub fn new(x : f32, y : f32, z : f32) -> Torque {
            Torque {
                x,
                y,
                z,
            }
        }
    }
}