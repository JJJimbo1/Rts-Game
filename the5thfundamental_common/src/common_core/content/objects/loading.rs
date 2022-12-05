use bevy::{prelude::*, asset::{LoadState, HandleId}, utils::HashMap, ecs::schedule::{StateData, }};
use crate::*;


pub struct CommonLoadingPlugin<T: StateData> {
    pub state: T,
}

impl<T: StateData + Clone> Plugin for CommonLoadingPlugin<T> {
    fn build(&self, app: &mut App) {
        app

            .add_asset::<Level>()
            .add_asset::<Map>()
            .add_asset::<ObjectAsset>()

            .add_asset_loader(LevelLoader)
            .add_asset_loader(MapLoader)
            .add_asset_loader(ObjectAssetLoader)

            .add_event::<ContentLoadEvent>()

            .add_system_set(SystemSet::on_enter(self.state.clone())
                .with_system(load_assets)
            )
            .add_system_set(SystemSet::on_update(self.state.clone())
                .with_system(convert_assets_to_prefabs)
            )
        ;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContentLoadEvent {
    Success,
    Failure,
}

// #[derive(Debug, Default, Clone)]
// #[derive(Resource)]
// pub struct MapAssets {
//     pub developer: Handle<MapPrefab>,

//     pub handle_ids: Option<[HandleId; 1]>,
// }

#[derive(Debug, Default, Clone)]
#[derive(Resource)]
pub struct ObjectAssets {
    pub crane_yard: Handle<ObjectAsset>,
    pub factory: Handle<ObjectAsset>,
    pub marine_squad: Handle<ObjectAsset>,
    pub resource_node: Handle<ObjectAsset>,
    pub resource_platform_unclaimed: Handle<ObjectAsset>,
    pub resource_platform_claimed: Handle<ObjectAsset>,
    pub tank: Handle<ObjectAsset>,

    pub handle_ids: Option<[HandleId; 7]>,
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let mut object_assets = ObjectAssets::default();

    object_assets.crane_yard = asset_server.load("objects/crane_yard.t5fobj");
    object_assets.resource_node = asset_server.load("objects/resource_node.t5fobj");
    object_assets.resource_platform_unclaimed = asset_server.load("objects/resource_platform_unclaimed.t5fobj");
    object_assets.resource_platform_claimed = asset_server.load("objects/resource_platform_claimed.t5fobj");
    object_assets.factory = asset_server.load("objects/factory.t5fobj");
    object_assets.marine_squad = asset_server.load("objects/marine_squad.t5fobj");
    object_assets.tank = asset_server.load("objects/tank.t5fobj");

    object_assets.handle_ids = Some([
        object_assets.crane_yard.id(),
        object_assets.resource_node.id(),
        object_assets.resource_platform_unclaimed.id(),
        object_assets.resource_platform_claimed.id(),
        object_assets.factory.id(),
        object_assets.marine_squad.id(),
        object_assets.tank.id()
    ]);

    commands.insert_resource(object_assets);
}

pub fn convert_assets_to_prefabs(
    mut stop: Local<bool>,
    mut content_load_events: EventWriter<ContentLoadEvent>,
    object_assets: Res<ObjectAssets>,
    asset_server: Res<AssetServer>,
    object_prefab_assets: Res<Assets<ObjectAsset>>,
    mut commands: Commands
) {
    if *stop { return; }
    if asset_server.get_group_load_state(object_assets.handle_ids.unwrap()) == LoadState::Failed { content_load_events.send(ContentLoadEvent::Failure); *stop = true; }

    let Some(crane_yard_prefab_asset) = object_prefab_assets.get(&object_assets.crane_yard) else { return; };
    let Some(resource_node_prefab_asset) = object_prefab_assets.get(&object_assets.resource_node) else { return; };
    let Some(resource_platform_unclaimed_prefab_asset) = object_prefab_assets.get(&object_assets.resource_platform_unclaimed) else { return; };
    let Some(resource_platform_claimed_prefab_asset) = object_prefab_assets.get(&object_assets.resource_platform_claimed) else { return; };
    let Some(factory_prefab_asset) = object_prefab_assets.get(&object_assets.factory) else { return; };
    let Some(marine_squad_prefab_asset) = object_prefab_assets.get(&object_assets.marine_squad) else { return; };
    let Some(tank_prefab_asset) = object_prefab_assets.get(&object_assets.tank) else { return; };

    let mut stacks : HashMap<ObjectType, (ActiveQueue, StackData)> = HashMap::new();

    stacks.insert(ObjectType::ResourceNode, resource_node_prefab_asset.stack.unwrap());
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

    commands.insert_resource(object_prefabs);
    content_load_events.send(ContentLoadEvent::Success);
    *stop = true;

}