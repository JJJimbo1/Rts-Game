use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{SerdeComponent, SerdeTransform};

#[derive(Debug, Default, Clone, Copy)]
#[derive(Component)]
pub struct Relative<D = Transform> {
    pub entity: Option<Entity>,
    pub data: D,
}

impl From<Turret> for Relative {
    fn from(value: Turret) -> Self {
        Self {
            entity: None,
            data: value.transform.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct Turret {
    pub transform: SerdeTransform
}

impl From<Relative> for Turret {
    fn from(value: Relative) -> Self {
        Self {
            transform: value.data.into()
        }
    }
}

impl SerdeComponent for Turret {
    fn saved(&self) -> Option<Self> {
        Some(*self)
    }
}