use std::marker::PhantomData;

use bevy::prelude::*;
use avian3d::prelude::{Collider, RigidBody, LinearVelocity};
use bevy_mod_event_group::IntoGroup;
use serde::{Serialize, Deserialize};
use superstruct::*;
use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[derive(Component)]
pub struct MarineSquad;

impl From<MarineSquad> for ObjectType {
    fn from(_: MarineSquad) -> Self {
        ObjectType::MarineSquad
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
pub struct MarineSquad {
    #[superstruct(only(Prefab, Bundle))]    pub health: Health,
    #[superstruct(only(Prefab, Bundle))]    pub controller: Navigator,
    #[superstruct(only(Prefab, Bundle))]    pub weapon_set: WeaponSet,
    #[superstruct(only(Prefab, Bundle))]    pub squad: Squad,
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub marker: MarineSquad,
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
    #[superstruct(only(Disk))]              pub disk_squad: Option<Squad>,
    #[superstruct(only(Disk))]              pub disk_path_finder: Option<PathFinder>,
    #[superstruct(only(Disk))]              pub disk_controller: Option<Navigator>,
    #[superstruct(only(Disk))]              pub disk_weapon_set: Option<WeaponSet>,
    #[superstruct(only(Disk))]              pub disk_velocity: Option<LinearVelocity>,
}

impl TryFrom<&ObjectAsset> for MarineSquadPrefab {
    type Error = ContentError;
    fn try_from(asset: &ObjectAsset) -> Result<Self, Self::Error> {
        let Some(health) = asset.health else { return Err(ContentError::MissingHealth); };
        let Some(asset_squad) = asset.asset_squad.clone() else { return Err(ContentError::MissingSquad); };
        let Some(controller) = asset.navigator else { return Err(ContentError::MissingController); };
        let Some(weapon_set) = asset.weapon_set.clone() else { return Err(ContentError::MissingWeapons); };
        let Some(collider_string) = asset.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };


        let collider = Collider::trimesh(vertices, indices);

        Ok(Self {
            health,
            squad: asset_squad.into(),
            controller,
            weapon_set,
            collider,
        })
    }
}

impl MarineSquadBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }

    pub fn with_disk_data(mut self, disk_data: Option<ObjectDiskData>) -> Self {
        let Some(disk_data) = disk_data else { return self; };
        if let Some(health) = disk_data.health { self.health = health; }
        if let Some(squad) = disk_data.squad { self.squad = squad; }
        if let Some(path_finder) = disk_data.path_finder { self.path_finder = path_finder; }
        if let Some(controller) = disk_data.navigator { self.controller = controller; }
        if let Some(weapon_set) = disk_data.weapon_set { self.weapon_set = weapon_set; }
        if let Some(velocity) = disk_data.velocity { self.velocity = velocity; }
        self
    }
}

impl From<MarineSquadPrefab> for MarineSquadBundle {
    fn from(prefab: MarineSquadPrefab) -> Self {
        Self {
            marker: MarineSquad,
            object_type: MarineSquad.into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            squad: prefab.squad,
            path_finder: PathFinder::default(),
            controller: prefab.controller,
            weapon_set: prefab.weapon_set,
            team_player: TeamPlayer::default(),
            selectable: Selectable::multiselect(),
            velocity: LinearVelocity::default(),
            rigid_body: RigidBody::Kinematic,
            collider: prefab.collider.clone(),
            transform: Transform::default(),
            visibility: Visibility::default(),
        }
    }
}

impl From<(MarineSquadDisk, &MarineSquadPrefab)> for MarineSquadBundle {
    fn from((save, prefab): (MarineSquadDisk, &MarineSquadPrefab)) -> Self {
        Self {
            marker: MarineSquad,
            object_type: MarineSquad.into(),
            snowflake: save.disk_snowflake.unwrap_or(Snowflake::new()),
            health: save.disk_health.unwrap_or(prefab.health),
            squad: save.disk_squad.unwrap_or_else(|| prefab.squad.clone()),
            path_finder: save.disk_path_finder.unwrap_or_default(),
            controller: save.disk_controller.unwrap_or(prefab.controller),
            weapon_set: save.disk_weapon_set.unwrap_or(prefab.weapon_set.clone()),
            team_player: save.team_player,
            selectable: Selectable::multiselect(),
            velocity: save.disk_velocity.unwrap_or(LinearVelocity::default()),
            rigid_body: RigidBody::Kinematic,
            collider: prefab.collider.clone(),
            transform: save.transform.into(),
            visibility: Visibility::default(),
        }
    }
}

