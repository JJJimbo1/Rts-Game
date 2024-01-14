
use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;
use serde::{Serialize, Deserialize};
use t5f_common::*;
use crate::*;

///Target Maximum : 125,829,120
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct BaseSaveState {
    pub commanders: Commanders,
    pub map: MapSerde,
    pub objects: SaveObjects,
}

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SaveObjects {
    pub crane_yards: Vec<CraneYardSerde>,
    pub factories: Vec<FactorySerde>,
    pub marine_squads: Vec<MarineSquadSerde>,
    pub resource_nodes: Vec<ResourceNodeSerde>,
    pub tanks: Vec<TankBaseSerde>,
}

#[derive(Debug, Default, Clone)]
#[derive(Resource)]
pub struct LoadingStatus {
    pub level_loaded: bool,
    pub map_loaded: bool,
    pub crane_yards_loaded: Option<bool>,
    pub resource_nodes_loaded: Option<bool>,
    pub factories_loaded: Option<bool>,
    pub marines_loaded: Option<bool>,
    pub tanks_loaded: Option<bool>,
}

impl LoadingStatus {
    pub fn complete(&self) -> bool {
        self.level_loaded
        & self.map_loaded
        & self.crane_yards_loaded.unwrap_or(true)
        & self.resource_nodes_loaded.unwrap_or(true)
        & self.factories_loaded.unwrap_or(true)
        & self.marines_loaded.unwrap_or(true)
        & self.tanks_loaded.unwrap_or(true)
    }

    // pub fn reset(&mut self) {
    //     self.level_loaded = false;
    //     self.map_loaded = false;
    //     self.crane_yards_loaded = Some(false);
    //     self.resource_nodes_loaded = Some(false);
    //     self.factories_loaded = Some(false);
    //     self.marines_loaded = Some(false);
    //     self.tanks_loaded = Some(false);
    // }
}

pub type SerdeCraneYardQuery<'a> = (&'a Snowflake, &'a Health, &'a Queues, &'a TeamPlayer, &'a Transform);
pub type SerdeFactoryQuery<'a> = (&'a Snowflake, &'a Health, &'a Queues, &'a TeamPlayer, &'a Transform);
pub type SerdeMarineSquadQuery<'a> = (&'a Snowflake, &'a Health, &'a Squad, &'a PathFinder, &'a Navigator, &'a WeaponSet, &'a Velocity, &'a TeamPlayer, &'a Transform);
pub type SerdeResourceNodeQuery<'a> = (&'a Snowflake, &'a ResourceNodePlatforms, &'a TeamPlayer, &'a Transform);
pub type SerdeTankBaseQuery<'a> = (&'a Snowflake, &'a Health, &'a PathFinder, &'a Navigator, &'a WeaponSet, &'a Reference, &'a Velocity, &'a TeamPlayer, &'a Transform);

#[derive(Debug, Clone, Copy)]
pub struct PersistencePlugin;

impl PersistencePlugin {

    pub fn save_game(
        mut save_events: EventReader<SaveEvent>,
        mut save_file: ResMut<SaveFile>,
        actors: Res<Commanders>,
        map: Res<MapSerde>,
        object: (
            Query<SerdeCraneYardQuery, With<CraneYardMarker>>,
            Query<SerdeFactoryQuery, With<FactoryMarker>>,
            Query<SerdeMarineSquadQuery, With<MarineSquadMarker>>,
            Query<SerdeResourceNodeQuery, With<ResourceNodePlatforms>>,
            Query<SerdeTankBaseQuery, With<TankBaseMarker>>,
        ),
    ) {
        if save_events.read().filter(|se| se.saving()).next().is_none() { return; };
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

        let base_save_state = BaseSaveState {
            commanders: actors.clone(),
            map: *map,
            objects
        };

        let Ok(data) = ron::ser::to_string(&base_save_state) else { return; };

        save_file.insert(MOD_LABEL, data);
        // for event in save_events.read() {

        //     // let level_asset = LevelAsset {
        //     //     save_state: save_file,
        //     // };

        //     // let root = std::env::current_dir().unwrap();
        //     // save_to_file(&level_asset, &format!("{}/the5thfundamental_game/assets/{}", root.as_path().display(), event.0)).unwrap();
        // }
    }

