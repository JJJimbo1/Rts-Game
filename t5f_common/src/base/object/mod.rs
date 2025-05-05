pub mod barracks;
pub mod crane_yard;
pub mod factory;
pub mod marine_squad;
pub mod resource_node;
pub mod tank;

pub use barracks::*;
pub use crane_yard::*;
pub use factory::*;
pub use marine_squad::*;
pub use resource_node::*;
pub use tank::*;

use std::fmt::Display;
use serde::{Serialize, Deserialize};
use bevy::{prelude::*, asset::{AssetLoader, io::Reader}, platform::collections::HashMap, reflect::TypePath};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_mod_event_group::{event_group, EventGroupAppExt};
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Component)]

pub enum ObjectType {
    CraneYard,
    ResourceNode,
    ResourcePlatformUnclaimed,
    ResourcePlatformClaimed,
    Barracks,
    Factory,
    CommunicationsCenter,
    MarineSquad,
    Marine,
    TankBase,
    TankGun,
}

// impl From<ObjectType> for AssetType {
//     fn from(object_type: ObjectType) -> Self {
//         Self::Object(object_type)
//     }
// }

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::CraneYard => write!(f, "Crane Yard"),
            ObjectType::ResourceNode => write!(f, "Resource Node"),
            ObjectType::ResourcePlatformUnclaimed => write!(f, "Resource Platform Unclaimed"),
            ObjectType::ResourcePlatformClaimed => write!(f, "Resource Platform"),
            ObjectType::Barracks => write!(f, "Barracks"),
            ObjectType::Factory => write!(f, "Factory"),
            ObjectType::CommunicationsCenter => write!(f, "Communications Center"),
            ObjectType::MarineSquad => write!(f, "Marine Squad"),
            ObjectType::Marine => write!(f, "Marine"),
            ObjectType::TankBase => write!(f, "Tank"),
            ObjectType::TankGun => write!(f, "Tank Gun"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
#[event_group(
    event_type(object_type),
    group(LoadObject),
    events(CraneYard, Barracks, Factory, ResourceNode, MarineSquad, TankBase),
)]
pub struct LoadObjects {
    pub object_type: ObjectType,
    pub spawn_data: ObjectSpawnData,
    pub disk_data: Option<ObjectDiskData>,
}

impl From<SpawnObjects> for LoadObjects {
    fn from(value: SpawnObjects) -> Self {
        LoadObjects {
            object_type: value.object_type,
            spawn_data: value.spawn_data,
            disk_data: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
#[event_group(
    event_type(object_type),
    group(SpawnObject),
    events(CraneYard, Barracks, Factory, ResourceNode, MarineSquad, TankBase),
)]
pub struct SpawnObjects {
    pub object_type: ObjectType,
    pub spawn_data: ObjectSpawnData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
#[event_group(
    event_type(object_type),
    group(FetchObject),
    events(CraneYard, Barracks, Factory, ResourceNode, MarineSquad, TankBase),
)]
pub struct FetchObjects {
    pub object_type: ObjectType,
    pub spawn_data: ObjectSpawnData,
    pub disk_data: Option<ObjectDiskData>,
}

impl From<SpawnObjects> for FetchObjects {
    fn from(value: SpawnObjects) -> Self {
        Self {
            object_type: value.object_type,
            spawn_data: value.spawn_data,
            disk_data: None,
        }
    }
}

