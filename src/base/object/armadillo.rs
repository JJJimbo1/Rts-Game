use std::marker::PhantomData;

use bevy::prelude::*;
use avian3d::prelude::{Collider, RigidBody, LinearVelocity};
use bevy_mod_event_group::IntoGroup;
use superstruct::*;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct Armadillo;

impl From<Armadillo> for ObjectType {
    fn from(_: Armadillo) -> Self {
        ObjectType::Armadillo
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
pub struct Armadillo {
    #[superstruct(only(Prefab, Bundle))]    pub health: Health,
    #[superstruct(only(Prefab, Bundle))]    pub controller: Navigator,
    #[superstruct(only(Prefab, Bundle))]    pub weapon_set: WeaponSet,
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub tank_marker: Armadillo,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub velocity: LinearVelocity,
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
    #[superstruct(only(Disk))]              pub disk_velocity: Option<LinearVelocity>,
}

impl TryFrom<&ObjectAsset> for ArmadilloPrefab {
    type Error = ContentError;
    fn try_from(prefab: &ObjectAsset) -> Result<Self, Self::Error> {
        let Some(health) = prefab.health else { return Err(ContentError::MissingHealth); };
        let Some(controller) = prefab.navigator else { return Err(ContentError::MissingController); };
        let Some(weapon_set) = prefab.weapon_set.clone() else { return Err(ContentError::MissingWeapons); };
        let Some(collider_string) = prefab.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let collider = Collider::trimesh(vertices, indices);

        Ok(Self {
            health,
            controller,
            weapon_set,
            collider,
        })
    }
}

impl ArmadilloBundle {
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
        if let Some(velocity) = disk_data.velocity { self.velocity = velocity; }
        self
    }
}

impl From<ArmadilloPrefab> for ArmadilloBundle {
    fn from(prefab: ArmadilloPrefab) -> Self {
        Self {
            tank_marker: Armadillo::default(),
            object_type: Armadillo::default().into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            path_finder: PathFinder::default(),
            controller: prefab.controller,
            weapon_set: prefab.weapon_set,
            team_player: TeamPlayer::default(),
            selectable: Selectable::multiselect(),
            velocity: LinearVelocity::default(),
            rigid_body: RigidBody::Kinematic,
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            transform: Transform::default(),
        }
    }
}

impl From<(ArmadilloDisk, &ArmadilloPrefab)> for ArmadilloBundle {
    fn from((save, prefab): (ArmadilloDisk, &ArmadilloPrefab)) -> Self {
        Self {
            tank_marker: Armadillo::default(),
            object_type: Armadillo::default().into(),
            snowflake: save.disk_snowflake.unwrap_or(Snowflake::new()),
            health: save.disk_health.unwrap_or(prefab.health),
            path_finder: save.disk_path_finder.unwrap_or_default(),
            controller: save.disk_controller.unwrap_or(prefab.controller),
            weapon_set: save.disk_weapon_set.unwrap_or(prefab.weapon_set.clone()),
            team_player: save.team_player,
            velocity: save.disk_velocity.unwrap_or(LinearVelocity::default()),
            rigid_body: RigidBody::Kinematic,
            collider: prefab.collider.clone(),
            selectable: Selectable::multiselect(),
            visibility: Visibility::default(),
            transform: save.transform,
        }
    }
}

impl<'a> From<ArmadilloDiskQuery<'a>> for ArmadilloDisk {
    fn from(object: ArmadilloDiskQuery) -> Self {
        Self {
            disk_snowflake: Some(*object.0),
            disk_health: object.1.slim(),
            disk_path_finder: object.2.slim(),
            disk_controller: object.3.slim(),
            disk_weapon_set: object.4.slim(),
            disk_velocity: object.5.slim(),
            team_player: *object.6,
            transform: *object.7,
        }
    }
}

impl From<ArmadilloDisk> for SpawnObject {
    fn from(value: ArmadilloDisk) -> Self {
        Self {
            object_type: ObjectType::Armadillo,
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
pub struct ArmadilloPlugin;

impl ArmadilloPlugin {
    pub fn spawn(
        mut spawn_events: EventReader<SpawnObject<Armadillo>>,
        mut client_requests: EventWriter<ClientRequest>,
        prefabs: Res<ObjectPrefabs>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in spawn_events.read() {
            commands.spawn(ArmadilloBundle::from(prefabs.armadillo_prefab.clone()).with_spawn_data(event.spawn_data.clone()).with_disk_data(event.disk_data.clone()));
            match event.spawn_mode {
                SpawnMode::Load => { status.armadillos_loaded = Some(true); },
                SpawnMode::Spawn => { client_requests.write(ClientRequest::SpawnObject(event.clone().into_group())); },
                SpawnMode::Fetch => { },
            }
        }
    }
}

impl Plugin for ArmadilloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, Self::spawn.run_if(resource_exists::<ObjectPrefabs>))
        ;
    }
}