use bevy::prelude::*;
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{LoadingStateAppExt, LoadingState, config::ConfigureLoadingState}};
use t5f_common::GameState;
use crate::{AssetType, MapType, ObjectType};


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
    pub fn get_scene(&self, asset_type: AssetType) -> Option<&Handle<Scene>> {
        match asset_type {
            AssetType::Map(map) => {
                match map {
                    MapType::Developer => Some(&self.developer),
                    // _ => None,
                }
            },
            AssetType::Object(object) => {
                match object {
                    ObjectType::CraneYard => Some(&self.crane_yard),
                    ObjectType::ResourceNode => Some(&self.resource_node),
                    ObjectType::ResourcePlatformUnclaimed => Some(&self.resource_platform_unclaimed),
                    ObjectType::ResourcePlatformClaimed => Some(&self.resource_platform_claimed),
                    ObjectType::Factory => Some(&self.factory),
                    ObjectType::Marine => Some(&self.marine),
                    ObjectType::TankBase => Some(&self.tank_base),
                    ObjectType::TankGun => Some(&self.tank_gun),
                    _ => None
                }
            }
        }
    }
}

pub struct BaseClientPlugin;

impl BaseClientPlugin {
    pub fn client_object_spawn(
        gltf_assets: Res<GltfAssets>,
        assets: Query<(Entity, &AssetType), Added<AssetType>>,
        mut commands: Commands,
    ) {
        assets.for_each(|(entity, asset)| {
            let Some(scene) = gltf_assets.get_scene(*asset) else { return; };
            commands.entity(entity).with_children(|parent| {
                parent.spawn(
                    SceneBundle {
                        scene: scene.clone(),
                        ..default()
                    }
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


        .add_systems(Update, Self::client_object_spawn.run_if(resource_exists::<GltfAssets>()));
    }
}