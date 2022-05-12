pub use content::*;
///Contains all the the data structures that should be loaded from a file (or included in the binary)
mod content {
    use std::{path::Path, time::Duration};

    use bevy::prelude::{Bundle, Component};
    use bevy::{
        prelude::{Commands, Entity, Transform},
    };
    use log::error;
    use serde::{
        Serialize, Deserialize,
    };
    use snowflake::ProcessUniqueId;
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
                    println!("{}", e);
                    Err(QLoaderError::ParseError)
                }
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct Map {
        pub ui_info : UIInfo,
        pub base : String,
        pub bounds : (f32, f32),
        pub box_collider : (f32, f32, f32),
    }

    impl QLoad<()> for Map {
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
                    error!("{}", e);
                    Err(QLoaderError::ParseError)
                }
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
    pub enum ObjectType {
        Building,
        Unit,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct QueueObject {
        pub queues : Vec<String>,
        pub cost : u128,
        pub time_to_build : Duration,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct ConstructorObject {
        pub builder : bool,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct TrainerObject {
        pub spawn_point : (f32, f32, f32),
        pub end_point : (f32, f32, f32),
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[derive(Component)]
    pub struct MobileObject {
        pub follow : bool,
        pub max_forward_speed : f32,
        pub max_backwards_speed : f32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GameObject {
        pub id : String,
        pub object_type : ObjectType,
        pub ui_info : Option<UIInfo>,
        pub queues : Option<QueueObject>,
        pub constructor : Option<ConstructorObject>,
        pub trainer : Option<TrainerObject>,
        pub economy : Option<EconomicObject>,
        pub power : Option<PowerObject>,
        pub mobility : Option<MobileObject>,
        pub health : Option<Health>,
        pub weapons : Option<WeaponSet>,
    }

    impl QLoad<()> for GameObject {
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
                    error!("{}", e);
                    Err(QLoaderError::ParseError)
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct BuildingUIData {
        pub stack_id : StackData,
        pub display_name : String,
        pub cost : u128,
        pub time_to_build : Duration,
        pub power_drain : u32,
    }

    impl TryFrom<GameObject> for BuildingUIData {
        type Error = String;
        fn try_from(value: GameObject) -> Result<Self, Self::Error> {
            match (value.ui_info.clone(), value.queues.clone(), value.power, StackData::try_from(value)) {
                (Some(ui), Some(qo), Some(po), Ok(sd)) => {
                    Ok(Self {
                        stack_id : sd,
                        display_name : ui.display_name,
                        cost : qo.cost,
                        time_to_build : qo.time_to_build,
                        power_drain : po.power_drain,
                    })
                },
                _ => { Err("Nope".to_string()) }
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct UnitUIData {
        pub stack_id : StackData,
        pub display_name : String,
        pub cost : u128,
        pub time_to_train : Duration,
    }

    impl TryFrom<GameObject> for UnitUIData {
        type Error = String;
        fn try_from(value: GameObject) -> Result<Self, Self::Error> {
            match (value.ui_info.clone(), value.queues.clone(), StackData::try_from(value)) {
                (Some(ui), Some(qo), Ok(sd)) => {
                    Ok(Self {
                        stack_id : sd,
                        display_name : ui.display_name,
                        cost : qo.cost,
                        time_to_train : qo.time_to_build,
                    })
                },
                _ => { Err("Nope".to_string()) }
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct QueueData {
        pub timer : f64,
        pub spawn_point : Option<(f32, f32, f32)>,
        pub end_point : Option<(f32, f32, f32)>,
        pub buffer : Vec<StackData>,
    }

    impl QueueData {
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

    #[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
    pub struct StackData {
        pub id : String,
        pub object_type : ObjectType,
        pub time_to_build : Duration,
        pub cost : u128,
    }

    impl TryFrom<GameObject> for StackData {
        type Error = String;
        fn try_from(value: GameObject) -> Result<Self, Self::Error> {
            match value.queues {
                Some(qo) => {
                    Ok(Self {
                        id : value.id,
                        object_type : value.object_type,
                        time_to_build : qo.time_to_build,
                        cost : qo.cost,
                    })
                },
                None => { Err("No queues".to_string()) }
            }
        }
    }


    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum AccessGroup {
        Restricted(Vec<String>),
        Unrestricted,
        Modded,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Group {
        pub id : String,
        pub access : AccessGroup,
    }

    use crate::{
        ResourceProvider,
    };

    #[derive(Debug, Clone)]
    pub struct BuildingPrefab(pub GameObject);

    #[derive(Debug, Clone)]
    pub struct UnitPrefab(pub GameObject);

    #[derive(Debug, Clone)]
    pub struct InstantiationData {
        pub transform : Transform,
        pub spawn_point : Option<(f32, f32, f32)>,
        pub end_point : Option<(f32, f32, f32)>,
        pub team_player : TeamPlayer,
        ///Whether or not this was spawned in from the server.
        pub multiplayer : bool,
        pub had_identifier : bool,
    }

    //TODO: This should be removed from common and implemented on game and server separately.
    pub trait Instantiate<D> {
        fn instantiate(&self, _world : Commands, _data : D) { }
        fn instantiate_with_entity(&self, _entity : Entity, _world : Commands, _data : D) { }
    }

    #[derive(Bundle)]
    pub struct MapBundle {
        snowflake : SnowFlake,
        transform : Transform,
        selectable : Selectable,
    }

    #[derive(Bundle)]
    pub struct BuildingBundle {
        snowflake : SnowFlake,
        transform : Transform,
        selectable : Selectable,
        save_data : SaveObject,
        team_player : TeamPlayer,
        immobile : Immobile,
    }

    #[derive(Bundle)]
    pub struct UnitBundle {
        snowflake : SnowFlake,
        transform : Transform,
        selectable : Selectable,
        save_data : SaveObject,
        team_player : TeamPlayer,
        velocity : Velocity,
    }
}