pub use content::*;
///Contains all the the data structures that should be loaded from a file (or included in the binary)
mod content {
    use std::{path::Path, time::Duration};

    use bevy::math::Vec2;
    use bevy::prelude::{Bundle, Component};
    use bevy::reflect::Reflect;
    use bevy::{
        prelude::{Commands, Entity, Transform},
    };
    use bevy_rapier3d::prelude::Velocity;
    use log::error;
    use serde::{
        Serialize, Deserialize,
    };
    use qloader::*;
    use ronfile::*;
    use crate::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct UIInfo {
        pub display_name : String,
        pub description : String,
        pub thumbnail : String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Level {
        pub ui_info : UIInfo,
        pub save_state : SaveFile,
    }

    impl QLoad<()> for Level {
        const PATHTYPE : PathType = PathType::Absolute;
        fn extensions() -> Vec<&'static str> {
            vec!["ron"]
        }
        fn load<S : AsRef<Path>>(_path : S) -> Result<Self, QLoaderError> {
            match RonFile::load::<Self, S>(_path) {
                Ok(x) => {
                    Ok(x)
                },
                Err(e) => {
                    Err(QLoaderError::ParseError)
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    #[derive(Serialize, Deserialize)]
    pub struct MapBounds(pub Vec2);

    // impl QLoad<()> for MapBounds {
    //     const PATHTYPE : PathType = PathType::Absolute;
    //     fn extensions() -> Vec<&'static str> {
    //         vec!["ron"]
    //     }
    //     fn load<S : AsRef<Path>>(_path : S) -> Result<Self, QLoaderError> {
    //         match RonFile::load::<Self, S>(_path) {
    //             Ok(x) => {
    //                 Ok(x)
    //             },
    //             Err(e) => {
    //                 error!("{}", e);
    //                 Err(QLoaderError::ParseError)
    //             }
    //         }
    //     }
    // }

    #[derive(Debug, Clone)]
    #[derive(Serialize, Deserialize)]
    pub struct Constructor {
        pub buildings : Vec<String>,
        pub builder : bool,
    }

    #[derive(Debug, Clone)]
    #[derive(Serialize, Deserialize)]
    pub struct Trainer {
        pub trainies : Vec<String>,
        pub spawn_point : (f32, f32, f32),
        pub end_point : (f32, f32, f32),
    }

    #[derive(Debug, Clone, Copy)]
    #[derive(Serialize, Deserialize)]
    #[derive(Component)]
    pub struct EconomicObject {
        //Money
        pub resource_gen : f64,
        pub resource_drain : f64,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct PowerObject {
        pub power_gen : u32,
        pub power_drain : u32,
    }

    #[derive(Debug, Clone, Copy)]
    #[derive(Serialize, Deserialize)]
    #[derive(Component)]
    pub struct MobileObject {
        pub follow : bool,
        pub max_forward_speed : f32,
        pub max_backwards_speed : f32,
    }

    impl SerdeComponent for MobileObject {
        fn saved(&self) -> Option<Self> {
            if !self.follow {
                None
            } else {
                Some(*self)
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct QueueData {
        pub timer : f64,
        // pub spawn_point : Option<(f32, f32, f32)>,
        // pub end_point : Option<(f32, f32, f32)>,
        pub buffer : Vec<StackData>,
    }

    impl QueueData {
        pub fn new() -> Self {
            Self {
                timer: 0.0,
                buffer: Vec::new(),
            }
        }

        pub fn set_timer(&mut self, time : f64) {
            self.timer = time;
        }

        pub fn time(&self, timer : f64) -> f64 {
            if self.timer < timer {
                return self.timer;
            }
            timer
        }

        pub fn update(&mut self, delta : f64) -> bool {
            if self.timer > 0.0 {
                self.timer -= delta;
            }
            self.is_ready()
        }

        pub fn is_ready(&self) -> bool {
            self.timer <= 0.0
        }
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    #[derive(Serialize, Deserialize)]
    pub struct StackData {
        // pub id : String,
        pub object_type : ObjectType,
        pub time_to_build : Duration,
        pub cost : u128,
        pub buffered : bool,
    }

    // #[derive(Debug, Clone)]
    // pub struct InstantiationData {
    //     pub transform : Transform,
    //     pub spawn_point : Option<(f32, f32, f32)>,
    //     pub end_point : Option<(f32, f32, f32)>,
    //     pub team_player : TeamPlayer,
    //     ///Whether or not this was spawned in from the server.
    //     pub multiplayer : bool,
    //     pub had_identifier : bool,
    // }

    // //TODO: This should be removed from common and implemented on game and server separately.
    // pub trait Instantiate<D> {
    //     fn instantiate(&self, _world : Commands, _data : D) { }
    //     fn instantiate_with_entity(&self, _entity : Entity, _world : Commands, _data : D) { }
    // }

    // #[derive(Bundle)]
    // pub struct MapBundle {
    //     snowflake : SnowFlake,
    //     transform : Transform,
    //     selectable : Selectable,
    // }

    // #[derive(Bundle)]
    // pub struct BuildingBundle {
    //     snowflake : SnowFlake,
    //     transform : Transform,
    //     selectable : Selectable,
    //     save_data : SaveObject,
    //     team_player : TeamPlayer,
    //     // immobile : Immobile,
    // }

    // #[derive(Bundle)]
    // pub struct UnitBundle {
    //     snowflake : SnowFlake,
    //     transform : Transform,
    //     selectable : Selectable,
    //     save_data : SaveObject,
    //     team_player : TeamPlayer,
    //     velocity : Velocity,
    // }
}