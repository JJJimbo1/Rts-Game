pub mod crane_yard;
pub mod factory;
pub mod marine_squad;
pub mod marine;
pub mod resource_node;
pub mod resource_platform_unclaimed;
pub mod resource_platform_claimed;
pub mod tank;

pub use crane_yard::*;
pub use factory::*;
pub use marine_squad::*;
pub use marine::*;
pub use resource_node::*;
pub use resource_platform_unclaimed::*;
pub use resource_platform_claimed::*;
pub use tank::*;

use bevy_pathfinding::{d2::GridMap, GridSpace};
use bevy_rapier3d::prelude::Collider;
use bevy::{prelude::*, utils::HashMap, math::Vec3Swizzles};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Clone)]
pub struct ObjectPrefabs {
    pub crane_yard_prefab: CraneYardPrefab,
    pub factory_prefab: FactoryPrefab,
    pub marine_squad_prefab: MarineSquadPrefab,
    pub resource_node_prefab: ResourceNodePrefab,
    pub resource_platform_unclaimed_prefab: ResourcePlatformUnclaimedPrefab,
    pub resource_platform_claimed_prefab: ResourcePlatformClaimedPrefab,
    pub tank_prefab: TankPrefab,
}

pub fn load_object_prefabs(
    manifest : Res<Manifest>,
    mut commands : Commands,
) {

    let root = std::env::current_dir().unwrap();
    let objects = format!("{}{}", root.as_path().display(), manifest.objects_path);

    let mut stacks : HashMap<ObjectType, (ActiveQueue, StackData)> = HashMap::new();

    let crane_yard_prefab : CraneYardPrefab = load_from_file(format!("{}crane_yard.ron", objects)).unwrap();
    let factory_prefab : FactoryPrefab = load_from_file(format!("{}factory.ron", objects)).unwrap();
    let marine_squad_prefab : MarineSquadPrefab = load_from_file(format!("{}marine_squad.ron", objects)).unwrap();
    let resource_node_prefab : ResourceNodePrefab = load_from_file(format!("{}resource_node.ron", objects)).unwrap();
    let resource_platform_unclaimed_prefab : ResourcePlatformUnclaimedPrefab = load_from_file(format!("{}resource_platform_unclaimed.ron", objects)).unwrap();
    let resource_platform_claimed_prefab : ResourcePlatformClaimedPrefab = load_from_file(format!("{}resource_platform_claimed.ron", objects)).unwrap();
    let tank_prefab : TankPrefab = load_from_file(format!("{}tank.ron", objects)).unwrap();

    // stacks.insert(ObjectType::CraneYard, crane_yard_prefab.stack);
    stacks.insert(ObjectType::Factory, factory_prefab.stack);
    stacks.insert(ObjectType::MarineSquad, marine_squad_prefab.stack);
    stacks.insert(ObjectType::ResourceNode, resource_node_prefab.stack);
    stacks.insert(ObjectType::Tank, tank_prefab.stack);

    let crane_yard_collider = decode(crane_yard_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i))).unwrap();
    let factory_collider = decode(factory_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i))).unwrap();
    let marine_squad_collider = decode(marine_squad_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i))).unwrap();
    let resource_node_collider = decode(resource_node_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i))).unwrap();
    let resource_platform_unclaimed_collider = decode(resource_platform_unclaimed_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i))).unwrap();
    let resource_platform_claimed_collider = decode(resource_platform_claimed_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i))).unwrap();
    let tank_collider = decode(tank_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i))).unwrap();

    let prefabs = ObjectPrefabs {
        crane_yard_prefab: crane_yard_prefab.with_real_queues(&stacks).with_real_collider(crane_yard_collider),
        factory_prefab: factory_prefab.with_real_queues(&stacks).with_real_collider(factory_collider),
        marine_squad_prefab: marine_squad_prefab.with_real_collider(marine_squad_collider),
        resource_node_prefab: resource_node_prefab.with_real_collider(resource_node_collider),
        resource_platform_unclaimed_prefab: resource_platform_unclaimed_prefab.with_real_collider(resource_platform_unclaimed_collider),
        resource_platform_claimed_prefab: resource_platform_claimed_prefab.with_real_collider(resource_platform_claimed_collider),
        tank_prefab: tank_prefab.with_real_collider(tank_collider),
    };

    commands.insert_resource(prefabs);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]

