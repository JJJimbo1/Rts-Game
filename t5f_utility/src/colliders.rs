use std::path::Path;
use bevy::math::Vec3;
use base64::Engine;

pub fn extract_trimesh<P: AsRef<Path>>(path: P) -> Option<(Vec<Vec3>, Vec<[u32; 3]>)> {
    let Some(scenes) = easy_gltf::load(&path).ok() else { return None; };
    let Some(scene) = scenes.first() else { return None; };
    // let Some(model) = {
    //     for m in scene.models {
    //         m.mode()
    //     }
    // }
    let Some(model) = scene.models.first() else { return None; };
    let vertices = model.vertices().iter().map(|v| Vec3::new(v.position.x, v.position.y, v.position.z)).collect::<Vec<Vec3>>();
    let indices = model.indices().unwrap_or(&Vec::default()).chunks_exact(3).map(|i| [i[0] as u32, i[1] as u32, i[2] as u32]).collect::<Vec<[u32; 3]>>();
    Some((vertices, indices))
}

pub fn encode(trimesh: (Vec<Vec3>, Vec<[u32; 3]>)) -> Option<String> {
    if let Ok(bytes) = bincode::serialize(&trimesh) {
        let engine = base64::engine::general_purpose::STANDARD;
        let encoded = engine.encode(&bytes);
        return Some(encoded);
    }
    None
}

pub fn decode(base64_string: String) -> Option<(Vec<Vec3>, Vec<[u32; 3]>)> {
    let engine = base64::engine::general_purpose::STANDARD;
    if let Ok(bytes) = engine.decode(base64_string) {
        if let Ok(decoded) = bincode::deserialize(&bytes) {
            return Some(decoded);
        }
    }
    None
}