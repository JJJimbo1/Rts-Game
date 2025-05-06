use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use superstruct::*;
use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct ResourceNode;

impl From<ResourceNode> for ObjectType {
    fn from(_: ResourceNode) -> Self {
        ObjectType::ResourceNode
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

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct ResourceNodePlatforms(pub [ResourcePlatform; 6]);

impl Slim for ResourceNodePlatforms {
    fn slim(&self) -> Option<Self> {
        if self.0.iter().fold(true, |b, platform| if let ResourcePlatform::Claimed(_, _) = platform { false } else { b }) {
            None
        } else {
            Some(*self)
        }
    }
}

#[superstruct{
    no_enum,
    variants(Bundle, Prefab, Disk),
    variant_attributes(derive(Debug, Clone)),
    specific_variant_attributes(
        Bundle(derive(Bundle)),
        Disk(derive(Serialize, Deserialize)),
    ),
}]
#[derive(Debug, Clone)]
pub struct ResourceNode {
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub resource_node_marker: ResourceNode,
    #[superstruct(only(Bundle))]            pub resource_node_platforms: ResourceNodePlatforms,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle, Disk))]      pub team_player: TeamPlayer,
    #[superstruct(only(Bundle, Disk))]      pub transform: Transform,
    #[superstruct(only(Disk))]              pub disk_resource_node: Option<ResourceNodePlatforms>,
    #[superstruct(only(Disk))]              pub disk_snowflake: Option<Snowflake>,
}

impl TryFrom<&ObjectAsset> for ResourceNodePrefab {
    type Error = ContentError;
    fn try_from(prefab: &ObjectAsset) -> Result<Self, Self::Error> {
        let Some(collider_string) = prefab.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let Ok(collider) = Collider::trimesh(vertices, indices) else { return Err(ContentError::ColliderDecodeError); };

        Ok(Self {
            collider,
        })
    }
}

impl ResourceNodeBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }

    pub fn with_disk_data(mut self, disk_data: Option<ObjectDiskData>) -> Self {
        let Some(disk_data) = disk_data else { return self; };
        if let Some(resource_node) = disk_data.resource_node { self.resource_node_platforms = resource_node; }
        self
    }
}

impl From<ResourceNodePrefab> for ResourceNodeBundle {
    fn from(prefab: ResourceNodePrefab) -> Self {
        Self {
            resource_node_marker: ResourceNode,
            resource_node_platforms: ResourceNodePlatforms::default(),
            object_type: ResourceNode.into(),
            snowflake: Snowflake::new(),
            team_player: TeamPlayer::default(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            transform: Transform::default(),
        }
    }
}

impl From<(ResourceNodeDisk, &ResourceNodePrefab)> for ResourceNodeBundle {
    fn from((save, prefab): (ResourceNodeDisk, &ResourceNodePrefab)) -> Self {
        Self {
            resource_node_marker: ResourceNode,
            resource_node_platforms: save.disk_resource_node.unwrap_or_else(|| ResourceNodePlatforms::default()),
            object_type: ResourceNode.into(),
            snowflake: save.disk_snowflake.unwrap_or_else(|| Snowflake::new()),
            team_player: save.team_player,
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            transform: save.transform,
        }
    }
}

impl<'a> From<ResourceNodeDiskQuery<'a>> for ResourceNodeDisk {
    fn from(object: ResourceNodeDiskQuery) -> Self {
        Self {
            disk_snowflake: object.0.slim(),
            disk_resource_node: object.1.slim(),
            team_player: *object.2,
            transform: *object.3,
        }
    }
}

impl From<ResourceNodeDisk> for LoadObjects {
    fn from(value: ResourceNodeDisk) -> Self {
        Self {
            object_type: ObjectType::ResourceNode,
            spawn_data: ObjectSpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform,
            },
            disk_data: Some(ObjectDiskData {
                resource_node: value.disk_resource_node,
                ..default()
            }),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct ResourcePlatformOwner(pub Option<(Entity, usize)>);

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct ResourcePlatformClaimedMarker;

impl From<ResourcePlatformClaimedMarker> for ObjectType {
    fn from(_: ResourcePlatformClaimedMarker) -> Self {
        ObjectType::ResourcePlatformClaimed
    }
}

#[superstruct{
    no_enum,
    variants(Bundle, Prefab),
    variant_attributes(derive(Debug, Clone)),
    specific_variant_attributes(
        Bundle(derive(Bundle)),
    ),
}]
#[derive(Debug, Clone)]
pub struct ResourcePlatformClaimed {
    #[superstruct(only(Prefab))]            pub stack: StackData,
    #[superstruct(only(Prefab, Bundle))]    pub health: Health,
    #[superstruct(only(Prefab, Bundle))]    pub economic_object: EconomicObject,
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub resource_platform_claimed_marker: ResourcePlatformClaimedMarker,
    #[superstruct(only(Bundle))]            pub resource_platform_owner: ResourcePlatformOwner,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub team_player: TeamPlayer,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub transform: Transform,
}

impl TryFrom<&ObjectAsset> for ResourcePlatformClaimedPrefab {
    type Error = ContentError;
    fn try_from(asset: &ObjectAsset) -> Result<Self, Self::Error> {
        let Some((_, stack)) = asset.stack.clone() else { return Err(ContentError::MissingStack); };
        let Some(health) = asset.health else { return Err(ContentError::MissingHealth); };
        let Some(economic_object) = asset.economic_object else { return Err(ContentError::MissingEconomic); };
        let Some(collider_string) = asset.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let Ok(collider) = Collider::trimesh(vertices, indices) else { return Err(ContentError::ColliderDecodeError); };

        Ok(Self {
            stack,
            health,
            economic_object,
            collider,
        })
    }
}

impl ResourcePlatformClaimedBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnData) -> Self {
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
            snowflake: Snowflake::new(),
            health: prefab.health,
            economic_object: prefab.economic_object,
            team_player: TeamPlayer::default(),
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            transform: Transform::default(),
        }
    }
}


