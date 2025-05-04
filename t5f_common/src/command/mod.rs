

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

use bevy::{ecs::{event::Event, entity::Entity}, math::Vec2, transform::components::Transform};

use crate::ObjectType;

#[derive(Debug, Clone)]
#[derive(Event)]
pub struct CommandEvent {
    pub player: TeamPlayer,
    pub objects: Vec<Entity>,
    pub command: CommandType,
}

impl CommandEvent {
    pub fn activate(&self) -> Option<&Vec<Entity>> {
        if self.command.is_activate() {
            Some(&self.objects)
        } else {
            None
        }
    }

    pub fn attack(&self) -> Option<&Vec<Entity>> {
        if self.command.is_attack() {
            Some(&self.objects)
        } else {
            None
        }
    }

    pub fn build(&self) -> Option<&Vec<Entity>> {
        if self.command.is_build() {
            Some(&self.objects)
        } else {
            None
        }
    }

    pub fn r#move(&self) -> Option<&Vec<Entity>> {
        if self.command.is_move() {
            Some(&self.objects)
        } else {
            None
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
    Begin(ObjectType),
    Finish(Transform),
}