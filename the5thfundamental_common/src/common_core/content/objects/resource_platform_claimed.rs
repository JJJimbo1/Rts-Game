use bevy::{prelude::*, ecs::schedule::StateData};
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use superstruct::*;
use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct ResourcePlatformClaimedMarker;

#[derive(Debug, Default, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct ResourcePlatformOwner(pub Option<(Entity, usize)>);

impl From<ResourcePlatformClaimedMarker> for ObjectType {
    fn from(_: ResourcePlatformClaimedMarker) -> Self {
        ObjectType::ResourcePlatformClaimed
    }
}

impl From<ResourcePlatformClaimedMarker> for AssetType {
    fn from(_: ResourcePlatformClaimedMarker) -> Self {
        Self::Object(ObjectType::ResourcePlatformClaimed)
    }
}

#[superstruct{
    variants(Bundle, Prefab),
    variant_attributes(derive(Debug, Clone)),
    specific_variant_attributes(
        Bundle(derive(Bundle)),
    ),
}]
#[derive(Debug, Clone)]
pub struct ResourcePlatformClaimed {
    #[superstruct(only(Prefab))]            pub cost: f64,
    #[superstruct(only(Bundle, Prefab))]    pub health: Health,
    #[superstruct(only(Bundle, Prefab))]    pub economic_object: EconomicObject,
    #[superstruct(only(Bundle, Prefab))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub resource_platform_claimed_marker: ResourcePlatformClaimedMarker,
    #[superstruct(only(Bundle))]            pub resource_platform_owner: ResourcePlatformOwner,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub team_player: TeamPlayer,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub computed_visibility: ComputedVisibility,
    #[superstruct(only(Bundle))]            pub transform: Transform,
    #[superstruct(only(Bundle))]            pub global_transform: GlobalTransform,
}

impl ResourcePlatformClaimedBundle {
    pub fn with_spawn_data(mut self, spawn_data: SpawnData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }

    pub fn with_platform(mut self, platform: ResourcePlatformOwner) -> Self {
        self.resource_platform_owner = platform;
        self
    }
}

impl From<ResourcePlatformClaimedPrefab> for ResourcePlatformClaimedBundle {
    fn from(prefab: ResourcePlatformClaimedPrefab) -> Self {
        Self {
            resource_platform_claimed_marker: ResourcePlatformClaimedMarker,
            resource_platform_owner: ResourcePlatformOwner::default(),
            object_type: ResourcePlatformClaimedMarker.into(),
            asset_type: ResourcePlatformClaimedMarker.into(),
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

pub struct ResourcePlatformClaimedPlugin<S: StateData> {
    state: S,
}

impl<S: StateData> ResourcePlatformClaimedPlugin<S> {
    pub fn new(state: S) -> Self {
        Self {
            state
        }
    }

    pub fn resource_platform_claimed_on_killed(
        mut activation_events: EventReader<ObjectKilledEvent>,
        prefabs: Res<ObjectPrefabs>,
        mut resource_nodes: Query<&mut ResourceNodePlatforms>,
        resource_platforms_unclaimed: Query<(&GlobalTransform, &ResourcePlatformOwner, &Snowflake)>,
        mut commands: Commands,
    ) {
        for event in activation_events.iter() {
            if let Ok((global_transform, platform, snowflake)) = resource_platforms_unclaimed.get(event.0) {
                let spawn_data = SpawnData {
                    snowflake: *snowflake,
                    teamplayer: TeamPlayer::default(),
                    transform: Transform::from(*global_transform),
                };
                if let Ok(mut node) = resource_nodes.get_mut(platform.0.unwrap().0) {
                    node.0[platform.0.unwrap().1] = ResourcePlatform::Unclaimed;
                }
                commands.spawn(ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_platform(*platform).with_spawn_data(spawn_data));
                commands.entity(event.0).despawn_recursive();
            }
        }
    }
}

impl<S: StateData> Plugin for ResourcePlatformClaimedPlugin<S> {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(self.state.clone())
            .with_system(Self::resource_platform_claimed_on_killed)
        );
    }
}