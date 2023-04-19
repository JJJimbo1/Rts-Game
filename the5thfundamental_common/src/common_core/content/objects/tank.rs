use bevy::{prelude::*, ecs::schedule::StateData};
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
use superstruct::*;
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct TankBaseMarker;
// pub struct TankBase(Option<(Entity, TankGun)>);

// impl AssetId for TankBase {
//     fn id(&self) -> Option<&'static str> {
//         ObjectType::from(*self).id()
//     }
// }

impl From<TankBaseMarker> for ObjectType {
    fn from(_: TankBaseMarker) -> Self {
        ObjectType::TankBase
    }
}

impl From<TankBaseMarker> for AssetType {
    fn from(_: TankBaseMarker) -> Self {
        Self::Object(ObjectType::TankBase)
    }
}


#[superstruct{
    variants(Bundle, Prefab, Serde),
    variant_attributes(derive(Debug, Clone)),
    specific_variant_attributes(
        Bundle(derive(Bundle)),
        Serde(derive(Serialize, Deserialize)),
    ),
}]
#[derive(Debug, Clone)]
pub struct TankBase {
    #[superstruct(only(Prefab))]            pub turret: Turret,
    #[superstruct(only(Bundle, Prefab))]    pub collider: Collider,
    #[superstruct(only(Bundle, Prefab))]    pub health: Health,
    #[superstruct(only(Bundle, Prefab))]    pub controller: Controller,
    #[superstruct(only(Bundle, Prefab))]    pub weapon_set: WeaponSet,
    #[superstruct(only(Bundle))]            pub tank_marker: TankBaseMarker,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub path_finder: GroundPathFinder,
    #[superstruct(only(Bundle))]            pub path: Path,
    #[superstruct(only(Bundle))]            pub relative: Relative,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle))]            pub velocity: Velocity,
    #[superstruct(only(Bundle))]            pub rigid_body: RigidBody,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub computed_visibility: ComputedVisibility,
    #[superstruct(only(Bundle))]            pub global_transform: GlobalTransform,
    #[superstruct(only(Bundle, Serde))]     pub team_player: TeamPlayer,
    #[superstruct(only(Bundle, Serde))]     pub transform: Transform,
    #[superstruct(only(Serde))]             pub serde_snowflake: Option<Snowflake>,
    #[superstruct(only(Serde))]             pub serde_health: Option<Health>,
    #[superstruct(only(Serde))]             pub serde_path_finder: Option<GroundPathFinder>,
    #[superstruct(only(Serde))]             pub serde_path: Option<Path>,
    #[superstruct(only(Serde))]             pub serde_controller: Option<Controller>,
    #[superstruct(only(Serde))]             pub serde_weapon_set: Option<WeaponSet>,
    #[superstruct(only(Serde))]             pub serde_turret: Option<Turret>,
    #[superstruct(only(Serde))]             pub serde_velocity: Option<SerdeVelocity>,
}

impl TankBaseBundle {
    pub fn with_spawn_data(mut self, spawn_data: SpawnData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }

    pub fn with_serde_data(mut self, serde_data: Option<SerdeData>) -> Self {
        let Some(serde_data) = serde_data else { return self; };
        if let Some(health) = serde_data.health { self.health = health; }
        if let Some(path_finder) = serde_data.path_finder { self.path_finder = path_finder; }
        if let Some(path) = serde_data.path { self.path = path; }
        if let Some(controller) = serde_data.controller { self.controller = controller; }
        if let Some(weapon_set) = serde_data.weapon_set { self.weapon_set = weapon_set; }
        if let Some(turret) = serde_data.turret { self.relative = turret.into(); }
        if let Some(velocity) = serde_data.velocity { self.velocity = velocity; }
        self
    }
}

