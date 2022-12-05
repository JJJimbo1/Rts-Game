use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Default, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct ResourcePlatformClaimed(pub Option<(Entity, usize)>);

impl AssetId for ResourcePlatformClaimed {
    fn id(&self) -> Option<&'static str> {
        ObjectType::ResourcePlatformClaimed.id()
    }
}

impl From<ResourcePlatformClaimed> for ObjectType {
    fn from(_: ResourcePlatformClaimed) -> Self {
        ObjectType::ResourcePlatformClaimed
    }
}

impl From<ResourcePlatformClaimed> for AssetType {
    fn from(_: ResourcePlatformClaimed) -> Self {
        Self::Object(ObjectType::ResourcePlatformClaimed)
    }
}

impl From<ResourcePlatformUnclaimed> for ResourcePlatformClaimed {
    fn from(resource_platform_unclaimed: ResourcePlatformUnclaimed) -> Self {
        ResourcePlatformClaimed(resource_platform_unclaimed.0)
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct ResourcePlatformClaimedBundle {
    pub resource_platform: ResourcePlatformClaimed,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub snowflake: Snowflake,
    pub health: Health,
    pub economic_object: EconomicObject,
    pub team_player: TeamPlayer,
    pub selectable: Selectable,
    pub collider: Collider,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl ResourcePlatformClaimedBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }

    pub fn with_platform(mut self, platform: ResourcePlatformClaimed) -> Self {
        self.resource_platform = platform;
        self
    }
}

impl From<ResourcePlatformClaimedPrefab> for ResourcePlatformClaimedBundle {
    fn from(prefab: ResourcePlatformClaimedPrefab) -> Self {
        Self {
            resource_platform: ResourcePlatformClaimed::default(),
            object_type: ResourcePlatformClaimed::default().into(),
            asset_type: ResourcePlatformClaimed::default().into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            economic_object: prefab.economic_object,
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

#[derive(Clone)]
pub struct ResourcePlatformClaimedPrefab {
    pub health: Health,
    pub economic_object: EconomicObject,
    pub cost: f64,
    pub collider: Collider,
}

impl TryFrom<&ObjectAsset> for ResourcePlatformClaimedPrefab {
    type Error = ContentError;
    fn try_from(prefab: &ObjectAsset) -> Result<Self, ContentError> {
        let Some(health) = prefab.health else { return Err(ContentError::MissingHealth); };
        let Some(economic_object) = prefab.economic_object else { return Err(ContentError::MissingEconomic); };
        let Some(stack) = prefab.stack else { return Err(ContentError::MissingStack); };
        let Some(collider_string) = prefab.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let collider = Collider::trimesh(vertices, indices);

        Ok(Self {
            health,
            economic_object,
            cost: stack.1.cost as f64,
            collider,
        })
    }
}


pub fn resource_platform_claimed_on_killed(
    mut activation_events: EventReader<ObjectKilledEvent>,
    prefabs: Res<ObjectPrefabs>,
    mut resource_nodes: Query<&mut ResourceNode>,
    resource_platforms_unclaimed: Query<(&GlobalTransform, &ResourcePlatformClaimed, &Snowflake)>,
    mut commands: Commands,
) {
    for event in activation_events.iter() {
        if let Ok((global_transform, platform, snowflake)) = resource_platforms_unclaimed.get(event.0) {
            let spawn_data = ObjectSpawnEventData {
                object_type: ObjectType::ResourcePlatformUnclaimed,
                snowflake: *snowflake,
                teamplayer: TeamPlayer::default(),
                transform: Transform::from(*global_transform),
            };
            if let Ok(mut node) = resource_nodes.get_mut(platform.0.unwrap().0) {
                node.0[platform.0.unwrap().1] = ResourcePlatform::Unclaimed;
            }
            commands.spawn(ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_platform((*platform).into()).with_spawn_data(spawn_data));
            commands.entity(event.0).despawn_recursive();
        }
    }
}