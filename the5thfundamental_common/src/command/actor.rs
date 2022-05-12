pub use actor::*;
pub mod actor {

    use std::collections::HashMap;
    use serde::{Serialize, Deserialize};
    use snowflake::ProcessUniqueId;


    use crate::*;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum AIDifficulty {
        Easy, Normal, Hard, Brutal, WurtziteBoronNitride, Custom
    }

    #[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
    pub struct AISettings {
        dynamic : Option<(f32, f32)>,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum ActorType {
        AI{difficulty : AIDifficulty, settings : AISettings},
        Player,
    }

    #[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
    pub struct Rating {
        pub economy_score : f64,
        pub production_score : f64,
        pub power_score : f64,
    }

    impl Rating {
        pub fn reset(&mut self) {
            self.economy_score = 0.0;
            self.production_score = 0.0;
            self.power_score = 0.0;
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Actor {
        pub actor_type : ActorType,
        pub rating : Rating,

        pub economy : Economy,

        #[serde(skip)]
        pub buildings : Vec<SnowFlake>,
        #[serde(skip)]
        pub units : Vec<SnowFlake>,
    }

    impl Actor {
        pub fn new(id : TeamPlayer, a_type : ActorType) -> Self {
            Self {
                actor_type : a_type,
                rating : Rating::default(),
                economy : Economy::default(),
                buildings : Vec::new(),
                units : Vec::new(),
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Actors {
        pub actors : HashMap<TeamPlayer, Actor>,
    }

    impl Actors {
        // pub fn new(teams : &[u8]) -> Self {

        // }
        pub fn assign_building(&mut self, actor : TeamPlayer, queues : SnowFlake) {
            if let Some(x) = self.actors.get_mut(&actor) {
                x.buildings.push(queues);
            }
        }

        pub fn reset_ratings(&mut self) {
            for a in self.actors.values_mut() {
                a.rating.reset();
            }
        }
    }
}