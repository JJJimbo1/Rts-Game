pub mod crane_yard;
pub mod factory;
pub mod marine_squad;
pub mod marine;
pub mod resource_node;
pub mod resource_platform_unclaimed;
pub mod resource_platform_claimed;
// pub mod tank_base;
// pub mod tank_gun;
pub mod tank;

pub use crane_yard::*;
pub use factory::*;
pub use marine_squad::*;
pub use marine::*;
pub use resource_node::*;
pub use resource_platform_unclaimed::*;
pub use resource_platform_claimed::*;
// pub use tank_base::*;
// pub use tank_gun::*;
pub use tank::*;

use bevy_pathfinding::{d2::GridMap, GridSpace, OGrid};
use bevy_rapier3d::prelude::Collider;
use bevy::{prelude::*, utils::HashMap, math::Vec3Swizzles};
use serde::{Serialize, Deserialize};
use crate::*;

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
            Self::Factory => { Some("factory") },
            Self::MarineSquad => { None },
            Self::Marine => { Some("marine") },
            Self::ResourceNode => { Some("resource_node") },
            Self::ResourcePlatformUnclaimed => { Some("resource_platform_unclaimed") },
            Self::ResourcePlatformClaimed => { Some("resource_platform_claimed") },
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

// impl ObjectPrefabs {
//     pub unsafe fn get_bundle(&self, object_type: ObjectType, spawn_data: ObjectSpawnEventData) -> Box<dyn Bundle> {
//         match object_type {
//             ObjectType::CraneYard => { Box::new(CraneYardBundle::from(self.crane_yard_prefab).with_spawn_data(spawn_data)) },
//             ObjectType::Factory => { Box::new(FactoryBundle::from(self.factory_prefab).with_spawn_data(spawn_data)) },
//             ObjectType::MarineSquad => { Box::new(MarineSquadBundle::from(self.marine_squad_prefab).with_spawn_data(spawn_data)) },
//             ObjectType::Marine => { Box::new(MarineBundle::default().with_spawn_data(spawn_data)) },
//             ObjectType::ResourceNode => { Box::new(ResourceNodeBundle::from(self.resource_node_prefab).with_spawn_data(spawn_data)) },
//             ObjectType::ResourcePlatformUnclaimed => { Box::new(ResourcePlatformUnclaimedBundle::from(self.resource_platform_unclaimed_prefab).with_spawn_data(spawn_data)) },
//             ObjectType::ResourcePlatformClaimed => { Box::new(ResourcePlatformClaimedBundle::from(self.resource_platform_claimed_prefab).with_spawn_data(spawn_data)) },
//             ObjectType::Tank => { Box::new(TankBundle::from(self.tank_prefab).with_spawn_data(spawn_data)) }
//         }
//     }
// }

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
    let tank_prefab : TankBasePrefab = load_from_file(format!("{}tank.ron", objects)).unwrap();

    // stacks.insert(ObjectType::CraneYard, crane_yard_prefab.stack);
    stacks.insert(ObjectType::Factory, factory_prefab.stack);
    stacks.insert(ObjectType::MarineSquad, marine_squad_prefab.stack);
    stacks.insert(ObjectType::ResourceNode, resource_node_prefab.stack);
    stacks.insert(ObjectType::TankBase, tank_prefab.stack);

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

impl From<SerdeCraneYard> for ObjectSpawnEvent {
    fn from(value: SerdeCraneYard) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::CraneYard,
            snowflake: Snowflake::new(),
            teamplayer: value.team_player,
            transform: value.transform.into(),
        })
    }
}

impl From<SerdeResourceNode> for ObjectSpawnEvent {
    fn from(value: SerdeResourceNode) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::ResourceNode,
            snowflake: Snowflake::new(),
            teamplayer: value.team_player,
            transform: value.transform.into(),
        })
    }
}

impl From<SerdeFactory> for ObjectSpawnEvent {
    fn from(value: SerdeFactory) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::Factory,
            snowflake: Snowflake::new(),
            teamplayer: value.team_player,
            transform: value.transform.into(),
        })
    }
}

impl From<SerdeMarineSquad> for ObjectSpawnEvent {
    fn from(value: SerdeMarineSquad) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::MarineSquad,
            snowflake: Snowflake::new(),
            teamplayer: value.team_player,
            transform: value.transform.into(),
        })
    }
}

impl From<SerdeTank> for ObjectSpawnEvent {
    fn from(value: SerdeTank) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::TankBase,
            snowflake: Snowflake::new(),
            teamplayer: value.team_player,
            transform: value.transform.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum SerdeObjectSpawnEvent {
    CraneYard(SerdeCraneYard),
    Factory(SerdeFactory),
    ResourceNode(SerdeResourceNode),
    MarineSquad(SerdeMarineSquad),
    Tank(SerdeTank),
}

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
            ObjectType::Factory => { entity = Some(commands.spawn( FactoryBundle::from(prefabs.factory_prefab.clone()).with_spawn_data(event.0.clone())).id()); }
            ObjectType::MarineSquad => { /*entity = Some(commands.spawn( MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.0)).id());*/ },
            ObjectType::Marine => { /*entity = Some(commands.spawn( MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.0)).id());*/ }
            ObjectType::ResourceNode => { entity = Some(commands.spawn( ResourceNodeBundle::from(prefabs.resource_node_prefab.clone()).with_spawn_data(event.0.clone())).id()); }
            ObjectType::ResourcePlatformUnclaimed => { /*entity = Some(commands.spawn( ResourcePlatformUnclaimedBundle::from(prefabs.resource_platform_unclaimed_prefab.clone()).with_spawn_data(event.0)).id())*/ }
            ObjectType::ResourcePlatformClaimed => { /*entity = Some(commands.spawn( ResourcePlatformClaimedBundle::from(prefabs.resource_platform_claimed_prefab.clone()).with_spawn_data(event.0)).id());*/ }
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


