use bevy::prelude::*;
use bevy_pathfinding::{PathFinder, Path};
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct Tank;

impl AssetId for Tank {
    fn id(&self) -> &'static str {
        ObjectType::from(*self).id()
    }
}

impl From<Tank> for ObjectType {
    fn from(_: Tank) -> Self {
        ObjectType::Tank
    }
}

impl From<Tank> for AssetType {
    fn from(_: Tank) -> Self {
        Self::Object(ObjectType::Tank)
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct TankBundle {
    pub tank: Tank,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub health: Health,
    pub snowflake: Snowflake,
    pub path_finder: GroundPathFinder,
    pub path: Path,
    pub mobile_object: MobileObject,
    pub weapon_set: WeaponSet,
    pub team_player: TeamPlayer,
    pub selectable: Selectable,
    pub velocity: Velocity,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl TankBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        self.team_player = spawn_data.team_player;
        self.transform = spawn_data.transform;
        self
    }
}

impl From<TankPrefab> for TankBundle {
    fn from(prefab: TankPrefab) -> Self {
        Self {
            tank: Tank,
            object_type: Tank.into(),
            asset_type: Tank.into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            path_finder: GroundPathFinder::default(),
            path: Path::default(),
            mobile_object: prefab.mobile_object,
            weapon_set: prefab.weapon_set,
            team_player: TeamPlayer::default(),
            selectable: Selectable::multiselect(),
            velocity: Velocity::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.real_collider.clone().unwrap(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(SerdeTank, &TankPrefab)> for TankBundle {
    fn from((save, prefab): (SerdeTank, &TankPrefab)) -> Self {
        Self {
            tank: Tank,
            object_type: Tank.into(),
            asset_type: Tank.into(),
            snowflake: save.snowflake.unwrap_or_else(|| Snowflake::new()),
            health: save.health.unwrap_or(prefab.health),
            path_finder: save.path_finder.unwrap_or_default(),
            path: save.path.unwrap_or_default(),
            mobile_object: save.mobile_object.unwrap_or(prefab.mobile_object),
            weapon_set: save.weapon_set.unwrap_or(prefab.weapon_set.clone()),
            team_player: save.team_player,
            velocity: save.velocity.unwrap_or(SerdeVelocity::default()).into(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.real_collider.clone().unwrap(),
            selectable: Selectable::multiselect(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}


#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct TankPrefab {
    pub stack: (ActiveQueue, StackData),
    pub health: Health,
    pub mobile_object: MobileObject,
    pub weapon_set: WeaponSet,
    pub collider_string: String,
    #[serde(skip)]
    pub real_collider: Option<Collider>,
}

impl TankPrefab {
    pub fn with_real_collider(mut self, collider: Collider) -> Self {
        self.real_collider = Some(collider);
        self
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SerdeTank {
    pub snowflake: Option<Snowflake>,
    pub health: Option<Health>,
    pub path_finder: Option<GroundPathFinder>,
    pub path: Option<Path>,
    pub mobile_object: Option<MobileObject>,
    pub weapon_set: Option<WeaponSet>,
    pub velocity: Option<SerdeVelocity>,
    pub team_player: TeamPlayer,
    pub transform: SerdeTransform,
}

impl<'a> From<SerdeTankQuery<'a>> for SerdeTank {
    fn from(object: SerdeTankQuery) -> Self {
        Self {
            snowflake: Some(*object.0),
            health: object.1.saved(),
            path_finder: object.2.saved(),
            path: object.3.clone().saved(),
            mobile_object: object.4.saved(),
            weapon_set: object.5.saved(),
            velocity: SerdeVelocity::from(*object.6).saved(),
            team_player: *object.7,
            transform: (*object.8).into(),
        }
    }
}
