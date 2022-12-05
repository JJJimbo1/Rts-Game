pub mod loading;
pub mod crane_yard;
pub mod factory;
pub mod marine_squad;
pub mod marine;
pub mod resource_node;
pub mod resource_platform_unclaimed;
pub mod resource_platform_claimed;
pub mod tank;

pub use loading::*;
pub use crane_yard::*;
pub use factory::*;
pub use marine_squad::*;
pub use marine::*;
pub use resource_node::*;
pub use resource_platform_unclaimed::*;
pub use resource_platform_claimed::*;
pub use tank::*;

use bevy_rapier3d::prelude::Collider;
use bevy::{prelude::*, utils::HashMap, math::Vec3Swizzles, reflect::TypeUuid, asset::{AssetLoader, LoadedAsset}};
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

impl AssetId for ObjectType {
    fn id(&self) -> Option<&'static str> {
        match self {
            Self::CraneYard => { Some("crane_yard") },
            Self::ResourceNode => { Some("resource_node") },
            Self::ResourcePlatformUnclaimed => { Some("resource_platform_unclaimed") },
            Self::ResourcePlatformClaimed => { Some("resource_platform_claimed") },
            Self::Factory => { Some("factory") },
            Self::MarineSquad => { None },
            Self::Marine => { Some("marine") },
            Self::TankBase => { Some("tank_base") },
            Self::TankGun => { Some("tank_gun") },
        }
    }
}

#[derive(Clone)]
#[derive(Resource)]
pub struct ObjectPrefabs {
    pub crane_yard_prefab: CraneYardPrefab,
    pub factory_prefab: FactoryPrefab,
    pub marine_squad_prefab: MarineSquadPrefab,
    pub resource_node_prefab: ResourceNodePrefab,
    pub resource_platform_unclaimed_prefab: ResourcePlatformUnclaimedPrefab,
    pub resource_platform_claimed_prefab: ResourcePlatformClaimedPrefab,
    pub tank_prefab: TankBasePrefab,
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

//TODO - impl from Serdes for SerdeObjectType


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
        // println!("patch");
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
            match ron::de::from_bytes::<Map>(bytes) {
                Ok(asset) => {

                },
                Err(e) => {
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
