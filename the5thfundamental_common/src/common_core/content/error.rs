use std::fmt::Display;

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
    MissingTurret,
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
            Self::MissingTurret => { "missing 'turret'" },
            Self::MissingColliderString => { "missing 'collider_string'" },
            Self::ColliderDecodeError => { "malformed 'collider_string'" }
        };
        write!(f, "Error loading prefab: {}", reason)
    }
}
