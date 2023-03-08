use bevy::{prelude::*, ecs::schedule::StateData};
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct ResourcePlatformUnclaimed(pub Option<(Entity, usize)>);

// impl AssetId for ResourcePlatformUnclaimed {
//     fn id(&self) -> Option<&'static str> {
//         // "resource_platform"
//         ObjectType::ResourcePlatformUnclaimed.id()
//     }
// }

impl From<ResourcePlatformUnclaimed> for ObjectType {
    fn from(_: ResourcePlatformUnclaimed) -> Self {
        ObjectType::ResourcePlatformUnclaimed
    }
}

impl From<ResourcePlatformUnclaimed> for AssetType {
    fn from(_: ResourcePlatformUnclaimed) -> Self {
        Self::Object(ObjectType::ResourcePlatformUnclaimed)
    }
}

impl From<ResourcePlatformClaimed> for ResourcePlatformUnclaimed {
    fn from(resource_platform_claimed: ResourcePlatformClaimed) -> Self {
        ResourcePlatformUnclaimed(resource_platform_claimed.0)
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct ResourcePlatformUnclaimedBundle {
    pub resource_platform: ResourcePlatformUnclaimed,
    pub object_type: ObjectType,
    pub snowflake: Snowflake,
    pub team_player: TeamPlayer,
    pub selectable: Selectable,
    pub collider: Collider,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl ResourcePlatformUnclaimedBundle {
    pub fn with_platform(mut self, platform: ResourcePlatformUnclaimed) -> Self {
        self.resource_platform = platform;
        self
    }
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }
}

impl From<ResourcePlatformUnclaimedPrefab> for ResourcePlatformUnclaimedBundle {
    fn from(prefab: ResourcePlatformUnclaimedPrefab) -> Self {
        Self {
            resource_platform: ResourcePlatformUnclaimed::default(),
            object_type: ResourcePlatformUnclaimed::default().into(),
            snowflake: Snowflake::new(),
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
pub struct ResourcePlatformUnclaimedPrefab {
    pub health: Health,
    pub collider: Collider,
}

impl TryFrom<&ObjectAsset> for ResourcePlatformUnclaimedPrefab {
    type Error = ContentError;
    fn try_from(prefab: &ObjectAsset) -> Result<Self, ContentError> {
        let Some(health) = prefab.health else { return Err(ContentError::MissingHealth); };
        let Some(collider_string) = prefab.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let collider = Collider::trimesh(vertices, indices);

        Ok(Self {
            health,
            collider,
        })
    }
}

pub struct ResourcePlatformUnclaimedPlugin<T: StateData> {
    state: T,
}

impl<T: StateData> ResourcePlatformUnclaimedPlugin<T> {
    pub fn resource_platform_unclaimed_on_activation(
        mut activation_events: EventReader<ActivationEvent>,
        mut actors: ResMut<Actors>,
        prefabs: Res<ObjectPrefabs>,
        mut resource_nodes: Query<&mut ResourceNode>,
        resource_platforms_unclaimed: Query<(&GlobalTransform, &ResourcePlatformUnclaimed, &Snowflake)>,
        mut commands: Commands,
    ) {
        for event in activation_events.iter() {
            if let Ok((global_transform, platform, snowflake)) = resource_platforms_unclaimed.get(event.entity) {
                if actors.actors.get_mut(&event.player).map_or(false, |actor| actor.economy.remove_resources(prefabs.resource_platform_claimed_prefab.cost)) {
                    let spawn_data = ObjectSpawnEventData {
                        object_type: ObjectType::ResourcePlatformClaimed,
                        snowflake: *snowflake,
                        teamplayer: event.player,
                        transform: Transform::from(*global_transform),
                    };
                    if let Ok(mut node) = resource_nodes.get_mut(platform.0.unwrap().0) {
                        node.0[platform.0.unwrap().1] = ResourcePlatform::Claimed(*snowflake, event.player);
                    }
                    commands.spawn(ResourcePlatformClaimedBundle::from(prefabs.resource_platform_claimed_prefab.clone()).with_platform((*platform).into()).with_spawn_data(spawn_data));
                    commands.entity(event.entity).despawn_recursive();
                }
            }
        }
    }
}

impl<T: StateData> Plugin for ResourcePlatformUnclaimedPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(self.state.clone())
            .with_system(Self::resource_platform_unclaimed_on_activation)
        );
    }
}