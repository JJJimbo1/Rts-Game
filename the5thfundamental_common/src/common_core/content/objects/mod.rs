pub mod crane_yard;
pub mod resource_node;
pub mod factory;
pub mod tank;

use bevy_rapier3d::prelude::Collider;
pub use crane_yard::*;
pub use resource_node::*;
pub use factory::*;
pub use tank::*;

use bevy::{prelude::*, utils::HashMap};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Clone)]
pub struct ObjectPrefabs {
    pub crane_yard_prefab: CraneYardPrefab,
    pub resource_node_prefab: ResourceNodePrefab,
    pub factory_prefab: FactoryPrefab,
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
    let resource_node_prefab : ResourceNodePrefab = load_from_file(format!("{}resource_node.ron", objects)).unwrap();
    let factory_prefab : FactoryPrefab = load_from_file(format!("{}factory.ron", objects)).unwrap();
    let tank_prefab : TankPrefab = load_from_file(format!("{}tank.ron", objects)).unwrap();

    // stacks.insert(ObjectType::CraneYard, crane_yard_prefab.stack);
    stacks.insert(ObjectType::ResourceNode, resource_node_prefab.stack);
    stacks.insert(ObjectType::Factory, factory_prefab.stack);
    stacks.insert(ObjectType::Tank, tank_prefab.stack);

    let crane_yard_collider = decode(crane_yard_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i))).unwrap();
    let resource_node_collider = decode(resource_node_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i))).unwrap();
    let factory_collider = decode(factory_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i))).unwrap();
    let tank_collider = decode(tank_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i))).unwrap();

    let prefabs = ObjectPrefabs {
        crane_yard_prefab: crane_yard_prefab.with_real_queues(&stacks).with_real_collider(crane_yard_collider),
        resource_node_prefab: resource_node_prefab.with_real_collider(resource_node_collider),
        factory_prefab: factory_prefab.with_real_queues(&stacks).with_real_collider(factory_collider),
        tank_prefab: tank_prefab.with_real_collider(tank_collider),
    };

    commands.insert_resource(prefabs);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]

pub enum ObjectType {
    CraneYard,
    ResourceNode,
    Factory,
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
            Self::ResourceNode => { "resource_node" },
            Self::Factory => { "factory" },
            Self::Tank => { "tank" }
        }
    }
}

// #[derive(Debug, Clone)]
// #[derive(Serialize, Deserialize)]
// pub enum ObjectPrefab {
//     CraneYard(CraneYardPrefab),
//     ResourceNode(ResourceNodePrefab),
//     Factory(FactoryPrefab),
//     Tank(TankPrefab),
// }

// #[derive(Debug, Clone)]
// #[derive(Serialize, Deserialize)]
// pub enum SerdeObject {
//     CraneYard(SerdeCraneYard),
//     ResourceNode(SerdeResourceNode),
//     Factory(SerdeFactory),
//     Tank(SerdeTank),
// }

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(SystemLabel)]
pub struct SpawnObjectSystem;

#[derive(Debug, Clone, Copy)]
pub struct ObjectSpawnEventData {
    pub snow_flake: Snowflake,
    pub object_type: ObjectType,
    pub team_player: TeamPlayer,
    pub transform: Transform,
}

#[derive(Debug, Clone, Copy)]
pub struct ObjectSpawnEvent(pub ObjectSpawnEventData);

pub fn spawn_object(
    mut spawn_events: EventReader<ObjectSpawnEvent>,
    prefabs: Res<ObjectPrefabs>,
    mut identifiers: ResMut<Identifiers>,
    mut commands: Commands,
) {
    for event in spawn_events.iter() {
        let entity;
        match event.0.object_type {
            ObjectType::CraneYard => { entity = Some(commands.spawn_bundle( CraneYardBundle::from(prefabs.crane_yard_prefab.clone()).with_spawn_data(event.0)).id()); }
            ObjectType::ResourceNode => { entity = Some(commands.spawn_bundle( ResourceNodeBundle::from(prefabs.resource_node_prefab.clone()).with_spawn_data(event.0)).id()); }
            ObjectType::Factory => { entity = Some(commands.spawn_bundle( FactoryBundle::from(prefabs.factory_prefab.clone()).with_spawn_data(event.0)).id()); }
            ObjectType::Tank => { entity = Some(commands.spawn_bundle( TankBundle::from(prefabs.tank_prefab.clone()).with_spawn_data(event.0)).id()); }
        }
        identifiers.insert(event.0.snow_flake, entity.unwrap());
    }
}


