pub use commands::*;
mod commands {
    use serde::{Serialize, Deserialize};
    use snowflake::ProcessUniqueId;
    use crate::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ServerCommand {
        None,
        //Message([char; 24]),
        CreateBuilding(ProcessUniqueId, [char; 24]),
        CreateUnit(ProcessUniqueId, [char; 24]),
        Validate(ProcessUniqueId, ProcessUniqueId),
        Move(f32, f32)
    }

    impl ServerCommand {
        ///The Message Must not exceed 24 characters.
        pub fn encode(message : &str) -> [char; 24] {
            let mut m : [char; 24] = ['\0'; 24];
            //message.char_indices(). (|x| x.0 < 24).collect::<[char; 24]>();
            for i in message.char_indices() {
                if i.0 >= 24 {
                    break;
                }
                m[i.0] = i.1;
            }
            m
        }

        pub fn decode(message : [char; 24]) -> String {
            let mut s = String::new();
            for i in 0..24 {
                if message[i] == '\0' {
                    continue;
                }
                s.push(message[i]);
            }
            s
        }
    }

    impl Default for ServerCommand {
        fn default() -> Self {
            Self::None
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ServerRequest {
        None,
        JoinLobby,
        //Message([char; 24]),
        CreateBuilding(ProcessUniqueId, [char; 24]),
        CreateUnit(ProcessUniqueId, [char; 24]),
        Move(f32, f32)
    }

    impl Default for ServerRequest {
        fn default() -> Self {
            Self::None
        }
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct ServerCommands {
        pub commands : Vec<ServerCommand>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct ServerRequests {
        pub commands : Vec<ServerRequest>,
    }
}