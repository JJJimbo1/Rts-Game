
pub mod server_plugin;

pub use server_plugin::*;




use bevy::prelude::*;
use t5f_common::{TeamPlayer, Snowflake};



#[derive(Debug, Clone,)]
pub struct ServerRequestEvent(ServerRequestEventData);

#[derive(Debug, Clone,)]
pub struct ServerRequestEventData {
    sender: TeamPlayer,
    objects: Option<Vec<Snowflake>>,
    request: ServerRequest,
}

#[derive(Debug, Clone,)]
pub enum ServerRequest {
    Activate,
    Move(Vec2),
    SpawnObject(String, Transform),
}