impl From<LoadObjects> for FetchObjects {
    fn from(value: LoadObjects) -> Self {
        Self {
            object_type: value.object_type,
            spawn_data: value.spawn_data,
            disk_data: value.disk_data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectSpawnData {
    pub snowflake: Snowflake,
    pub teamplayer: TeamPlayer,
    pub transform: Transform,
}

impl From<ObjectSpawnData> for (Snowflake, TeamPlayer, Transform) {
    fn from(value: ObjectSpawnData) -> Self {
        (value.snowflake, value.teamplayer, value.transform)
    }
}

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ObjectDiskData {
    pub health: Option<Health>,
    pub queues: Option<Queues>,
    pub path_finder: Option<PathFinder>,
    pub navigator: Option<Navigator>,
    pub weapon_set: Option<WeaponSet>,
    pub reference: Option<Reference>,
    pub squad: Option<Squad>,
    pub velocity: Option<Velocity>,
    pub resource_node: Option<ResourceNodePlatforms>
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Asset, TypePath)]
pub struct ObjectAsset {
    pub stack: Option<(ActiveQueue, StackData)>,
    pub health: Option<Health>,
    pub asset_queues: Option<AssetQueues>,
    pub economic_object: Option<EconomicObject>,
    pub asset_squad: Option<AssetSquad>,
    pub navigator: Option<Navigator>,
    pub weapon_set: Option<WeaponSet>,
    pub reference: Option<Reference>,
    pub collider_string: Option<String>,
}

#[derive(Default)]
pub struct ObjectAssetLoader;

impl AssetLoader for ObjectAssetLoader {
    type Asset = ObjectAsset;
    type Settings = ();
    type Error = LoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = ron::de::from_bytes::<ObjectAsset>(&bytes).or_else(|_| bincode::deserialize(&bytes).map(|object_asset| object_asset)).unwrap();
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

#[derive(Debug, Default, Clone)]
#[derive(Resource)]
#[derive(AssetCollection)]
pub struct ObjectAssets {
    #[asset(path = "objects/crane_yard.ron")]
    pub crane_yard: Handle<ObjectAsset>,
    #[asset(path = "objects/resource_node.ron")]
    pub resource_node: Handle<ObjectAsset>,
    #[asset(path = "objects/resource_platform_unclaimed.ron")]
    pub resource_platform_unclaimed: Handle<ObjectAsset>,
    #[asset(path = "objects/resource_platform_claimed.ron")]
    pub resource_platform_claimed: Handle<ObjectAsset>,
    #[asset(path = "objects/barracks.ron")]
    pub barracks: Handle<ObjectAsset>,
    #[asset(path = "objects/factory.ron")]
    pub factory: Handle<ObjectAsset>,
    #[asset(path = "objects/marine_squad.ron")]
    pub marine_squad: Handle<ObjectAsset>,
    #[asset(path = "objects/tank.ron")]
    pub tank: Handle<ObjectAsset>,
}

#[derive(Debug, Clone)]
#[derive(Resource)]
pub struct ObjectPrefabs {
    pub crane_yard_prefab: CraneYardPrefab,
    pub resource_node_prefab: ResourceNodePrefab,
    pub resource_platform_unclaimed_prefab: ResourcePlatformUnclaimedPrefab,
    pub resource_platform_claimed_prefab: ResourcePlatformClaimedPrefab,
    pub barracks_prefab: BarracksPrefab,
    pub factory_prefab: FactoryPrefab,
    pub marine_squad_prefab: MarineSquadPrefab,
    pub tank_prefab: TankBasePrefab,
}

impl FromWorld for ObjectPrefabs {
    fn from_world(world: &mut World) -> Self {
        println!("CREATING OBJECTPREFABS");
        let cell = world.as_unsafe_world_cell();
        let assets = unsafe {
            cell.get_resource_mut::<Assets<ObjectAsset>>().expect("Failed to get Assets<ObjectAsset>")
        };

        let object_assets = unsafe {
            cell
            .get_resource::<ObjectAssets>()
            .expect("Failed to get ObjectAssets")
        };

        let crane_yard_prefab_asset = assets.get(&object_assets.crane_yard).expect("Failed to load crane_yard");
        let resource_node_prefab_asset = assets.get(&object_assets.resource_node).expect("Failed to load resource_node");
        let resource_platform_claimed_prefab_asset = assets.get(&object_assets.resource_platform_claimed).expect("Failed to load resource_platform_claimed");
        let resource_platform_unclaimed_prefab_asset = assets.get(&object_assets.resource_platform_unclaimed).expect("Failed to load resource_platform_unclaimed");
        let barracks_prefab_asset = assets.get(&object_assets.barracks).expect("Failed to load barracks");
        let factory_prefab_asset = assets.get(&object_assets.factory).expect("Failed to load factory");
        let marine_squad_prefab_asset = assets.get(&object_assets.marine_squad).expect("Failed to load marine_squad");
        let tank_prefab_asset = assets.get(&object_assets.tank).expect("Failed to load tank");

        let mut stacks : HashMap<ObjectType, (ActiveQueue, StackData)> = HashMap::new();

        stacks.insert(ObjectType::Barracks, barracks_prefab_asset.stack.clone().unwrap());
        stacks.insert(ObjectType::Factory, factory_prefab_asset.stack.clone().unwrap());
        stacks.insert(ObjectType::MarineSquad, marine_squad_prefab_asset.stack.clone().unwrap());
        stacks.insert(ObjectType::TankBase, tank_prefab_asset.stack.clone().unwrap());

        let crane_yard_prefab = CraneYardPrefab::try_from((crane_yard_prefab_asset, &stacks)).unwrap();
        let resource_node_prefab = ResourceNodePrefab::try_from(resource_node_prefab_asset).unwrap();
        let resource_platform_claimed_prefab = ResourcePlatformClaimedPrefab::try_from(resource_platform_claimed_prefab_asset).unwrap();
        let resource_platform_unclaimed_prefab = ResourcePlatformUnclaimedPrefab::try_from(resource_platform_unclaimed_prefab_asset).unwrap();
        let barracks_prefab = BarracksPrefab::try_from((barracks_prefab_asset, &stacks)).unwrap();
        let factory_prefab = FactoryPrefab::try_from((factory_prefab_asset, &stacks)).unwrap();
        let marine_squad_prefab = MarineSquadPrefab::try_from(marine_squad_prefab_asset).unwrap();
        let tank_prefab = TankBasePrefab::try_from(tank_prefab_asset).unwrap();

        let object_prefabs = ObjectPrefabs {
            crane_yard_prefab,
            resource_node_prefab,
            resource_platform_unclaimed_prefab,
            resource_platform_claimed_prefab,
            barracks_prefab,
            factory_prefab,
            marine_squad_prefab,
            tank_prefab,
        };

        object_prefabs
    }
}

pub struct ObjectPlugin;

impl ObjectPlugin {
    pub fn patch_grid_spawn(
        mut grid_map: ResMut<GridMap>,
        pathing_space: Res<GridSpace>,
        objects: Query<(&Transform, &ObjectType), (Added<ObjectType>, With<Collider>)>,
    ) {
        let mut recompute = false;
        objects.iter().for_each(|(transform, object_type)| {
            let max = match object_type {
                ObjectType::CraneYard => { Some((8, 8)) },
                ObjectType::Factory => { Some((11, 11)) },
                ObjectType::ResourceNode => { Some((9, 9))}
                _ => { None }
            };
            if let Some((x_max, y_max)) = max {
                let mut blocks = Vec::new();
                for x_offset in -x_max..=x_max {
                    for y_offset in -y_max..=y_max {
                        let (x, y) = pathing_space.position_to_index(transform.translation.xz() + Vec2::new(x_offset as f32, y_offset as f32));
                        blocks.push((x, y));
                    }
                }
                grid_map.0.add_objects(blocks);
                recompute = true;
            }
        });
        if recompute {
            grid_map.0.precompute();
        }
    }

    pub fn patch_grid_kill(
        mut grid_map: ResMut<GridMap>,
        pathing_space: Res<GridSpace>,
        mut kills: EventReader<ObjectKilledEvent>,
        objects: Query<(&Transform, &ObjectType), With<Collider>>,
    ) {
        let mut recompute = false;
        for kill in kills.read() {
            let Ok((transform, object_type)) = objects.get(kill.0) else { continue; };
            let max = match object_type {
                ObjectType::CraneYard => { Some((8, 8)) },
                ObjectType::Factory => { Some((11, 11)) },
                ObjectType::ResourceNode => { Some((9, 9))}
                _ => { None }
            };
            if let Some((x_max, y_max)) = max {
                let mut blocks = Vec::new();
                for x_offset in -x_max..=x_max {
                    for y_offset in -y_max..=y_max {
                        let (x, y) = pathing_space.position_to_index(transform.translation.xz() + Vec2::new(x_offset as f32, y_offset as f32));
                        blocks.push((x, y));
                    }
                }
                grid_map.0.remove_objects(blocks);
                recompute = true;
            }
        }
        if recompute {
            grid_map.0.precompute();
        }
    }

    pub fn show_grid(
        grid_map: ResMut<GridMap>,
        pathing_space: Res<GridSpace>,
        mut gizmos: Gizmos,
    ) {
        for object in grid_map.0.blocks() {
            let xy = pathing_space.index_to_position(*object);
            let xyz = xy.extend(0.0).xzy();
            gizmos.line(xyz, xyz + Vec3::Y * 10.0, Srgba::GREEN);
            for object in grid_map.0.object_nodes(*object).unwrap_or(&Vec::new()) {
                let xy = pathing_space.index_to_position(*object);
                let xyz = xy.extend(0.0).xzy();
                gizmos.line(xyz, xyz + Vec3::Y * 20.0, Srgba::BLUE);
            }
        }

    }
}

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event_group::<LoadObjects>()
            .add_event_group::<SpawnObjects>()
            .add_event_group::<FetchObjects>()
            .add_event::<ObjectKilledEvent>()
            .add_plugins((
                CraneYardPlugin,
                ResourceNodePlugin,
                BarracksPlugin,
                FactoryPlugin,
                MarineSquadPlugin,
                TankPlugin
            ))
            .add_systems(Update, (Self::patch_grid_spawn, Self::patch_grid_kill, Self::show_grid))
        ;
    }
}