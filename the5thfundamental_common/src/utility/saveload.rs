
use std::{fmt, fs::{File, OpenOptions}, io::Write, path::Path};

use bevy::{prelude::*, ecs::schedule::ShouldRun, asset::LoadState};
use bevy_rapier3d::prelude::Velocity;
use ron::{de::from_reader, extensions::Extensions, ser::{PrettyConfig, to_string_pretty,}};
use serde::{Serialize, Deserialize};
use crate::{*, pathing::Path as FPath};

#[derive(Debug, Clone, Copy)]
pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveEvent>()
            .add_event::<LoadEvent>()
            .add_event::<LevelLoadedEvent>()

            .add_system(save_game.with_run_criteria(should_run_save_system))
            .add_system(load_game.with_run_criteria(should_run_load_system))

        ;
    }
}

#[derive(Debug, Clone)]
pub struct SaveEvent(pub String);

#[derive(Debug, Clone)]
pub struct LoadEvent(pub Handle<LevelAsset>);

#[derive(Debug, Clone, Copy)]
pub enum LevelLoadedEvent {
    Success,
    Failure,
}

// #[derive(Debug, Clone)]
// #[derive(Serialize, Deserialize)]
// pub struct SaveMap(pub String);

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SaveObjects {
    crane_yards: Vec<CraneYardSerde>,
    factories: Vec<FactorySerde>,
    marine_squads: Vec<MarineSquadSerde>,
    resource_nodes: Vec<ResourceNodeSerde>,
    tanks: Vec<TankBaseSerde>,
}

///Target Maximum : 125,829,120
#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SaveState {
    pub actors: Actors,
    pub map: SerdeMap,
    pub objects: SaveObjects,
}

pub type SerdeCraneYardQuery<'a> = (&'a Snowflake, &'a Health, &'a Queues, &'a TeamPlayer, &'a Transform);
pub type SerdeFactoryQuery<'a> = (&'a Snowflake, &'a Health, &'a Queues, &'a TeamPlayer, &'a Transform);
pub type SerdeMarineSquadQuery<'a> = (&'a Snowflake, &'a Health, &'a Squad, &'a GroundPathFinder, &'a FPath, &'a Controller, &'a WeaponSet, &'a Velocity, &'a TeamPlayer, &'a Transform);
pub type SerdeResourceNodeQuery<'a> = (&'a Snowflake, &'a ResourceNodePlatforms, &'a TeamPlayer, &'a Transform);
pub type SerdeTankBaseQuery<'a> = (&'a Snowflake, &'a Health, &'a GroundPathFinder, &'a FPath, &'a Controller, &'a WeaponSet, &'a Relative, &'a Velocity, &'a TeamPlayer, &'a Transform);

pub fn save_game(
    mut save_event_reader: EventReader<SaveEvent>,
    actors: Res<Actors>,
    map: Res<SerdeMap>,
    object: (
        Query<SerdeCraneYardQuery, With<CraneYardMarker>>,
        Query<SerdeFactoryQuery, With<FactoryMarker>>,
        Query<SerdeMarineSquadQuery, With<MarineSquadMarker>>,
        Query<SerdeResourceNodeQuery, With<ResourceNodePlatforms>>,
        Query<SerdeTankBaseQuery, With<TankBaseMarker>>,
    ),
) {
    for event in save_event_reader.iter() {
        // println!("SAVE");
        let crane_yards = object.0.iter().map(|object| CraneYardSerde::from(object)).collect();
        let factories = object.1.iter().map(|object| FactorySerde::from(object)).collect();
        let marine_squads = object.2.iter().map(|object| MarineSquadSerde::from(object)).collect();
        let resource_nodes = object.3.iter().map(|object| ResourceNodeSerde::from(object)).collect();
        let tanks = object.4.iter().map(|object| TankBaseSerde::from(object)).collect();

        let objects = SaveObjects {
            crane_yards,
            factories,
            marine_squads,
            resource_nodes,
            tanks,
        };

        let save_file = SaveState {
            actors: actors.clone(),
            map: *map,
            objects
        };

        let level_asset = LevelAsset {
            save_state: save_file,
        };

        let root = std::env::current_dir().unwrap();
        // println!("{}", &format!("{}/the5thfundamental_game/assets/{}", root.as_path().display(), event.0.clone()));
        save_to_file(&level_asset, &format!("{}/the5thfundamental_game/assets/{}", root.as_path().display(), event.0)).unwrap();
    }
}

pub fn load_game(
    mut load_level: Local<Option<Handle<LevelAsset>>>,
    mut load_event_reader: EventReader<LoadEvent>,
    mut level_loaded_event_writer: EventWriter<LevelLoadedEvent>,
    mut spawn_events_writer: EventWriter<ObjectSpawnEvent>,
    asset_server: Res<AssetServer>,
    level_assets: Res<Assets<LevelAsset>>,
    maps: Res<Assets<MapAsset>>,
    map_assets: Res<MapAssets>,
    mut commands: Commands
) {
    for event in load_event_reader.iter() {
        *load_level = Some(event.0.clone());
    }

    let Some(level_handle) = load_level.clone() else { return; };
    if asset_server.get_load_state(level_handle.clone()) == LoadState::Failed { level_loaded_event_writer.send(LevelLoadedEvent::Failure); *load_level = None; }
    let Some(level) = level_assets.get(&level_handle) else { return; };
    let map_handle: Handle<MapAsset> = map_assets.from_serde_map(&level.save_state.map).clone();
    if asset_server.get_load_state(map_handle.clone()) == LoadState::Failed { level_loaded_event_writer.send(LevelLoadedEvent::Failure); *load_level = None; }
    let Some(map) = maps.get(&map_handle) else { return; };

    commands.insert_resource(level.save_state.actors.clone());
    commands.insert_resource(level.save_state.map.clone());

    let bounds = match level.save_state.map {
        SerdeMap::Developer(developer) => { let map = map.try_into().unwrap(); commands.spawn(DeveloperBundle::from((developer, &map))); map.bounds }
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
    for node in &level.save_state.objects.resource_nodes {
        println!("{:?}", node.serde_resource_node);
    }

    for object in &level.save_state.objects.crane_yards { spawn_events_writer.send(object.clone().into()); }
    for object in &level.save_state.objects.resource_nodes { spawn_events_writer.send(object.clone().into()); }
    for object in &level.save_state.objects.factories { spawn_events_writer.send(object.clone().into()); }
    for object in &level.save_state.objects.marine_squads { spawn_events_writer.send(object.clone().into()); }
    for object in &level.save_state.objects.tanks { spawn_events_writer.send(object.clone().into()); }
    level_loaded_event_writer.send(LevelLoadedEvent::Success);
    *load_level = None;
}

pub fn should_run_save_system(
    actors: Option<Res<Actors>>,
    map: Option<Res<SerdeMap>>,
) -> ShouldRun {
    (actors.is_some() && map.is_some()).into()
}

pub fn should_run_load_system(
    prefabs: Option<Res<ObjectPrefabs>>,
) -> ShouldRun {
    prefabs.is_some().into()
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
                    Ok(_i) => {
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