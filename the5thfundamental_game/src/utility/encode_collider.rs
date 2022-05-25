use std::path::Path;

use bevy::math::Vec3;

pub fn extract_trimesh<P: AsRef<Path>>(path: P) -> Option<(Vec<Vec3>, Vec<[u32; 3]>)> {
    if let Ok(scenes) = easy_gltf::load(&path) {
        if let Some(scene) = scenes.first() {
            if let Some(model) = scene.models.get(0) {
                let vertices = model.vertices().iter().map(|v| Vec3::new(v.position.x, v.position.y, v.position.z)).collect::<Vec<Vec3>>();
                let indices = model.indices().unwrap_or(&Vec::default()).chunks_exact(3).map(|i| [i[0] as u32, i[1] as u32, i[2] as u32]).collect::<Vec<[u32; 3]>>();
                return Some((vertices, indices));
            }
        }
    }
    None
}

pub fn encode(trimesh: (Vec<Vec3>, Vec<[u32; 3]>)) -> Option<String> {
    if let Ok(bytes) = bincode::serialize(&trimesh) {
        let encoded = base64::encode(&bytes);
        return Some(encoded);
    }
    None
}