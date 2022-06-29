pub use entity_cleanup::*;
mod entity_cleanup {
    
    use bevy::{ecs::entity::Entity, prelude::Query};

    use crate::ResourceNode;

    #[derive(Debug, Clone, Default)]
    pub struct DirtyEntities {
        pub entities : Vec<Entity>,
    }

    // pub fn entity_cleanup(
    //     dirties: Res<DirtyEntities>,
    //     resource_nodes: Query<ResourceNode>,
    //     resource_platform

    // ) {

    // }
}