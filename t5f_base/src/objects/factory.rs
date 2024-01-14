use std::marker::PhantomData;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use superstruct::*;
use t5f_common::*;
use t5f_utility::colliders::decode;
use crate::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct FactoryMarker;

impl ObjectMarker for FactoryMarker { }

impl From<FactoryMarker> for ObjectType {
    fn from(_: FactoryMarker) -> Self {
        ObjectType::Factory
    }
}

impl From<FactoryMarker> for AssetType {
    fn from(_: FactoryMarker) -> Self {
        Self::Object(ObjectType::Factory)
    }
}

#[superstruct{
    variants(Bundle, Prefab, Ghost, Serde),
    variant_attributes(derive(Debug, Clone)),
    specific_variant_attributes(
        Bundle(derive(Bundle)),
        Ghost(derive(Bundle)),
        Serde(derive(Serialize, Deserialize)),
    ),
}]
#[derive(Debug, Clone)]
pub struct Factory {
    #[superstruct(only(Prefab, Bundle))]    pub health: Health,
    #[superstruct(only(Prefab, Bundle))]    pub queues: Queues,
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub factory: FactoryMarker,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle, Ghost))]     pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle, Ghost))]     pub visibility: Visibility,
    #[superstruct(only(Bundle, Ghost))]     pub inherited_visibility: InheritedVisibility,
    #[superstruct(only(Bundle, Ghost))]     pub view_visibility: ViewVisibility,
    #[superstruct(only(Bundle, Ghost))]     pub global_transform: GlobalTransform,
    #[superstruct(only(Bundle, Serde))]     pub team_player: TeamPlayer,
    #[superstruct(only(Bundle, Serde))]     pub transform: Transform,
    #[superstruct(only(Serde))]             pub serde_snowflake: Option<Snowflake>,
    #[superstruct(only(Serde))]             pub serde_health: Option<Health>,
    #[superstruct(only(Serde))]             pub serde_queues: Option<Queues>,
}

impl TryFrom<(&ObjectAsset, &HashMap<String, (ActiveQueue, StackData)>)> for FactoryPrefab {
    type Error = ContentError;
    fn try_from((asset, stacks): (&ObjectAsset, &HashMap<String, (ActiveQueue, StackData)>)) -> Result<Self, Self::Error> {
        let Some(health) = asset.health else { return Err(ContentError::MissingHealth); };
        let Some(asset_queues) = asset.asset_queues.clone() else { return Err(ContentError::MissingQueues); };
        let Some(collider_string) = asset.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let queues = Queues::from((&asset_queues, stacks));
        let collider = Collider::trimesh(vertices, indices);

        Ok(Self {
            health,
            queues,
            collider,
        })
    }
}

impl FactoryBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnData) -> Self {
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }

    pub fn with_serde_data(mut self, serde_data: Option<ObjectSerdeData>) -> Self {
        let Some(serde_data) = serde_data else { return self; };
        if let Some(health) = serde_data.health { self.health = health; }
        if let Some(queues) = serde_data.queues { self.queues = queues; }
        self
    }
}

