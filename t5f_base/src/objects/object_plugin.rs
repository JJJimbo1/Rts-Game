use std::marker::PhantomData;
use bevy::{prelude::*, math::Vec3Swizzles};
use t5f_common::*;
use crate::*;



pub struct ObjectPlugin;

impl ObjectPlugin {
    pub fn load_objects(
        mut load_events: EventReader<ObjectLoadEvent<AnyObjectMarker>>,
        (
            mut load_crane_yards,
            mut load_resource_nodes,
            mut load_factories,
            mut load_marine_squads,
            mut load_tanks
        ): (
            EventWriter<ObjectLoadEvent<CraneYardMarker>>,
            EventWriter<ObjectLoadEvent<ResourceNodeMarker>>,
            EventWriter<ObjectLoadEvent<FactoryMarker>>,
            EventWriter<ObjectLoadEvent<MarineSquadMarker>>,
            EventWriter<ObjectLoadEvent<TankBaseMarker>>,
        ),
    ) {
        for event in load_events.read() {
            match event.0.object_type {
                ObjectType::CraneYard => { load_crane_yards.send(ObjectLoadEvent(event.0.clone(), PhantomData)); }
                ObjectType::ResourceNode => { load_resource_nodes.send(ObjectLoadEvent(event.0.clone(), PhantomData)); }
                ObjectType::Factory => { load_factories.send(ObjectLoadEvent(event.0.clone(), PhantomData)); }
                ObjectType::MarineSquad => { load_marine_squads.send(ObjectLoadEvent(event.0.clone(), PhantomData)); },
                ObjectType::TankBase => { load_tanks.send(ObjectLoadEvent(event.0.clone(), PhantomData)); },
                _ => { }
            }
        }
    }

    pub fn spawn_objects(
        mut spawn_events: EventReader<ObjectSpawnEvent<AnyObjectMarker>>,
        (
            mut spawn_crane_yards,
            mut spawn_resource_nodes,
            mut spawn_factories,
            mut spawn_marine_squads,
            mut spawn_tanks
        ): (
            EventWriter<ObjectSpawnEvent<CraneYardMarker>>,
            EventWriter<ObjectSpawnEvent<ResourceNodeMarker>>,
            EventWriter<ObjectSpawnEvent<FactoryMarker>>,
            EventWriter<ObjectSpawnEvent<MarineSquadMarker>>,
            EventWriter<ObjectSpawnEvent<TankBaseMarker>>,
        ),
    ) {
        for event in spawn_events.read() {
            match event.0.object_type {
                ObjectType::CraneYard => { spawn_crane_yards.send(ObjectSpawnEvent(event.0.clone(), PhantomData)); }
                ObjectType::ResourceNode => { spawn_resource_nodes.send(ObjectSpawnEvent(event.0.clone(), PhantomData)); }
                ObjectType::Factory => { spawn_factories.send(ObjectSpawnEvent(event.0.clone(), PhantomData)); }
                ObjectType::MarineSquad => { spawn_marine_squads.send(ObjectSpawnEvent(event.0.clone(), PhantomData)); },
                ObjectType::TankBase => { spawn_tanks.send(ObjectSpawnEvent(event.0.clone(), PhantomData)); },
                _ => { }
            }
        }
    }

    pub fn patch_grid_spawn(
        mut grid_map: ResMut<GridMap>,
        pathing_space: Res<GridSpace>,
        objects: Query<(&Transform, &ObjectType), Added<ObjectType>>,
    ) {
        let mut recompute = false;
        objects.for_each(|(transform, object_type)| {
            let max = match *object_type {
                ObjectType::CraneYard => { Some((8, 8)) },
                ObjectType::Factory => { Some((11, 11)) },
                ObjectType::ResourceNode => { Some((9, 9))}
                _ => { None }
            };
            if let Some((x_max, y_max)) = max {
                let mut blocks = Vec::new();
                for x_offset in -x_max..=x_max {
                    for y_offset in -y_max..=y_max {
                        let (x, y) = pathing_space.position_to_index(transform.translation.xz() + Vec2::new(x_offset as f32, y_offset as f32));
                        blocks.push((x, y));
                    }
                }
                grid_map.0.add_objects(blocks);
                recompute = true;
            }
        });
        if recompute {
            grid_map.0.precompute();
        }
    }

    pub fn patch_grid_kill(
        mut grid_map: ResMut<GridMap>,
        pathing_space: Res<GridSpace>,
        mut kills: EventReader<ObjectKilledEvent>,
        objects: Query<(&Transform, &ObjectType)>,
    ) {
        let mut recompute = false;
        for kill in kills.read() {
            let Ok((transform, object_type)) = objects.get(kill.0) else { continue; };
            let max = match *object_type {
                ObjectType::CraneYard => { Some((8, 8)) },
                ObjectType::Factory => { Some((11, 11)) },
                ObjectType::ResourceNode => { Some((9, 9))}
                _ => { None }
            };
            if let Some((x_max, y_max)) = max {
                let mut blocks = Vec::new();
                for x_offset in -x_max..=x_max {
                    for y_offset in -y_max..=y_max {
                        let (x, y) = pathing_space.position_to_index(transform.translation.xz() + Vec2::new(x_offset as f32, y_offset as f32));
                        blocks.push((x, y));
                    }
                }
                grid_map.0.remove_objects(blocks);
                recompute = true;
            }
        }
        if recompute {
            grid_map.0.precompute();
        }
    }

    pub fn show_grid(
        grid_map: ResMut<GridMap>,
        pathing_space: Res<GridSpace>,
        mut gizmos: Gizmos,
    ) {
        for object in grid_map.0.blocks() {
            let xy = pathing_space.index_to_position(*object);
            let xyz = xy.extend(0.0).xzy();
            gizmos.line(xyz, xyz + Vec3::Y * 10.0, Color::GREEN);
            for object in grid_map.0.object_nodes(*object).unwrap_or(&Vec::new()) {
                let xy = pathing_space.index_to_position(*object);
                let xyz = xy.extend(0.0).xzy();
                gizmos.line(xyz, xyz + Vec3::Y * 20.0, Color::PURPLE);
            }
        }

    }
}

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ObjectLoadEvent<AnyObjectMarker>>()
            .add_event::<ObjectSpawnEvent<AnyObjectMarker>>()
            // .add_event::<ActivationEvent>()
            .add_event::<ObjectKilledEvent>()
            .add_systems(Update, (
                Self::load_objects,
                Self::spawn_objects,
                Self::patch_grid_spawn.after(Self::load_objects).after(Self::spawn_objects),
                Self::patch_grid_kill.after(Self::patch_grid_spawn),
                Self::show_grid,
            ))
            .add_plugins((
                CraneYardPlugin,
                ResourceNodePlugin,
                FactoryPlugin,
                MarineSquadPlugin,
                TankPlugin
            ))
        ;
    }
}