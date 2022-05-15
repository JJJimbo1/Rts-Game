pub use entity_cleanup::*;
mod entity_cleanup {
    
    use bevy::ecs::entity::Entity;

    #[derive(Debug, Clone, Default)]
    pub struct DirtyEntities {
        pub entities : Vec<Entity>,
    }
}