

pub mod commander;
pub mod navigation;
pub mod pathfinder;
pub mod command_plugin;
pub mod reference;
pub mod select;
pub mod snowflake;
pub mod squad;
pub mod teamplayer;

pub use commander::*;
pub use navigation::*;
pub use pathfinder::*;
pub use command_plugin::*;
pub use reference::*;
pub use select::*;
pub use snowflake::*;
pub use squad::*;
pub use teamplayer::*;

pub use pathing::*;

use bevy::{ecs::{event::Event, entity::Entity}, math::Vec2, utils::HashSet, transform::components::Transform};

#[derive(Debug, Clone)]
#[derive(Event)]
pub struct CommandEvent {
    pub player: TeamPlayer,
    pub object: Option<CommandObject>,
    pub command: CommandType,
}

impl CommandEvent {
    pub fn activate_command(&self) -> Option<Vec<Entity>> {
        if self.command.is_activate() {
            self.object.as_ref().and_then(|object| Some(object.entities().clone()))
        } else {
            None
        }
    }

    pub fn attack_command(&self) -> Option<Vec<Entity>> {
        if self.command.is_attack() {
            self.object.as_ref().and_then(|object| Some(object.entities().clone()))
        } else {
            None
        }
    }

    pub fn build_command(&self) -> Option<Vec<Entity>> {
        if self.command.is_build() {
            self.object.as_ref().and_then(|object| Some(object.entities().clone()))
        } else {
            None
        }
    }

    pub fn move_command(&self) -> Option<Vec<Entity>> {
        if self.command.is_move() {
            self.object.as_ref().and_then(|object| Some(object.entities().clone()))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandObject {
    Structure(Entity),
    Units(HashSet<Entity>)
}

impl CommandObject {
    pub fn structure(&self) -> Option<Entity> {
        match self {
            Self::Structure(entity) => { Some(*entity) }
            _ => { None }
        }
    }

    pub fn units(&self) -> Option<Vec<Entity>> {
        match self {
            Self::Units(entities) => { Some(entities.iter().cloned().collect()) }
            _ => { None }
        }
    }

    pub fn entities(&self) -> Vec<Entity> {
        match self {
            Self::Structure(entity) => { vec![*entity] }
            Self::Units(entities) => { entities.iter().cloned().collect() }
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandType {
    Activate,
    Attack(Entity),
    Build(BuildStatus),
    Move(Vec2),
}

impl CommandType {
    pub fn is_activate(&self) -> bool {
        match self {
            Self::Activate => true,
            _ => false,
        }
    }

    pub fn is_attack(&self) -> bool {
        match self {
            Self::Attack(_) => true,
            _ => false,
        }
    }

    pub fn is_build(&self) -> bool {
        match self {
            Self::Build(_) => true,
            _ => false,
        }
    }

    pub fn is_move(&self) -> bool {
        match self {
            Self::Move(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BuildStatus {
    Begin(String),
    Finish(Transform),
}