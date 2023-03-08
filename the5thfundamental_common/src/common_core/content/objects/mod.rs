pub mod crane_yard;
pub mod factory;
pub mod marine_squad;
pub mod marine;
pub mod resource_node;
pub mod resource_platform_unclaimed;
pub mod resource_platform_claimed;
pub mod tank;

use bevy_asset_loader::prelude::AssetCollection;
pub use crane_yard::*;
pub use factory::*;
pub use marine_squad::*;
pub use marine::*;
pub use resource_node::*;
pub use resource_platform_unclaimed::*;
pub use resource_platform_claimed::*;
pub use tank::*;

use std::fmt::Display;
use bevy::{prelude::*, math::Vec3Swizzles, reflect::TypeUuid, asset::{AssetLoader, LoadedAsset}, utils::HashMap, ecs::schedule::StateData};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]

pub enum ObjectType {
    CraneYard,
    ResourceNode,
    ResourcePlatformUnclaimed,
    ResourcePlatformClaimed,
    Factory,
    MarineSquad,
    Marine,
    TankBase,
    TankGun,
}

impl From<ObjectType> for AssetType {
    fn from(object_type: ObjectType) -> Self {
        Self::Object(object_type)
    }
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::CraneYard => write!(f, "Crane Yard"),
            ObjectType::ResourceNode => write!(f, "Resource Node"),
            ObjectType::ResourcePlatformUnclaimed => write!(f, "Resource Platform Unclaimed"),
            ObjectType::ResourcePlatformClaimed => write!(f, "Resource Platform"),
            ObjectType::Factory => write!(f, "Factory"),
            ObjectType::MarineSquad => write!(f, "Marine Squad"),
            ObjectType::Marine => write!(f, "Marine"),
            ObjectType::TankBase => write!(f, "Tank"),
            ObjectType::TankGun => write!(f, "Tank Gun"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(SystemLabel)]
pub struct SpawnObjectSystem;

#[derive(Debug, Clone, Copy)]
pub struct ObjectSpawnEventData {
    pub object_type: ObjectType,
    pub snowflake: Snowflake,
    pub teamplayer: TeamPlayer,
    pub transform: Transform,
}

impl From<ObjectSpawnEventData> for (Snowflake, TeamPlayer, Transform) {
    fn from(value: ObjectSpawnEventData) -> Self {
        (value.snowflake, value.teamplayer, value.transform)
    }
}

//TODO: Maybe make this separate for each object?
#[derive(Debug, Clone)]
pub struct ObjectSpawnEvent(pub ObjectSpawnEventData);

// #[derive(Debug, Clone)]
// pub enum SerdeObjectSpawnEvent {
//     CraneYard(SerdeCraneYard),
//     Factory(SerdeFactory),
//     ResourceNode(SerdeResourceNode),
//     MarineSquad(SerdeMarineSquad),
//     Tank(SerdeTank),
// }


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(TypeUuid)]
#[uuid = "ad37501b-8697-4e9f-adc2-1d4ace71b4b0"]
pub struct ObjectAsset {
    pub stack: Option<(ActiveQueue, StackData)>,
    pub health: Option<Health>,
    pub prefab_queues: Option<PrefabQueues>,
    pub economic_object: Option<EconomicObject>,
    pub prefab_squad: Option<PrefabSquad>,
    pub controller: Option<Controller>,
    pub weapon_set: Option<WeaponSet>,
    pub turret: Option<Turret>,
    pub collider_string: Option<String>,
}

pub struct ObjectAssetLoader;

impl AssetLoader for ObjectAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            match ron::de::from_bytes::<ObjectAsset>(bytes) {
                Ok(asset) => {
                    info!("{:?}", asset);

                },
                Err(e) => {
                    info!("{}", e);
                    error!("{}", e);
                    error!("{}", bytes.len());
                }
            }
            let custom_asset = ron::de::from_bytes::<ObjectAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["t5fobj"]
    }
}

#[derive(Debug, Default, Clone)]
#[derive(Resource)]
#[derive(AssetCollection)]
pub struct ObjectAssets {
    #[asset(path = "objects/crane_yard.t5fobj")]
    pub crane_yard: Handle<ObjectAsset>,
    #[asset(path = "objects/resource_node.t5fobj")]
    pub resource_node: Handle<ObjectAsset>,
    #[asset(path = "objects/resource_platform_unclaimed.t5fobj")]
    pub resource_platform_unclaimed: Handle<ObjectAsset>,
    #[asset(path = "objects/resource_platform_claimed.t5fobj")]
    pub resource_platform_claimed: Handle<ObjectAsset>,
    #[asset(path = "objects/factory.t5fobj")]
    pub factory: Handle<ObjectAsset>,
    #[asset(path = "objects/marine_squad.t5fobj")]
    pub marine_squad: Handle<ObjectAsset>,
    #[asset(path = "objects/tank.t5fobj")]
    pub tank: Handle<ObjectAsset>,
}

#[derive(Clone)]
#[derive(Resource)]
pub struct ObjectPrefabs {
    pub crane_yard_prefab: CraneYardPrefab,
    pub resource_node_prefab: ResourceNodePrefab,
    pub resource_platform_unclaimed_prefab: ResourcePlatformUnclaimedPrefab,
    pub resource_platform_claimed_prefab: ResourcePlatformClaimedPrefab,
    pub factory_prefab: FactoryPrefab,
    pub marine_squad_prefab: MarineSquadPrefab,
    pub tank_prefab: TankBasePrefab,
}

impl FromWorld for ObjectPrefabs {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let objects = cell
            .get_resource_mut::<Assets<ObjectAsset>>()
            .expect("Failed to get Assets<ObjectAssets>");
        let object_assets = cell
            .get_resource::<ObjectAssets>()
            .expect("Failed to get ObjectAssets");

