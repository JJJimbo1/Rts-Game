pub mod level;
pub mod map;
pub mod object;
pub mod error;

use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt}};
pub use level::*;
pub use map::*;
pub use object::*;
pub use error::*;

use bevy::prelude::*;

use crate::{DiskPlugin, GameState};

pub static BASE_LABEL: &'static str = "base";

pub struct BaseLoadingPlugin;

impl Plugin for BaseLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<LevelAsset>()
            .init_asset::<MapAsset>()
            .init_asset::<ObjectAsset>()

            .init_asset_loader::<LevelLoader>()
            .init_asset_loader::<MapAssetLoader>()
            .init_asset_loader::<ObjectAssetLoader>()

            .add_loading_state(LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Loading)
                .load_collection::<MapAssets>()
                .load_collection::<ObjectAssets>()
                .finally_init_resource::<ObjectPrefabs>()
            )
        ;
    }
}


pub struct BasePlugins;

impl PluginGroup for BasePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = bevy::app::PluginGroupBuilder::start::<BasePlugins>();
        let group = group
            .add(BaseLoadingPlugin)
            .add(LevelPlugin)
            .add(MapPlugin)
            .add(ObjectPlugin)
            .add(DiskPlugin)
            .add(BaseClientPlugin);

        group
    }
}


#[derive(Debug, Default, Clone)]
#[derive(Resource)]
#[derive(AssetCollection)]
pub struct GltfAssets {
    #[asset(path = "models/developer.glb#Scene0")]
    pub developer: Handle<Scene>,
    #[asset(path = "models/crane_yard.glb#Scene0")]
    pub crane_yard: Handle<Scene>,
    #[asset(path = "models/resource_node.glb#Scene0")]
    pub resource_node: Handle<Scene>,
    #[asset(path = "models/resource_platform_unclaimed.glb#Scene0")]
    pub resource_platform_unclaimed: Handle<Scene>,
    #[asset(path = "models/resource_platform_claimed.glb#Scene0")]
    pub resource_platform_claimed: Handle<Scene>,
    #[asset(path = "models/barracks.glb#Scene0")]
    pub barracks: Handle<Scene>,
    #[asset(path = "models/factory.glb#Scene0")]
    pub factory: Handle<Scene>,
    #[asset(path = "models/marine.glb#Scene0")]
    pub marine: Handle<Scene>,
    #[asset(path = "models/tank_base.glb#Scene0")]
    pub tank_base: Handle<Scene>,
    #[asset(path = "models/tank_gun.glb#Scene0")]
    pub tank_gun: Handle<Scene>

}

impl GltfAssets {
    pub fn get_map(&self, map_type: MapType) -> Option<&Handle<Scene>>  {
        match map_type {
            MapType::Developer => Some(&self.developer),
            // _ => None,
        }
    }
    pub fn get_object(&self, object_type: ObjectType) -> Option<&Handle<Scene>> {
        match object_type {
            ObjectType::CraneYard => Some(&self.crane_yard),
            ObjectType::ResourceNode => Some(&self.resource_node),
            ObjectType::ResourcePlatformUnclaimed => Some(&self.resource_platform_unclaimed),
            ObjectType::ResourcePlatformClaimed => Some(&self.resource_platform_claimed),
            ObjectType::Barracks => Some(&self.barracks),
            ObjectType::Factory => Some(&self.factory),
            ObjectType::Marine => Some(&self.marine),
            ObjectType::TankBase => Some(&self.tank_base),
            ObjectType::TankGun => Some(&self.tank_gun),
            _ => None
        }
    }
}

pub struct BaseClientPlugin;

impl BaseClientPlugin {
    pub fn client_spawn(
        gltf_assets: Res<GltfAssets>,
        maps: Query<(Entity, &MapType), Added<MapType>>,
        objects: Query<(Entity, &ObjectType), Added<ObjectType>>,
        mut commands: Commands,
    ) {
        maps.iter().for_each(|(entity, map)| {
            let Some(scene) = gltf_assets.get_map(*map) else { return; };
            commands.entity(entity).with_children(|parent| {
                parent.spawn(
                    SceneRoot(scene.clone())
                );
            });
        });
        objects.iter().for_each(|(entity, object)| {
            let Some(scene) = gltf_assets.get_object(*object) else { return; };
            commands.entity(entity).with_children(|parent| {
                parent.spawn(
                    SceneRoot(scene.clone())
                );
            });
        });
    }
}

impl Plugin for BaseClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        println!("BASE CLIENT PLUGIN");
        app
            .add_loading_state(LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Loading)
                .load_collection::<GltfAssets>()
            )


        .add_systems(Update, Self::client_spawn.run_if(resource_exists::<GltfAssets>));
        // .add_systems(Update, Self::client_object_spawn);
    }
}