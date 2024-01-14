use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
use serde::{Serialize, Deserialize};
use superstruct::*;
use t5f_common::*;
use t5f_utility::colliders::decode;
use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[derive(Component)]
pub struct MarineSquadMarker;

impl ObjectMarker for MarineSquadMarker { }

impl From<MarineSquadMarker> for ObjectType {
    fn from(_: MarineSquadMarker) -> Self {
        ObjectType::MarineSquad
    }
}

impl From<MarineSquadMarker> for AssetType {
    fn from(_: MarineSquadMarker) -> Self {
        Self::Object(ObjectType::MarineSquad)
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
pub struct MarineSquad {
    #[superstruct(only(Prefab, Bundle))]    pub health: Health,
    #[superstruct(only(Prefab, Bundle))]    pub controller: Navigator,
    #[superstruct(only(Prefab, Bundle))]    pub weapon_set: WeaponSet,
    #[superstruct(only(Prefab, Bundle))]    pub squad: Squad,
    #[superstruct(only(Prefab, Bundle))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub marker: MarineSquadMarker,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub velocity: Velocity,
    #[superstruct(only(Bundle))]            pub path_finder: PathFinder,
    #[superstruct(only(Bundle))]            pub selectable: Selectable,
    #[superstruct(only(Bundle))]            pub rigid_body: RigidBody,
    #[superstruct(only(Bundle))]            pub visibility: Visibility,
    #[superstruct(only(Bundle))]            pub inherited_visibility: InheritedVisibility,
    #[superstruct(only(Bundle))]            pub view_visibility: ViewVisibility,
    #[superstruct(only(Bundle))]            pub global_transform: GlobalTransform,
    #[superstruct(only(Bundle, Serde))]     pub team_player: TeamPlayer,
    #[superstruct(only(Bundle, Serde))]     pub transform: Transform,
    #[superstruct(only(Serde))]             pub serde_snowflake: Option<Snowflake>,
    #[superstruct(only(Serde))]             pub serde_health: Option<Health>,
    #[superstruct(only(Serde))]             pub serde_squad: Option<Squad>,
    #[superstruct(only(Serde))]             pub serde_path_finder: Option<PathFinder>,
    #[superstruct(only(Serde))]             pub serde_controller: Option<Navigator>,
    #[superstruct(only(Serde))]             pub serde_weapon_set: Option<WeaponSet>,
    #[superstruct(only(Serde))]             pub serde_velocity: Option<Velocity>,
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

    pub fn with_serde_data(mut self, serde_data: Option<ObjectSerdeData>) -> Self {
        let Some(serde_data) = serde_data else { return self; };
        if let Some(health) = serde_data.health { self.health = health; }
        if let Some(squad) = serde_data.squad { self.squad = squad; }
        if let Some(path_finder) = serde_data.path_finder { self.path_finder = path_finder; }
        if let Some(controller) = serde_data.navigator { self.controller = controller; }
        if let Some(weapon_set) = serde_data.weapon_set { self.weapon_set = weapon_set; }
        if let Some(velocity) = serde_data.velocity { self.velocity = velocity; }
        self
    }
}

impl From<MarineSquadPrefab> for MarineSquadBundle {
    fn from(prefab: MarineSquadPrefab) -> Self {
        Self {
            marker: MarineSquadMarker,
            object_type: MarineSquadMarker::default().into(),
            asset_type: MarineSquadMarker::default().into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            squad: prefab.squad,
            path_finder: PathFinder::default(),
            controller: prefab.controller,
            weapon_set: prefab.weapon_set,
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

impl From<(MarineSquadSerde, &MarineSquadPrefab)> for MarineSquadBundle {
    fn from((save, prefab): (MarineSquadSerde, &MarineSquadPrefab)) -> Self {
        Self {
            marker: MarineSquadMarker,
            object_type: MarineSquadMarker::default().into(),
            asset_type: MarineSquadMarker::default().into(),
            snowflake: save.serde_snowflake.unwrap_or(Snowflake::new()),
            health: save.serde_health.unwrap_or(prefab.health),
            squad: save.serde_squad.unwrap_or_else(|| prefab.squad.clone()),
            path_finder: save.serde_path_finder.unwrap_or_default(),
            controller: save.serde_controller.unwrap_or(prefab.controller),
            weapon_set: save.serde_weapon_set.unwrap_or(prefab.weapon_set.clone()),
            team_player: save.team_player,
            selectable: Selectable::multiselect(),
            velocity: save.serde_velocity.unwrap_or(Velocity::default()),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl<'a> From<SerdeMarineSquadQuery<'a>> for MarineSquadSerde {
    fn from(object: SerdeMarineSquadQuery) -> Self {
        Self {
            serde_snowflake: Some(*object.0),
            serde_health: object.1.slim(),
            serde_squad: object.2.slim(),
            serde_path_finder: object.3.slim(),
            serde_controller: object.4.slim(),
            serde_weapon_set: object.5.slim(),
            serde_velocity: (*object.6).slim(),
            team_player: *object.7,
            transform: (*object.8).into(),
        }
    }
}

impl From<MarineSquadSerde> for ObjectLoadEvent<AnyObjectMarker> {
    fn from(value: MarineSquadSerde) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::MarineSquad,
            spawn_data: ObjectSpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform.into(),
            },
            serde_data: Some(ObjectSerdeData {
                health: value.serde_health,
                squad: value.serde_squad,
                path_finder: value.serde_path_finder,
                navigator: value.serde_controller,
                weapon_set: value.serde_weapon_set,
                velocity: value.serde_velocity.map(|vel| vel.into()),
                ..default()
            }),
        }, PhantomData
        )
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Marine;

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
    pub visibility: Visibility,
    pub view_visibility: ViewVisibility,
    pub inherited_visibility: InheritedVisibility,
    pub transform: TransformBundle,
}

impl MarineBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnData) -> Self {
        self.transform = TransformBundle::from_transform(spawn_data.transform);
        self
    }
}

impl Default for MarineBundle {
    fn default() -> Self {
        Self {
            marine: Marine,
            object_type: Marine.into(),
            asset_type: Marine.into(),
            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            transform: TransformBundle::default(),
        }
    }
}

pub struct MarineSquadPlugin;

impl MarineSquadPlugin {
    pub fn load(
        mut load_events: EventReader<ObjectLoadEvent<MarineSquadMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut status: ResMut<LoadingStatus>,
        mut commands: Commands,
    ) {
        for event in load_events.read() {
            let marine_squad = MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.spawn_data().clone()).with_serde_data(event.serde_data().clone());
            let entity = commands.spawn(marine_squad).id();
            identifiers.insert(event.spawn_data().snowflake, entity);
            let teamplayer = event.spawn_data().teamplayer;
            let squad = event.serde_data().clone().and_then(|serde_data| serde_data.squad).unwrap_or(prefabs.marine_squad_prefab.squad.clone());

            let mut offset: f32 = 0.0;
            for (object, _) in squad.members.iter() {
                let Ok(object_type) = (object.clone()).try_into() else { continue; };
                let spawn_data = ObjectSpawnData {
                    snowflake: Snowflake::new(),
                    teamplayer,
                    transform: Transform::from_xyz(offset, 0.0, 0.0),
                };
                match object_type {
                    ObjectType::Marine => {
                        commands.entity(entity).with_children(|child_builder| {
                            child_builder.spawn(MarineBundle::default().with_spawn_data(spawn_data));
                        });
                    },
                    _ => { },
                };
                offset += 0.75;
            }
            println!("Marine Squads Loaded");
            status.marines_loaded = Some(true);
        }
    }

    pub fn spawn(
        mut spawn_events: EventReader<ObjectSpawnEvent<MarineSquadMarker>>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut commands: Commands,
    ) {
        for event in spawn_events.read() {
            let marine_squad = MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.spawn_data().clone());
            let entity = commands.spawn(marine_squad).id();
            identifiers.insert(event.spawn_data().snowflake, entity);
            let teamplayer = event.spawn_data().teamplayer;
            let squad = prefabs.marine_squad_prefab.squad.clone();

            let mut offset: f32 = 0.0;
            for (object, _) in squad.members.iter() {
                let Ok(object_type) = (object.clone()).try_into() else { continue; };
                let spawn_data = ObjectSpawnData {
                    snowflake: Snowflake::new(),
                    teamplayer,
                    transform: Transform::from_xyz(offset, 0.0, 0.0),
                };
                match object_type {
                    ObjectType::Marine => {
                        commands.entity(entity).with_children(|child_builder| {
                            child_builder.spawn(MarineBundle::default().with_spawn_data(spawn_data));
                        });
                    },
                    _ => { },
                };
                offset += 0.75;
            }
        }
    }

    // pub fn spawn_marine_squad(
    //     mut spawn_events: EventReader<ObjectSpawnEvent<MarineSquadMarker>>,
    //     prefabs: Res<ObjectPrefabs>,
    //     mut identifiers: ResMut<Identifiers>,
    //     mut new_marine_squads: Query<(Entity, &TeamPlayer, &mut Squad), Added<MarineSquadMarker>>,
    //     mut commands: Commands,
    // ) {
    //     for event in spawn_events.iter() {
    //         if event.0.object_type != ObjectType::MarineSquad { continue; }
    //         let entity = commands.spawn(MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.0.spawn_data).with_serde_data(event.0.serde_data.clone())).id();
    //         identifiers.insert(event.0.spawn_data.snowflake, entity);
    //     }
    //     new_marine_squads.for_each_mut(|(entity, teamplayer, squad)| {
    //         let mut offset: f32 = 0.0;
    //         for (object_type, _) in prefabs.marine_squad_prefab.squad.members.iter().take(squad.current_members.into()) {
    //             let marine_transform = Transform::from_xyz(offset, 0.0, 0.0);
    //             let spawn_data = SpawnData {
    //                 snowflake: Snowflake::new(),
    //                 teamplayer: *teamplayer,
    //                 transform: marine_transform,
    //             };
    //             match *object_type {
    //                 ObjectType::Marine => {
    //                     commands.entity(entity).with_children(|child_builder| {
    //                         child_builder.spawn(MarineBundle::default().with_spawn_data(spawn_data));
    //                     });
    //                 },
    //                 _ => { },
    //             };
    //             offset += 0.75
    //         }
    //     });
    // }
}

impl Plugin for MarineSquadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ObjectLoadEvent<MarineSquadMarker>>()
            .add_event::<ObjectSpawnEvent<MarineSquadMarker>>()
            .add_systems(Update, (
                Self::load,
                Self::spawn
            ).run_if(resource_exists::<ObjectPrefabs>()))
        ;
    }
}
