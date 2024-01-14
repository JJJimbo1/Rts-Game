use std::marker::PhantomData;

use bevy::{prelude::*, asset::{AssetLoader, io::Reader, AsyncReadExt}, reflect::{TypeUuid, TypePath}};
use serde::{Serialize, Deserialize};
use crate::*;

pub struct AnyLevelMarker;
pub trait LevelMarker { }
impl LevelMarker for AnyLevelMarker { }

#[derive(Debug, Clone)]
#[derive(Event)]
pub struct LevelLoadEvent<M: LevelMarker>(pub LevelLoadEventData, pub PhantomData<M>);

impl<M: LevelMarker> LevelLoadEvent<M> {
    // pub fn spawn_data(&self) -> &SpawnData {
    //     &self.0.spawn_data
    // }

    // pub fn serde_data(&self) -> &LevelSerdeData {
    //     &self.0.serde_data
    // }
}

#[derive(Debug, Clone)]
pub struct LevelLoadEventData {
    // pub object_type: LevelType,
    // pub serde_data: LevelSerdeData,
}

#[derive(Debug, Clone)]
pub struct LevelSerdeData {
    // pub save_state: SaveState,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Asset, TypePath, TypeUuid)]
#[uuid = "215a1291-b752-4d19-97fd-6827d6cbaee0"]
pub struct LevelAsset {
    // pub save_state : SaveState,
}

#[derive(Default)]
pub struct LevelLoader;

impl AssetLoader for LevelLoader {
    type Asset = LevelAsset;
    type Settings = ();
    type Error = LoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<LevelAsset, LoaderError>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let asset = ron::de::from_bytes::<LevelAsset>(&bytes).or_else(|_| bincode::deserialize(&bytes))?;
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["t5flvl"]
    }
}


pub struct LevelPlugin;

impl LevelPlugin {
    pub fn load_level(
        mut load_events: EventReader<LevelLoadEvent<AnyLevelMarker>>,
        // mut load_map: EventWriter<MapLoadEvent<AnyMapMarker>>,
        // mut load_objects: EventWriter<ObjectLoadEvent<AnyObjectMarker>>,
        mut status: ResMut<LoadingStatus>,
        // mut commands: Commands,
    ) {
        for _ in load_events.read() {
            println!("LEVEL LOADED");
            // let save = &event.serde_data().save_state;
            // commands.insert_resource(save.commanders.clone());

            // let event_data = MapLoadEventData {
            //     map: save.map.clone(),
            // };
            // load_map.send(MapLoadEvent(event_data, PhantomData));

            // for object in &save.objects.crane_yards { status.crane_yards_loaded = Some(false); load_objects.send(object.clone().into()); }
            // for object in &save.objects.resource_nodes { status.resource_nodes_loaded = Some(false); load_objects.send(object.clone().into()); }
            // for object in &save.objects.factories { status.factories_loaded = Some(false); load_objects.send(object.clone().into()); }
            // for object in &save.objects.marine_squads { status.marines_loaded = Some(false); load_objects.send(object.clone().into()); }
            // for object in &save.objects.tanks { status.tanks_loaded = Some(false); load_objects.send(object.clone().into()); }
            status.level_loaded = true;
        }
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LevelLoadEvent<AnyLevelMarker>>()
            .add_systems(Update,
                Self::load_level
            )
        ;
    }
}