#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct ResourcePlatformUnclaimedMarker;

impl From<ResourcePlatformUnclaimedMarker> for ObjectType {
    fn from(_: ResourcePlatformUnclaimedMarker) -> Self {
        ObjectType::ResourcePlatformUnclaimed
    }
}

#[superstruct{
    no_enum,
    variants(Bundle, Prefab),
    variant_attributes(derive(Debug, Clone)),
    specific_variant_attributes(
        Bundle(derive(Bundle)),
    ),
}]
#[derive(Debug, Clone)]
pub struct ResourcePlatformUnclaimed {
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub resource_platform_unclaimed_marker: ResourcePlatformUnclaimedMarker,
    #[superstruct(only(Bundle))]            pub resource_platform_owner: ResourcePlatformOwner,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub team_player: TeamPlayer,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub transform: Transform,
}

impl TryFrom<&ObjectAsset> for ResourcePlatformUnclaimedPrefab {
    type Error = ContentError;
    fn try_from(asset: &ObjectAsset) -> Result<Self, Self::Error> {
        let Some(collider_string) = asset.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let Ok(collider) = Collider::trimesh(vertices, indices) else { return Err(ContentError::ColliderDecodeError); };

        Ok(Self {
            collider,
        })
    }
}

impl ResourcePlatformUnclaimedBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnData) -> Self {
        self.snowflake = spawn_data.snowflake;
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
            object_type: ResourcePlatformUnclaimedMarker.into(),
            resource_platform_owner: ResourcePlatformOwner::default(),
            snowflake: Snowflake::new(),
            team_player: TeamPlayer::default(),
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            transform: Transform::default(),
        }
    }
}









pub struct ResourceNodePlugin;

impl ResourceNodePlugin {
    // pub fn _spawn(

    // ) {

    // }
    pub fn load(
        mut load_events: EventReader<LoadObject<ResourceNode>>,
        prefabs: Res<ObjectPrefabs>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in load_events.read() {
            let resource_node = ResourceNodeBundle::from(prefabs.resource_node_prefab.clone()).with_spawn_data(event.spawn_data.clone()).with_disk_data(event.disk_data.clone());
            let entity = commands.spawn(resource_node).id();
            let transform = event.spawn_data.transform;
            let resource_node = event.disk_data.clone().and_then(|disk_data| disk_data.resource_node).unwrap_or(ResourceNodePlatforms::default());

            for i in 0..resource_node.0.len() {
                let rotation = 1.0472 * i as f32;
                let mut platform_transform = transform.mul_transform(Transform::from_rotation(Quat::from_rotation_y(rotation)));
                platform_transform.translation += platform_transform.right() * 17.0;
                let platform_type = resource_node.0[i];
                let platform = ResourcePlatformOwner(Some((entity, i)));
                match platform_type {
                    ResourcePlatform::Unclaimed => {
                        let spawn_data = ObjectSpawnData {
                            snowflake: Snowflake::new(),
                            teamplayer: TeamPlayer::default(),
                            transform: platform_transform,
                        };
                        commands.spawn(ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_platform(platform).with_spawn_data(spawn_data));
                    }
                    ResourcePlatform::Claimed(snowflake, player) => {
                        let spawn_data = ObjectSpawnData {
                            snowflake,
                            teamplayer: player,
                            transform: platform_transform,
                        };
                        commands.spawn(ResourcePlatformClaimedBundle::from(prefabs.resource_platform_claimed_prefab.clone()).with_platform(platform).with_spawn_data(spawn_data));
                    }
                }
            }
            status.resource_nodes_loaded = Some(true);
        }
    }

