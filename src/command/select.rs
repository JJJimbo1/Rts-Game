use bevy::prelude::Component;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SelectableType {
    ///This entity cannot be selected at the same time as other entities.
    Single,
    ///This entity can be selected at the same time as other entities.
    MultiSelect,
    //Attempting to select this entity will clear selection.
    Clear,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct Selectable {
    pub selected: bool,
    pub context: SelectableType,
}

impl Selectable {
    pub fn single() -> Self {
        Self {
            selected: false,
            context: SelectableType::Single
        }
    }

    pub fn multiselect() -> Self {
        Self {
            selected: false,
            context: SelectableType::MultiSelect
        }
    }

    pub fn clear() -> Self {
        Self {
            selected: false,
            context: SelectableType::Clear
        }
    }
}
