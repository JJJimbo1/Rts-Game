
use bevy::{ecs::system::EntityCommands, gltf::{Gltf, GltfMesh}, math::Vec3Swizzles, prelude::*};
use bimap::BiMap;
use bevy_pathfinding::{PathFinder, Path};
use qloader::QLoader;
use snowflake::ProcessUniqueId;
use the5thfundamental_common::*;
use crate::*;






pub fn object_spawn_system(
    master_queue : Res<MasterQueue>,
    mut init_requests : ResMut<InitRequests>,

    gltf_assets : Res<QLoader<GltfAsset, AssetServer>>,
    gltfs : Res<Assets<Gltf>>,
    gltf_meshes : Res<Assets<GltfMesh>>,

    mut identifiers : ResMut<Identifiers>,
    mut actors : ResMut<Actors>,
    mut commands : Commands,

    // weapons : &mut Vec<(ProcessUniqueId, Weapons)>,
) {
    for r in init_requests.requests.iter() {
        match r.0 {
            ObjectType::Building => {
                // info!("{}", r.1); 
                if let Some(prefab) = master_queue.building_prefabs.get(&r.1) {
                    let trans = r.2.transform;

                    let save_data = SaveObject {
                        otype : ObjectType::Building,
                        prefab : LimitedBuffer::small_from_string(&prefab.0.id),
                    };

                    let sf = SnowFlake(ProcessUniqueId::new());

                    // id_converter.insert(prefab.id, sf);

                    let mut entity_commands = commands.spawn();

                    let entity = entity_commands.id();
                    entity_commands
                        .insert(save_data)
                        .insert(sf)
                        .insert(trans)
                        .insert(r.2.team_player)
                        .insert(Selectable {
                            selected : false,
                            context : SelectableContext::Single
                        })
                        .insert(Immobile::default());

                    actors.assign_building(r.2.team_player, sf);
                    identifiers.insert(sf, entity);

                    match_tags(&gltf_assets, &gltfs, &gltf_meshes, sf, trans, &prefab.0, &master_queue, entity_commands);
                }
            },
            ObjectType::Unit => {
                // info!("{}", r.1);
                if let Some(prefab) = master_queue.unit_prefabs.get(&r.1) {
                    let trans = if let Some(t) = r.2.spawn_point {
                        r.2.transform.mul_transform(Transform::from_xyz(t.0,t.1,t.2))
                    } else {
                        r.2.transform
                    };

                    let save_data = SaveObject {
                        otype : ObjectType::Unit,
                        prefab : LimitedBuffer::small_from_string(&prefab.0.id),
                    };

                    let sf = SnowFlake(ProcessUniqueId::new());

                    // id_converter.insert(prefab.id, sf);

                    let mut entity_commands = commands.spawn();

                    let entity = entity_commands.id();
                    entity_commands
                        .insert(save_data)
                        .insert(sf)
                        .insert(trans)
                        .insert(r.2.team_player)
                        .insert(Selectable {
                            selected : false,
                            context : SelectableContext::MultiSelect
                        })
                        .insert(Velocity::default());

                    if let Some(x) = r.2.end_point {
                        let end = trans.mul_vec3(Vec3::new(x.0, x.1, x.2)).xz();
                        let pf = PathFinder::default();
                        let mut path = Path::default();
                        // pf.set_start(trans.translation.xz());
                        // pf.set_start(end);
                        path.0 = Some(vec![end]);


                        entity_commands.insert(pf).insert(path);
                    }

                    actors.assign_building(r.2.team_player, sf);
                    identifiers.insert(sf, entity);

                    match_tags(&gltf_assets, &gltfs, &gltf_meshes, sf, trans, &prefab.0, &master_queue, entity_commands);
                }
            }
        }
    }
    for r in init_requests.with_entities_requests.iter() {
        match r.0 {
            ObjectType::Building => {
                // info!("{}", r.1);
                if let Some(prefab) = master_queue.building_prefabs.get(&r.1) {
                    let trans = r.2.transform;

                    let save_data = SaveObject {
                        otype : ObjectType::Building,
                        prefab : LimitedBuffer::small_from_string(&prefab.0.id),
                    };

                    let sf = SnowFlake(ProcessUniqueId::new());
                    let mut entity_commands = commands.entity(r.3);
                    entity_commands
                        .insert(save_data)
                        .insert(sf)
                        .insert(trans)
                        .insert(r.2.team_player)
                        .insert(Selectable {
                            selected : false,
                            context : SelectableContext::Single
                        })
                        .insert(Immobile::default());

                    actors.assign_building(r.2.team_player, sf);
                    identifiers.insert(sf, r.3);

                    match_tags(&gltf_assets, &gltfs, &gltf_meshes, sf, trans, &prefab.0, &master_queue, entity_commands);
                }
            },
            ObjectType::Unit => {
                // info!("{}", r.1);
                if let Some(prefab) = master_queue.unit_prefabs.get(&r.1) {
                    let trans = if let Some(t) = r.2.spawn_point {
                        r.2.transform.mul_transform(Transform::from_xyz(t.0,t.1,t.2))
                    } else {
                        r.2.transform
                    };

                    let save_data = SaveObject {
                        otype : ObjectType::Unit,
                        prefab : LimitedBuffer::small_from_string(&prefab.0.id),
                    };

                    let sf = SnowFlake(ProcessUniqueId::new());
                    let mut entity_commands = commands.entity(r.3);
                    entity_commands
                        .insert(save_data)
                        .insert(sf)
                        .insert(trans)
                        .insert(r.2.team_player)
                        .insert(Selectable {
                            selected : false,
                            context : SelectableContext::MultiSelect
                        })
                        .insert(Velocity::default());

                    actors.assign_building(r.2.team_player, sf);
                    identifiers.insert(sf, r.3);

                    if let Some(x) = r.2.end_point {
                        let end = trans.mul_vec3(Vec3::new(x.0, x.1, x.2)).xz();
                        let pf = PathFinder::default();
                        let mut path = Path::default();
                        path.0 = Some(vec![end]);


                        entity_commands.insert(pf).insert(path);
                    }

                    match_tags(&gltf_assets, &gltfs, &gltf_meshes, sf, trans, &prefab.0, &master_queue, entity_commands);
                }
            }
        }
    }
    init_requests.requests.clear();
    init_requests.with_entities_requests.clear();
}