    pub fn spawn(
        mut spawn_events: EventReader<SpawnObject<ResourceNode>>,
        prefabs: Res<ObjectPrefabs>,
        mut commands: Commands,
    ) {
        for event in spawn_events.read() {
            let resource_node = ResourceNodeBundle::from(prefabs.resource_node_prefab.clone()).with_spawn_data(event.spawn_data.clone());
            commands.spawn(resource_node);
            let transform = event.spawn_data.transform;
            let resource_node = ResourceNodePlatforms::default();

            for i in 0..resource_node.0.len() {
                let rotation = 1.0472 * i as f32;
                let mut platform_transform = transform.mul_transform(Transform::from_rotation(Quat::from_rotation_y(rotation)));
                platform_transform.translation += platform_transform.right() * 17.0;
                let snowflake = Snowflake::new();
                let spawn_data = ObjectSpawnData {
                    snowflake,
                    teamplayer: TeamPlayer::default(),
                    transform: platform_transform,
                };
                let platform = ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_spawn_data(spawn_data);
                commands.spawn(platform);
            }
        }
    }

    pub fn on_activation(
        mut command_events: EventReader<CommandEvent>,
        mut actors: ResMut<Commanders>,
        mut resource_nodes: Query<&mut ResourceNodePlatforms>,
        resource_platforms_unclaimed: Query<(&GlobalTransform, &ResourcePlatformOwner, &Snowflake), With<ResourcePlatformUnclaimedMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut commands: Commands,
    ) {
        for event in command_events.read() {
            if let Some(entity) = event.activate().and_then(|entities| entities.first().cloned()) {
                if let Ok((global_transform, platform, snowflake)) = resource_platforms_unclaimed.get(entity) {
                    if actors.commanders.get_mut(&event.player).map_or(false, |actor| actor.economy.remove_resources(prefabs.resource_platform_claimed_prefab.stack.cost as f64)) {
                        let spawn_data = ObjectSpawnData {
                            snowflake: *snowflake,
                            teamplayer: event.player,
                            transform: Transform::from(*global_transform),
                        };
                        if let Ok(mut node) = resource_nodes.get_mut(platform.0.unwrap().0) {
                            node.0[platform.0.unwrap().1] = ResourcePlatform::Claimed(*snowflake, event.player);
                        }
                        commands.spawn(ResourcePlatformClaimedBundle::from(prefabs.resource_platform_claimed_prefab.clone()).with_platform(*platform).with_spawn_data(spawn_data));
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }

    pub fn on_killed(
        mut killed_events: EventReader<ObjectKilledEvent>,
        mut resource_nodes: Query<&mut ResourceNodePlatforms>,
        resource_platforms_claimed: Query<(&GlobalTransform, &ResourcePlatformOwner, &Snowflake)>,
        prefabs: Res<ObjectPrefabs>,
        mut commands: Commands,
    ) {
        for event in killed_events.read() {
            if let Ok((global_transform, platform, snowflake)) = resource_platforms_claimed.get(event.0) {
                let spawn_data = ObjectSpawnData {
                    snowflake: *snowflake,
                    teamplayer: TeamPlayer::default(),
                    transform: Transform::from(*global_transform),
                };
                if let Ok(mut node) = resource_nodes.get_mut(platform.0.unwrap().0) {
                    node.0[platform.0.unwrap().1] = ResourcePlatform::Unclaimed;
                }
                commands.spawn(ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_platform(*platform).with_spawn_data(spawn_data));
                commands.entity(event.0).despawn();
            }
        }
    }
}

impl Plugin for ResourceNodePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                Self::load,
                Self::spawn,
                Self::on_activation,
                Self::on_killed,
            ).run_if(resource_exists::<ObjectPrefabs>))
        ;
    }
}