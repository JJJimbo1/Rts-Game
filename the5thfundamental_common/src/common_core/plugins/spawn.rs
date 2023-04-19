use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(SystemLabel)]
pub struct SpawnObjectSystem;

#[derive(Debug, Clone)]
pub struct ObjectSpawnEventData {
    pub object_type: ObjectType,
    pub spawn_data: SpawnData,
    pub serde_data: Option<SerdeData>,
}

impl From<ObjectSpawnEventData> for (Snowflake, TeamPlayer, Transform) {
    fn from(value: ObjectSpawnEventData) -> Self {
        (value.spawn_data.snowflake, value.spawn_data.teamplayer, value.spawn_data.transform)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpawnData {
    pub snowflake: Snowflake,
    pub teamplayer: TeamPlayer,
    pub transform: Transform,
}

#[derive(Debug, Default, Clone)]
pub struct SerdeData {
    pub health: Option<Health>,
    pub queues: Option<Queues>,
    pub path_finder: Option<GroundPathFinder>,
    pub path: Option<Path>,
    pub controller: Option<Controller>,
    pub weapon_set: Option<WeaponSet>,
    pub turret: Option<Turret>,
    pub squad: Option<Squad>,
    pub velocity: Option<Velocity>,
    pub resource_node: Option<ResourceNodePlatforms>
}