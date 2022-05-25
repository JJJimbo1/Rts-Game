
use bevy::{ecs::system::EntityCommands, gltf::{Gltf, GltfMesh}, math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity, AsyncCollider, MassProperties};
use bevy_pathfinding::{PathFinder, Path};
use qloader::QLoader;
use snowflake::ProcessUniqueId;
use the5thfundamental_common::*;
use crate::*;



// pub fn client_map_spawn(
//     gltf_assets : Res<QLoader<GltfAsset, AssetServer>>,
//     gltfs : Res<Assets<Gltf>>,
//     // maps: Res<QLoader<Map, ()>>,
//     query: Query<(Entity, &MapType), Added<MapType>>,
//     mut commands: Commands,
// ) {
//     query.for_each(|(entity, map)| {
//         if let Some(gltf) = gltf_assets.get(map.id()).and_then(|handle| gltfs.get(handle.0.clone())) {
//             commands.entity(entity).with_children(|parent| {
//                 parent.spawn_scene(gltf.scenes[0].clone());
//             });
//         }
        


//     });

// }




// pub fn object_spawn_system_set() -> SystemSet {
//     SystemSet::on_update(GameState::SingleplayerGame).after(SpawnObjectSystem)
//         .with_system(client_object_spawn)
//         // .with_system(client_spawn_object::<ResourceNode>)
//         // .with_system(client_spawn_object::<Factory>)
//         // .with_system(client_spawn_object::<Tank>)
// }

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


// pub fn object_spawn_system(
//     // master_queue : Res<MasterQueue>,
//     // mut init_requests : ResMut<InitRequests>,

//     gltf_assets : Res<QLoader<GltfAsset, AssetServer>>,
//     gltfs : Res<Assets<Gltf>>,
//     gltf_meshes : Res<Assets<GltfMesh>>,
//     meshes: Res<Assets<Mesh>>,

//     mut identifiers : ResMut<Identifiers>,
//     mut actors : ResMut<Actors>,
//     mut commands : Commands,

//     // weapons : &mut Vec<(ProcessUniqueId, Weapons)>,
// ) {
//     for r in init_requests.requests.iter() {
//         match r.0 {
//             ObjectType::Building => {
//                 // info!("{}", r.1); 
//                 if let Some(prefab) = master_queue.building_prefabs.get(&r.1) {
//                     let trans = r.2.transform;

//                     let save_data = SaveObject {
//                         otype : ObjectType::Building,
//                         prefab : LimitedBuffer::from(prefab.0.id.clone()),
//                     };

//                     let sf = SnowFlake(ProcessUniqueId::new());

//                     // id_converter.insert(prefab.id, sf);

//                     let mut entity_commands = commands.spawn();

//                     let entity = entity_commands.id();
//                     entity_commands
//                         .insert(save_data)
//                         .insert(sf)
//                         .insert(trans)
//                         .insert(r.2.team_player)
//                         .insert(Selectable {
//                             selected : false,
//                             context : SelectableContext::Single
//                         });
//                         // .insert(Immobile::default());

//                     actors.assign_building(r.2.team_player, sf);
//                     identifiers.insert(sf, entity);

//                     match_tags(&gltf_assets, &gltfs, &gltf_meshes, &meshes, sf, trans, &prefab.0, &master_queue, entity_commands);
//                 }
//             },
//             ObjectType::Unit => {
//                 // info!("{}", r.1);
//                 if let Some(prefab) = master_queue.unit_prefabs.get(&r.1) {
//                     let trans = if let Some(t) = r.2.spawn_point {
//                         r.2.transform.mul_transform(Transform::from_xyz(t.0,t.1,t.2))
//                     } else {
//                         r.2.transform
//                     };

//                     let save_data = SaveObject {
//                         otype : ObjectType::Unit,
//                         prefab : LimitedBuffer::from(prefab.0.id.clone()),
//                     };

//                     let sf = SnowFlake(ProcessUniqueId::new());

//                     // id_converter.insert(prefab.id, sf);

//                     let mut entity_commands = commands.spawn();

//                     let entity = entity_commands.id();
//                     entity_commands
//                         .insert(save_data)
//                         .insert(sf)
//                         .insert(trans)
//                         .insert(r.2.team_player)
//                         .insert(Selectable {
//                             selected : false,
//                             context : SelectableContext::MultiSelect
//                         })
//                         .insert(MassProperties { mass : 1.0, ..default()})
//                         .insert(Velocity::default());

//                     // println!("{:?}", r.2.end_point);

//                     if let Some(x) = r.2.end_point {
//                         let end = trans.mul_vec3(Vec3::new(x.0, x.1, x.2)).xz();
//                         let pf = PathFinder::default();
//                         let mut path = Path::default();
//                         path.0 = Some(vec![trans.translation.xz(), end]);


//                         entity_commands.insert(pf).insert(path);
//                     }

//                     // actors.assign_building(r.2.team_player, sf);
//                     identifiers.insert(sf, entity);

//                     match_tags(&gltf_assets, &gltfs, &gltf_meshes, &meshes, sf, trans, &prefab.0, &master_queue, entity_commands);
//                 }
//             }
//         }
//     }
//     for r in init_requests.with_entities_requests.iter() {
//         match r.0 {
//             ObjectType::Building => {
//                 // info!("{}", r.1);
//                 if let Some(prefab) = master_queue.building_prefabs.get(&r.1) {
//                     let trans = r.2.transform;

//                     let save_data = SaveObject {
//                         otype : ObjectType::Building,
//                         prefab : LimitedBuffer::from(prefab.0.id.clone()),
//                     };