        let crane_yard_prefab_asset = objects.get(&object_assets.crane_yard).expect("Failed to load crane_yard");
        let resource_node_prefab_asset = objects.get(&object_assets.resource_node).expect("Failed to load resource_node");
        let resource_platform_unclaimed_prefab_asset = objects.get(&object_assets.resource_platform_unclaimed).expect("Failed to load resource_platform_unclaimed");
        let resource_platform_claimed_prefab_asset = objects.get(&object_assets.resource_platform_claimed).expect("Failed to load resource_platform_claimed");
        let factory_prefab_asset = objects.get(&object_assets.factory).expect("Failed to load factory");
        let marine_squad_prefab_asset = objects.get(&object_assets.marine_squad).expect("Failed to load marine_squad");
        let tank_prefab_asset = objects.get(&object_assets.tank).expect("Failed to load tank");

        let mut stacks : HashMap<ObjectType, (ActiveQueue, StackData)> = HashMap::new();

        // stacks.insert(ObjectType::ResourceNode, resource_node_prefab_asset.stack.unwrap());
        stacks.insert(ObjectType::Factory, factory_prefab_asset.stack.unwrap());
        stacks.insert(ObjectType::MarineSquad, marine_squad_prefab_asset.stack.unwrap());
        stacks.insert(ObjectType::TankBase, tank_prefab_asset.stack.unwrap());

        let crane_yard_prefab = CraneYardPrefab::try_from((crane_yard_prefab_asset, &stacks)).unwrap();
        let resource_node_prefab = ResourceNodePrefab::try_from(resource_node_prefab_asset).unwrap();
        let resource_platform_unclaimed_prefab = ResourcePlatformUnclaimedPrefab::try_from(resource_platform_unclaimed_prefab_asset).unwrap();
        let resource_platform_claimed_prefab = ResourcePlatformClaimedPrefab::try_from(resource_platform_claimed_prefab_asset).unwrap();
        let factory_prefab = FactoryPrefab::try_from((factory_prefab_asset, &stacks)).unwrap();
        let marine_squad_prefab = MarineSquadPrefab::try_from(marine_squad_prefab_asset).unwrap();
        let tank_prefab = TankBasePrefab::try_from(tank_prefab_asset).unwrap();

        let object_prefabs = ObjectPrefabs {
            crane_yard_prefab,
            resource_node_prefab,
            resource_platform_unclaimed_prefab,
            resource_platform_claimed_prefab,
            factory_prefab,
            marine_squad_prefab,
            tank_prefab,
        };

        info!("ObjectPrefabs");
        object_prefabs
    }
}

pub struct ObjectPlugin<T: StateData> {
    state: T,
}

impl<T: StateData> ObjectPlugin<T> {
    pub fn spawn_standard_objects(
        mut spawn_events: EventReader<ObjectSpawnEvent>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut commands: Commands,
    ) {
        for event in spawn_events.iter() {
            let mut entity = None;
            match event.0.object_type {
                ObjectType::CraneYard => { entity = Some(commands.spawn( CraneYardBundle::from(prefabs.crane_yard_prefab.clone()).with_spawn_data(event.0.clone())).id()); }
                ObjectType::ResourceNode => { entity = Some(commands.spawn( ResourceNodeBundle::from(prefabs.resource_node_prefab.clone()).with_spawn_data(event.0.clone())).id()); }
                ObjectType::ResourcePlatformUnclaimed => { /*entity = Some(commands.spawn( ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_spawn_data(event.0)).id())*/ }
                ObjectType::ResourcePlatformClaimed => { /*entity = Some(commands.spawn( ResourcePlatformClaimedBundle::from(prefabs.resource_platform_claimed_prefab.clone()).with_spawn_data(event.0)).id());*/ }
                ObjectType::Factory => { entity = Some(commands.spawn( FactoryBundle::from(prefabs.factory_prefab.clone()).with_spawn_data(event.0.clone())).id()); }
                ObjectType::MarineSquad => { /*entity = Some(commands.spawn( MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.0)).id());*/ },
                ObjectType::Marine => { /*entity = Some(commands.spawn( MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.0)).id());*/ }
                ObjectType::TankBase => { /*entity = Some(commands.spawn( TankBaseBundle::from(prefabs.tank_prefab.clone()).with_spawn_data(event.0)).id());*/ },
                ObjectType::TankGun => { }
                // _ => { }
            }
            if let Some(x) = entity {
                identifiers.insert(event.0.snowflake, x);
            }
        }
    }
    
    pub fn patch_grid_map(
        mut grid_map: ResMut<OGrid>,
        pathing_space: Res<GridSpace>,
    
        objects: Query<(&Transform, &ObjectType), Added<ObjectType>>,
    ) {
        let mut recompute = false;
        objects.for_each(|(transform, object_type)| {
            let max = match *object_type {
                ObjectType::CraneYard => { Some((8, 8)) },
                // ObjectType::Factory => { Some((11, 11)) },
                ObjectType::ResourceNode => { Some((9, 9))}
                _ => { None }
            };
            if let Some((x_max, y_max)) = max {
                for x_offset in -x_max..=x_max {
                    for y_offset in -y_max..=y_max {
                        let (x, y) = pathing_space.position_to_index(transform.translation.xz() + Vec2::new(x_offset as f32, y_offset as f32)); if let Some(x) = grid_map.0.get_cell_mut(x, y) {x.blocked = true}
                    }
                }
                recompute = true;
            }
        });
        if recompute {
            grid_map.0.precompute();
        }
    }
}

impl<T: StateData> Plugin for ObjectPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(self.state.clone())
            .with_system(Self::spawn_standard_objects)
            .with_system(Self::patch_grid_map)
        );
    }
}