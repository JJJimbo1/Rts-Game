use bevy::{asset::{io::Reader, AssetLoader}, utils::HashMap, prelude::*};
use thiserror::Error;

#[derive(Debug, Clone)]
#[derive(Event)]
pub enum SaveEvent {
    Save(String),
    Finished(String),
}

impl SaveEvent {
    pub fn new(file: String) -> Self {
        Self::Save(file)
    }

    pub fn finished(&self) -> Self {
        Self::Finished(self.file())
    }

    pub fn saving(&self) -> bool {
        match &self {
            &Self::Save(_) => true,
            &Self::Finished(_) => false,
        }
    }

    pub fn finishing(&self) -> bool {
        match &self {
            &Self::Save(_) => false,
            &Self::Finished(_) => true,
        }
    }

    pub fn file(&self) -> String {
        match &self {
            &SaveEvent::Save(file) => file.clone(),
            &SaveEvent::Finished(file) => file.clone()
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Event)]
pub enum LoadEvent {
    Load(String),
    Finished,
}

impl LoadEvent {
    pub fn loading(&self) -> bool {
        match &self {
            &Self::Load(_) => true,
            &Self::Finished => false,
        }
    }

    pub fn file(&self) -> Option<String> {
        match &self {
            &Self::Load(file) => Some(file.clone()),
            &Self::Finished => None
        }
    }
}


#[derive(Debug, Default, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SaveLoadType {
    Saving,
    #[default]
    Loading,
}

#[derive(Debug, Error)]
pub enum SaveLoadError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse RON: {0}")]
    RonDeserialization(#[from] ron::error::SpannedError),
    #[error("Could not parse Binary: {0}")]
    BincodeDeserialization(#[from] bincode::Error),
}










use std::{path::Path, fs::OpenOptions, io::Write};
use serde::Serialize;
use ron::{ser::{PrettyConfig, to_string_pretty}, extensions::Extensions};
use log::error;

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
                        return Err(SaveLoadError::Io(e));
                    }
                }
            },
            Err(e) => { log::error!("{}", e); return Err(SaveLoadError::Io(e)); }
    };
}
/*
pub fn load_from_file<D, P : AsRef<Path>>(path : P) -> Result<D, SaveLoadError> where for<'de> D : Deserialize<'de> {
    match OpenOptions::new()
        .read(true)
        .write(false)
        .open(&path) {
            Ok(x) => {
                match from_reader::<File, D>(x) {
                    Ok(d) => { return Ok(d); }
                    Err(_e) => {
                        error!("{}", std::any::type_name::<D>());
                        error!("{}", path.as_ref().display());
                        error!("{}", _e);
                    }
                }
            },
            Err(_e) => { }
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
                error!("{}", std::any::type_name::<D>());
                error!("{}", path.as_ref().display());
                error!("{}", _e);
            }
    };
    return Err(SaveLoadError::FileReadError);
}



*/


#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Resource, Asset, TypePath)]
pub enum SaveFile {
    None,
    File(String),
    #[serde(skip)]
    Handle(Handle<Self>),
    Data(HashMap<String, (String, bool)>),
}

impl SaveFile {
    pub fn new(file: String) -> Self {
        Self::File(file)
    }

    pub fn file(&self) -> Option<&String> {
        if let Self::File(file) = self { Some(file) } else { None }
    }

    pub fn handle(&self) -> Option<Handle<Self>> {
        if let Self::Handle(handle) = self {
            Some(handle.clone())
        } else {
            None
        }
    }

    pub fn data(&self) -> Option<&HashMap<String, (String, bool)>> {
        if let Self::Data(data) = self {
            Some(data)
        } else {
            None
        }
    }

    pub fn get(&self, label: &str) -> Option<&String> {
        if let Self::Data(data) = self {
            data.get(&label.to_owned()).and_then(|(data, _)| Some(data))
        } else {
            None
        }
    }

    pub fn insert(&mut self, label: &str, bytes: String) {
        if let Self::Data(data) = self {
            data.insert(label.to_owned(), (bytes, false));
        }
    }

    pub fn set_finished(&mut self, label: &str) {
        if let Self::Data(data) = self {
            let Some(mut _mod) = data.get_mut(&label.to_owned()) else { return; };
            _mod.1 = true;
        }
    }

    pub fn all_loaded(&self) -> bool {
        if let Self::Data(data) = self {
            for (_, loaded) in data.values() {
                if !loaded { return false; }
            }
            return true;
        }
        return false;
    }

    pub fn reset(&mut self) {
        if let Self::Data(data) = self {
            for (_, loaded) in data.values_mut() {
                *loaded = false;
            }
        }
    }
}

#[derive(Default)]
pub struct SaveFileAssetLoader;

impl AssetLoader for SaveFileAssetLoader {
    type Asset = SaveFile;
    type Settings = ();
    type Error = SaveLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = ron::de::from_bytes::<SaveFile>(&bytes)?;//.or_else(|_| bincode::deserialize(&bytes))?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["t5fsav"]
    }
}



pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_asset::<SaveFile>()
            .init_asset_loader::<SaveFileAssetLoader>()

            .insert_resource(SaveFile::None)

            .add_event::<SaveEvent>()
            .add_event::<LoadEvent>();
    }
}