use bevy::ecs::event::Event;
use serde::{Serialize, Deserialize};

pub mod lobby;
pub mod client;
pub mod server;

pub use lobby::*;
pub use client::*;
pub use server::*;

use crate::SpawnObjects;

pub const SERVER_ADDRESS: &'static str = "127.0.0.1:40256";
pub const CLIENT_ADDRESS: &'static str = "0.0.0.0:0";

///Sent from the client to the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientRequests {
    pub requests: Vec<ClientRequest>,
}

impl From<ClientRequest> for ClientRequests {
    fn from(value: ClientRequest) -> Self {
        Self {
            requests: vec![value],
        }
    }
}

impl From<Vec<ClientRequest>> for ClientRequests {
    fn from(value: Vec<ClientRequest>) -> Self {
        Self {
            requests: value
        }
    }
}

#[derive(Debug, Default, Clone, Event, Serialize, Deserialize)]
pub enum ClientRequest {
    #[default]
    Empty,
    SpawnObject(SpawnObjects),
}

///Sent from the server to the client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCommands {
    pub commands: Vec<ServerCommand>,
}

impl From<ServerCommand> for ServerCommands {
    fn from(value: ServerCommand) -> Self {
        Self {
            commands: vec![value],
        }
    }
}

impl From<Vec<ServerCommand>> for ServerCommands {
    fn from(value: Vec<ServerCommand>) -> Self {
        Self {
            commands: value
        }
    }
}

///Sent between the server and the client
#[derive(Debug, Default, Clone, Event, Serialize, Deserialize)]
pub enum ServerCommand {
    #[default]
    Empty,
    SpawnObject(SpawnObjects),
}

// impl ServerCommand {
//     pub fn new() -> Self {
//         Self::SpawnObject
//     }
// }

pub enum NetMode {
    ///Singleplayer, not connected to any server
    Single,
    ///Connected to server as a client
    Client,
    ///Is the server
    Server,
    ///Server + Client
    Host,
}

impl NetMode {
    pub fn is_singleplayer(&self) -> bool {
        match self {
            NetMode::Single => true,
            NetMode::Client => false,
            NetMode::Server => false,
            NetMode::Host => false,
        }
    }

    pub fn is_multiplayer(&self) -> bool {
        !self.is_singleplayer()
    }
}