pub fn clear_buffer_system(
    mut requests : ResMut<InitRequests>,
    mut current_placement : ResMut<CurrentPlacement>,
    mut queues : Query<&mut Queues>,
) {
    if let (Some(e), Some(d)) = (current_placement.constructor, current_placement.data.clone()) {
        if let Ok(mut q) = queues.get_mut(e) {
            if let Some(x) = q.building_queue.as_mut() {
                let mut to_remove : Option<usize> = None;
                for i in 0..x.data().buffer.len() {
                    if x.data().buffer[i] == d {
                        match current_placement.status {
                            PlacementStatus::Completed(e) => {
                                requests.request(ObjectType::Building, current_placement.data.clone().unwrap().id.clone(), current_placement.ins_data.clone().unwrap(), Some(e));
                                to_remove = Some(i);
                                current_placement.constructor = None;
                                current_placement.data = None;
                                current_placement.ins_data = None;
                                current_placement.entity = None;
                                current_placement.status = PlacementStatus::Idle;
                            },
                            _ => { }
                        }
                    }
                }
                if let Some(i) = to_remove {
                    x.data_mut().buffer.remove(i);
                }
            }
            else {
                println!("3");
            }
        }
        else {
            println!("2");
        }
    }
    else {
        // println!("1");
    }
}

pub fn match_tags(
    gltf_assets : &QLoader<GltfAsset, AssetServer>,
    gltfs : &Assets<Gltf>,
    gltf_meshes : &Assets<GltfMesh>,

    sf : SnowFlake,
    trans : Transform,
    prefab : &GameObject,
    master_queue : &MasterQueue,
    mut entity_commands : EntityCommands,
) {
    let (mesh, material) = {
        let m1 = gltf_assets.get(&prefab.id).clone();
        let m2 = gltfs.get(m1.unwrap().0.clone());
        let m3 = gltf_meshes.get(m2.unwrap().meshes[0].clone());
        let m4 = m3.unwrap().primitives[0].clone();
        (m4.mesh, m4.material.unwrap())
    };

    let pbr = PbrBundle {
        mesh,
        material,
        transform : trans,
        ..Default::default()
    };

    entity_commands.insert_bundle(pbr);

    // if let Ok(col) = Collider::from_gltf(sf, &models.get(&prefab.id).unwrap().0) {
    //     entity_commands.insert(col);
    // }

    let col = Collider::new_sphere(sf, 5.0);
    entity_commands.insert(col);

    if let Some(x) = prefab.mobility {
        entity_commands.insert(x);
    }

    if let Some(x) = prefab.economy {
        entity_commands.insert(ResourceProvider { strength : x.resource_gen });
    }
    if let Some(x) = &prefab.weapons {
        entity_commands.insert(x.clone());
    }

    if let Some(queue) = master_queue.get(&prefab.id) {
        entity_commands.insert(queue.clone());
    }

    if let Some(x) = prefab.health {
        entity_commands.insert(x);
    }

    // entity_commands.insert(PathFinder::default());
}