use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse RON: {0}")]
    RDeserialization(#[from] ron::error::SpannedError),
    #[error("Could not parse Binary: {0}")]
    BDeserialization(#[from] bincode::Error),
}

#[derive(Debug, Clone, Copy)]
pub enum FailureReason {
    LevelNotFound,
    MapNotFound,
}

#[derive(Debug, Clone, Copy)]
pub enum ContentError {
    MissingBounds,
    MissingStack,
    MissingHealth,
    MissingQueues,
    MissingEconomic,
    MissingSquad,
    MissingController,
    MissingWeapons,
    MissingReference,
    MissingColliderString,
    ColliderDecodeError,
}

impl Display for ContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason = match self {
            Self::MissingBounds => { "missing 'bounds'" },
            Self::MissingStack => { "missing 'stack'" },
            Self::MissingHealth => { "missing 'health'" },
            Self::MissingQueues => { "missing 'prefab_queues'" },
            Self::MissingEconomic => { "missing 'economic_object'" },
            Self::MissingSquad => { "missing 'prefab_squad'" },
            Self::MissingController => { "missing 'controller'" },
            Self::MissingWeapons => { "missing 'weapon_set'" },
            Self::MissingReference => { "missing 'reference'" },
            Self::MissingColliderString => { "missing 'collider_string'" },
            Self::ColliderDecodeError => { "malformed 'collider_string'" }
        };
        write!(f, "Error loading prefab: {}", reason)
    }
}