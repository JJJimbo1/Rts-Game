
#[derive(Debug, Clone)]
pub struct Manifest {
    pub saves_path: &'static str,
    pub maps_path: &'static str,
    pub objects_path: &'static str,
    pub colliders_path: &'static str
    //missions : Vec<LevelData>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            saves_path: "/assets/saves/objects/",
            maps_path: "/assets/maps/",
            objects_path: "/assets/objects/",
            colliders_path: "/assets/models/",
        }
    }
}
