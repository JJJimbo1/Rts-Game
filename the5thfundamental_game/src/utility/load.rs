pub use load::*;
mod load {

    use bevy::{ecs::system::EntityCommands, gltf::{Gltf, GltfMesh}, prelude::*, prelude::shape::Plane};
    use bimap::BiMap;
    use bevy_pathfinding::{PathFinder, Path};
    use qloader::*;
    use snowflake::ProcessUniqueId;
    use crate::*;
    
    pub fn load_save_file(
        save_file : Res<SaveFile>,
        maps : Res<QLoader<Map, ()>>,
        gltf_assets : Res<QLoader<GltfAsset, AssetServer>>,
        gltfs : Res<Assets<Gltf>>,
        gltf_meshes : Res<Assets<GltfMesh>>,
        models : Res<QLoader<ModelAsset, ()>>,
        master_queue : Res<MasterQueue>,
        mut identifiers : ResMut<Identifiers>,
        mut actors : ResMut<Actors>,
        mut commands : Commands,
    ) /*-> Result<(), SaveLoadError>*/ {
        match initialize_map(&gltf_assets, &gltfs, &gltf_meshes, &models, &save_file.map, maps, &mut identifiers, &mut commands) {
            Ok(()) => { },
            Err(e) => { println!("{}", e); }
        }
        let mut weapons = Vec::new();
        let mut id_converter = save_file.id_converter.clone();
        match initialize_buildings(&gltf_assets, &gltfs, &gltf_meshes, &models, &mut id_converter, &save_file.buildings, &master_queue, &mut identifiers, &mut actors, &mut commands, &mut weapons) {
            Ok(()) => { },
            Err(e) => { println!("{}", e); }
        }
        match initialize_units(&gltf_assets, &gltfs, &gltf_meshes, &models, &mut id_converter, &save_file.units, &master_queue, &mut identifiers, &mut commands, &mut weapons) {
            Ok(()) => { },
            Err(e) => { println!("{}", e); }
        }
        match fix_id_refs(&mut id_converter, &mut identifiers, &mut weapons, &mut commands) {
            Ok(()) => { },
            Err(e) => { println!("{}", e); }
        }
        /*Ok(())*/
    }

