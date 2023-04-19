use bevy::{prelude::*, ecs::schedule::StateData};
use bevy_rapier3d::prelude::Collider;
use superstruct::*;
use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct ResourcePlatformUnclaimedMarker;

impl From<ResourcePlatformUnclaimedMarker> for ObjectType {
    fn from(_: ResourcePlatformUnclaimedMarker) -> Self {
        ObjectType::ResourcePlatformUnclaimed
    }
}

impl From<ResourcePlatformUnclaimedMarker> for AssetType {
    fn from(_: ResourcePlatformUnclaimedMarker) -> Self {
        Self::Object(ObjectType::ResourcePlatformUnclaimed)
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
pub struct ResourcePlatformUnclaimed {
    #[superstruct(only(Bundle, Prefab))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub resource_platform_unclaimed_marker: ResourcePlatformUnclaimedMarker,
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

impl ResourcePlatformUnclaimedBundle {
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

impl From<ResourcePlatformUnclaimedPrefab> for ResourcePlatformUnclaimedBundle {
    fn from(prefab: ResourcePlatformUnclaimedPrefab) -> Self {
        Self {
            resource_platform_unclaimed_marker: ResourcePlatformUnclaimedMarker,
            resource_platform_owner: ResourcePlatformOwner::default(),
            object_type: ResourcePlatformUnclaimedMarker.into(),
            asset_type: ResourcePlatformUnclaimedMarker.into(),
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

// #[derive(Clone)]
// pub struct ResourcePlatformUnclaimedPrefab {
//     pub collider: Collider,
// }

impl TryFrom<&ObjectAsset> for ResourcePlatformUnclaimedPrefab {
    type Error = ContentError;
    fn try_from(prefab: &ObjectAsset) -> Result<Self, ContentError> {
        let Some(collider_string) = prefab.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let collider = Collider::trimesh(vertices, indices);

        Ok(Self {
            collider,
        })
    }
}

pub struct ResourcePlatformUnclaimedPlugin<S: StateData> {
    state: S,
}

impl<S: StateData> ResourcePlatformUnclaimedPlugin<S> {
    pub fn new(state: S) -> Self {
        Self {
            state
        }
    }

    pub fn resource_platform_unclaimed_on_activation(
        mut activation_events: EventReader<ActivationEvent>,
        mut actors: ResMut<Actors>,
        prefabs: Res<ObjectPrefabs>,
        mut resource_nodes: Query<&mut ResourceNodePlatforms>,
        resource_platforms_unclaimed: Query<(&GlobalTransform, &ResourcePlatformOwner, &Snowflake)>,
        mut commands: Commands,
    ) {
        for event in activation_events.iter() {
            if let Ok((global_transform, platform, snowflake)) = resource_platforms_unclaimed.get(event.entity) {
                if actors.actors.get_mut(&event.player).map_or(false, |actor| actor.economy.remove_resources(prefabs.resource_platform_claimed_prefab.cost)) {
                    let spawn_data = SpawnData {
                        snowflake: *snowflake,
                        teamplayer: event.player,
                        transform: Transform::from(*global_transform),
                    };
                    if let Ok(mut node) = resource_nodes.get_mut(platform.0.unwrap().0) {
                        node.0[platform.0.unwrap().1] = ResourcePlatform::Claimed(*snowflake, event.player);
                    }
                    commands.spawn(ResourcePlatformClaimedBundle::from(prefabs.resource_platform_claimed_prefab.clone()).with_platform(*platform).with_spawn_data(spawn_data));
                    commands.entity(event.entity).despawn_recursive();
                }
            }
        }
    }
}

impl<S: StateData> Plugin for ResourcePlatformUnclaimedPlugin<S> {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(self.state.clone())
            .with_system(Self::resource_platform_unclaimed_on_activation)
        );
    }
}