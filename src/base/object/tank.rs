use std::{f32::consts::PI, marker::PhantomData};

use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
use superstruct::*;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct TankBase;

impl From<TankBase> for ObjectType {
    fn from(_: TankBase) -> Self {
        ObjectType::TankBase
    }
}

#[superstruct{
    no_enum,
    variants(Bundle, Prefab, Disk),
    variant_attributes(derive(Debug, Clone)),
    specific_variant_attributes(
        Bundle(derive(Bundle)),
        Disk(derive(Serialize, Deserialize)),
    ),
}]
#[derive(Debug, Clone)]
pub struct TankBase {
    #[superstruct(only(Prefab, Bundle))]    pub reference: Reference,
    #[superstruct(only(Prefab, Bundle))]    pub health: Health,
    #[superstruct(only(Prefab, Bundle))]    pub controller: Navigator,
    #[superstruct(only(Prefab, Bundle))]    pub weapon_set: WeaponSet,
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub tank_marker: TankBase,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub velocity: Velocity,
    #[superstruct(only(Bundle))]            pub path_finder: PathFinder,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle))]            pub rigid_body: RigidBody,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle, Disk))]      pub team_player: TeamPlayer,
    #[superstruct(only(Bundle, Disk))]      pub transform: Transform,
    #[superstruct(only(Disk))]              pub disk_snowflake: Option<Snowflake>,
    #[superstruct(only(Disk))]              pub disk_health: Option<Health>,
    #[superstruct(only(Disk))]              pub disk_path_finder: Option<PathFinder>,
    #[superstruct(only(Disk))]              pub disk_controller: Option<Navigator>,
    #[superstruct(only(Disk))]              pub disk_weapon_set: Option<WeaponSet>,
    #[superstruct(only(Disk))]              pub disk_velocity: Option<Velocity>,
    #[superstruct(only(Disk))]              pub disk_reference: Option<Reference>,
}

impl TryFrom<&ObjectAsset> for TankBasePrefab {
    type Error = ContentError;
    fn try_from(prefab: &ObjectAsset) -> Result<Self, Self::Error> {
        let Some(health) = prefab.health else { return Err(ContentError::MissingHealth); };
        let Some(controller) = prefab.navigator else { return Err(ContentError::MissingController); };
        let Some(weapon_set) = prefab.weapon_set.clone() else { return Err(ContentError::MissingWeapons); };
        let Some(collider_string) = prefab.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some(reference) = prefab.reference.clone() else { return Err(ContentError::MissingReference); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let Ok(collider) = Collider::trimesh(vertices, indices) else { return Err(ContentError::ColliderDecodeError); };

        Ok(Self {
            health,
            controller,
            weapon_set,
            reference,
            collider,
        })
    }
}

impl TankBaseBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }

    pub fn with_disk_data(mut self, disk_data: Option<ObjectDiskData>) -> Self {
        let Some(disk_data) = disk_data else { return self; };
        if let Some(health) = disk_data.health { self.health = health; }
        if let Some(path_finder) = disk_data.path_finder { self.path_finder = path_finder; }
        if let Some(controller) = disk_data.navigator { self.controller = controller; }
        if let Some(weapon_set) = disk_data.weapon_set { self.weapon_set = weapon_set; }
        if let Some(reference) = disk_data.reference { self.reference = reference; }
        if let Some(velocity) = disk_data.velocity { self.velocity = velocity; }
        self
    }

    pub fn with_reference(mut self, entity: Entity) -> Self {
        self.reference.references[0].1 = Some(entity);
        self
    }
}

impl From<TankBasePrefab> for TankBaseBundle {
    fn from(prefab: TankBasePrefab) -> Self {
        Self {
            tank_marker: TankBase::default(),
            object_type: TankBase::default().into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            path_finder: PathFinder::default(),
            controller: prefab.controller,
            weapon_set: prefab.weapon_set,
            reference: prefab.reference.into(),
            team_player: TeamPlayer::default(),
            selectable: Selectable::multiselect(),
            velocity: Velocity::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            transform: Transform::default(),
        }
    }
}

impl From<(TankBaseDisk, &TankBasePrefab)> for TankBaseBundle {
    fn from((save, prefab): (TankBaseDisk, &TankBasePrefab)) -> Self {
        Self {
            tank_marker: TankBase::default(),
            object_type: TankBase::default().into(),
            snowflake: save.disk_snowflake.unwrap_or(Snowflake::new()),
            health: save.disk_health.unwrap_or(prefab.health),
            path_finder: save.disk_path_finder.unwrap_or_default(),
            controller: save.disk_controller.unwrap_or(prefab.controller),
            weapon_set: save.disk_weapon_set.unwrap_or(prefab.weapon_set.clone()),
            reference: save.disk_reference.unwrap_or(prefab.reference.clone()),
            team_player: save.team_player,
            velocity: save.disk_velocity.unwrap_or(Velocity::default()),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.collider.clone(),
            selectable: Selectable::multiselect(),
            visibility: Visibility::default(),
            transform: save.transform,
        }
    }
}

