use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct CraneYard;

impl AssetId for CraneYard {
    fn id(&self) -> &'static str {
        ObjectType::from(*self).id()
    }
}

impl From<CraneYard> for ObjectType {
    fn from(_: CraneYard) -> Self {
        ObjectType::CraneYard
    }
}

impl From<CraneYard> for AssetType {
    fn from(_: CraneYard) -> Self {
        Self::Object(ObjectType::CraneYard)
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct CraneYardBundle {
    pub crane_yard: CraneYard,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub snowflake: Snowflake,
    pub health: Health,
    pub queues: Queues,
    pub team_player: TeamPlayer,
    pub selectable: Selectable,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl CraneYardBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        self.team_player = spawn_data.team_player;
        self.transform = spawn_data.transform;
        self
    }
}

impl From<CraneYardPrefab> for CraneYardBundle {
    fn from(prefab: CraneYardPrefab) -> Self {
        Self {
            crane_yard: CraneYard,
            object_type: CraneYard.into(),
            asset_type: CraneYard.into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            queues: prefab.real_queues.unwrap(),
            team_player: TeamPlayer::default(),
            selectable: Selectable::single(),
            collider: prefab.real_collider.clone().unwrap(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(SerdeCraneYard, &CraneYardPrefab)> for CraneYardBundle {
    fn from((save, prefab): (SerdeCraneYard, &CraneYardPrefab)) -> Self {
        Self {
            crane_yard: CraneYard,
            object_type: CraneYard.into(),
            asset_type: CraneYard.into(),
            snowflake: save.snowflake.unwrap_or_else(|| Snowflake::new()),
            health: save.health.unwrap_or(prefab.health),
            queues: save.queues.unwrap_or(prefab.real_queues.as_ref().cloned().unwrap()),
            team_player: save.team_player,
            selectable: Selectable::single(),
            collider: prefab.real_collider.clone().unwrap(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct CraneYardPrefab {
    pub health: Health,
    pub queues: PrefabQueues,
    #[serde(skip)]
    pub real_queues: Option<Queues>,
    pub collider_string: String,
    #[serde(skip)]
    pub real_collider: Option<Collider>,
}

impl CraneYardPrefab {
    pub fn with_real_queues(mut self, stacks: &HashMap<ObjectType, (ActiveQueue, StackData)>) -> Self {
        let mut queues = Queues::new();
        for s in self.queues.objects.iter() {
            let (active, data) = stacks[s];
            queues.push_data_to_queue(active, data);
        }
        self.real_queues = Some(queues);
        self
    }

    pub fn with_real_collider(mut self, collider: Collider) -> Self {
        self.real_collider = Some(collider);
        self
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SerdeCraneYard {
    pub snowflake: Option<Snowflake>,
    pub health: Option<Health>,
    pub queues: Option<Queues>,
    pub team_player: TeamPlayer,
    pub transform: SerdeTransform,
}

impl<'a> From<SerdeCraneYardQuery<'a>> for SerdeCraneYard {
    fn from(object: SerdeCraneYardQuery) -> Self {
        Self {
            snowflake: Some(*object.0),
            health: object.1.saved(),
            queues: object.2.saved(),
            team_player: *object.3,
            transform: (*object.4).into(),
        }
    }
}