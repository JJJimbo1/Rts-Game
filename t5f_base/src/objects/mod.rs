pub mod crane_yard;
pub mod factory;
pub mod marine_squad;
pub mod resource_node;
pub mod tank;
pub mod object_plugin;

use bevy_asset_loader::prelude::AssetCollection;
use bevy_rapier3d::prelude::Velocity;
pub use crane_yard::*;
pub use factory::*;
pub use marine_squad::*;
pub use resource_node::*;
pub use tank::*;
pub use object_plugin::*;

use std::{fmt::Display, marker::PhantomData};
use bevy::{prelude::*, asset::{AssetLoader, io::Reader, AsyncReadExt}, utils::HashMap, reflect::{TypePath, TypeUuid}};
use serde::{Serialize, Deserialize};
use t5f_common::*;
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

impl TryFrom<String> for ObjectType {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "crane_yard" => { return Ok(ObjectType::CraneYard); },
            "resource_node" => { return Ok(ObjectType::ResourceNode); },
            "resource_platform_claimed" => { return Ok(ObjectType::ResourcePlatformClaimed); },
            "resource_platform_unclaimed" => { return Ok(ObjectType::ResourcePlatformUnclaimed); },
            "factory" => { return Ok(ObjectType::Factory); },
            "marine_squad" => { return Ok(ObjectType::MarineSquad); },
            "marine" => { return Ok(ObjectType::Marine); },
            "tank_base" => { return Ok(ObjectType::TankBase); },
            "tank_gun" => { return Ok(ObjectType::TankGun); },
            _ => { }
        }
        Err("'Tryfrom<String> for ObjectType' did not work".to_owned())
    }
}

impl From<ObjectType> for String {
    fn from(value: ObjectType) -> Self {
        match value {
            ObjectType::CraneYard => { "crane_yard".to_owned() },
            ObjectType::ResourceNode => { "resource_node".to_owned() },
            ObjectType::ResourcePlatformClaimed => { "resource_platform_claimed".to_owned() },
            ObjectType::ResourcePlatformUnclaimed => { "resource_platform_unclaimed".to_owned() },
            ObjectType::Factory => { "factory".to_owned() },
            ObjectType::MarineSquad => { "marine_squad".to_owned() },
            ObjectType::Marine => { "marine".to_owned() },
            ObjectType::TankBase => { "tank_base".to_owned() },
            ObjectType::TankGun => { "tank_gun".to_owned() },
        }
    }
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

pub trait ObjectMarker { }

#[derive(Debug, Clone)]
pub struct AnyObjectMarker;

impl ObjectMarker for AnyObjectMarker { }

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Event)]
pub struct ObjectLoadEvent<M: ObjectMarker>(pub ObjectSpawnEventData, pub PhantomData<M>);

impl<M: ObjectMarker> ObjectLoadEvent<M> {
    pub fn spawn_data(&self) -> &ObjectSpawnData {
        &self.0.spawn_data
    }

    pub fn serde_data(&self) -> &Option<ObjectSerdeData> {
        &self.0.serde_data
    }
}

//TODO: Maybe make this separate for each object?
#[derive(Debug, Clone)]
#[derive(Event)]
pub struct ObjectSpawnEvent<T: ObjectMarker>(pub ObjectSpawnEventData, pub PhantomData<T>);

impl<T: ObjectMarker> ObjectSpawnEvent<T> {
    pub fn spawn_data(&self) -> &ObjectSpawnData {
        &self.0.spawn_data
    }

    pub fn serde_data(&self) -> &Option<ObjectSerdeData> {
        &self.0.serde_data
    }
}


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ObjectSpawnEventData {
    pub object_type: ObjectType,
    pub spawn_data: ObjectSpawnData,
    pub serde_data: Option<ObjectSerdeData>,
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
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
pub struct ObjectSerdeData {
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
#[derive(Asset, TypePath, TypeUuid)]
#[uuid = "ad37501b-8697-4e9f-adc2-1d4ace71b4b0"]
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

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<ObjectAsset, LoaderError>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let asset = ron::de::from_bytes::<ObjectAsset>(&bytes)?;//.or_else(|_| bincode::deserialize(&bytes))?;
            // println!("{:?}", asset);
            Ok(asset)
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
        println!("CREATING OBJECTPREFABS");
        let cell = world.cell();
        let assets = cell.get_resource_mut::<Assets<ObjectAsset>>().expect("Failed to get Assets<ObjectAsset>");

        let object_assets = cell
            .get_resource::<ObjectAssets>()
            .expect("Failed to get ObjectAssets");

        let crane_yard_prefab_asset = assets.get(&object_assets.crane_yard).expect("Failed to load crane_yard");
        let resource_node_prefab_asset = assets.get(&object_assets.resource_node).expect("Failed to load resource_node");
        let resource_platform_claimed_prefab_asset = assets.get(&object_assets.resource_platform_claimed).expect("Failed to load resource_platform_claimed");
        let resource_platform_unclaimed_prefab_asset = assets.get(&object_assets.resource_platform_unclaimed).expect("Failed to load resource_platform_unclaimed");
        let factory_prefab_asset = assets.get(&object_assets.factory).expect("Failed to load factory");
        let marine_squad_prefab_asset = assets.get(&object_assets.marine_squad).expect("Failed to load marine_squad");
        let tank_prefab_asset = assets.get(&object_assets.tank).expect("Failed to load tank");

        let mut stacks : HashMap<String, (ActiveQueue, StackData)> = HashMap::new();

        stacks.insert(ObjectType::Factory.into(), factory_prefab_asset.stack.clone().unwrap());
        stacks.insert(ObjectType::MarineSquad.into(), marine_squad_prefab_asset.stack.clone().unwrap());
        stacks.insert(ObjectType::TankBase.into(), tank_prefab_asset.stack.clone().unwrap());

        let crane_yard_prefab = CraneYardPrefab::try_from((crane_yard_prefab_asset, &stacks)).unwrap();
        let resource_node_prefab = ResourceNodePrefab::try_from(resource_node_prefab_asset).unwrap();
        let resource_platform_claimed_prefab = ResourcePlatformClaimedPrefab::try_from(resource_platform_claimed_prefab_asset).unwrap();
        let resource_platform_unclaimed_prefab = ResourcePlatformUnclaimedPrefab::try_from(resource_platform_unclaimed_prefab_asset).unwrap();
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

        object_prefabs
    }
}