//                     let sf = SnowFlake(ProcessUniqueId::new());
//                     let mut entity_commands = commands.entity(r.3);
//                     entity_commands
//                         .insert(save_data)
//                         .insert(sf)
//                         .insert(trans)
//                         .insert(r.2.team_player)
//                         .insert(Selectable {
//                             selected : false,
//                             context : SelectableContext::Single
//                         });
//                         // .insert(Immobile::default());

//                     actors.assign_building(r.2.team_player, sf);
//                     identifiers.insert(sf, r.3);

//                     match_tags(&gltf_assets, &gltfs, &gltf_meshes, &meshes, sf, trans, &prefab.0, &master_queue, entity_commands);
//                 }
//             },
//             ObjectType::Unit => {
//                 // info!("{}", r.1);
//                 if let Some(prefab) = master_queue.unit_prefabs.get(&r.1) {
//                     let trans = if let Some(t) = r.2.spawn_point {
//                         r.2.transform.mul_transform(Transform::from_xyz(t.0,t.1,t.2))
//                     } else {
//                         r.2.transform
//                     };

//                     let save_data = SaveObject {
//                         otype : ObjectType::Unit,
//                         prefab : LimitedBuffer::from(prefab.0.id.clone()),
//                     };

//                     let sf = SnowFlake(ProcessUniqueId::new());
//                     let mut entity_commands = commands.entity(r.3);
//                     entity_commands
//                         .insert(save_data)
//                         .insert(sf)
//                         .insert(trans)
//                         .insert(r.2.team_player)
//                         .insert(Selectable {
//                             selected : false,
//                             context : SelectableContext::MultiSelect
//                         })
//                         .insert(Velocity::default());

//                     actors.assign_building(r.2.team_player, sf);
//                     identifiers.insert(sf, r.3);


//                     if let Some(x) = r.2.end_point {
//                         let end = trans.mul_vec3(Vec3::new(x.0, x.1, x.2)).xz();
//                         let pf = PathFinder::default();
//                         let mut path = Path::default();
//                         path.0 = Some(vec![trans.translation.xz(), end]);
//                         entity_commands
//                             .insert(pf)
//                             .insert(path);
//                     }

//                     match_tags(&gltf_assets, &gltfs, &gltf_meshes, &meshes, sf, trans, &prefab.0, &master_queue, entity_commands);
//                 }
//             }
//         }
//     }
//     init_requests.requests.clear();
//     init_requests.with_entities_requests.clear();
// }

//TODO: This has got to go.

pub fn clear_buffer(
    mut spawn_event_writer: EventWriter<ObjectSpawnEvent>,
    // mut requests : ResMut<InitRequests>,
    mut current_placement : ResMut<CurrentPlacement<CLICK_BUFFER>>,
    mut queues : Query<&mut Queues>,
    mut commands: Commands,
) {
    if let (Some(e), Some(d)) = (current_placement.constructor, current_placement.data.clone()) {
        if let Ok(mut q) = queues.get_mut(e) {
            if let Some(x) = q.structures_queue.as_mut() {
                let mut to_remove : Option<usize> = None;
                for i in 0..x.data.buffer.len() {
                    if x.data.buffer[i] == d {
                        match current_placement.status {
                            PlacementStatus::Completed(e) => {
                                spawn_event_writer.send(ObjectSpawnEvent(current_placement.spawn_data.unwrap()));
                                // requests.request(ObjectType::Building, current_placement.data.clone().unwrap().id.clone(), current_placement.spawn_data.clone().unwrap(), Some(e));
                                to_remove = Some(i);
                                current_placement.constructor = None;
                                current_placement.data = None;
                                current_placement.spawn_data = None;
                                current_placement.entity = None;
                                current_placement.status = PlacementStatus::Idle;
                                commands.entity(e).despawn();
                            },
                            _ => { }
                        }
                    }
                }
                if let Some(i) = to_remove {
                    x.data.buffer.remove(i);
                }
            }
            else {
                // println!("3");
            }
        }
        else {
            // println!("2");
        }
    }
    else {
        // println!("1");
    }
}

// pub fn match_tags(
//     gltf_assets : &QLoader<GltfAsset, AssetServer>,
//     gltfs : &Assets<Gltf>,
//     gltf_meshes : &Assets<GltfMesh>,
//     meshes : &Assets<Mesh>,

//     _sf : SnowFlake,
//     trans : Transform,
//     prefab : &GameObject,
//     master_queue : &MasterQueue,
//     mut entity_commands : EntityCommands,
// ) {
//     let (mesh, material) = {
//         let m1 = gltf_assets.get(&prefab.id).clone();
//         let m2 = gltfs.get(m1.unwrap().0.clone());
//         let m3 = gltf_meshes.get(m2.unwrap().meshes[0].clone());
//         let m4 = m3.unwrap().primitives[0].clone();
//         (m4.mesh, m4.material.unwrap())
//     };

//     let pbr = PbrBundle {
//         mesh: mesh.clone(),
//         material,
//         transform : trans,
//         ..Default::default()
//     };

//     entity_commands.insert_bundle(pbr);

//     if let Some(m) = meshes.get(mesh) {
//         entity_commands.insert(RigidBody::KinematicVelocityBased);
//         entity_commands.insert(Collider::bevy_mesh(m).unwrap());
//     }

//     if let Some(x) = prefab.mobility {
//         let mut x = x;
//         x.follow = true;
//         entity_commands.insert(x);
//     }

//     if let Some(x) = prefab.economy {
//         entity_commands.insert(ResourceProvider { strength : x.resource_gen });
//     }
//     if let Some(x) = &prefab.weapons {
//         entity_commands.insert(x.clone());
//     }

//     if let Some(queue) = master_queue.get(&prefab.id) {
//         entity_commands.insert(queue.clone());
//     }

//     if let Some(x) = prefab.health {
//         entity_commands.insert(x);
//     }

//     // entity_commands.insert(PathFinder::default());
// }