impl<'a> From<TankBaseDiskQuery<'a>> for TankBaseDisk {
    fn from(object: TankBaseDiskQuery) -> Self {
        Self {
            disk_snowflake: Some(*object.0),
            disk_health: object.1.slim(),
            disk_path_finder: object.2.slim(),
            disk_controller: object.3.slim(),
            disk_weapon_set: object.4.slim(),
            disk_velocity: object.6.slim(),
            disk_reference: Some(object.5.clone()),
            team_player: *object.7,
            transform: *object.8,
        }
    }
}

impl From<TankBaseDisk> for SpawnObject {
    fn from(value: TankBaseDisk) -> Self {
        Self {
            object_type: ObjectType::TankBase,
            spawn_data: ObjectSpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform.into(),
            },
            disk_data: Some(ObjectDiskData {
                health: value.disk_health,
                path_finder: value.disk_path_finder,
                navigator: value.disk_controller,
                weapon_set: value.disk_weapon_set,
                reference: value.disk_reference,
                velocity: value.disk_velocity.map(|vel| vel.into()),
                ..default()
            }),
            spawn_mode: SpawnMode::Load,
            phantom_data: PhantomData,
        }
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

#[derive(Debug, Clone, Bundle)]
pub struct TankGunBundle {
    pub tank_gun: TankGun,
    pub object_type: ObjectType,
    pub snowflake: Snowflake,
    pub teamplayer: TeamPlayer,
    pub visibility: Visibility,
    pub transform: Transform,
}

impl TankGunBundle {
    pub fn with_spawn_data(mut self, spawn_data: &ObjectSpawnData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.teamplayer = spawn_data.teamplayer;
        self.transform = spawn_data.transform.into();
        self
    }
}

impl Default for TankGunBundle {
    fn default() -> Self {
        Self {
            tank_gun: TankGun,
            object_type: TankGun.into(),
            snowflake: Snowflake::new(),
            teamplayer: TeamPlayer::default(),
            visibility: Visibility::default(),
            transform: Transform::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TankPlugin;

impl TankPlugin {
    pub fn spawn(
        mut spawn_events: EventReader<SpawnObject<TankBase>>,
        mut client_requests: EventWriter<ClientRequest>,
        prefabs: Res<ObjectPrefabs>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in spawn_events.read() {
            let reference = event.disk_data.clone().and_then(|disk_data| disk_data.reference).unwrap_or(prefabs.tank_prefab.reference.clone());
            let Some(mut transform) = reference.references[0].0 else { continue; };
            transform.rotate_local_y(PI);
            let gun_spawn_data = ObjectSpawnData {
                snowflake: Snowflake::new(),
                teamplayer: TeamPlayer::default(),
                transform,
            };
            let turret = TankGunBundle::default().with_spawn_data(&gun_spawn_data);
            let turret_entity = commands.spawn(turret).id();

            let tank = TankBaseBundle::from(prefabs.tank_prefab.clone()).with_spawn_data(event.spawn_data.clone()).with_disk_data(event.disk_data.clone()).with_reference(turret_entity);
            let tank_entity = commands.spawn(tank).id();
            commands.entity(tank_entity).add_child(turret_entity);
            match event.spawn_mode {
                SpawnMode::Load => { status.tanks_loaded = Some(true); },
                SpawnMode::Spawn => { client_requests.write(ClientRequest::SpawnObject(event.clone().into())); },
                SpawnMode::Fetch => { },
            }
        }
    }

    pub fn aim_tank_gun(
        time: Res<Time>,
        mut transforms: Query<&mut Transform>,
        global_transforms: Query<&mut GlobalTransform>,
        mut weapons: Query<(Entity, &mut Reference, &WeaponSet)>,
    ) {
        weapons.iter_mut().for_each(|(entity, mut reference, weapon_set)| {
            let Some(transform) = transforms.get(entity).ok().cloned() else { return; };
            let Some(gun_entity) = reference.references.iter().map(|f| f.1).next().flatten() else { return; };
            let Some(global_gun_transform) = global_transforms.get(gun_entity).ok() else { return; };
            if let Some(mut gun_transform) = transforms.get_mut(gun_entity).ok() {

                let desired_rotation = if let Some(global_target_transform) = weapon_set.weapons.get(0).and_then(|weapon| weapon.target.get_target()).and_then(|t| global_transforms.get(t).ok().cloned()) {
                    let new_transform = Transform::from(*global_gun_transform).looking_at(global_target_transform.translation(), Vec3::Y);
                    new_transform.rotation * transform.rotation.inverse()
                } else {
                    Quat::IDENTITY
                };

                let difference = gun_transform.rotation .angle_between(desired_rotation);
                let speed = (1.5 / difference) * time.delta_secs();
                let new_rotation = gun_transform.rotation.slerp(desired_rotation, speed.clamp(0.0, 1.0));
                gun_transform.rotation = new_rotation;
                reference.references = vec![(Some(*gun_transform), Some(gun_entity))];
            }
        });
    }
}

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                Self::spawn.run_if(resource_exists::<ObjectPrefabs>),
                Self::aim_tank_gun
            ))
        ;
    }
}