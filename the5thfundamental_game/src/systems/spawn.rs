
use bevy::{gltf::Gltf, prelude::*};
use bevy_rapier3d::prelude::Collider;
use qloader::QLoader;
use the5thfundamental_common::*;
use crate::*;



pub fn client_object_spawn(
    gltf_assets: Res<QLoader<GltfAsset, AssetServer>>,
    gltfs: Res<Assets<Gltf>>,
    mut identifiers: ResMut<Identifiers>,
    maps: Query<(Entity, &Snowflake, &MapType), (Added<MapType>, With<Collider>)>,
    objects: Query<(Entity, &Snowflake, &ObjectType), Added<ObjectType>>,
    mut commands: Commands,
) {
    maps.for_each(|(entity, snowflake, object)| {
        if let Some(gltf) = gltf_assets.get(object.id()).and_then(|handle| gltfs.get(handle.0.clone())) {
            commands.entity(entity).with_children(|parent| {
                parent.spawn_scene(gltf.scenes[0].clone());
            });
        }

        identifiers.insert(*snowflake, entity);
    });

    objects.for_each(|(entity, snowflake, object)| {
        if let Some(gltf) = gltf_assets.get(object.id()).and_then(|handle| gltfs.get(handle.0.clone())) {
            commands.entity(entity).with_children(|parent| {
                parent.spawn_scene(gltf.scenes[0].clone());
            });
        }
        identifiers.insert(*snowflake, entity);
        // commands.
    });
}