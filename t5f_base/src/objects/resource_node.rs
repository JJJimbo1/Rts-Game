use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use superstruct::*;
use t5f_common::*;
use t5f_utility::colliders::decode;
use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct ResourceNodeMarker;

impl ObjectMarker for ResourceNodeMarker { }

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
    variants(Bundle, Prefab, Serde),
    variant_attributes(derive(Debug, Clone)),
    specific_variant_attributes(
        Bundle(derive(Bundle)),
        Serde(derive(Serialize, Deserialize)),
    ),
}]
#[derive(Debug, Clone)]
pub struct ResourceNode {
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub resource_node_marker: ResourceNodeMarker,
    #[superstruct(only(Bundle))]            pub resource_node_platforms: ResourceNodePlatforms,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub view_visibility: ViewVisibility,
    #[superstruct(only(Bundle))]            pub inherited_visibility: InheritedVisibility,
    #[superstruct(only(Bundle))]            pub global_transform: GlobalTransform,
    #[superstruct(only(Bundle, Serde))]     pub team_player: TeamPlayer,
    #[superstruct(only(Bundle, Serde))]     pub transform: Transform,
    #[superstruct(only(Serde))]             pub serde_snowflake: Option<Snowflake>,
    #[superstruct(only(Serde))]             pub serde_resource_node: Option<ResourceNodePlatforms>,
}

impl TryFrom<&ObjectAsset> for ResourceNodePrefab {
    type Error = ContentError;
    fn try_from(prefab: &ObjectAsset) -> Result<Self, Self::Error> {
        let Some(collider_string) = prefab.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let collider = Collider::trimesh(vertices, indices);

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

    pub fn with_serde_data(mut self, serde_data: Option<ObjectSerdeData>) -> Self {
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
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
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
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: save.transform,
            global_transform: GlobalTransform::default(),
        }
    }
}

impl<'a> From<SerdeResourceNodeQuery<'a>> for ResourceNodeSerde {
    fn from(object: SerdeResourceNodeQuery) -> Self {
        Self {
            serde_snowflake: object.0.slim(),
            serde_resource_node: object.1.slim(),
            team_player: *object.2,
            transform: *object.3,
        }
    }
}

impl From<ResourceNodeSerde> for ObjectLoadEvent<AnyObjectMarker> {
    fn from(value: ResourceNodeSerde) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::ResourceNode,
            spawn_data: ObjectSpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform,
            },
            serde_data: Some(ObjectSerdeData {
                resource_node: value.serde_resource_node,
                ..default()
            }),
        }, PhantomData
        )
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