impl<'a> From<MarineSquadDiskQuery<'a>> for MarineSquadDisk {
    fn from(object: MarineSquadDiskQuery) -> Self {
        Self {
            disk_snowflake: Some(*object.0),
            disk_health: object.1.slim(),
            disk_squad: object.2.slim(),
            disk_path_finder: object.3.slim(),
            disk_controller: object.4.slim(),
            disk_weapon_set: object.5.slim(),
            disk_velocity: (*object.6).slim(),
            team_player: *object.7,
            transform: (*object.8).into(),
        }
    }
}

impl From<MarineSquadDisk> for SpawnObject {
    fn from(value: MarineSquadDisk) -> Self {
        Self {
            object_type: ObjectType::MarineSquad,
            spawn_data: ObjectSpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform.into(),
            },
            disk_data: Some(ObjectDiskData {
                health: value.disk_health,
                squad: value.disk_squad,
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

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component)]
pub struct Marine;

impl From<Marine> for ObjectType {
    fn from(_: Marine) -> Self {
        ObjectType::Marine
    }
}

#[derive(Clone, Bundle)]
pub struct MarineBundle {
    pub marine: Marine,
    pub object_type: ObjectType,
    pub visibility: Visibility,
    pub transform: Transform,
}

impl MarineBundle {
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

impl Default for MarineBundle {
    fn default() -> Self {
        Self {
            marine: Marine,
            object_type: Marine.into(),
            visibility: Visibility::default(),
            transform: Transform::default(),
        }
    }
}

/*
( 1,    0   )
( 0.5,  √3/2)
(-0.5,  √3/2)
(-1,    0   )
(-0.5, -√3/2)
( 0.5, -√3/2)
*/

#[derive(Debug, Clone, Copy, Deref)]
pub struct MarineSpawnPoints([Vec2; 6]);

impl Default for MarineSpawnPoints {
    fn default() -> Self {
        Self([
            Vec2::new(1.0, 0.0),
            Vec2::new(0.5, 3.0_f32.sqrt() / 2.0),
            Vec2::new(-0.5, 3.0_f32.sqrt() / 2.0),
            Vec2::new(-1.0, 0.0),
            Vec2::new(-0.5, -3.0_f32.sqrt() / 2.0),
            Vec2::new(0.5, -3.0_f32.sqrt() / 2.0),
        ])
    }
}

pub struct MarineSquadPlugin;

impl MarineSquadPlugin {
    pub fn spawn(
        points: Local<MarineSpawnPoints>,
        mut spawn_events: EventReader<SpawnObject<MarineSquad>>,
        mut client_requests: EventWriter<ClientRequest>,
        prefabs: Res<ObjectPrefabs>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in spawn_events.read() {
            commands.spawn(MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.spawn_data.clone()).with_disk_data(event.disk_data.clone())).with_children(|parent| {
                let squad = event.disk_data.clone().and_then(|disk_data| disk_data.squad).unwrap_or(prefabs.marine_squad_prefab.squad.clone());
                for ((object_type, _), point) in squad.members.iter().zip(points.0.iter()) {
                    let transform = Transform::from_translation(Vec3::from((*point, 0.0)).xzy());
                    match object_type {
                        ObjectType::Marine => {
                            parent.spawn(MarineBundle::default().with_transform(transform));
                        },
                        _ => { },
                    };
                }
            });
            match event.spawn_mode {
                SpawnMode::Load => { status.marines_loaded = Some(true); },
                SpawnMode::Spawn => { client_requests.write(ClientRequest::SpawnObject(event.clone().into_group())); },
                SpawnMode::Fetch => { },
            }
        }
    }
}

impl Plugin for MarineSquadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                Self::spawn
            ).run_if(resource_exists::<ObjectPrefabs>))
        ;
    }
}
