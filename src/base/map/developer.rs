use avian3d::prelude::Collider;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use superstruct::superstruct;
use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct DeveloperMarker;

impl From<DeveloperMarker> for MapType {
    fn from(_: DeveloperMarker) -> Self {
        Self::Developer
    }
}

#[superstruct{
    variants(Bundle, Prefab, Serde),
    variant_attributes(derive(Debug, Clone)),
    specific_variant_attributes(
        Bundle(derive(Bundle)),
        Serde(derive(Default, Copy, Serialize, Deserialize)),
    ),
}]
pub struct Developer {
    #[superstruct(only(Prefab))]            pub bounds: MapBounds,
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub developer: DeveloperMarker,
    #[superstruct(only(Bundle))]            pub map_type: MapType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub transform: Transform,
}

impl TryFrom<&MapAsset> for DeveloperPrefab {
    type Error = ContentError;
    fn try_from(map : &MapAsset) -> Result<Self, ContentError> {
        let Some(bounds) = map.bounds else { return Err(ContentError::MissingBounds); };
        let Some(collider_string) = map.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let collider = Collider::trimesh(vertices, indices);

        Ok(Self {
            bounds,
            collider,
        })
    }
}

impl From<DeveloperPrefab> for DeveloperBundle {
    fn from(prefab: DeveloperPrefab) -> Self {
        Self {
            developer: DeveloperMarker,
            map_type: DeveloperMarker.into(),
            snowflake: Snowflake::new(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            transform: Transform::default(),
        }
    }
}

impl From<(DeveloperSerde, &DeveloperPrefab)> for DeveloperBundle {
    fn from((_save, prefab): (DeveloperSerde, &DeveloperPrefab)) -> Self {
        Self {
            developer: DeveloperMarker,
            map_type: DeveloperMarker.into(),
            snowflake: Snowflake::new(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            transform: Transform::default(),
        }
    }
}