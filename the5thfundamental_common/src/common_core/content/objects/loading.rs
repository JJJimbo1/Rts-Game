// use bevy::{prelude::*, asset::{LoadState, HandleId}, utils::HashMap, ecs::schedule::{StateData, }};
// use bevy_asset_loader::prelude::{AssetCollection, LoadingStateAppExt, LoadingState};
// use crate::*;


// pub struct CommonLoadingPlugin<T: StateData> {
//     pub loading_state: T,
//     pub next_state: T,
// }

// impl<T: StateData + Clone> Plugin for CommonLoadingPlugin<T> {
//     fn build(&self, app: &mut App) {
//         app

//             .add_asset::<Level>()
//             .add_asset::<Map>()
//             .add_asset::<ObjectAsset>()

//             .add_asset_loader(LevelLoader)
//             .add_asset_loader(MapLoader)
//             .add_asset_loader(ObjectAssetLoader)

//             .add_loading_state(LoadingState::new(self.loading_state.clone())
//                 .with_collection::<ObjectAssets>()
//                 .init_resource::<ObjectPrefabs>()
//             )

//             .add_event::<ContentLoadEvent>()
//         ;
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub enum ContentLoadEvent {
//     Success,
//     Failure,
// }

// #[derive(Debug, Default, Clone)]
// #[derive(Resource)]
// #[derive(AssetCollection)]
// pub struct ObjectAssets {
//     #[asset(path = "objects/crane_yard.t5fobj")]
//     pub crane_yard: Handle<ObjectAsset>,
//     #[asset(path = "objects/resource_node.t5fobj")]
//     pub resource_node: Handle<ObjectAsset>,
//     #[asset(path = "objects/resource_platform_unclaimed.t5fobj")]
//     pub resource_platform_unclaimed: Handle<ObjectAsset>,
//     #[asset(path = "objects/resource_platform_claimed.t5fobj")]
//     pub resource_platform_claimed: Handle<ObjectAsset>,
//     #[asset(path = "objects/factory.t5fobj")]
//     pub factory: Handle<ObjectAsset>,
//     #[asset(path = "objects/marine_squad.t5fobj")]
//     pub marine_squad: Handle<ObjectAsset>,
//     #[asset(path = "objects/tank.t5fobj")]
//     pub tank: Handle<ObjectAsset>,
// }

// #[derive(Clone)]
// #[derive(Resource)]
// pub struct ObjectPrefabs {
//     pub crane_yard_prefab: CraneYardPrefab,
//     pub resource_node_prefab: ResourceNodePrefab,
//     pub resource_platform_unclaimed_prefab: ResourcePlatformUnclaimedPrefab,
//     pub resource_platform_claimed_prefab: ResourcePlatformClaimedPrefab,
//     pub factory_prefab: FactoryPrefab,
//     pub marine_squad_prefab: MarineSquadPrefab,
//     pub tank_prefab: TankBasePrefab,
// }

// impl FromWorld for ObjectPrefabs {
//     fn from_world(world: &mut World) -> Self {
//         let cell = world.cell();
//         let objects = cell
//             .get_resource_mut::<Assets<ObjectAsset>>()
//             .expect("Failed to get Assets<ObjectAssets>");
//         let object_assets = cell
//             .get_resource::<ObjectAssets>()
//             .expect("Failed to get ObjectAssets");

//         let crane_yard_prefab_asset = objects.get(&object_assets.crane_yard).expect("Failed to load crane_yard");
//         let resource_node_prefab_asset = objects.get(&object_assets.resource_node).expect("Failed to load resource_node");
//         let resource_platform_unclaimed_prefab_asset = objects.get(&object_assets.resource_platform_unclaimed).expect("Failed to load resource_platform_unclaimed");
//         let resource_platform_claimed_prefab_asset = objects.get(&object_assets.resource_platform_claimed).expect("Failed to load resource_platform_claimed");
//         let factory_prefab_asset = objects.get(&object_assets.factory).expect("Failed to load factory");
//         let marine_squad_prefab_asset = objects.get(&object_assets.marine_squad).expect("Failed to load marine_squad");
//         let tank_prefab_asset = objects.get(&object_assets.tank).expect("Failed to load tank");

//         let mut stacks : HashMap<ObjectType, (ActiveQueue, StackData)> = HashMap::new();

//         stacks.insert(ObjectType::ResourceNode, resource_node_prefab_asset.stack.unwrap());
//         stacks.insert(ObjectType::Factory, factory_prefab_asset.stack.unwrap());
//         stacks.insert(ObjectType::MarineSquad, marine_squad_prefab_asset.stack.unwrap());
//         stacks.insert(ObjectType::TankBase, tank_prefab_asset.stack.unwrap());

//         let crane_yard_prefab = CraneYardPrefab::try_from((crane_yard_prefab_asset, &stacks)).unwrap();
//         let resource_node_prefab = ResourceNodePrefab::try_from(resource_node_prefab_asset).unwrap();
//         let resource_platform_unclaimed_prefab = ResourcePlatformUnclaimedPrefab::try_from(resource_platform_unclaimed_prefab_asset).unwrap();
//         let resource_platform_claimed_prefab = ResourcePlatformClaimedPrefab::try_from(resource_platform_claimed_prefab_asset).unwrap();
//         let factory_prefab = FactoryPrefab::try_from((factory_prefab_asset, &stacks)).unwrap();
//         let marine_squad_prefab = MarineSquadPrefab::try_from(marine_squad_prefab_asset).unwrap();
//         let tank_prefab = TankBasePrefab::try_from(tank_prefab_asset).unwrap();

//         let object_prefabs = ObjectPrefabs {
//             crane_yard_prefab,
//             resource_node_prefab,
//             resource_platform_unclaimed_prefab,
//             resource_platform_claimed_prefab,
//             factory_prefab,
//             marine_squad_prefab,
//             tank_prefab,
//         };

//         object_prefabs
//     }
// }

// pub fn load_assets(
//     asset_server: Res<AssetServer>,
//     mut commands: Commands,
// ) {
//     let mut object_assets = ObjectAssets::default();

//     object_assets.crane_yard = asset_server.load("objects/crane_yard.t5fobj");
//     object_assets.resource_node = asset_server.load("objects/resource_node.t5fobj");
//     object_assets.resource_platform_unclaimed = asset_server.load("objects/resource_platform_unclaimed.t5fobj");
//     object_assets.resource_platform_claimed = asset_server.load("objects/resource_platform_claimed.t5fobj");
//     object_assets.factory = asset_server.load("objects/factory.t5fobj");
//     object_assets.marine_squad = asset_server.load("objects/marine_squad.t5fobj");
//     object_assets.tank = asset_server.load("objects/tank.t5fobj");

//     object_assets.handle_ids = Some([
//         object_assets.crane_yard.id(),
//         object_assets.resource_node.id(),
//         object_assets.resource_platform_unclaimed.id(),
//         object_assets.resource_platform_claimed.id(),
//         object_assets.factory.id(),
//         object_assets.marine_squad.id(),
//         object_assets.tank.id()
//     ]);

//     commands.insert_resource(object_assets);
// }

// pub fn convert_assets_to_prefabs(
//     mut stop: Local<bool>,
//     mut content_load_events: EventWriter<ContentLoadEvent>,
//     object_assets: Res<ObjectAssets>,
//     asset_server: Res<AssetServer>,
//     object_prefab_assets: Res<Assets<ObjectAsset>>,
//     mut commands: Commands
// ) {
//     if *stop { return; }
//     if asset_server.get_group_load_state(object_assets.handle_ids.unwrap()) == LoadState::Failed { content_load_events.send(ContentLoadEvent::Failure); *stop = true; }

//     let Some(crane_yard_prefab_asset) = object_prefab_assets.get(&object_assets.crane_yard) else { return; };
//     let Some(resource_node_prefab_asset) = object_prefab_assets.get(&object_assets.resource_node) else { return; };
//     let Some(resource_platform_unclaimed_prefab_asset) = object_prefab_assets.get(&object_assets.resource_platform_unclaimed) else { return; };
//     let Some(resource_platform_claimed_prefab_asset) = object_prefab_assets.get(&object_assets.resource_platform_claimed) else { return; };
//     let Some(factory_prefab_asset) = object_prefab_assets.get(&object_assets.factory) else { return; };
//     let Some(marine_squad_prefab_asset) = object_prefab_assets.get(&object_assets.marine_squad) else { return; };
//     let Some(tank_prefab_asset) = object_prefab_assets.get(&object_assets.tank) else { return; };

//     let mut stacks : HashMap<ObjectType, (ActiveQueue, StackData)> = HashMap::new();

//     stacks.insert(ObjectType::ResourceNode, resource_node_prefab_asset.stack.unwrap());
//     stacks.insert(ObjectType::Factory, factory_prefab_asset.stack.unwrap());
//     stacks.insert(ObjectType::MarineSquad, marine_squad_prefab_asset.stack.unwrap());
//     stacks.insert(ObjectType::TankBase, tank_prefab_asset.stack.unwrap());

//     let crane_yard_prefab = CraneYardPrefab::try_from((crane_yard_prefab_asset, &stacks)).unwrap();
//     let resource_node_prefab = ResourceNodePrefab::try_from(resource_node_prefab_asset).unwrap();
//     let resource_platform_unclaimed_prefab = ResourcePlatformUnclaimedPrefab::try_from(resource_platform_unclaimed_prefab_asset).unwrap();
//     let resource_platform_claimed_prefab = ResourcePlatformClaimedPrefab::try_from(resource_platform_claimed_prefab_asset).unwrap();
//     let factory_prefab = FactoryPrefab::try_from((factory_prefab_asset, &stacks)).unwrap();
//     let marine_squad_prefab = MarineSquadPrefab::try_from(marine_squad_prefab_asset).unwrap();
//     let tank_prefab = TankBasePrefab::try_from(tank_prefab_asset).unwrap();

//     let object_prefabs = ObjectPrefabs {
//         crane_yard_prefab,
//         resource_node_prefab,
//         resource_platform_unclaimed_prefab,
//         resource_platform_claimed_prefab,
//         factory_prefab,
//         marine_squad_prefab,
//         tank_prefab,
//     };

//     commands.insert_resource(object_prefabs);
//     content_load_events.send(ContentLoadEvent::Success);
//     *stop = true;

// }