    pub fn load_game(

        mut load_events: EventReader<LoadEvent>,
        load_file: ResMut<SaveFile>,


        mut load_level: EventWriter<LevelLoadEvent<AnyLevelMarker>>,
        mut load_map: EventWriter<MapLoadEvent<AnyMapMarker>>,
        mut load_objects: EventWriter<ObjectLoadEvent<AnyObjectMarker>>,

        // mut loading: Local<(Option<Handle<LevelAsset>>, bool)>,
        // mut load_file: EventReader<LoadFile>,
        // // asset_server: Res<AssetServer>,
        // // level_assets: Res<Assets<LevelAsset>>,
        mut status: ResMut<LoadingStatus>,

        mut commands: Commands,
        // mut level_loaded: EventWriter<LevelLoadedEvent>,
    ) {
        // println!("LOADINGDFADSFASDf");
        if load_events.read().filter(|le| le.loading()).next().is_none() { return; };
        let Some(data) = load_file.get(&MOD_LABEL) else { return; };
        let Ok(base_save_state): Result<BaseSaveState, _> = ron::de::from_str(&data) else { return; };

        commands.insert_resource(base_save_state.commanders);

        let level_load_event_data = LevelLoadEventData {

        };
        let level_load_event = LevelLoadEvent::<AnyLevelMarker>(level_load_event_data, PhantomData::<AnyLevelMarker>);
        load_level.send(level_load_event);

        let map_load_event_data = MapLoadEventData {
            map: base_save_state.map.clone(),
        };
        let map_load_event = MapLoadEvent::<AnyMapMarker>(map_load_event_data, PhantomData::<AnyMapMarker>);
        load_map.send(map_load_event);

        let objects = base_save_state.objects;
        for object in &objects.crane_yards { status.crane_yards_loaded = Some(false); load_objects.send(object.clone().into()); }
        for object in &objects.resource_nodes { status.resource_nodes_loaded = Some(false); load_objects.send(object.clone().into()); }
        for object in &objects.factories { status.factories_loaded = Some(false); load_objects.send(object.clone().into()); }
        for object in &objects.marine_squads { status.marines_loaded = Some(false); load_objects.send(object.clone().into()); }
        for object in &objects.tanks { status.tanks_loaded = Some(false); load_objects.send(object.clone().into()); }

        // if let Some(file) = &(*loading).0 {
        //     let handle = file.clone();
        //     let mut level = None;
        //     if let Some(load_state) = asset_server.get_load_state(handle.clone()) {
        //         match load_state {
        //             LoadState::NotLoaded => { error!("how1"); return; },
        //             LoadState::Loading => { error!("how2"); return; },
        //             LoadState::Loaded => { level = level_assets.get(&handle); },
        //             LoadState::Failed => { level_loaded.send(LevelLoadedEvent::Failure(FailureReason::LevelNotFound)); },
        //         }
        //         let Some(level) = level else { level_loaded.send(LevelLoadedEvent::Failure(FailureReason::LevelNotFound)); return; };
        //         let event_data = LevelLoadEventData {
        //             // serde_data: LevelSerdeData {
        //             //     save_state: level.save_state.clone(),
        //             // }
        //         };
        //         let event = LevelLoadEvent(event_data, PhantomData);
        //         load_level.send(event);
        //         (*loading).0 = None;
        //         (*loading).1 = true;
        //     }
        // }

        // if loading.1 && status.complete() {
        //     level_loaded.send(LevelLoadedEvent::Success);
        //     status.reset();
        //     loading.1 = false;
        // }

        // for file in load_file.read() {
        //     let handle = file.0.clone();
        //     (*loading).0 = Some(handle);
        // }
    }

    pub fn finish_loading_game(
        status: Res<LoadingStatus>,
        mut load_file: ResMut<SaveFile>,
    ) {
        if status.complete() {
            load_file.set_finished(MOD_LABEL);
        }
    }
}

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        //TODO: Fix running every frame.
        app
            // .add_event::<>()
            .insert_resource(LoadingStatus::default())
            .add_systems(Update, Self::save_game.run_if(resource_exists::<Commanders>()).run_if(resource_exists::<MapSerde>()).run_if(resource_exists::<SaveFile>()))
            .add_systems(Update, Self::load_game.run_if(resource_exists::<ObjectPrefabs>()).run_if(resource_exists::<SaveFile>()))
            .add_systems(Update, Self::finish_loading_game.run_if(resource_exists::<SaveFile>()))
        ;
    }
}