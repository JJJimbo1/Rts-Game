
use bevy::prelude::*;
use the5thfundamental_common::*;
use crate::{*, utility::assets::GltfAsset};



pub fn client_object_spawn(
    mut asset_server: ResMut<AssetServer>,
    mut identifiers: ResMut<Identifiers>,
    assets: Query<(Entity, &Snowflake, &AssetType), Added<AssetType>>,
    mut commands: Commands,
) {

    assets.for_each(|(entity, snowflake, asset)| {
        let scene = asset_server.load(GltfAsset::from(*asset));
        commands.entity(entity).with_children(|parent| {
            parent.spawn(
                SceneBundle {
                    scene,
                    ..default()
                }
            );
        });
    });

    // maps.for_each(|(entity, snowflake, object)| {
    //     if let Some(gltf) = object.id().and_then(|id| gltf_assets.get(id)).and_then(|handle| gltfs.get(&handle.0.clone())) {
    //         commands.entity(entity).with_children(|parent| {
    //             parent.spawn(
    //                 SceneBundle {
    //                     scene: gltf.scenes[0].clone(),
    //                     ..default()
    //                 }
    //             );
    //         });
    //     }

    //     identifiers.insert(*snowflake, entity);
    // });

    // objects.for_each(|(entity, snowflake, object)| {
    //     // println!("{:?}", object);
    //     if let Some(gltf) = object.id().and_then(|id| gltf_assets.get(id)).and_then(|handle| gltfs.get(&handle.0.clone())) {
    //         // println!("Spawning: {:?}", object);
    //         commands.entity(entity).with_children(|parent| {
    //             parent.spawn(
    //                 SceneBundle {
    //                     scene: gltf.scenes[0].clone(),
    //                     ..default()
    //                 }
    //             );
    //         });
    //     }
    //     identifiers.insert(*snowflake, entity);
    //     // commands.
    // });
}