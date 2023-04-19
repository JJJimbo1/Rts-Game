use bevy::{prelude::*, utils::HashMap, ecs::schedule::StateData};
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use superstruct::*;

use crate::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct CraneYardMarker;

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

#[superstruct{
    variants(Bundle, Prefab, Serde),
    variant_attributes(derive(Debug, Clone)),
    specific_variant_attributes(
        Bundle(derive(Bundle)),
        Serde(derive(Serialize, Deserialize)),
    ),
}]
#[derive(Debug, Clone)]
pub struct CraneYard {
    #[superstruct(only(Bundle, Prefab))]    pub health: Health,
    #[superstruct(only(Bundle, Prefab))]    pub queues: Queues,
    #[superstruct(only(Bundle, Prefab))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub crane_yard: CraneYardMarker,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub asset_type: AssetType,
    #[superstruct(only(Bundle))]     pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub computed_visibility: ComputedVisibility,
    #[superstruct(only(Bundle))]            pub global_transform: GlobalTransform,
    #[superstruct(only(Bundle, Serde))]     pub team_player: TeamPlayer,
    #[superstruct(only(Bundle, Serde))]     pub transform: Transform,
    #[superstruct(only(Serde))]             pub serde_snowflake: Option<Snowflake>,
    #[superstruct(only(Serde))]             pub serde_health: Option<Health>,
    #[superstruct(only(Serde))]             pub serde_queues: Option<Queues>,
}

impl CraneYardBundle {
    pub fn with_spawn_data(mut self, spawn_data: SpawnData) -> Self {
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }

    pub fn with_serde_data(mut self, serde_data: Option<SerdeData>) -> Self {
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
            computed_visibility: ComputedVisibility::default(),
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
            computed_visibility: ComputedVisibility::default(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl TryFrom<(&ObjectAsset, &HashMap<ObjectType, (ActiveQueue, StackData)>)> for CraneYardPrefab {
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

impl<'a> From<SerdeCraneYardQuery<'a>> for CraneYardSerde {
    fn from(object: SerdeCraneYardQuery) -> Self {
        Self {
            serde_snowflake: Some(*object.0),
            serde_health: object.1.saved(),
            serde_queues: object.2.saved(),
            team_player: *object.3,
            transform: (*object.4).into(),
        }
    }
}

impl From<CraneYardSerde> for ObjectSpawnEvent {
    fn from(value: CraneYardSerde) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::CraneYard,
            spawn_data: SpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform.into(),
            },
            serde_data: Some(SerdeData {
                health: value.serde_health,
                queues: value.serde_queues,
                ..default()
            }),
        })
    }
}

pub struct CraneYardPlugin<S: StateData> {
    pub state: S,
}

impl<S: StateData> CraneYardPlugin<S> {
    pub fn new(state: S) -> Self {
        Self {
            state
        }
    }
}

impl<S: StateData> Plugin for CraneYardPlugin<S> {
    fn build(&self, _app: &mut App) {
        
    }
}