impl From<TankBasePrefab> for TankBaseBundle {
    fn from(prefab: TankBasePrefab) -> Self {
        Self {
            tank_marker: TankBaseMarker::default(),
            object_type: TankBaseMarker::default().into(),
            asset_type: TankBaseMarker::default().into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            path_finder: GroundPathFinder::default(),
            path: Path::default(),
            controller: prefab.controller,
            weapon_set: prefab.weapon_set,
            relative: prefab.turret.into(),
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

impl From<(TankBaseSerde, &TankBasePrefab)> for TankBaseBundle {
    fn from((save, prefab): (TankBaseSerde, &TankBasePrefab)) -> Self {
        Self {
            tank_marker: TankBaseMarker::default(),
            object_type: TankBaseMarker::default().into(),
            asset_type: TankBaseMarker::default().into(),
            snowflake: save.serde_snowflake.unwrap_or(Snowflake::new()),
            health: save.serde_health.unwrap_or(prefab.health),
            path_finder: save.serde_path_finder.unwrap_or_default(),
            path: save.serde_path.unwrap_or_default(),
            controller: save.serde_controller.unwrap_or(prefab.controller),
            weapon_set: save.serde_weapon_set.unwrap_or(prefab.weapon_set.clone()),
            relative: save.serde_turret.unwrap_or(prefab.turret).into(),
            team_player: save.team_player,
            velocity: save.serde_velocity.unwrap_or(SerdeVelocity::default()).into(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.collider.clone(),
            selectable: Selectable::multiselect(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: save.transform,
            global_transform: GlobalTransform::default(),
        }
    }
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

impl<'a> From<SerdeTankBaseQuery<'a>> for TankBaseSerde {
    fn from(object: SerdeTankBaseQuery) -> Self {
        Self {
            serde_snowflake: Some(*object.0),
            serde_health: object.1.saved(),
            serde_path_finder: object.2.saved(),
            serde_path: object.3.saved(),
            serde_controller: object.4.saved(),
            serde_weapon_set: object.5.saved(),
            serde_turret: Turret::from(*object.6).saved(),
            serde_velocity: SerdeVelocity::from(*object.7).saved(),
            team_player: *object.8,
            transform: *object.9,
        }
    }
}

impl From<TankBaseSerde> for ObjectSpawnEvent {
    fn from(value: TankBaseSerde) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::TankBase,
            spawn_data: SpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform.into(),
            },
            serde_data: Some(SerdeData {
                health: value.serde_health,
                path_finder: value.serde_path_finder,
                path: value.serde_path,
                controller: value.serde_controller,
                weapon_set: value.serde_weapon_set,
                turret: value.serde_turret,
                velocity: value.serde_velocity.map(|vel| vel.into()),
                ..default()
            }),
        })
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct TankGun;

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
    pub fn with_spawn_data(mut self, spawn_data: &SpawnData) -> Self {
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
pub struct TankPlugin<S: StateData> {
    state: S
}

impl<S: StateData> TankPlugin<S> {
    pub fn new(state: S) -> Self {
        Self {
            state
        }
    }

    pub fn spawn_tank(
        mut spawn_events: EventReader<ObjectSpawnEvent>,
        prefabs: Res<ObjectPrefabs>,
        mut new_tanks: Query<&mut Relative, Added<TankBaseMarker>>,
        new_tank_guns: Query<(Entity, &Parent), Added<TankGun>>,

        mut identifiers: ResMut<Identifiers>,
        mut commands: Commands,
    ) {
        spawn_events.iter().filter_map(|event| (event.0.object_type == ObjectType::TankBase).then_some(event.0.clone())).for_each(|data| {
            let tank_turret_transform = Transform::from(prefabs.tank_prefab.turret.transform);
            let gun_spawn_data = SpawnData {
                snowflake: Snowflake::new(),
                teamplayer: TeamPlayer::default(),
                transform: tank_turret_transform,
            };

            let mut gun_entity = None;
            let base_entity = commands.spawn(TankBaseBundle::from(prefabs.tank_prefab.clone()).with_spawn_data(data.spawn_data).with_serde_data(data.serde_data)).with_children(|parent| {
                gun_entity = Some(parent.spawn(TankGunBundle::default().with_spawn_data(&gun_spawn_data)).id());
            }).id();

            identifiers.insert(data.spawn_data.snowflake, base_entity);
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

impl<S: StateData> Plugin for TankPlugin<S> {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(self.state.clone())
            .with_system(Self::spawn_tank)
            .with_system(Self::aim_tank_gun)
        );
    }
}