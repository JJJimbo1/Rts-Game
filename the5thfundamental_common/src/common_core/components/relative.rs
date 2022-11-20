use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{SerdeComponent, SerdeTransform};



#[derive(Debug, Default, Clone, Copy)]
#[derive(Component)]
pub struct Relative<D = Transform> {
    pub entity: Option<Entity>,
    pub data: D,
}

impl From<SerdeTurret> for Relative {
    fn from(value: SerdeTurret) -> Self {
        Self {
            entity: None,
            data: value.transform.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct SerdeTurret {
    pub transform: SerdeTransform
}

impl From<Relative> for SerdeTurret {
    fn from(value: Relative) -> Self {
        Self {
            transform: value.data.into()
        }
    }
}

impl SerdeComponent for SerdeTurret {
    fn saved(&self) -> Option<Self> {
        let transform = Transform::from(self.transform);
        // if transform.translation.x > 0.0 {

        // }



        Some(*self)
    }
}