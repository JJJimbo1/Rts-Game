use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
use superstruct::*;
use serde::{Serialize, Deserialize};
use t5f_common::*;
use t5f_utility::colliders::decode;
use crate::*;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[derive(Component)]
pub struct TankBaseMarker;

impl ObjectMarker for TankBaseMarker { }

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
    #[superstruct(only(Prefab, Bundle))]    pub reference: Reference,
    #[superstruct(only(Prefab, Bundle))]    pub health: Health,
    #[superstruct(only(Prefab, Bundle))]    pub controller: Navigator,
    #[superstruct(only(Prefab, Bundle))]    pub weapon_set: WeaponSet,
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub tank_marker: TankBaseMarker,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub velocity: Velocity,
    #[superstruct(only(Bundle))]            pub path_finder: PathFinder,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle))]            pub rigid_body: RigidBody,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub view_visibility: ViewVisibility,
    #[superstruct(only(Bundle))]            pub inherited_visibility: InheritedVisibility,
    #[superstruct(only(Bundle))]            pub global_transform: GlobalTransform,
    #[superstruct(only(Bundle, Serde))]     pub team_player: TeamPlayer,
    #[superstruct(only(Bundle, Serde))]     pub transform: Transform,
    #[superstruct(only(Serde))]             pub serde_snowflake: Option<Snowflake>,
    #[superstruct(only(Serde))]             pub serde_health: Option<Health>,
    #[superstruct(only(Serde))]             pub serde_path_finder: Option<PathFinder>,
    #[superstruct(only(Serde))]             pub serde_controller: Option<Navigator>,
    #[superstruct(only(Serde))]             pub serde_weapon_set: Option<WeaponSet>,
    #[superstruct(only(Serde))]             pub serde_velocity: Option<Velocity>,
    #[superstruct(only(Serde))]             pub serde_reference: Option<Reference>,
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

        let collider = Collider::trimesh(vertices, indices);

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

    pub fn with_serde_data(mut self, serde_data: Option<ObjectSerdeData>) -> Self {
        let Some(serde_data) = serde_data else { return self; };
        if let Some(health) = serde_data.health { self.health = health; }
        if let Some(path_finder) = serde_data.path_finder { self.path_finder = path_finder; }
        if let Some(controller) = serde_data.navigator { self.controller = controller; }
        if let Some(weapon_set) = serde_data.weapon_set { self.weapon_set = weapon_set; }
        if let Some(reference) = serde_data.reference { self.reference = reference; }
        if let Some(velocity) = serde_data.velocity { self.velocity = velocity; }
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
            tank_marker: TankBaseMarker::default(),
            object_type: TankBaseMarker::default().into(),
            asset_type: TankBaseMarker::default().into(),
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
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
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
            controller: save.serde_controller.unwrap_or(prefab.controller),
            weapon_set: save.serde_weapon_set.unwrap_or(prefab.weapon_set.clone()),
            reference: save.serde_reference.unwrap_or(prefab.reference.clone()),
            team_player: save.team_player,
            velocity: save.serde_velocity.unwrap_or(Velocity::default()),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.collider.clone(),
            selectable: Selectable::multiselect(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: save.transform,
            global_transform: GlobalTransform::default(),
        }
    }
}

impl<'a> From<SerdeTankBaseQuery<'a>> for TankBaseSerde {
    fn from(object: SerdeTankBaseQuery) -> Self {
        Self {
            serde_snowflake: Some(*object.0),
            serde_health: object.1.slim(),
            serde_path_finder: object.2.slim(),
            serde_controller: object.3.slim(),
            serde_weapon_set: object.4.slim(),
            serde_velocity: object.6.slim(),
            serde_reference: Some(object.5.clone()),
            team_player: *object.7,
            transform: *object.8,
        }
    }
}

impl From<TankBaseSerde> for ObjectLoadEvent<AnyObjectMarker> {
    fn from(value: TankBaseSerde) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::TankBase,
            spawn_data: ObjectSpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform.into(),
            },
            serde_data: Some(ObjectSerdeData {
                health: value.serde_health,
                path_finder: value.serde_path_finder,
                navigator: value.serde_controller,
                weapon_set: value.serde_weapon_set,
                reference: value.serde_reference,
                velocity: value.serde_velocity.map(|vel| vel.into()),
                ..default()
            }),
        },
        PhantomData)
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
    pub view_visibility: ViewVisibility,
    pub inherited_visibility: InheritedVisibility,
    pub transform: TransformBundle,
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
            asset_type: TankGun.into(),
            snowflake: Snowflake::new(),
            teamplayer: TeamPlayer::default(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: TransformBundle::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TankPlugin;

impl TankPlugin {
    pub fn load(
        mut load_events: EventReader<ObjectLoadEvent<TankBaseMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in load_events.read() {
            let reference = event.serde_data().clone().and_then(|serde_data| serde_data.reference).unwrap_or(prefabs.tank_prefab.reference.clone());
            let Some(transform) = reference.references[0].0 else { continue; };
            let gun_spawn_data = ObjectSpawnData {
                snowflake: Snowflake::new(),
                teamplayer: TeamPlayer::default(),
                transform,
            };
            let turret = TankGunBundle::default().with_spawn_data(&gun_spawn_data);
            let turret_entity = commands.spawn(turret).id();
            identifiers.insert(gun_spawn_data.snowflake, turret_entity);

            let tank = TankBaseBundle::from(prefabs.tank_prefab.clone()).with_spawn_data(event.spawn_data().clone()).with_serde_data(event.serde_data().clone()).with_reference(turret_entity);
            let tank_entity = commands.spawn(tank).id();
            identifiers.insert(event.spawn_data().snowflake, tank_entity);
            commands.entity(tank_entity).add_child(turret_entity);
            println!("Tanks Loaded");
            status.tanks_loaded = Some(true);
        }
    }

    pub fn spawn(
        mut spawn_events: EventReader<ObjectSpawnEvent<TankBaseMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut commands: Commands,
    ) {
        for event in spawn_events.read() {
            let reference = prefabs.tank_prefab.reference.clone();
            let Some(transform) = reference.references[0].0 else { continue; };
            let gun_spawn_data = ObjectSpawnData {
                snowflake: Snowflake::new(),
                teamplayer: TeamPlayer::default(),
                transform,
            };
            let turret = TankGunBundle::default().with_spawn_data(&gun_spawn_data);
            let turret_entity = commands.spawn(turret).id();
            identifiers.insert(gun_spawn_data.snowflake, turret_entity);

            let tank = TankBaseBundle::from(prefabs.tank_prefab.clone()).with_spawn_data(event.spawn_data().clone()).with_reference(turret_entity);
            let tank_entity = commands.spawn(tank).id();
            identifiers.insert(event.spawn_data().snowflake, tank_entity);
            commands.entity(tank_entity).add_child(turret_entity);
        }
    }

    pub fn aim_tank_gun(
        mut transforms: Query<&mut Transform>,
        global_transforms: Query<&mut GlobalTransform>,
        mut weapons: Query<(Entity, &mut Reference, &WeaponSet)>,
    ) {
        weapons.for_each_mut(|(entity, mut reference, weapon_set)| {
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
                let speed = 0.025 / difference;
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
            .add_event::<ObjectLoadEvent<TankBaseMarker>>()
            .add_event::<ObjectSpawnEvent<TankBaseMarker>>()
            .add_systems(Update, (
                (
                    Self::load,
                    Self::spawn
                ).run_if(resource_exists::<ObjectPrefabs>()),
                Self::aim_tank_gun
            ))
        ;
    }
}