    pub fn initialize_map(
        gltf_assets : &QLoader<GltfAsset, AssetServer>,
        gltfs : &Assets<Gltf>,
        gltf_meshes : &Assets<GltfMesh>,
        models : &QLoader<ModelAsset, ()>,

        map : &str,
        maps : Res<QLoader<Map, ()>>,

        identifiers : &mut Identifiers,
        commands : &mut Commands
    ) -> Result<(), SaveLoadError> {

        commands.spawn_bundle(DirectionalLightBundle  {
            directional_light : DirectionalLight {
                // shadows_enabled : true,
                ..Default::default()
            },

            transform: Transform::from_xyz(800.0, 1800.0, 800.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        });

        let sf = SnowFlake(ProcessUniqueId::new());

        let mut entity_commands = commands.spawn();
        entity_commands
            .insert(sf)
            .insert(Selectable {
                selected : false,
                context : SelectableContext::Clear,
            });

        identifiers.insert(sf, entity_commands.id());

        if let Some(m) = maps.get(map.clone()) {
            let (mesh, material) = {
                let m1 = gltf_assets.get(&m.base).clone();
                let m2 = gltfs.get(m1.unwrap().0.clone());
                let m3 = gltf_meshes.get(m2.unwrap().meshes[0].clone());
                let m4 = m3.unwrap().primitives[0].clone();
                (m4.mesh, m4.material.unwrap())
            };

            let pbr = PbrBundle {
                mesh,
                material,
                ..Default::default()
            };

            entity_commands.insert_bundle(pbr);

            if let Ok(col) = Collider::from_gltf(sf, &models.get(&m.base).unwrap().0) {
                entity_commands.insert(col);
            }
            // let col = Collider::new_box(sf, m.box_collider.0, m.box_collider.1, m.box_collider.2);
        } else {
            return Err(SaveLoadError::MapNotFoundError(map.to_string()));
        }

        Ok(())
    }

    pub fn initialize_buildings(
        gltf_assets : &QLoader<GltfAsset, AssetServer>,
        gltfs : &Assets<Gltf>,
        gltf_meshes : &Assets<GltfMesh>,
        models : &QLoader<ModelAsset, ()>,

        id_converter : &mut BiMap<SnowFlake, SnowFlake>,
        buildings : &Vec<SaveBuilding>,
        master_queue : &MasterQueue,
        identifiers : &mut Identifiers,
        actors : &mut Actors,
        commands : &mut Commands,

        weapons : &mut Vec<(SnowFlake, WeaponSet)>,
    ) -> Result<(), SaveLoadError> {
        println!("Buildings: {}", buildings.len());
        let mut entities = Vec::new();
        for building in buildings.iter() {
            if let Some(prefab) = master_queue.building_prefabs.get(&building.prefab) {
                let trans = building.transform.to_transform();

                let save_data = SaveObject {
                    otype : ObjectType::Building,
                    prefab : LimitedBuffer::small_from_string(&building.prefab),
                };

                let sf = SnowFlake(ProcessUniqueId::new());

                if let Some(x) = building.id {
                    id_converter.insert(x, sf);
                }

                let mut entity_commands = commands.spawn();

                let entity = entity_commands.id();
                entities.push(entity);
                entity_commands
                    .insert(save_data)
                    .insert(sf)
                    .insert(trans)
                    .insert(building.teamplayer)
                    .insert(Selectable {
                        selected : false,
                        context : SelectableContext::Single
                    })
                    .insert(Immobile::default());

                actors.assign_building(building.teamplayer, sf);
                identifiers.insert(sf, entity);

                match match_save_tags(&gltf_assets, &gltfs, &gltf_meshes, &models, sf, trans, &building.save_tags, &prefab.0, master_queue, entity_commands, weapons) {
                    Ok(()) => { },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn initialize_units(
        gltf_assets : &QLoader<GltfAsset, AssetServer>,
        gltfs : &Assets<Gltf>,
        gltf_meshes : &Assets<GltfMesh>,
        models : &QLoader<ModelAsset, ()>,

        id_converter : &mut BiMap<SnowFlake, SnowFlake>,
        units : &Vec<SaveUnit>,
        master_queue : &MasterQueue,
        identifiers : &mut Identifiers,
        commands : &mut Commands,

        weapons : &mut Vec<(SnowFlake, WeaponSet)>,
    ) -> Result<(), SaveLoadError> {
        println!("Units: {}", units.len());
        let mut entities = Vec::new();
        for unit in units.iter() {
            if let Some(prefab) = master_queue.unit_prefabs.get(&unit.prefab) {

                let trans = unit.transform.to_transform();

                let save_data = SaveObject {
                    otype : ObjectType::Unit,
                    prefab : LimitedBuffer::small_from_string(&unit.prefab),
                };

                let sf = SnowFlake(ProcessUniqueId::new());

                if let Some(x) = unit.id {
                    id_converter.insert(x, sf);
                }

                let mut entity_commands = commands.spawn();

                let entity = entity_commands.id();
                entities.push(entity);

                entity_commands
                    .insert(save_data)
                    .insert(sf)
                    .insert(trans)
                    .insert(unit.teamplayer)
                    .insert(Selectable {
                        selected : false,
                        context : SelectableContext::MultiSelect
                    })
                    .insert(unit.save_tags.as_ref().unwrap_or(&SaveTags::empty()).velocity.unwrap_or(Velocity::default()));

                identifiers.insert(sf, entity);

                match match_save_tags(&gltf_assets, &gltfs, &gltf_meshes, &models, sf, trans, &unit.save_tags, &prefab.0, master_queue, entity_commands, weapons) {
                    Ok(()) => { },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    fn fix_id_refs(
        id_converter : &mut BiMap<SnowFlake, SnowFlake>,
        identifiers : &mut Identifiers,
        weapons : &mut Vec<(SnowFlake, WeaponSet)>,
        commands : &mut Commands,
    ) -> Result<(), SaveLoadError> {

        for (_, weapon) in weapons.iter_mut() {
            for w in weapon.weapons.iter_mut() {
                match w.target {
                    Target::AutoTarget(sf) => {
                        if let Some(l) = id_converter.get_by_left(&sf) {
                            w.target = Target::AutoTarget(*l)
                        }
                    },
                    Target::ManualTarget(sf) => {
                        if let Some(l) = id_converter.get_by_left(&sf) {
                            w.target = Target::ManualTarget(*l)
                        }
                    },
                    Target::None => { }
                }
            }
        }

        for (id, weapon) in weapons.iter() {
            if let Some(e) = identifiers.get_entity(*id) {
                let mut entity_commands = commands.entity(e);
                entity_commands.insert(weapon.clone());
            }
        }
        Ok(())
    }






    fn match_save_tags(
        gltf_assets : &QLoader<GltfAsset, AssetServer>,
        gltfs : &Assets<Gltf>,
        gltf_meshes : &Assets<GltfMesh>,
        models : &QLoader<ModelAsset, ()>,

        sf : SnowFlake,
        trans : Transform,
        save_tags : &Option<SaveTags>,
        prefab : &GameObject,
        master_queue : &MasterQueue,
        mut entity_commands : EntityCommands,

        weapons : &mut Vec<(SnowFlake, WeaponSet)>,
    ) -> Result<(), SaveLoadError>{
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

        if let Ok(col) = Collider::from_gltf(sf, &models.get(&prefab.id).unwrap().0) {
            entity_commands.insert(col);
        }

        if let Some(x) = prefab.mobility {
            entity_commands.insert(x);
        }

        if let Some(x) = prefab.economy {
            entity_commands.insert(ResourceProvider { strength : x.resource_gen });
        }

        if let Some(st) = save_tags {
            if let Some(x) = &st.weapons {
                weapons.push((sf, x.clone()));
                //entity_commands.insert(x.clone());
            } else {
                if let Some(x) = &prefab.weapons {
                    weapons.push((sf, x.clone()));
                    // entity_commands.insert(wpns);
                }
            }

            if let Some(queue) = st.queue.clone() {
                entity_commands.insert(queue);
            } else {
                if let Some(queue) = master_queue.get(&prefab.id) {
                    entity_commands.insert(queue.clone());
                }
            }

            if let Some(x) = st.health {
                entity_commands.insert(x);
            } else if let Some(x) = prefab.health {
                entity_commands.insert(x);
            }

            if let Some(x) = &st.finder {
                entity_commands.insert(x.clone());
            } else {
                entity_commands.insert(PathFinder::default());
            }

            if let Some(x) = &st.path {
                entity_commands.insert(x.clone());
            } else {
                entity_commands.insert(Path::default());
            }
        } else {
            if let Some(x) = &prefab.weapons {
                weapons.push((sf, x.clone()));
                // entity_commands.insert(wpns);
            }

            if let Some(queue) = master_queue.get(&prefab.id) {
                entity_commands.insert(queue.clone());
            }

            if let Some(x) = prefab.health {
                entity_commands.insert(x);
            }

            entity_commands.insert(PathFinder::default());
            entity_commands.insert(Path::default());
        }

        Ok(())
    }
}