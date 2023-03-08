
use bevy::prelude::*;
use the5thfundamental_common::*;
use crate::{*, utility::assets::GltfAssets};



pub fn client_object_spawn(
    gltf_assets: Res<GltfAssets>,
    // identifiers: Res<Identifiers>,
    assets: Query<(Entity, &Snowflake, &AssetType), Added<AssetType>>,
    mut commands: Commands,
) {

    assets.for_each(|(entity, _snowflake, asset)| {
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