pub enum ObjectType {
    CraneYard,
    Factory,
    MarineSquad,
    Marine,
    ResourceNode,
    ResourcePlatformUnclaimed,
    ResourcePlatformClaimed,
    Tank,
}

impl From<ObjectType> for AssetType {
    fn from(object_type: ObjectType) -> Self {
        Self::Object(object_type)
    }
}

impl AssetId for ObjectType {
    fn id(&self) -> &'static str {
        match self {
            Self::CraneYard => { "crane_yard" },
            Self::Factory => { "factory" },
            Self::MarineSquad => { "marine_squad" },
            Self::Marine => { "marine" },
            Self::ResourceNode => { "resource_node" },
            Self::ResourcePlatformUnclaimed => { "resource_platform_unclaimed" },
            Self::ResourcePlatformClaimed => { "resource_platform_claimed" },
            Self::Tank => { "tank" }
        }
    }
}

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Squad {
    max_members: u8,
    members: u8,
    member_ids: Vec<Entity>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(SystemLabel)]
pub struct SpawnObjectSystem;

#[derive(Debug, Clone, Copy)]
pub struct ObjectSpawnEventData {
    pub snowflake: Snowflake,
    pub object_type: ObjectType,
    pub team_player: TeamPlayer,
    pub transform: Transform,
}

//TODO: Maybe make this separate for each object?
#[derive(Debug, Clone, Copy)]
pub struct ObjectSpawnEvent(pub ObjectSpawnEventData);

pub fn spawn_standard_objects(
    mut spawn_events: EventReader<ObjectSpawnEvent>,
    prefabs: Res<ObjectPrefabs>,
    mut identifiers: ResMut<Identifiers>,
    mut commands: Commands,
) {
    for event in spawn_events.iter() {
        let mut entity = None;
        match event.0.object_type {
            ObjectType::CraneYard => { entity = Some(commands.spawn_bundle( CraneYardBundle::from(prefabs.crane_yard_prefab.clone()).with_spawn_data(event.0)).id()); }
            ObjectType::Factory => { entity = Some(commands.spawn_bundle( FactoryBundle::from(prefabs.factory_prefab.clone()).with_spawn_data(event.0)).id()); }
            ObjectType::MarineSquad => { entity = Some(commands.spawn_bundle( MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.0)).id()); },
            ObjectType::Marine => { /*entity = Some(commands.spawn_bundle( MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.0)).id());*/ }
            ObjectType::ResourceNode => { entity = Some(commands.spawn_bundle( ResourceNodeBundle::from(prefabs.resource_node_prefab.clone()).with_spawn_data(event.0)).id()); }
            ObjectType::ResourcePlatformUnclaimed => { /*entity = Some(commands.spawn_bundle( ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_spawn_data(event.0)).id())*/ }
            ObjectType::ResourcePlatformClaimed => { /*entity = Some(commands.spawn_bundle( ResourcePlatformClaimedBundle::from(prefabs.resource_platform_claimed_prefab.clone()).with_spawn_data(event.0)).id());*/ }
            ObjectType::Tank => { entity = Some(commands.spawn_bundle( TankBundle::from(prefabs.tank_prefab.clone()).with_spawn_data(event.0)).id()); },
            // _ => { }
        }
        if let Some(x) = entity {
            identifiers.insert(event.0.snowflake, x);
        }
    }
}

pub fn patch_grid_map(
    mut grid_map: ResMut<GridMap>,
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
                    let (x, y) = pathing_space.position_to_index(transform.translation.xz() + Vec2::new(x_offset as f32, y_offset as f32)); if let Some(x) = grid_map.get_cell_mut(x, y) {x.blocked = true}
                }
            }
            recompute = true;
        }
    });
    if recompute {
        grid_map.precompute();
    }
}


