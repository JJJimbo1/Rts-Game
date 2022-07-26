use std::path::Path;

use serde::{Serialize, Deserialize};
use ronfile::RonFile;
use qloader::*;
use crate::SaveFile;





















#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Level {
    // pub ui_info : UIInfo,
    pub save_state : SaveFile,
}

impl QLoad<()> for Level {
    const PATHTYPE : PathType = PathType::Absolute;
    fn extensions() -> Vec<&'static str> {
        vec!["ron"]
    }
    fn load<S : AsRef<Path>>(_path : S) -> Result<Self, QLoaderError> {
        match RonFile::load::<Self, S>(_path) {
            Ok(x) => {
                Ok(x)
            },
            Err(_e) => {
                Err(QLoaderError::ParseError)
            }
        }
    }
}