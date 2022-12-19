use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Default, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Marine;

// impl AssetId for Marine {
//     fn id(&self) -> Option<&'static str> {
//         ObjectType::from(*self).id()
//     }
// }

impl From<Marine> for ObjectType {
    fn from(_: Marine) -> Self {
        ObjectType::Marine
    }
}

impl From<Marine> for AssetType {
    fn from(_: Marine) -> Self {
        Self::Object(ObjectType::Marine)
    }
}


#[derive(Clone)]
#[derive(Bundle)]
pub struct MarineBundle {
    pub marine: Marine,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub snowflake: Snowflake,
    // pub health: Health,
    // pub path_finder: GroundPathFinder,
    // pub path: Path,
    // pub controller: Controller,
    // pub weapon_set: WeaponSet,
    pub team_player: TeamPlayer,
    // pub selectable: Selectable,
    // pub velocity: Velocity,
    // pub rigid_body: RigidBody,
    // pub collider: Collider,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl MarineBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }
}

impl Default for MarineBundle {
    fn default() -> Self {
        Self {
            marine: Marine,
            object_type: Marine.into(),
            asset_type: Marine.into(),
            snowflake: Snowflake::new(),
            // health: prefab.health,
            // path_finder: GroundPathFinder::default(),
            // path: Path::default(),
            // controller: prefab.controller,
            // weapon_set: prefab.weapon_set,
            team_player: TeamPlayer::default(),
            // selectable: Selectable::multiselect(),
            // velocity: Velocity::default(),
            // rigid_body: RigidBody::KinematicVelocityBased,
            // collider: prefab.real_collider.clone().unwrap(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

// impl From<(SerdeMarine, &MarinePrefab)> for MarineBundle {
//     fn from((save, prefab): (SerdeMarine, &MarinePrefab)) -> Self {
//         Self {
//             marine: Marine,
//             object_type: Marine::default().into(),
//             asset_type: Marine::default().into(),
//             snowflake: save.snowflake.unwrap_or_else(|| Snowflake::new()),
//             // health: save.health.unwrap_or(prefab.health),
//             // path_finder: save.path_finder.unwrap_or_default(),
//             // path: save.path.unwrap_or_default(),
//             // controller: save.controller.unwrap_or(prefab.controller),
//             // weapon_set: save.weapon_set.unwrap_or(prefab.weapon_set.clone()),
//             // team_player: save.team_player,
//             // velocity: save.velocity.unwrap_or(SerdeVelocity::default()).into(),
//             // rigid_body: RigidBody::KinematicVelocityBased,
//             // collider: prefab.real_collider.clone().unwrap(),
//             // selectable: Selectable::multiselect(),
//             transform: save.transform.into(),
//             global_transform: GlobalTransform::default(),
//         }
//     }
// }


// #[derive(Clone)]
// #[derive(Serialize, Deserialize)]
// pub struct MarinePrefab {
//     // pub stack: (ActiveQueue, StackData),
//     // pub health: Health,
//     // pub controller: Controller,
//     // pub weapon_set: WeaponSet,
//     // pub collider_string: String,
//     // #[serde(skip)]
//     // pub real_collider: Option<Collider>,
// }

// impl MarinePrefab {
//     pub fn with_real_collider(mut self, collider: Collider) -> Self {
//         self.real_collider = Some(collider);
//         self
//     }
// }

// #[derive(Debug, Clone)]
// #[derive(Serialize, Deserialize)]
// pub struct SerdeMarine {
//     pub snowflake: Option<Snowflake>,
//     // pub health: Option<Health>,
//     // pub path_finder: Option<GroundPathFinder>,
//     // pub path: Option<Path>,
//     // pub controller: Option<Controller>,
//     // pub weapon_set: Option<WeaponSet>,
//     // pub velocity: Option<SerdeVelocity>,
//     // pub team_player: TeamPlayer,
//     pub transform: SerdeTransform,
// }

// impl<'a> From<SerdeMarineQuery<'a>> for SerdeMarine {
//     fn from(object: SerdeMarineQuery) -> Self {
//         Self {
//             snowflake: Some(*object.0),
//             // marine_: object.1.saved(),
//             // health: object.2.saved(),
//             // path_finder: object.3.saved(),
//             // path: object.4.saved(),
//             // controller: object.5.saved(),
//             // weapon_set: object.6.saved(),
//             // velocity: SerdeVelocity::from(*object.7).saved(),
//             // team_player: *object.8,
//             transform: (*object.1).into(),
//         }
//     }
// }
