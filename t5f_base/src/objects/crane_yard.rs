use std::marker::PhantomData;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use superstruct::*;
use t5f_common::*;
use t5f_utility::colliders::decode;

use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct CraneYardMarker;

impl ObjectMarker for CraneYardMarker { }

impl From<CraneYardMarker> for ObjectType {
    fn from(_: CraneYardMarker) -> Self {
        ObjectType::CraneYard
    }
}

impl From<CraneYardMarker> for AssetType {
    fn from(_: CraneYardMarker) -> Self {
        Self::Object(ObjectType::CraneYard)
    }
}

// #[derive(Debug, Clone, Copy)]
// #[derive(Serialize, Deserialize)]
// #[derive(Component)]
// pub struct CraneYardGhostMarker;

// impl ObjectMarker for CraneYardGhostMarker { }

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
pub struct CraneYard {
    #[superstruct(only(Prefab, Bundle))]    pub health: Health,
    #[superstruct(only(Prefab, Bundle))]    pub queues: Queues,
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub crane_yard: CraneYardMarker,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle, Ghost))]     pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle, Ghost))]     pub visibility: Visibility,
    #[superstruct(only(Bundle, Ghost))]     pub view_visibility: ViewVisibility,
    #[superstruct(only(Bundle, Ghost))]     pub inherited_visibility: InheritedVisibility,
    #[superstruct(only(Bundle, Ghost))]     pub global_transform: GlobalTransform,
    #[superstruct(only(Bundle, Serde))]     pub team_player: TeamPlayer,
    #[superstruct(only(Bundle, Serde))]     pub transform: Transform,
    #[superstruct(only(Serde))]             pub serde_snowflake: Option<Snowflake>,
    #[superstruct(only(Serde))]             pub serde_health: Option<Health>,
    #[superstruct(only(Serde))]             pub serde_queues: Option<Queues>,
}

impl TryFrom<(&ObjectAsset, &HashMap<String, (ActiveQueue, StackData)>)> for CraneYardPrefab {
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

impl CraneYardBundle {
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

impl From<CraneYardPrefab> for CraneYardBundle {
    fn from(prefab: CraneYardPrefab) -> Self {
        Self {
            crane_yard: CraneYardMarker,
            object_type: CraneYardMarker.into(),
            asset_type: CraneYardMarker.into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            queues: prefab.queues.clone(),
            team_player: TeamPlayer::default(),
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(CraneYardSerde, &CraneYardPrefab)> for CraneYardBundle {
    fn from((save, prefab): (CraneYardSerde, &CraneYardPrefab)) -> Self {
        Self {
            crane_yard: CraneYardMarker,
            object_type: CraneYardMarker.into(),
            asset_type: CraneYardMarker.into(),
            snowflake: save.serde_snowflake.unwrap_or(Snowflake::new()),
            health: save.serde_health.unwrap_or(prefab.health),
            queues: save.serde_queues.unwrap_or(prefab.queues.clone()),
            team_player: save.team_player,
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl CraneYardGhost {
    pub fn new() -> Self {
        Self {
            asset_type: CraneYardMarker.into(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl<'a> From<SerdeCraneYardQuery<'a>> for CraneYardSerde {
    fn from(object: SerdeCraneYardQuery) -> Self {
        Self {
            serde_snowflake: Some(*object.0),
            serde_health: object.1.slim(),
            serde_queues: object.2.slim(),
            team_player: *object.3,
            transform: (*object.4).into(),
        }
    }
}

impl From<CraneYardSerde> for ObjectLoadEvent<AnyObjectMarker> {
    fn from(value: CraneYardSerde) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::CraneYard,
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

pub struct CraneYardPlugin;

impl CraneYardPlugin {
    pub fn load(
        mut load_events: EventReader<ObjectLoadEvent<CraneYardMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in load_events.read() {
            let crane_yard = CraneYardBundle::from(prefabs.crane_yard_prefab.clone()).with_spawn_data(event.spawn_data().clone()).with_serde_data(event.serde_data().clone());
            let entity = commands.spawn(crane_yard).id();
            identifiers.insert(event.spawn_data().snowflake, entity);
            println!("Crane Yards Loaded");
            status.crane_yards_loaded = Some(true);
        }
    }

    pub fn spawn(
        mut spawn_events: EventReader<ObjectSpawnEvent<CraneYardMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut commands: Commands,
    ) {
        for event in spawn_events.read() {
            let crane_yard = CraneYardBundle::from(prefabs.crane_yard_prefab.clone()).with_spawn_data(event.spawn_data().clone());
            let entity = commands.spawn(crane_yard).id();
            identifiers.insert(event.spawn_data().snowflake, entity);
        }
    }

    pub fn spawn_ghost(
        mut ghost: Local<Option<Entity>>,
        mut command_events: EventReader<CommandEvent>,
        mut spawn_events: EventWriter<ObjectSpawnEvent<CraneYardMarker>>,
        mut commands: Commands,
    ) {
        for event in command_events.read() {
            let Some(object) = &event.object else { continue; };
            let CommandType::Build(build) = &event.command else { continue; };
            match (object, build) {
                (CommandObject::Structure(entity), BuildStatus::Begin(building)) => {
                    let Ok(ObjectType::CraneYard) = ObjectType::try_from(building.clone()) else { continue; };
                    let Some(mut ghost_commands) = commands.get_entity(*entity) else { continue; };
                    ghost_commands.insert(CraneYardGhost::new());
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
                            object_type: ObjectType::CraneYard,
                            spawn_data: spawn_data,
                            serde_data: None,
                        };

                        let spawn_event = ObjectSpawnEvent::<CraneYardMarker>(spawn_event_data, PhantomData);

                        spawn_events.send(spawn_event);
                    }
                },
                _ => { }
            }
        }
    }

    pub fn save() {

    }
}

impl Plugin for CraneYardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ObjectLoadEvent<CraneYardMarker>>()
            .add_event::<ObjectSpawnEvent<CraneYardMarker>>()
            .add_systems(Update, (
                Self::load,
                Self::spawn,
            ).run_if(resource_exists::<ObjectPrefabs>()))
        ;
    }
}