pub use commands::*;
mod commands {
    use bevy::prelude::*;

    use crate::Snowflake;


    #[derive(Debug, Clone)]
    pub struct MoveCommand{
        pub position : Vec2,
        pub units : Vec<Snowflake>,
    }
    #[derive(Debug, Clone)]
    pub struct AttackCommand{
        pub target : Snowflake,
        pub units : Vec<Snowflake>,
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