pub use error_state::*;
mod error_state {
    use std::{fmt::{self, write}, path::Path};

    use amethyst_gltf::{GltfSceneAsset, GltfSceneFormat};
    use obj::Obj;

    use zipqueue::ZipQueue;
    use the5thfundamental_common::{Collider, Immobile, Level, Map, MasterQueue, ObjectData, PositionConstraint, QueueData, Queues, Snowflake, StackData, TeamPlayer, Torque, Velocity};

    use crate::*;

    pub struct ErrorState{
        pub error : StateError,
    }

    impl<'a, 'b> State<GameData<'a, 'b>, StateEvent<Bindings>> for ErrorState {
        // fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        
        // }

        fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
            let StateData {world, .. } = data;
            world.delete_all();
        }

        fn update(&mut self, _data: StateData<'_, GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>, StateEvent<Bindings>> {
            log::error!("{}", self.error);
            Trans::None
        }
    }

    #[derive(Debug)]
    pub enum StateError {
        None,
        LoadError,
        GameError,
    }

    impl StateError {
        pub fn is_not_none(&self) -> bool {
            if let Self::None = self {
                return false;
            }
            true
        }
    }

    impl fmt::Display for StateError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::None => {
                    write!(f, "No error occured.")
                }
                Self::LoadError => {
                    write!(f, "Error occured during load state. More info will be added intermittetly.")
                },
                Self::GameError => {
                    write!(f, "Error occured during game state. More info will be added intermittetly.")
                }
            }
        }
    }
}