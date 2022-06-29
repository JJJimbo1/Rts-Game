use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct ResourceNode(pub [ResourcePlatform; 6]);

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

impl SerdeComponent for ResourceNode {
    fn saved(&self) -> Option<Self> {
        if self.0.iter().fold(true, |b, platform| if let ResourcePlatform::Claimed(_, _) = platform { false } else { b }) {
            println!("no save");
            None
        } else {
            Some(*self)
        }
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct ResourceNodeBundle {
    pub resource_node: ResourceNode,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub snowflake: Snowflake,
    pub team_player: TeamPlayer,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl ResourceNodeBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.team_player;
        self.transform = spawn_data.transform;
        self
    }
}

impl From<ResourceNodePrefab> for ResourceNodeBundle {
    fn from(prefab: ResourceNodePrefab) -> Self {
        Self {
            resource_node: ResourceNode::default(),
            object_type: ResourceNode::default().into(),
            asset_type: ResourceNode::default().into(),
            snowflake: Snowflake::new(),
            team_player: TeamPlayer::default(),
            collider: prefab.real_collider.clone().unwrap(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(SerdeResourceNode, &ResourceNodePrefab)> for ResourceNodeBundle {
    fn from((save, prefab): (SerdeResourceNode, &ResourceNodePrefab)) -> Self {
        Self {
            resource_node: save.resource_node.unwrap_or(ResourceNode::default()),
            object_type: ResourceNode::default().into(),
            asset_type: ResourceNode::default().into(),
            snowflake: save.snowflake.unwrap_or_else(|| Snowflake::new()),
            team_player: save.team_player,
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
    pub resource_node: Option<ResourceNode>,
    pub snowflake: Option<Snowflake>,
    pub team_player: TeamPlayer,
    pub transform: SerdeTransform,
}

impl<'a> From<SerdeResourceNodeQuery<'a>> for SerdeResourceNode {
    fn from(object: SerdeResourceNodeQuery) -> Self {
        Self {
            resource_node: object.0.saved(),
            snowflake: object.1.saved(),
            team_player: *object.2,
            transform: (*object.3).into(),
        }
    }
}

pub fn resource_node_spawn(
    prefabs: Res<ObjectPrefabs>,
    mut resource_nodes: Query<(Entity, &Transform, &ResourceNode), Added<ResourceNode>>,
    mut commands: Commands,
) {
    resource_nodes.for_each_mut(|(entity, transform, resource_node)| {
        for i in 0..resource_node.0.len() {
            let rotation = 1.0472 * i as f32;
            let mut platform_transform = transform.mul_transform(Transform::from_rotation(Quat::from_rotation_y(rotation)));
            platform_transform.translation += platform_transform.right() * 17.0;
            let platform_type = resource_node.0[i];
            match platform_type {
                ResourcePlatform::Unclaimed => {
                    let spawn_data = ObjectSpawnEventData {
                        snowflake: Snowflake::new(),
                        object_type: ObjectType::ResourcePlatformUnclaimed,
                        team_player: TeamPlayer::default(),
                        transform: platform_transform,
                    };
                    let platform = ResourcePlatformUnclaimed(Some((entity, i)));
                    commands.spawn_bundle(ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_platform(platform).with_spawn_data(spawn_data));
                }
                ResourcePlatform::Claimed(snowflake, player) => {
                    let spawn_data = ObjectSpawnEventData {
                        snowflake,
                        object_type: ObjectType::ResourcePlatformClaimed,
                        team_player: player,
                        transform: platform_transform,
                    };
                    let platform = ResourcePlatformClaimed(Some((entity, i)));
                    commands.spawn_bundle(ResourcePlatformClaimedBundle::from(prefabs.resource_platform_claimed_prefab.clone()).with_platform(platform).with_spawn_data(spawn_data));
                }
            }
        }
    });
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum ResourcePlatform {
    Unclaimed,
    Claimed(Snowflake, TeamPlayer),
}

impl Default for ResourcePlatform {
    fn default() -> Self {
        Self::Unclaimed
    }
}