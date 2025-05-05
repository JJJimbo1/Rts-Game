use serde::{Serialize, Deserialize};
use bevy::{prelude::*, platform::collections::HashMap};
use avian3d::prelude::LinearVelocity;
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
    pub crane_yards: Vec<CraneYardDisk>,
    pub barracks: Vec<BarracksDisk>,
    pub factories: Vec<FactoryDisk>,
    pub marine_squads: Vec<MarineSquadDisk>,
    pub resource_nodes: Vec<ResourceNodeDisk>,
    pub tanks: Vec<TankBaseDisk>,
}

#[derive(Debug, Default, Clone)]
#[derive(Resource)]
pub struct LoadingStatus {
    pub level_loaded: bool,
    pub map_loaded: bool,
    pub crane_yards_loaded: Option<bool>,
    pub resource_nodes_loaded: Option<bool>,
    pub barracks_loaded: Option<bool>,
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
}

pub type CraneYardDiskQuery<'a> = (&'a Snowflake, &'a Health, &'a Queues, &'a TeamPlayer, &'a Transform);
pub type BarracksDiskQuery<'a> = (&'a Snowflake, &'a Health, &'a Queues, &'a TeamPlayer, &'a Transform);
pub type FactoryDiskQuery<'a> = (&'a Snowflake, &'a Health, &'a Queues, &'a TeamPlayer, &'a Transform);
pub type MarineSquadDiskQuery<'a> = (&'a Snowflake, &'a Health, &'a Squad, &'a PathFinder, &'a Navigator, &'a WeaponSet, &'a LinearVelocity, &'a TeamPlayer, &'a Transform);
pub type ResourceNodeDiskQuery<'a> = (&'a Snowflake, &'a ResourceNodePlatforms, &'a TeamPlayer, &'a Transform);
pub type TankBaseDiskQuery<'a> = (&'a Snowflake, &'a Health, &'a PathFinder, &'a Navigator, &'a WeaponSet, &'a Reference, &'a LinearVelocity, &'a TeamPlayer, &'a Transform);

#[derive(Debug, Clone, Copy)]
pub struct DiskPlugin;

impl DiskPlugin {

    pub fn save_game(
        mut save_events: ParamSet<(
            EventReader<SaveEvent>,
            EventWriter<SaveEvent>,
        )>,
        mut save_file: ResMut<SaveFile>,
        actors: Res<Commanders>,
        map: Res<MapSerde>,
        object: (
            Query<CraneYardDiskQuery, With<CraneYard>>,
            Query<BarracksDiskQuery, With<Barracks>>,
            Query<FactoryDiskQuery, With<Factory>>,
            Query<MarineSquadDiskQuery, With<MarineSquad>>,
            Query<ResourceNodeDiskQuery, With<ResourceNodePlatforms>>,
            Query<TankBaseDiskQuery, With<TankBase>>,
        ),
    ) {
        let mut file = None;

        for event in save_events.p0().read().filter(|event| event.saving()) {
            let crane_yards = object.0.iter().map(|object| CraneYardDisk::from(object)).collect();
            let barracks = object.1.iter().map(|object| BarracksDisk::from(object)).collect();
            let factories = object.2.iter().map(|object| FactoryDisk::from(object)).collect();
            let marine_squads = object.3.iter().map(|object| MarineSquadDisk::from(object)).collect();
            let resource_nodes = object.4.iter().map(|object| ResourceNodeDisk::from(object)).collect();
            let tanks = object.5.iter().map(|object| TankBaseDisk::from(object)).collect();

            let objects = SaveObjects {
                crane_yards,
                barracks,
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

            *save_file = SaveFile::Data(HashMap::new());
            let Ok(data) = ron::ser::to_string(&base_save_state) else { return; };
            save_file.insert(BASE_LABEL, data);
            file = Some(event.finished());
        }
        if let Some(event) = file {
            save_events.p1().write(event);
        }
    }

    pub fn load_game(

        mut load_events: EventReader<LoadEvent>,
        load_file: Res<SaveFile>,


        mut load_level: EventWriter<LoadLevels>,
        mut load_map: EventWriter<MapLoadEvent>,
        mut load_objects: EventWriter<LoadObjects>,

        mut status: ResMut<LoadingStatus>,

        mut commands: Commands,
    ) {
        if load_events.read().filter(|le| le.loading()).next().is_none() { return; };
        let Some(data) = load_file.get(&BASE_LABEL) else { return; };
        let Ok(base_save_state): Result<BaseSaveState, _> = ron::de::from_str(&data) else { error!("Error parsing base save file"); return; };

        commands.insert_resource(base_save_state.commanders);

        let level_load_event = LoadLevels{ };
        load_level.write(level_load_event);

        let map_load_event = MapLoadEvent {
            map_serde: base_save_state.map.clone()
        };
        load_map.write(map_load_event);

        let objects = base_save_state.objects;
        for object in &objects.crane_yards { status.crane_yards_loaded = Some(false); load_objects.write(object.clone().into()); }
        for object in &objects.resource_nodes { status.resource_nodes_loaded = Some(false); load_objects.write(object.clone().into()); }
        for object in &objects.factories { status.factories_loaded = Some(false); load_objects.write(object.clone().into()); }
        for object in &objects.marine_squads { status.marines_loaded = Some(false); load_objects.write(object.clone().into()); }
        for object in &objects.tanks { status.tanks_loaded = Some(false); load_objects.write(object.clone().into()); }
    }

    pub fn finish_loading_game(
        status: Res<LoadingStatus>,
        mut load_file: ResMut<SaveFile>,
    ) {
        if status.complete() {
            load_file.set_finished(BASE_LABEL);
        }
    }
}

impl Plugin for DiskPlugin {
    fn build(&self, app: &mut App) {
        //TODO: Fix running every frame.
        app
            // .add_event::<>()
            .insert_resource(LoadingStatus::default())
            .add_systems(Update, Self::save_game.run_if(resource_exists::<Commanders>).run_if(resource_exists::<MapSerde>).run_if(resource_exists::<SaveFile>))
            // .add_systems(Update, Self::load_game.run_if(resource_exists::<ObjectPrefabs>).run_if(resource_exists::<SaveFile>))
            .add_systems(Update, Self::load_game.run_if(resource_exists::<ObjectPrefabs>).run_if(resource_exists::<SaveFile>))
            .add_systems(Update, Self::finish_loading_game.run_if(resource_exists::<SaveFile>))
        ;
    }
}