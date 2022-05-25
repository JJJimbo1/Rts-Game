use bevy::math::Vec3;

pub fn decode(base64_string: String) -> Option<(Vec<Vec3>, Vec<[u32; 3]>)> {
    if let Ok(bytes) = base64::decode(base64_string) {
        if let Ok(decoded) = bincode::deserialize(&bytes) {
            return Some(decoded);
        }
    }
    None
}