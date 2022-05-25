use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use snowflake::ProcessUniqueId;
use crate::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct ResourceNode;

impl AssetId for ResourceNode {
    fn id(&self) -> &'static str {
        ObjectType::from(*self).id()
    }
}

impl From<ResourceNode> for ObjectType {
    fn from(_: ResourceNode) -> Self {
        ObjectType::ResourceNode
    }
}

impl From<ResourceNode> for AssetType {
    fn from(_: ResourceNode) -> Self {
        Self::Object(ObjectType::ResourceNode)
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct ResourceNodeBundle {
    pub resource_node: ResourceNode,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub snowflake: Snowflake,
    pub health: Health,
    pub economic_object: EconomicObject,
    pub team_player: TeamPlayer,
    pub selectable: Selectable,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl ResourceNodeBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        // self.snow_flake = spawn_data.snow_flake;
        self.team_player = spawn_data.team_player;
        self.transform = spawn_data.transform;
        self
    }
}

impl From<ResourceNodePrefab> for ResourceNodeBundle {
    fn from(prefab: ResourceNodePrefab) -> Self {
        Self {
            // snow_flake: SnowFlake(ProcessUniqueId::new()),
            resource_node: ResourceNode,
            object_type: ResourceNode.into(),
            asset_type: ResourceNode.into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            economic_object: prefab.economic_object,
            team_player: TeamPlayer::default(),
            selectable: Selectable::single(),
            collider: prefab.real_collider.clone().unwrap(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(SerdeResourceNode, &ResourceNodePrefab)> for ResourceNodeBundle {
    fn from((save, prefab): (SerdeResourceNode, &ResourceNodePrefab)) -> Self {
        Self {
            resource_node: ResourceNode,
            object_type: ResourceNode.into(),
            asset_type: ResourceNode.into(),
            snowflake: save.snowflake.unwrap_or_else(|| Snowflake::new()),
            health: save.health.unwrap_or(prefab.health),
            economic_object: prefab.economic_object,
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
pub struct ResourceNodePrefab {
    pub stack: (ActiveQueue, StackData),
    pub health: Health,
    pub economic_object: EconomicObject,
    pub collider_string: String,
    #[serde(skip)]
    pub real_collider: Option<Collider>,
}

impl ResourceNodePrefab {
    pub fn with_real_collider(mut self, collider: Collider) -> Self {
        self.real_collider = Some(collider);
        self
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SerdeResourceNode {
    pub snowflake: Option<Snowflake>,
    pub health: Option<Health>,
    // pub economic_object: Option<EconomicObject>,
    pub team_player: TeamPlayer,
    pub transform: SerdeTransform,
}

impl<'a> From<SerdeResourceNodeQuery<'a>> for SerdeResourceNode {
    fn from(object: SerdeResourceNodeQuery) -> Self {
        Self {
            snowflake: Some(*object.0),
            health: object.1.saved(),
            // economic_object: Some(*object.1),
            team_player: *object.2,
            transform: (*object.3).into(),
        }
    }
}