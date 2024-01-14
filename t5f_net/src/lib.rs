pub mod client;
pub mod server;

pub use client::*;
pub use server::*;

pub const SERVER_ADDRESS: &'static str = "127.0.0.1:40443";

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
