use bevy::{prelude::*, ecs::schedule::StateData};
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use superstruct::*;
use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct ResourceNodeMarker;

impl From<ResourceNodeMarker> for ObjectType {
    fn from(_: ResourceNodeMarker) -> Self {
        ObjectType::ResourceNode
    }
}

impl From<ResourceNodeMarker> for AssetType {
    fn from(_: ResourceNodeMarker) -> Self {
        Self::Object(ObjectType::ResourceNode)
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct ResourceNodePlatforms(pub [ResourcePlatform; 6]);

impl SerdeComponent for ResourceNodePlatforms {
    fn saved(&self) -> Option<Self> {
        if self.0.iter().fold(true, |b, platform| if let ResourcePlatform::Claimed(_, _) = platform { false } else { b }) {
            None
        } else {
            Some(*self)
        }
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
pub struct ResourceNode {
    // #[superstruct(only(Prefab))]            pub health: Health,
    #[superstruct(only(Bundle, Prefab))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub resource_node_marker: ResourceNodeMarker,
    #[superstruct(only(Bundle))]            pub resource_node_platforms: ResourceNodePlatforms,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub computed_visibility: ComputedVisibility,
    #[superstruct(only(Bundle))]            pub global_transform: GlobalTransform,
    #[superstruct(only(Bundle, Serde))]     pub team_player: TeamPlayer,
    #[superstruct(only(Bundle, Serde))]     pub transform: Transform,
    #[superstruct(only(Serde))]             pub serde_snowflake: Option<Snowflake>,
    #[superstruct(only(Serde))]             pub serde_resource_node: Option<ResourceNodePlatforms>,
}

impl ResourceNodeBundle {
    pub fn with_spawn_data(mut self, spawn_data: SpawnData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }

    pub fn with_serde_data(mut self, serde_data: Option<SerdeData>) -> Self {
        let Some(serde_data) = serde_data else { return self; };
        if let Some(resource_node) = serde_data.resource_node { self.resource_node_platforms = resource_node; }
        self
    }
}

impl From<ResourceNodePrefab> for ResourceNodeBundle {
    fn from(prefab: ResourceNodePrefab) -> Self {
        Self {
            resource_node_marker: ResourceNodeMarker,
            resource_node_platforms: ResourceNodePlatforms::default(),
            object_type: ResourceNodeMarker.into(),
            asset_type: ResourceNodeMarker.into(),
            snowflake: Snowflake::new(),
            team_player: TeamPlayer::default(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(ResourceNodeSerde, &ResourceNodePrefab)> for ResourceNodeBundle {
    fn from((save, prefab): (ResourceNodeSerde, &ResourceNodePrefab)) -> Self {
        Self {
            resource_node_marker: ResourceNodeMarker,
            resource_node_platforms: save.serde_resource_node.unwrap_or_else(|| ResourceNodePlatforms::default()),
            object_type: ResourceNodeMarker.into(),
            asset_type: ResourceNodeMarker.into(),
            snowflake: save.serde_snowflake.unwrap_or_else(|| Snowflake::new()),
            team_player: save.team_player,
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: save.transform,
            global_transform: GlobalTransform::default(),
        }
    }
}

// #[derive(Clone)]
// pub struct ResourceNodePrefab {
//     pub health: Health,
//     pub collider: Collider,
// }

impl TryFrom<&ObjectAsset> for ResourceNodePrefab {
    type Error = ContentError;
    fn try_from(prefab: &ObjectAsset) -> Result<Self, ContentError> {
        // let Some(health) = prefab.health else { return Err(ContentError::MissingHealth); };
        let Some(collider_string) = prefab.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let collider = Collider::trimesh(vertices, indices);

        Ok(Self {
            // health,
            collider,
        })
    }
}

// #[derive(Debug, Clone)]
// #[derive(Serialize, Deserialize)]
// pub struct SerdeResourceNode {
//     pub snowflake: Option<Snowflake>,
//     pub resource_node: Option<ResourceNodePlatforms>,
//     pub team_player: TeamPlayer,
//     pub transform: SerdeTransform,
// }

impl<'a> From<SerdeResourceNodeQuery<'a>> for ResourceNodeSerde {
    fn from(object: SerdeResourceNodeQuery) -> Self {
        Self {
            serde_snowflake: object.0.saved(),
            serde_resource_node: object.1.saved(),
            team_player: *object.2,
            transform: *object.3,
        }
    }
}

impl From<ResourceNodeSerde> for ObjectSpawnEvent {
    fn from(value: ResourceNodeSerde) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::ResourceNode,
            spawn_data: SpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform,
            },
            serde_data: Some(SerdeData {
                resource_node: value.serde_resource_node,
                ..default()
            }),
        })
    }
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

pub struct ResourceNodePlugin<S: StateData> {
    state: S,
}

impl<S: StateData> ResourceNodePlugin<S> {
    pub fn new(state: S) -> Self {
        Self {
            state
        }
    }

    pub fn resource_node_spawn(
        prefabs: Res<ObjectPrefabs>,
        mut resource_nodes: Query<(Entity, &Transform, &ResourceNodePlatforms), Added<ResourceNodePlatforms>>,
        mut commands: Commands,
    ) {
        resource_nodes.for_each_mut(|(entity, transform, resource_node)| {
            for i in 0..resource_node.0.len() {
                let rotation = 1.0472 * i as f32;
                let mut platform_transform = transform.mul_transform(Transform::from_rotation(Quat::from_rotation_y(rotation)));
                platform_transform.translation += platform_transform.right() * 17.0;
                let platform_type = resource_node.0[i];
                let platform = ResourcePlatformOwner(Some((entity, i)));
                match platform_type {
                    ResourcePlatform::Unclaimed => {
                        let spawn_data = SpawnData {
                            snowflake: Snowflake::new(),
                            teamplayer: TeamPlayer::default(),
                            transform: platform_transform,
                        };
                        commands.spawn(ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_platform(platform).with_spawn_data(spawn_data));
                    }
                    ResourcePlatform::Claimed(snowflake, player) => {
                        let spawn_data = SpawnData {
                            snowflake,
                            teamplayer: player,
                            transform: platform_transform,
                        };
                        commands.spawn(ResourcePlatformClaimedBundle::from(prefabs.resource_platform_claimed_prefab.clone()).with_platform(platform).with_spawn_data(spawn_data));
                    }
                }
            }
        });
    }
}

impl<S: StateData> Plugin for ResourceNodePlugin<S> {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(self.state.clone())
            .with_system(Self::resource_node_spawn)
        );
    }
}