impl From<ResourcePlatformClaimedMarker> for AssetType {
    fn from(_: ResourcePlatformClaimedMarker) -> Self {
        Self::Object(ObjectType::ResourcePlatformClaimed)
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
pub struct ResourcePlatformClaimed {
    #[superstruct(only(Prefab))]            pub stack: StackData,
    #[superstruct(only(Prefab, Bundle))]    pub health: Health,
    #[superstruct(only(Prefab, Bundle))]    pub economic_object: EconomicObject,
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub resource_platform_claimed_marker: ResourcePlatformClaimedMarker,
    #[superstruct(only(Bundle))]            pub resource_platform_owner: ResourcePlatformOwner,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub team_player: TeamPlayer,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub view_visibility: ViewVisibility,
    #[superstruct(only(Bundle))]            pub inherited_visibility: InheritedVisibility,
    #[superstruct(only(Bundle))]            pub transform: Transform,
    #[superstruct(only(Bundle))]            pub global_transform: GlobalTransform,
}

impl TryFrom<&ObjectAsset> for ResourcePlatformClaimedPrefab {
    type Error = ContentError;
    fn try_from(asset: &ObjectAsset) -> Result<Self, Self::Error> {
        let Some((_, stack)) = asset.stack.clone() else { return Err(ContentError::MissingStack); };
        let Some(health) = asset.health else { return Err(ContentError::MissingHealth); };
        let Some(economic_object) = asset.economic_object else { return Err(ContentError::MissingEconomic); };
        let Some(collider_string) = asset.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let collider = Collider::trimesh(vertices, indices);

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
            asset_type: ResourcePlatformClaimedMarker.into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            economic_object: prefab.economic_object,
            team_player: TeamPlayer::default(),
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
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
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub resource_platform_unclaimed_marker: ResourcePlatformUnclaimedMarker,
    #[superstruct(only(Bundle))]            pub resource_platform_owner: ResourcePlatformOwner,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub team_player: TeamPlayer,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub view_visibility: ViewVisibility,
    #[superstruct(only(Bundle))]            pub inherited_visibility: InheritedVisibility,
    #[superstruct(only(Bundle))]            pub transform: Transform,
    #[superstruct(only(Bundle))]            pub global_transform: GlobalTransform,
}

impl TryFrom<&ObjectAsset> for ResourcePlatformUnclaimedPrefab {
    type Error = ContentError;
    fn try_from(asset: &ObjectAsset) -> Result<Self, Self::Error> {
        let Some(collider_string) = asset.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let collider = Collider::trimesh(vertices, indices);

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
            asset_type: ResourcePlatformUnclaimedMarker.into(),
            resource_platform_owner: ResourcePlatformOwner::default(),
            snowflake: Snowflake::new(),
            team_player: TeamPlayer::default(),
            selectable: Selectable::single(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}









pub struct ResourceNodePlugin;

impl ResourceNodePlugin {
    pub fn load(
        mut load_events: EventReader<ObjectLoadEvent<ResourceNodeMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in load_events.read() {
            let resource_node = ResourceNodeBundle::from(prefabs.resource_node_prefab.clone()).with_spawn_data(event.spawn_data().clone()).with_serde_data(event.serde_data().clone());
            let entity = commands.spawn(resource_node).id();
            let transform = event.spawn_data().transform;
            let resource_node = event.serde_data().clone().and_then(|serde_data| serde_data.resource_node).unwrap_or(ResourceNodePlatforms::default());

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
                        identifiers.insert(event.spawn_data().snowflake, entity);
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
            println!("Resource Nodes Loaded");
            status.resource_nodes_loaded = Some(true);
        }
    }

    pub fn spawn(
        mut spawn_events: EventReader<ObjectSpawnEvent<ResourceNodeMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut commands: Commands,
    ) {
        for event in spawn_events.read() {
            let resource_node = ResourceNodeBundle::from(prefabs.resource_node_prefab.clone()).with_spawn_data(event.spawn_data().clone());
            let entity = commands.spawn(resource_node).id();
            identifiers.insert(event.spawn_data().snowflake, entity);
            let transform = event.spawn_data().transform;
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
                let entity = commands.spawn(platform).id();
                identifiers.insert(snowflake, entity);
            }
        }
    }

    // pub fn resource_node_spawn(
    //     prefabs: Res<ObjectPrefabs>,
    //     mut resource_nodes: Query<(Entity, &Transform, &ResourceNodePlatforms), Added<ResourceNodePlatforms>>,
    //     mut commands: Commands,
    // ) {
    //     resource_nodes.for_each_mut(|(entity, transform, resource_node)| {
    //         for i in 0..resource_node.0.len() {
    //             let rotation = 1.0472 * i as f32;
    //             let mut platform_transform = transform.mul_transform(Transform::from_rotation(Quat::from_rotation_y(rotation)));
    //             platform_transform.translation += platform_transform.right() * 17.0;
    //             let platform_type = resource_node.0[i];
    //             let platform = ResourcePlatformOwner(Some((entity, i)));
    //             match platform_type {
    //                 ResourcePlatform::Unclaimed => {
    //                     let spawn_data = SpawnData {
    //                         snowflake: Snowflake::new(),
    //                         teamplayer: TeamPlayer::default(),
    //                         transform: platform_transform,
    //                     };
    //                     commands.spawn(ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_platform(platform).with_spawn_data(spawn_data));
    //                 }
    //                 ResourcePlatform::Claimed(snowflake, player) => {
    //                     let spawn_data = SpawnData {
    //                         snowflake,
    //                         teamplayer: player,
    //                         transform: platform_transform,
    //                     };
    //                     commands.spawn(ResourcePlatformClaimedBundle::from(prefabs.resource_platform_claimed_prefab.clone()).with_platform(platform).with_spawn_data(spawn_data));
    //                 }
    //             }
    //         }
    //     });
    // }

    pub fn on_activation(
        mut command_events: EventReader<CommandEvent>,
        mut actors: ResMut<Commanders>,
        mut resource_nodes: Query<&mut ResourceNodePlatforms>,
        resource_platforms_unclaimed: Query<(&GlobalTransform, &ResourcePlatformOwner, &Snowflake), With<ResourcePlatformUnclaimedMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut commands: Commands,
    ) {
        for event in command_events.read() {
            if let Some(entity) = event.activate_command().and_then(|entities| entities.first().cloned()) {
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
                        commands.entity(entity).despawn_recursive();
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
                commands.entity(event.0).despawn_recursive();
            }
        }
    }
}

impl Plugin for ResourceNodePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ObjectLoadEvent<ResourceNodeMarker>>()
            .add_event::<ObjectSpawnEvent<ResourceNodeMarker>>()
            .add_systems(Update, (
                Self::load,
                Self::spawn,
                Self::on_activation,
                Self::on_killed,
            ).run_if(resource_exists::<ObjectPrefabs>()))
        ;
    }
}