impl From<FactoryPrefab> for FactoryBundle {
    fn from(prefab: FactoryPrefab) -> Self {
        Self {
            factory: FactoryMarker,
            object_type: FactoryMarker.into(),
            asset_type: FactoryMarker.into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            queues: prefab.queues.clone(),
            team_player: TeamPlayer::default(),
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(FactorySerde, &FactoryPrefab)> for FactoryBundle {
    fn from((save, prefab): (FactorySerde, &FactoryPrefab)) -> Self {
        Self {
            factory: FactoryMarker,
            object_type: FactoryMarker.into(),
            asset_type: FactoryMarker.into(),
            snowflake: save.serde_snowflake.unwrap_or(Snowflake::new()),
            health: save.serde_health.unwrap_or(prefab.health),
            queues: save.serde_queues.unwrap_or(prefab.queues.clone()),
            team_player: save.team_player,
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}


impl FactoryGhost {
    pub fn new() -> Self {
        Self {
            asset_type: FactoryMarker.into(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}


impl<'a> From<SerdeFactoryQuery<'a>> for FactorySerde {
    fn from(object: SerdeFactoryQuery) -> Self {
        Self {
            serde_snowflake: Some(*object.0),
            serde_health: object.1.slim(),
            serde_queues: object.2.slim(),
            team_player: *object.3,
            transform: (*object.4).into(),
        }
    }
}

impl From<FactorySerde> for ObjectLoadEvent<AnyObjectMarker> {
    fn from(value: FactorySerde) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::Factory,
            spawn_data: ObjectSpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform.into(),
            },
            serde_data: Some(ObjectSerdeData {
                health: value.serde_health,
                queues: value.serde_queues,
                ..default()
            }),
        }, PhantomData
        )
    }
}

pub struct FactoryPlugin;

impl FactoryPlugin {

    pub fn load(
        mut load_events: EventReader<ObjectLoadEvent<FactoryMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in load_events.read() {
            let factory = FactoryBundle::from(prefabs.factory_prefab.clone()).with_spawn_data(event.spawn_data().clone()).with_serde_data(event.serde_data().clone());
            let entity = commands.spawn(factory).id();
            identifiers.insert(event.spawn_data().snowflake, entity);
            println!("Factories Loaded");
            status.factories_loaded = Some(true);
        }
    }

    pub fn spawn(
        mut spawn_events: EventReader<ObjectSpawnEvent<FactoryMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut commands: Commands,
    ) {
        for event in spawn_events.read() {
            let factory = FactoryBundle::from(prefabs.factory_prefab.clone()).with_spawn_data(event.spawn_data().clone());
            let entity = commands.spawn(factory).id();
            identifiers.insert(event.spawn_data().snowflake, entity);
        }
    }

    pub fn spawn_ghost(
        mut ghost: Local<Option<Entity>>,
        mut command_events: EventReader<CommandEvent>,
        mut spawn_events: EventWriter<ObjectSpawnEvent<FactoryMarker>>,
        mut commands: Commands,
    ) {
        for event in command_events.read() {
            let Some(object) = &event.object else { continue; };
            let CommandType::Build(build) = &event.command else { continue; };
            match (object, build) {
                (CommandObject::Structure(entity), BuildStatus::Begin(building)) => {
                    let Ok(ObjectType::Factory) = ObjectType::try_from(building.clone()) else { continue; };
                    let Some(mut ghost_commands) = commands.get_entity(*entity) else { continue; };
                    ghost_commands.insert(FactoryGhost::new());
                    *ghost = Some(ghost_commands.id());
                }
                (CommandObject::Structure(entity), BuildStatus::Finish(transform)) => {
                    if ghost.map_or(false, |ghost_entity| *entity == ghost_entity) {
                        let spawn_data = ObjectSpawnData {
                            snowflake: Snowflake::new(),
                            teamplayer: event.player,
                            transform: *transform,
                        };

                        let spawn_event_data = ObjectSpawnEventData {
                            object_type: ObjectType::Factory,
                            spawn_data: spawn_data,
                            serde_data: None,
                        };

                        let spawn_event = ObjectSpawnEvent::<FactoryMarker>(spawn_event_data, PhantomData);

                        spawn_events.send(spawn_event);
                    }
                },
                _ => { }
            }
        }
    }

    pub fn factory_system(
        mut spawn_events: EventWriter<ObjectSpawnEvent<AnyObjectMarker>>,
        mut queues: Query<(&Transform, &TeamPlayer, &mut Queues), With<FactoryMarker>>
    ) {
        queues.for_each_mut(|(transform, teamplayer, mut queues)| {

            for data in queues.queues[&ActiveQueue::Infantry].buffer.spine() {
                let Ok(object) = data.object.clone().try_into() else { continue; };
                let mut transform = *transform;
                transform.translation += transform.forward() * 20.0;
                let spawn_data = ObjectSpawnEventData {
                    object_type: object,
                    spawn_data: ObjectSpawnData {
                        snowflake: Snowflake::new(),
                        teamplayer: *teamplayer,
                        transform
                    },
                    serde_data: None,
                };
                spawn_events.send(objects::ObjectSpawnEvent(spawn_data, PhantomData));
            }
            for data in queues.queues[&ActiveQueue::Vehicles].buffer.spine() {
                let Ok(object) = data.object.clone().try_into() else { continue; };
                let mut transform = *transform;
                transform.translation += transform.forward() * 20.0;
                let spawn_data = ObjectSpawnEventData {
                    object_type: object,
                    spawn_data: ObjectSpawnData {
                        snowflake: Snowflake::new(),
                        teamplayer: *teamplayer,
                        transform,
                    },
                    serde_data: None,
                };
                spawn_events.send(objects::ObjectSpawnEvent::<AnyObjectMarker>(spawn_data, PhantomData));
            }

            queues.queues.get_mut(&ActiveQueue::Infantry).unwrap().buffer.clear();
            queues.queues.get_mut(&ActiveQueue::Vehicles).unwrap().buffer.clear();
        });
    }
}

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ObjectLoadEvent<FactoryMarker>>()
            .add_event::<ObjectSpawnEvent<FactoryMarker>>()
            .add_systems(Update, (
                (
                    Self::load,
                    Self::spawn,
                    Self::spawn_ghost,
                ).run_if(resource_exists::<ObjectPrefabs>()),
                Self::factory_system,
            ))
        ;
    }
}
