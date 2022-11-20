
use std::{fmt, fs::{File, OpenOptions}, io::Write, path::Path};

use bevy::{prelude::*, ecs::schedule::ShouldRun};
use bevy_rapier3d::prelude::Velocity;
use ron::{de::from_reader, extensions::Extensions, ser::{PrettyConfig, to_string_pretty,}};
use serde::{Serialize, Deserialize};
use bevy_pathfinding::{Path as FPath, d2::{GridMap, GridCell}, GridSpace, OGrid};
use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveEvent>()
            .add_event::<LoadEvent>()
            .add_event::<SaveLoaded>()

            .add_system(save_game.with_run_criteria(should_run_save_system))
            .add_system(load_game.with_run_criteria(should_run_load_system))

        ;
    }
}

#[derive(Debug, Clone)]
pub struct SaveEvent(pub String);

#[derive(Debug, Clone)]
pub struct LoadEvent(pub String);

#[derive(Debug, Clone, Copy)]
pub struct SaveLoaded;

// #[derive(Debug, Clone)]
// #[derive(Serialize, Deserialize)]
// pub struct SaveMap(pub String);

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SaveObjects {
    crane_yards: Vec<SerdeCraneYard>,
    factories: Vec<SerdeFactory>,
    marine_squads: Vec<SerdeMarineSquad>,
    resource_nodes: Vec<SerdeResourceNode>,
    tanks: Vec<SerdeTank>,
}

///Target Maximum : 125,829,120
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub actors: Actors,
    pub map: SerdeMap,
    pub objects: SaveObjects,
}

pub type SerdeCraneYardQuery<'a> = (&'a Snowflake, &'a Health, &'a Queues, &'a TeamPlayer, &'a Transform);
pub type SerdeFactoryQuery<'a> = (&'a Snowflake, &'a Health, &'a Queues, &'a TeamPlayer, &'a Transform);
pub type SerdeMarineSquadQuery<'a> = (&'a Snowflake, &'a MarineSquad, &'a Health, &'a Squad, &'a GroundPathFinder, &'a FPath, &'a Controller, &'a WeaponSet, &'a Velocity, &'a TeamPlayer, &'a Transform);
pub type SerdeResourceNodeQuery<'a> = (&'a Snowflake, &'a ResourceNode, &'a TeamPlayer, &'a Transform);
pub type SerdeTankBaseQuery<'a> = (&'a Snowflake, &'a Health, &'a GroundPathFinder, &'a FPath, &'a Controller, &'a WeaponSet, &'a Relative, &'a Velocity, &'a TeamPlayer, &'a Transform);

pub fn save_game(
    mut save_event_reader: EventReader<SaveEvent>,
    actors: Res<Actors>,
    map: Res<SerdeMap>,
    object: (
        Query<SerdeCraneYardQuery, With<CraneYard>>,
        Query<SerdeFactoryQuery, With<Factory>>,
        Query<SerdeMarineSquadQuery, With<MarineSquad>>,
        Query<SerdeResourceNodeQuery, With<ResourceNode>>,
        Query<SerdeTankBaseQuery, With<TankBase>>,
    ),
) {
    for event in save_event_reader.iter() {
        println!("SAVE");
        let crane_yards = object.0.iter().map(|object| SerdeCraneYard::from(object)).collect();
        let factories = object.1.iter().map(|object| SerdeFactory::from(object)).collect();
        let marine_squads = object.2.iter().map(|object| SerdeMarineSquad::from(object)).collect();
        let resource_nodes = object.3.iter().map(|object| SerdeResourceNode::from(object)).collect();
        let tanks = object.4.iter().map(|object| SerdeTank::from(object)).collect();

        let objects = SaveObjects {
            crane_yards,
            factories,
            marine_squads,
            resource_nodes,
            tanks,
        };

        let save_file = SaveFile {
            actors: actors.clone(),
            map: *map,
            objects
        };

        let root = std::env::current_dir().unwrap();
        save_to_file(&save_file, &format!("{}{}", root.as_path().display(), event.0)).unwrap();
    }
}

