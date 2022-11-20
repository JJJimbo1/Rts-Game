use bevy::prelude::*;
use bevy_pathfinding::Path;
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct TankBase(Option<(Entity, TankGun)>);

impl AssetId for TankBase {
    fn id(&self) -> Option<&'static str> {
        ObjectType::from(*self).id()
    }
}

impl From<TankBase> for ObjectType {
    fn from(_: TankBase) -> Self {
        ObjectType::TankBase
    }
}

impl From<TankBase> for AssetType {
    fn from(_: TankBase) -> Self {
        Self::Object(ObjectType::TankBase)
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct TankBaseBundle {
    pub tank: TankBase,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub health: Health,
    pub snowflake: Snowflake,
    pub path_finder: GroundPathFinder,
    pub path: Path,
    pub controller: Controller,
    pub weapon_set: WeaponSet,
    pub turret: Turret,
    pub team_player: TeamPlayer,
    pub selectable: Selectable,
    pub velocity: Velocity,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl TankBaseBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }
}

impl From<TankBasePrefab> for TankBaseBundle {
    fn from(prefab: TankBasePrefab) -> Self {
        Self {
            tank: TankBase::default(),
            object_type: TankBase::default().into(),
            asset_type: TankBase::default().into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            path_finder: GroundPathFinder::default(),
            path: Path::default(),
            controller: prefab.controller,
            weapon_set: prefab.weapon_set,
            turret: prefab.turret.into(),
            team_player: TeamPlayer::default(),
            selectable: Selectable::multiselect(),
            velocity: Velocity::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.real_collider.clone().unwrap(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(SerdeTankBase, &TankBasePrefab)> for TankBaseBundle {
    fn from((save, prefab): (SerdeTankBase, &TankBasePrefab)) -> Self {
        Self {
            tank: TankBase::default(),
            object_type: TankBase::default().into(),
            asset_type: TankBase::default().into(),
            snowflake: save.snowflake.unwrap_or_else(|| Snowflake::new()),
            health: save.health.unwrap_or(prefab.health),
            path_finder: save.path_finder.unwrap_or_default(),
            path: save.path.unwrap_or_default(),
            controller: save.controller.unwrap_or(prefab.controller),
            weapon_set: save.weapon_set.unwrap_or(prefab.weapon_set.clone()),
            turret: save.turret.unwrap_or(prefab.turret).into(),
            team_player: save.team_player,
            velocity: save.velocity.unwrap_or(SerdeVelocity::default()).into(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.real_collider.clone().unwrap(),
            selectable: Selectable::multiselect(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}


#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct TankBasePrefab {
    pub stack: (ActiveQueue, StackData),
    pub health: Health,
    pub controller: Controller,
    pub weapon_set: WeaponSet,
    pub turret: SerdeTurret,
    pub collider_string: String,
    #[serde(skip)]
    pub real_collider: Option<Collider>,
}

impl TankBasePrefab {
    pub fn with_real_collider(mut self, collider: Collider) -> Self {
        self.real_collider = Some(collider);
        self
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SerdeTankBase {
    pub snowflake: Option<Snowflake>,
    pub health: Option<Health>,
    pub path_finder: Option<GroundPathFinder>,
    pub path: Option<Path>,
    pub controller: Option<Controller>,
    pub weapon_set: Option<WeaponSet>,
    pub turret: Option<SerdeTurret>,
    pub velocity: Option<SerdeVelocity>,
    pub team_player: TeamPlayer,
    pub transform: SerdeTransform,
}

impl<'a> From<SerdeTankBaseQuery<'a>> for SerdeTankBase {
    fn from(object: SerdeTankBaseQuery) -> Self {
        Self {
            snowflake: Some(*object.0),
            health: object.1.saved(),
            path_finder: object.2.saved(),
            path: object.3.saved(),
            controller: object.4.saved(),
            weapon_set: object.5.saved(),
            turret: SerdeTurret::from(*object.6).saved(),
            velocity: SerdeVelocity::from(*object.7).saved(),
            team_player: *object.8,
            transform: (*object.9).into(),
        }
    }
}

pub fn tank_spawn(
    prefabs: Res<ObjectPrefabs>,
    mut resource_nodes: Query<(Entity, &Transform), Added<TankBase>>,
    mut commands: Commands,
) {
    resource_nodes.for_each_mut(|(entity, transform)| {
        println!("Spawning Tank");

        let tank_turret_offset = Transform::from(prefabs.tank_prefab.turret.transform);

        // let tank_turret_offset = Transform::from_translation(Vec3::new(0.0, 1.81797, -0.28511));
        let spawn_data = ObjectSpawnEventData {
            snowflake: Snowflake::new(),
            object_type: ObjectType::TankGun,
            teamplayer: TeamPlayer::default(),
            transform: tank_turret_offset,
        };

        commands.entity(entity).with_children(|parent| {
            parent.spawn(TankGunBundle::default().with_spawn_data(spawn_data));
        });
    });
}