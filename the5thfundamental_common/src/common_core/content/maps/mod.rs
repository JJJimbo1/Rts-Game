pub mod developer;

use bevy_rapier3d::prelude::Collider;
pub use developer::*;

use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{load_from_file, Manifest, AssetId, AssetType, decode};

#[derive(Clone)]
pub struct MapPrefabs {
    pub developer_prefab: DeveloperPrefab,
    // pub direction_prefab: DirectionPrefab,
    // pub sandbox_prefab: SandboxPrefab,
}

pub fn load_map_prefabs(
    manifest : Res<Manifest>,
    mut commands : Commands,
) {

    let root = std::env::current_dir().unwrap();
    let maps = format!("{}{}", root.as_path().display(), manifest.maps_path);

    // let mut stacks : HashMap<MapType, (ActiveQueue, StackData)> = HashMap::new();

    let mut developer_prefab : DeveloperPrefab = load_from_file(format!("{}developer.ron", maps)).unwrap();
    // let direction_prefab : ResourceNodePrefab = load_from_file(format!("{}direction.ron", objects)).unwrap();
    // let sandbox_prefab : FactoryPrefab = load_from_file(format!("{}sandbox.ron", objects)).unwrap();

    developer_prefab.real_collider = decode(developer_prefab.collider_string.clone()).map_or(None, |(v, i)| Some(Collider::trimesh(v, i)));


    let prefabs = MapPrefabs {
        developer_prefab,
        // direction_prefab,
        // sandbox_prefab,
    };

    commands.insert_resource(prefabs);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]

pub enum MapType {
    Developer,
    // Direction,
    // Sandbox,
}

impl From<MapType> for AssetType {
    fn from(map_type: MapType) -> Self {
        Self::Map(map_type)
    }
}

impl AssetId for MapType {
    fn id(&self) -> &'static str {
        match self {
            Self::Developer => { "developer" },
            // Self::Direction => { "direction" },
            // Self::Sandbox => { "sandbox" },
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum SerdeMap {
    Developer(SerdeDeveloper),
}

// #[derive(Debug, Clone, Copy)]
// #[derive(Serialize, Deserialize)]
// pub struct SerdeMap {
//     map_type: MapType,
//     map: SerdeMapType,
// }

impl AssetId for SerdeMap {
    fn id(&self) -> &'static str {
        match self {
            Self::Developer(_) => { "developer" },
            // Self::Direction => { "direction" },
            // Self::Sandbox => { "sandbox" },
        }
    }
}

// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
// #[derive(SystemLabel)]
// pub struct MapObjectSystem;

// #[derive(Debug, Clone, Copy)]
// pub struct MapSpawnEventData {
//     pub map_type: MapType,
// }

// #[derive(Debug, Clone, Copy)]
// pub struct MapSpawnEvent(pub MapSpawnEventData);

// pub fn spawn_map(
//     mut spawn_events: EventReader<MapSpawnEvent>,
//     prefabs: Res<MapPrefabs>,
//     // mut identifiers: ResMut<Identifiers>,
//     mut commands: Commands,
// ) {
//     for event in spawn_events.iter() {
//         let entity;
//         match event.0.map_type {
//             MapType::Developer => { entity = commands.spawn_bundle(DeveloperBundle::from(prefabs.developer_prefab.clone())).id(); }
//             // MapType::Direction => { entity = commands.spawn_bundle(ResourceNodeBundle::from(prefabs.resource_node_prefab.clone())).id(); }
//             // MapType::Sandbox => { entity = commands.spawn_bundle(FactoryBundle::from(prefabs.factory_prefab.clone())).id(); }
//         }
//     }
// }

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct MapBounds(pub Vec2);

impl Default for MapBounds {
    fn default() -> Self {
        Self(Vec2::new(1000.0, 1000.0))
    }
}