pub fn load_game(
    mut load_event_reader: EventReader<LoadEvent>,
    mut loaded_event_writer: EventWriter<SaveLoaded>,
    mut spawn_events_writer: EventWriter<ObjectSpawnEvent>,
    object_prefabs: Res<ObjectPrefabs>,
    map_prefabs: Res<MapPrefabs>,
    mut commands: Commands
) {
    for event in load_event_reader.iter() {
        let root = std::env::current_dir().unwrap();
        //TODO: Remove unwrap
        let save_file: SaveFile = load_from_file(format!("{}{}", root.as_path().display(), event.0)).unwrap();
        commands.insert_resource(save_file.actors.clone());
        commands.insert_resource(save_file.map.clone());

        let bounds = match save_file.map {
            SerdeMap::Developer(developer) => { commands.spawn(DeveloperBundle::from((developer, &map_prefabs.developer_prefab))); &map_prefabs.developer_prefab.bounds }
        };

        commands.insert_resource(bounds.clone());
        commands.insert_resource({
            // TODO: Map analyzation.
            OGrid(GridMap::new(bounds.0.x as usize, bounds.0.y as usize)
                .with_cells(|x, z| GridCell::new(x, z, false ))
                .precomputed()
            )}
        );
        commands.insert_resource(GridSpace::new(bounds.0.x as usize, bounds.0.y as usize));

        // save_file.objects.crane_yards.iter().for_each(|object| { spawn_events_writer.send((*object).into()); });
        // save_file.objects.resource_nodes.iter().for_each(|object| { spawn_events_writer.send((*object).into()); });
        // save_file.objects.factories.iter().for_each(|object| { spawn_events_writer.send((*object).into()); });
        // save_file.objects.marine_squads.iter().for_each(|object| { spawn_events_writer.send((*object).into()); });
        // save_file.objects.tanks.iter().for_each(|object| { spawn_events_writer.send((*object).into()); });

        for object in save_file.objects.crane_yards { spawn_events_writer.send(object.into()); }
        for object in save_file.objects.resource_nodes { spawn_events_writer.send(object.into()); }
        for object in save_file.objects.factories { spawn_events_writer.send(object.into()); }
        for object in save_file.objects.marine_squads { spawn_events_writer.send(object.into()); }
        for object in save_file.objects.tanks { spawn_events_writer.send(object.into()); }
        loaded_event_writer.send(SaveLoaded);
    }
}

pub fn should_run_save_system(
    actors: Option<Res<Actors>>,
    map: Option<Res<SerdeMap>>,
) -> ShouldRun {

    if actors.is_some() && map.is_some() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn should_run_load_system(
    prefabs: Option<Res<ObjectPrefabs>>,
) -> ShouldRun {

    if prefabs.is_some() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn save_to_file<S : Serialize, P : AsRef<Path>>(item : &S, path : P) -> Result<(), SaveLoadError> {
    let mut _f = match OpenOptions::new()
        .create(true)
        .write(true)
        .open(path) {
            Ok(mut x) => {
                match x.set_len(0) {
                    Ok(_) => { },
                    Err(e) => { log::error!("{}", e); }
                }
                let pretty = PrettyConfig::new()
                    .depth_limit(usize::MAX)
                    .extensions(Extensions::IMPLICIT_SOME);
                let s = to_string_pretty(item, pretty).expect("Serialization failed");

                match x.write(s.as_bytes()) {
                // match bincode::serialize(item) {
                    Ok(i) => {
                        return Ok(())
                        // match x.write(&i) {
                        //     Ok(_) => { return Ok(()); },
                        //     Err(e) => { log::error!("{}", e); return Err(SaveLoadError::FileWriteError); }
                        // }
                    },
                    Err(e) => {
                        log::error!("{}", e);
                        return Err(SaveLoadError::FileWriteError);
                    }
                }
            },
            Err(e) => { log::error!("{}", e); return Err(SaveLoadError::FileWriteError); }
    };
}

pub fn load_from_file<D, P : AsRef<Path>>(path : P) -> Result<D, SaveLoadError> where for<'de> D : Deserialize<'de> {
    match OpenOptions::new()
        .read(true)
        .write(false)
        .open(&path) {
            Ok(x) => {
                match from_reader::<File, D>(x) {
                    Ok(d) => { return Ok(d); }
                    Err(_e) => {
                        println!();
                        error!("{}", std::any::type_name::<D>());
                        error!("{}", path.as_ref().display());
                        error!("{}", _e);
                        println!();
                    }
                }
                // if let Ok(d) = from_reader::<File, D>(x) {
                //     return Ok(d);
                // }
            },
            Err(_e) => {
                println!();
                error!("{}", std::any::type_name::<D>());
                error!("{}", path.as_ref().display());
                error!("{}", _e);
                println!()
            }
    };
    match OpenOptions::new()
        .read(true)
        .write(false)
        .open(&path) {
            Ok(x) => {
                if let Ok(d) = bincode::deserialize_from::<File, D>(x) {
                    return Ok(d);
                }
            },
            Err(_e) => {
                println!();
                error!("{}", std::any::type_name::<D>());
                error!("{}", path.as_ref().display());
                error!("{}", _e);
                println!()
            }
    };
    return Err(SaveLoadError::FileReadError);
}

#[derive(Debug)]
pub enum SaveLoadError {
    MapNotFoundError(String),
    ModelNotFoundError(String),
    ObjNotFoundError(String),
    // ColliderError(ColliderError),
    FileWriteError,
    FileReadError,
}

impl fmt::Display for SaveLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MapNotFoundError(s) => {
                write!(f, "Map file <{}> either not found or currupted", s)
            },
            Self::ModelNotFoundError(s) => {
                write!(f, "Gltf file <{}> either not found or currupted", s)
            },
            Self::ObjNotFoundError(s) => {
                write!(f, "Obj file <{}> either not found or currupted", s)
            },
            // Self::ColliderError(e) => {
            //     write!(f, "{}", e)
            // },
            Self::FileWriteError => {
                write!(f, "Failed to read or serialize file")
            },
            Self::FileReadError => {
                write!(f, "Failed to write to file")
            }
        }
    }
}