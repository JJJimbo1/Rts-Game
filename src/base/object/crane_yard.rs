use std::marker::PhantomData;

use bevy::{prelude::*, platform::collections::HashMap};
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use superstruct::*;
use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct CraneYard;

impl From<CraneYard> for ObjectType {
    fn from(_: CraneYard) -> Self {
        ObjectType::CraneYard
    }
}

#[superstruct{
    no_enum,
    variants(Bundle, Prefab, Ghost, Disk),
    variant_attributes(derive(Debug, Clone)),
    specific_variant_attributes(
        Bundle(derive(Bundle)),
        Ghost(derive(Bundle)),
        Disk(derive(Serialize, Deserialize)),
    ),
}]
#[derive(Debug, Clone)]
pub struct CraneYard {
    #[superstruct(only(Prefab, Bundle))]        pub health: Health,
    #[superstruct(only(Prefab, Bundle))]        pub queues: Queues,
    #[superstruct(only(Prefab, Bundle))]        pub collider: Collider,
    #[superstruct(only(Bundle))]                pub crane_yard: CraneYard,
    #[superstruct(only(Bundle, Ghost))]         pub object_type: ObjectType,
    #[superstruct(only(Bundle))]                pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]                pub selectable: Selectable,
    #[superstruct(only(Bundle, Ghost))]         pub visibility: Visibility,
    #[superstruct(only(Bundle, Disk))]          pub team_player: TeamPlayer,
    #[superstruct(only(Bundle, Ghost, Disk))]   pub transform: Transform,
    #[superstruct(only(Disk))]                  pub disk_snowflake: Option<Snowflake>,
    #[superstruct(only(Disk))]                  pub disk_health: Option<Health>,
    #[superstruct(only(Disk))]                  pub disk_queues: Option<Queues>,
}

impl CraneYardBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnData) -> Self {
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }

    pub fn with_disk_data(mut self, disk_data: Option<ObjectDiskData>) -> Self {
        let Some(disk_data) = disk_data else { return self; };
        if let Some(health) = disk_data.health { self.health = health; }
        if let Some(queues) = disk_data.queues { self.queues = queues; }
        self
    }
}


impl TryFrom<(&ObjectAsset, &HashMap<ObjectType, (ActiveQueue, StackData)>)> for CraneYardPrefab {
    type Error = ContentError;
    fn try_from((asset, stacks): (&ObjectAsset, &HashMap<ObjectType, (ActiveQueue, StackData)>)) -> Result<Self, Self::Error> {
        let Some(health) = asset.health else { return Err(ContentError::MissingHealth); };
        let Some(asset_queues) = asset.asset_queues.clone() else { return Err(ContentError::MissingQueues); };
        let Some(collider_string) = asset.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let queues = Queues::from((&asset_queues, stacks));
        let Ok(collider) = Collider::trimesh(vertices, indices) else { return Err(ContentError::ColliderDecodeError); };

        Ok(Self {
            health,
            queues,
            collider,
        })
    }
}

impl From<CraneYardPrefab> for CraneYardBundle {
    fn from(prefab: CraneYardPrefab) -> Self {
        Self {
            crane_yard: CraneYard,
            object_type: CraneYard.into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            queues: prefab.queues.clone(),
            team_player: TeamPlayer::default(),
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            transform: Transform::default(),
        }
    }
}

impl From<(CraneYardDisk, &CraneYardPrefab)> for CraneYardBundle {
    fn from((save, prefab): (CraneYardDisk, &CraneYardPrefab)) -> Self {
        Self {
            crane_yard: CraneYard,
            object_type: CraneYard.into(),
            snowflake: save.disk_snowflake.unwrap_or(Snowflake::new()),
            health: save.disk_health.unwrap_or(prefab.health),
            queues: save.disk_queues.unwrap_or(prefab.queues.clone()),
            team_player: save.team_player,
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            transform: save.transform.into(),
            visibility: Visibility::default(),
        }
    }
}

impl CraneYardGhost {
    pub fn new() -> Self {
        Self {
            object_type: CraneYard.into(),
            transform: Transform::default(),
            visibility: Visibility::default(),
        }
    }
}

impl<'a> From<CraneYardDiskQuery<'a>> for CraneYardDisk {
    fn from(object: CraneYardDiskQuery) -> Self {
        Self {
            disk_snowflake: Some(*object.0),
            disk_health: object.1.slim(),
            disk_queues: object.2.slim(),
            team_player: *object.3,
            transform: (*object.4).into(),
        }
    }
}

impl From<CraneYardDisk> for SpawnObject {
    fn from(value: CraneYardDisk) -> Self {
        Self {
            object_type: ObjectType::CraneYard,
            spawn_data: ObjectSpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform.into(),
            },
            disk_data: Some(ObjectDiskData {
                health: value.disk_health,
                queues: value.disk_queues,
                ..default()
            }),
            spawn_mode: SpawnMode::Load,
            phantom_data: PhantomData,
        }
    }
}

pub struct CraneYardPlugin;

impl CraneYardPlugin {
    pub fn spawn(
        mut spawn_events: EventReader<SpawnObject<CraneYard>>,
        mut client_requests: EventWriter<ClientRequest>,
        prefabs: Res<ObjectPrefabs>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in spawn_events.read() {
            commands.spawn(CraneYardBundle::from(prefabs.crane_yard_prefab.clone()).with_spawn_data(event.spawn_data.clone()).with_disk_data(event.disk_data.clone()));
            match event.spawn_mode {
                SpawnMode::Load => { status.crane_yards_loaded = Some(true); },
                SpawnMode::Spawn => { client_requests.write(ClientRequest::SpawnObject(event.clone().into())); },
                SpawnMode::Fetch => { },
            }
        }
    }

    pub fn ghost(
        mut ghost: Local<Option<Entity>>,
        mut command_events: EventReader<CommandEvent>,
        mut spawn_events: EventWriter<SpawnObject>,
        mut commands: Commands,
    ) {
        for event in command_events.read() {
            let Some(entity) = event.objects.first() else { return; };
            let CommandType::Build(build) = &event.command else { continue; };
            match build {
                BuildStatus::Begin(building) => {
                    let Ok(ObjectType::CraneYard) = ObjectType::try_from(building.clone()) else { continue; };
                    let Ok(mut ghost_commands) = commands.get_entity(*entity) else { continue; };
                    ghost_commands.insert(CraneYardGhost::new());
                    *ghost = Some(ghost_commands.id());
                }
                BuildStatus::Finish(transform) => {
                    if ghost.map_or(false, |ghost_entity| *entity == ghost_entity) {
                        let spawn_data = ObjectSpawnData {
                            snowflake: Snowflake::new(),
                            teamplayer: event.player,
                            transform: *transform,
                        };

                        let spawn_event = SpawnObject {
                            object_type: ObjectType::CraneYard,
                            spawn_data: spawn_data,
                            disk_data: None,
                            spawn_mode: SpawnMode::Spawn,
                            phantom_data: PhantomData,
                        };

                        spawn_events.write(spawn_event);
                    }
                },
                // _ => { }
            }
        }
    }

    pub fn save() {

    }
}

impl Plugin for CraneYardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                Self::spawn,
                Self::ghost,
            ).run_if(resource_exists::<ObjectPrefabs>))
        ;
    }
}