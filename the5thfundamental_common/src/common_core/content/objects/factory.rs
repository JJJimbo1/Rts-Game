use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct Factory;

// impl AssetId for Factory {
//     fn id(&self) -> Option<&'static str> {
//         ObjectType::from(*self).id()
//     }
// }

impl From<Factory> for ObjectType {
    fn from(_: Factory) -> Self {
        ObjectType::Factory
    }
}

impl From<Factory> for AssetType {
    fn from(_: Factory) -> Self {
        Self::Object(ObjectType::Factory)
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct FactoryBundle {
    pub factory: Factory,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub snowflake: Snowflake,
    pub health: Health,
    pub queues: Queues,
    pub team_player: TeamPlayer,
    pub selectable: Selectable,
    pub collider: Collider,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl FactoryBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }
}

impl From<FactoryPrefab> for FactoryBundle {
    fn from(prefab: FactoryPrefab) -> Self {
        Self {
            factory: Factory,
            object_type: Factory.into(),
            asset_type: Factory.into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            queues: prefab.queues.clone(),
            team_player: TeamPlayer::default(),
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(SerdeFactory, &FactoryPrefab)> for FactoryBundle {
    fn from((save, prefab): (SerdeFactory, &FactoryPrefab)) -> Self {
        Self {
            factory: Factory,
            object_type: Factory.into(),
            asset_type: Factory.into(),
            snowflake: save.snowflake.unwrap_or_else(|| Snowflake::new()),
            health: save.health.unwrap_or(prefab.health),
            queues: save.queues.unwrap_or(prefab.queues.clone()),
            team_player: save.team_player,
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}

#[derive(Clone)]
pub struct FactoryPrefab {
    pub health: Health,
    pub queues: Queues,
    pub collider: Collider,
}

impl TryFrom<(&ObjectAsset, &HashMap<ObjectType, (ActiveQueue, StackData)>)> for FactoryPrefab {
    type Error = ContentError;
    fn try_from((prefab, stacks): (&ObjectAsset, &HashMap<ObjectType, (ActiveQueue, StackData)>)) -> Result<Self, ContentError> {
        let Some(health) = prefab.health else { return Err(ContentError::MissingHealth); };
        let Some(prefab_queues) = prefab.prefab_queues.clone() else { return Err(ContentError::MissingQueues); };
        let Some(collider_string) = prefab.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let queues = Queues::from((&prefab_queues, stacks));
        let collider = Collider::trimesh(vertices, indices);

        Ok(Self {
            health,
            queues,
            collider,
        })
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SerdeFactory {
    pub snowflake: Option<Snowflake>,
    pub health: Option<Health>,
    pub queues: Option<Queues>,
    pub team_player: TeamPlayer,
    pub transform: SerdeTransform,
}

impl<'a> From<SerdeFactoryQuery<'a>> for SerdeFactory {
    fn from(object: SerdeFactoryQuery) -> Self {
        Self {
            snowflake: Some(*object.0),
            health: object.1.saved(),
            queues: object.2.saved(),
            team_player: *object.3,
            transform: (*object.4).into(),
        }
    }
}

impl From<SerdeFactory> for ObjectSpawnEvent {
    fn from(value: SerdeFactory) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::Factory,
            snowflake: Snowflake::new(),
            teamplayer: value.team_player,
            transform: value.transform.into(),
        })
    }
}

pub fn factory_system(
    mut spawn_events: EventWriter<ObjectSpawnEvent>,
    mut queues: Query<(&Transform, &TeamPlayer, &mut Queues), With<Factory>>
) {
    queues.for_each_mut(|(transform, teamplayer, mut queues)| {

        for data in queues.queues[&ActiveQueue::Infantry].buffer.spine() {
            let mut transform = *transform;
            transform.translation += transform.forward() * 20.0;
            let spawn_data = ObjectSpawnEventData {
                object_type: data.object_type,
                snowflake: Snowflake::new(),
                teamplayer: *teamplayer,
                transform
            };
            spawn_events.send(ObjectSpawnEvent(spawn_data));
        }
        for data in queues.queues[&ActiveQueue::Vehicles].buffer.spine() {
            let mut transform = *transform;
            transform.translation += transform.forward() * 20.0;
            let spawn_data = ObjectSpawnEventData {
                object_type: data.object_type,
                snowflake: Snowflake::new(),
                teamplayer: *teamplayer,
                transform
            };
            spawn_events.send(ObjectSpawnEvent(spawn_data));
        }

        queues.queues.get_mut(&ActiveQueue::Infantry).unwrap().buffer.clear();
        queues.queues.get_mut(&ActiveQueue::Vehicles).unwrap().buffer.clear();


    });
}