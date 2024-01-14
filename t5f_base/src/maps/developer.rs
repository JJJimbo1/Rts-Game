use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use serde::{Serialize, Deserialize};
use superstruct::superstruct;
use t5f_common::{Snowflake, MapBounds};
use t5f_utility::colliders::decode;
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

impl From<DeveloperMarker> for AssetType {
    fn from(_: DeveloperMarker) -> Self {
        Self::Map(MapType::Developer)
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
    #[superstruct(only(Bundle))]            pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub view_visibility: ViewVisibility,
    #[superstruct(only(Bundle))]            pub inherited_visibility: InheritedVisibility,
    #[superstruct(only(Bundle))]            pub transform: TransformBundle,
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
            asset_type: DeveloperMarker.into(),
            snowflake: Snowflake::new(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: TransformBundle::default(),
        }
    }
}

impl From<(DeveloperSerde, &DeveloperPrefab)> for DeveloperBundle {
    fn from((_save, prefab): (DeveloperSerde, &DeveloperPrefab)) -> Self {
        Self {
            developer: DeveloperMarker,
            map_type: DeveloperMarker.into(),
            asset_type: DeveloperMarker.into(),
            snowflake: Snowflake::new(),
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: TransformBundle::default(),
        }
    }
}

// impl<'a> From<SerdeDeveloperQuery<'a>> for SerdeDeveloper {
//     fn from(object: SerdeDevelopoerQuery) -> Self {
//         Self {
//             // health: object.0.saved(),
//             // queues: object.1.saved(),
//             // team_player: *object.2,
//             // transform: (*object.3).into(),
//         }
//     }
// }

pub struct DeveloperPlugin;

impl DeveloperPlugin {
    // pub fn load
}

impl Plugin for DeveloperPlugin {
    fn build(&self, _app: &mut App) {

    }
}