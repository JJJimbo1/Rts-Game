use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use superstruct::*;
use t5f_utility::colliders::decode;
use crate::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct Barracks;

impl From<Barracks> for ObjectType {
    fn from(_: Barracks) -> Self {
        ObjectType::Barracks
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
pub struct Barracks {
    #[superstruct(only(Prefab, Bundle))]        pub health: Health,
    #[superstruct(only(Prefab, Bundle))]        pub queues: Queues,
    #[superstruct(only(Prefab, Bundle))]        pub collider: Collider,
    #[superstruct(only(Bundle))]                pub factory: Barracks,
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

impl TryFrom<(&ObjectAsset, &HashMap<ObjectType, (ActiveQueue, StackData)>)> for BarracksPrefab {
    type Error = ContentError;
    fn try_from((asset, stacks): (&ObjectAsset, &HashMap<ObjectType, (ActiveQueue, StackData)>)) -> Result<Self, Self::Error> {
        let Some(health) = asset.health else { return Err(ContentError::MissingHealth); };
        let Some(asset_queues) = asset.asset_queues.clone() else { return Err(ContentError::MissingQueues); };
        let Some(collider_string) = asset.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let queues = Queues::from((&asset_queues, stacks));
        let Ok(collider) = Collider::trimesh(vertices, indices) else { return Err(ContentError::ColliderDecodeError)};

        Ok(Self {
            health,
            queues,
            collider,
        })
    }
}

impl BarracksBundle {
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

impl From<BarracksPrefab> for BarracksBundle {
    fn from(prefab: BarracksPrefab) -> Self {
        Self {
            factory: Barracks,
            object_type: Barracks.into(),
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

impl From<(BarracksDisk, &BarracksPrefab)> for BarracksBundle {
    fn from((save, prefab): (BarracksDisk, &BarracksPrefab)) -> Self {
        Self {
            factory: Barracks,
            object_type: Barracks.into(),
            snowflake: save.disk_snowflake.unwrap_or(Snowflake::new()),
            health: save.disk_health.unwrap_or(prefab.health),
            queues: save.disk_queues.unwrap_or(prefab.queues.clone()),
            team_player: save.team_player,
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            transform: save.transform.into(),
        }
    }
}


impl BarracksGhost {
    pub fn new() -> Self {
        Self {
            object_type: Barracks.into(),
            visibility: Visibility::default(),
            transform: Transform::default(),
        }
    }
}


impl<'a> From<BarracksDiskQuery<'a>> for BarracksDisk {
    fn from(object: BarracksDiskQuery) -> Self {
        Self {
            disk_snowflake: Some(*object.0),
            disk_health: object.1.slim(),
            disk_queues: object.2.slim(),
            team_player: *object.3,
            transform: (*object.4).into(),
        }
    }
}

impl From<BarracksDisk> for LoadObjects {
    fn from(value: BarracksDisk) -> Self {
        Self {
            object_type: ObjectType::Barracks,
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
        }
    }
}

pub struct BarracksPlugin;

impl BarracksPlugin {
    pub fn spawn(
        mut load_events: EventReader<LoadObject<Barracks>>,
        mut spawn_events: EventReader<SpawnObject<Barracks>>,
        mut fetch_events: EventReader<FetchObject<Barracks>>,
        mut client_requests: EventWriter<ClientRequest>,
        prefabs: Res<ObjectPrefabs>,
        mut loading_status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in load_events.read() {
            commands.spawn(BarracksBundle::from(prefabs.barracks_prefab.clone()).with_spawn_data(event.spawn_data.clone()).with_disk_data(event.disk_data.clone()));
            loading_status.factories_loaded = Some(true);
        }

        for event in spawn_events.read() {
            commands.spawn(BarracksBundle::from(prefabs.barracks_prefab.clone()).with_spawn_data(event.spawn_data.clone()));
            client_requests.send(ClientRequest::SpawnObject(event.clone().into()));
        }

        for event in fetch_events.read() {
            commands.spawn(BarracksBundle::from(prefabs.barracks_prefab.clone()).with_spawn_data(event.spawn_data.clone()));
        }

    }

    pub fn ghost(
        mut ghost: Local<Option<Entity>>,
        mut command_events: EventReader<CommandEvent>,
        mut spawn_events: EventWriter<SpawnObjects>,
        mut commands: Commands,
    ) {
        for event in command_events.read() {
            let Some(entity) = event.objects.first() else { return; };
            let CommandType::Build(build) = &event.command else { continue; };
            match build {
                BuildStatus::Begin(building) => {
                    let Ok(ObjectType::Barracks) = ObjectType::try_from(building.clone()) else { continue; };
                    let Some(mut ghost_commands) = commands.get_entity(*entity) else { continue; };
                    ghost_commands.insert(BarracksGhost::new());
                    *ghost = Some(ghost_commands.id());
                }
                BuildStatus::Finish(transform) => {
                    if ghost.map_or(false, |ghost_entity| *entity == ghost_entity) {
                        let spawn_data = ObjectSpawnData {
                            snowflake: Snowflake::new(),
                            teamplayer: event.player,
                            transform: *transform,
                        };

                        let spawn_event = SpawnObjects {
                            object_type: ObjectType::Barracks,
                            spawn_data: spawn_data,
                        };

                        spawn_events.send(spawn_event);
                    }
                },
                // _ => { }
            }
        }
    }

    pub fn barracks_system(
        mut spawn_events: EventWriter<SpawnObjects>,
        mut queues: Query<(&Transform, &TeamPlayer, &mut Queues), With<Barracks>>
    ) {
        queues.iter_mut().for_each(|(transform, teamplayer, mut queues)| {
            for data in queues.queues[&ActiveQueue::Infantry].buffer.spine() {
                let mut transform = *transform;
                transform.translation += transform.forward() * 20.0;
                let spawn_data = SpawnObjects {
                    object_type: data.object,
                    spawn_data: ObjectSpawnData {
                        snowflake: Snowflake::new(),
                        teamplayer: *teamplayer,
                        transform
                    },
                };
                spawn_events.send(spawn_data);
            }
            queues.queues.get_mut(&ActiveQueue::Infantry).unwrap().buffer.clear();
        });
    }
}

impl Plugin for BarracksPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                (
                    Self::spawn,
                    Self::ghost,
                ).run_if(resource_exists::<ObjectPrefabs>),
                Self::barracks_system,
            ))
        ;
    }
}
