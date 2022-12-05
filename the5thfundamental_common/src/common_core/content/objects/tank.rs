use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct TankBase;
// pub struct TankBase(Option<(Entity, TankGun)>);

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
    pub turret: Relative,
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
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(SerdeTank, &TankBasePrefab)> for TankBaseBundle {
    fn from((save, prefab): (SerdeTank, &TankBasePrefab)) -> Self {
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
            collider: prefab.collider.clone(),
            selectable: Selectable::multiselect(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}


#[derive(Clone)]
pub struct TankBasePrefab {
    pub health: Health,
    pub controller: Controller,
    pub weapon_set: WeaponSet,
    pub turret: Turret,
    pub collider: Collider,
}

impl TryFrom<&ObjectAsset> for TankBasePrefab {
    type Error = ContentError;
    fn try_from(prefab: &ObjectAsset) -> Result<Self, ContentError> {
        let Some(health) = prefab.health else { return Err(ContentError::MissingHealth); };
        let Some(controller) = prefab.controller else { return Err(ContentError::MissingController); };
        let Some(weapon_set) = prefab.weapon_set.clone() else { return Err(ContentError::MissingWeapons); };
        let Some(collider_string) = prefab.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some(turret) = prefab.turret else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let collider = Collider::trimesh(vertices, indices);

        Ok(Self {
            health,
            controller,
            weapon_set,
            turret: turret.into(),
            collider,
        })
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SerdeTank {
    pub snowflake: Option<Snowflake>,
    pub health: Option<Health>,
    pub path_finder: Option<GroundPathFinder>,
    pub path: Option<Path>,
    pub controller: Option<Controller>,
    pub weapon_set: Option<WeaponSet>,
    pub turret: Option<Turret>,
    pub velocity: Option<SerdeVelocity>,
    pub team_player: TeamPlayer,
    pub transform: SerdeTransform,
}

impl<'a> From<SerdeTankBaseQuery<'a>> for SerdeTank {
    fn from(object: SerdeTankBaseQuery) -> Self {
        Self {
            snowflake: Some(*object.0),
            health: object.1.saved(),
            path_finder: object.2.saved(),
            path: object.3.saved(),
            controller: object.4.saved(),
            weapon_set: object.5.saved(),
            turret: Turret::from(*object.6).saved(),
            velocity: SerdeVelocity::from(*object.7).saved(),
            team_player: *object.8,
            transform: (*object.9).into(),
        }
    }
}

impl From<SerdeTank> for ObjectSpawnEvent {
    fn from(value: SerdeTank) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::TankBase,
            snowflake: Snowflake::new(),
            teamplayer: value.team_player,
            transform: value.transform.into(),
        })
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct TankGun;

impl AssetId for TankGun {
    fn id(&self) -> Option<&'static str> {
        ObjectType::from(*self).id()
    }
}

impl From<TankGun> for ObjectType {
    fn from(_: TankGun) -> Self {
        ObjectType::TankGun
    }
}

impl From<TankGun> for AssetType {
    fn from(_: TankGun) -> Self {
        Self::Object(ObjectType::TankGun)
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct TankGunBundle {
    pub tank_gun: TankGun,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub snowflake: Snowflake,
    pub teamplayer: TeamPlayer,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl TankGunBundle {
    pub fn with_spawn_data(mut self, spawn_data: &ObjectSpawnEventData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.teamplayer = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }
}

impl Default for TankGunBundle {
    fn default() -> Self {
        Self {
            tank_gun: TankGun,
            object_type: TankGun.into(),
            asset_type: TankGun.into(),
            snowflake: Snowflake::new(),
            teamplayer: TeamPlayer::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TankPlugin;

impl TankPlugin {
    pub fn spawn_tank(
        mut spawn_events: EventReader<ObjectSpawnEvent>,
        prefabs: Res<ObjectPrefabs>,
        mut new_tanks: Query<&mut Relative, Added<TankBase>>,
        new_tank_guns: Query<(Entity, &Parent), Added<TankGun>>,

        mut identifiers: ResMut<Identifiers>,
        mut commands: Commands,
    ) {
        spawn_events.iter().filter_map(|event| (event.0.object_type == ObjectType::TankBase).then_some(event.0)).for_each(|data| {
            let tank_turret_transform = Transform::from(prefabs.tank_prefab.turret.transform);
            let gun_spawn_data = ObjectSpawnEventData {
                object_type: ObjectType::TankGun,
                snowflake: Snowflake::new(),
                teamplayer: TeamPlayer::default(),
                transform: tank_turret_transform,
            };

            let mut gun_entity = None;
            let base_entity = commands.spawn(TankBaseBundle::from(prefabs.tank_prefab.clone()).with_spawn_data(data)).with_children(|parent| {
                gun_entity = Some(parent.spawn(TankGunBundle::default().with_spawn_data(&gun_spawn_data)).id());
            }).id();

            identifiers.insert(data.snowflake, base_entity);
            identifiers.insert(gun_spawn_data.snowflake, gun_entity.unwrap());
        });

        new_tank_guns.for_each(|(entity, parent)| {
            let Ok(mut relative) = new_tanks.get_mut(parent.get()) else { return; };
            relative.entity = Some(entity);
        });

    }

    pub fn aim_tank_gun(
        mut transforms: Query<&mut Transform>,
        global_transforms: Query<&mut GlobalTransform>,
        mut weapons: Query<(Entity, &mut Relative, &WeaponSet)>,
    ) {
        weapons.for_each_mut(|(entity, mut relative, weapon_set)| {
            let Some(transform) = transforms.get(entity).ok().cloned() else { return; };
            let Some(global_gun_transform) = relative.entity.and_then(|gun_entity| global_transforms.get(gun_entity).ok()) else { return; };
            if let Some(mut gun_transform) = relative.entity.and_then(|gun_entity| transforms.get_mut(gun_entity).ok()) {
                let desired_rotation = if let Some(global_target_transform) = weapon_set.weapons.get(0).and_then(|weapon| weapon.target.get_target()).and_then(|t| global_transforms.get(t).ok().cloned()) {
                    let new_transform = Transform::from(*global_gun_transform).looking_at(global_target_transform.translation(), Vec3::Y);
                    new_transform.rotation * transform.rotation.inverse()
                } else {
                    Quat::IDENTITY
                };

                let difference = gun_transform.rotation.angle_between(desired_rotation);
                let speed = 0.025 / difference;
                let new_rotation = gun_transform.rotation.slerp(desired_rotation, speed.clamp(0.0, 1.0));
                gun_transform.rotation = new_rotation;
                relative.data = *gun_transform;
            }

        });

    }
}