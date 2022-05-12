pub use select::*;
mod select {
    use bevy::prelude::Component;
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum SelectableContext {
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
        pub selected : bool,
        pub context : SelectableContext,
    }
}