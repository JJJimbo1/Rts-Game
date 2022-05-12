pub use commands::*;
mod commands {
    use bevy::prelude::*;
    use serde::{Serialize, Deserialize};

    use crate::SnowFlake;


    #[derive(Debug, Clone)]
    pub struct MoveCommand{
        pub position : Vec2,
        pub units : Vec<SnowFlake>,
    }
    #[derive(Debug, Clone)]
    pub struct AttackCommand{
        pub target : SnowFlake,
        pub units : Vec<SnowFlake>,
    }
    // #[derive(Debug, Copy, Clone)]
    // pub enum ActorCommand {
    //     None,
    //     Move(Vec2),
    //     Attack(ProcessUniqueId),
    // }

    // impl Default for ActorCommand {
    //     fn default() -> Self {
    //         Self::None
    //     }
    // }
}