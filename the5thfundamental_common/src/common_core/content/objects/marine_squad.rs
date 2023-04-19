use bevy::{prelude::*, ecs::schedule::StateData};
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
use serde::{Serialize, Deserialize};
use superstruct::*;
use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[derive(Component)]
pub struct MarineSquadMarker;

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
    #[superstruct(only(Bundle, Prefab))]    pub health: Health,
    #[superstruct(only(Bundle, Prefab))]    pub squad: Squad,
    #[superstruct(only(Bundle, Prefab))]    pub controller: Controller,
    #[superstruct(only(Bundle, Prefab))]    pub weapon_set: WeaponSet,
    #[superstruct(only(Bundle, Prefab))]    pub collider: Collider,
    #[superstruct(only(Bundle))]            pub marine_squad: MarineSquadMarker,
    #[superstruct(only(Bundle))]            pub object_type: ObjectType,
    #[superstruct(only(Bundle))]            pub asset_type: AssetType,
    #[superstruct(only(Bundle))]            pub snowflake: Snowflake,
    #[superstruct(only(Bundle))]            pub path_finder: GroundPathFinder,
    #[superstruct(only(Bundle))]            pub path: Path,
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
    #[superstruct(only(Serde))]             pub serde_squad: Option<Squad>,
    #[superstruct(only(Serde))]             pub serde_path_finder: Option<GroundPathFinder>,
    #[superstruct(only(Serde))]             pub serde_path: Option<Path>,
    #[superstruct(only(Serde))]             pub serde_controller: Option<Controller>,
    #[superstruct(only(Serde))]             pub serde_weapon_set: Option<WeaponSet>,
    #[superstruct(only(Serde))]             pub serde_velocity: Option<SerdeVelocity>,
}

impl MarineSquadBundle {
    pub fn with_spawn_data(mut self, spawn_data: SpawnData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }

    pub fn with_serde_data(mut self, serde_data: Option<SerdeData>) -> Self {
        let Some(serde_data) = serde_data else { return self; };
        if let Some(health) = serde_data.health { self.health = health; }
        if let Some(squad) = serde_data.squad { self.squad = squad; }
        if let Some(path_finder) = serde_data.path_finder { self.path_finder = path_finder; }
        if let Some(path) = serde_data.path { self.path = path; }
        if let Some(controller) = serde_data.controller { self.controller = controller; }
        if let Some(weapon_set) = serde_data.weapon_set { self.weapon_set = weapon_set; }
        if let Some(velocity) = serde_data.velocity { self.velocity = velocity; }
        self
    }
}

impl From<MarineSquadPrefab> for MarineSquadBundle {
    fn from(prefab: MarineSquadPrefab) -> Self {
        Self {
            marine_squad: MarineSquadMarker,
            object_type: MarineSquadMarker::default().into(),
            asset_type: MarineSquadMarker::default().into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            squad: prefab.squad,
            path_finder: GroundPathFinder::default(),
            path: Path::default(),
            controller: prefab.controller,
            weapon_set: prefab.weapon_set,
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

impl From<(MarineSquadSerde, &MarineSquadPrefab)> for MarineSquadBundle {
    fn from((save, prefab): (MarineSquadSerde, &MarineSquadPrefab)) -> Self {
        Self {
            marine_squad: MarineSquadMarker,
            object_type: MarineSquadMarker::default().into(),
            asset_type: MarineSquadMarker::default().into(),
            snowflake: save.serde_snowflake.unwrap_or(Snowflake::new()),
            health: save.serde_health.unwrap_or(prefab.health),
            squad: save.serde_squad.unwrap_or_else(|| prefab.squad.clone()),
            path_finder: save.serde_path_finder.unwrap_or_default(),
            path: save.serde_path.unwrap_or_default(),
            controller: save.serde_controller.unwrap_or(prefab.controller),
            weapon_set: save.serde_weapon_set.unwrap_or(prefab.weapon_set.clone()),
            team_player: save.team_player,
            selectable: Selectable::multiselect(),
            velocity: save.serde_velocity.unwrap_or(SerdeVelocity::default()).into(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl TryFrom<&ObjectAsset> for MarineSquadPrefab {
    type Error = ContentError;
    fn try_from(prefab: &ObjectAsset) -> Result<Self, ContentError> {
        let Some(health) = prefab.health else { return Err(ContentError::MissingHealth); };
        let Some(squad) = prefab.prefab_squad.clone() else { return Err(ContentError::MissingSquad); };
        let Some(controller) = prefab.controller else { return Err(ContentError::MissingController); };
        let Some(weapon_set) = prefab.weapon_set.clone() else { return Err(ContentError::MissingWeapons); };
        let Some(collider_string) = prefab.collider_string.clone() else { return Err(ContentError::MissingColliderString); };
        let Some((vertices, indices)) = decode(collider_string) else { return Err(ContentError::ColliderDecodeError); };

        let collider = Collider::trimesh(vertices, indices);

        Ok(Self {
            health,
            squad: squad.into(),
            controller,
            weapon_set,
            collider,
        })
    }
}

impl<'a> From<SerdeMarineSquadQuery<'a>> for MarineSquadSerde {
    fn from(object: SerdeMarineSquadQuery) -> Self {
        Self {
            serde_snowflake: Some(*object.0),
            serde_health: object.1.saved(),
            serde_squad: object.2.saved(),
            serde_path_finder: object.3.saved(),
            serde_path: object.4.saved(),
            serde_controller: object.5.saved(),
            serde_weapon_set: object.6.saved(),
            serde_velocity: SerdeVelocity::from(*object.7).saved(),
            team_player: *object.8,
            transform: (*object.9).into(),
        }
    }
}

impl From<MarineSquadSerde> for ObjectSpawnEvent {
    fn from(value: MarineSquadSerde) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::MarineSquad,
            spawn_data: SpawnData {
                snowflake: Snowflake::new(),
                teamplayer: value.team_player,
                transform: value.transform.into(),
            },
            serde_data: Some(SerdeData {
                health: value.serde_health,
                squad: value.serde_squad,
                path_finder: value.serde_path_finder,
                path: value.serde_path,
                controller: value.serde_controller,
                weapon_set: value.serde_weapon_set,
                velocity: value.serde_velocity.map(|vel| vel.into()),
                ..default()
            }),
        })
    }
}

pub struct MarineSquadPlugin<S: StateData> {
    state: S,
}

impl<S: StateData> MarineSquadPlugin<S> {
    pub fn new(state: S) -> Self {
        Self {
            state
        }
    }

    pub fn spawn_marine_squad(
        mut spawn_events: EventReader<ObjectSpawnEvent>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut new_marine_squads: Query<(Entity, &TeamPlayer, &mut Squad), Added<MarineSquadMarker>>,
        mut commands: Commands,
    ) {
        for event in spawn_events.iter() {
            if event.0.object_type != ObjectType::MarineSquad { continue; }
            let entity = commands.spawn(MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.0.spawn_data).with_serde_data(event.0.serde_data.clone())).id();
            identifiers.insert(event.0.spawn_data.snowflake, entity);
        }
        new_marine_squads.for_each_mut(|(entity, teamplayer, squad)| {
            let mut offset: f32 = 0.0;
            for (object_type, _) in prefabs.marine_squad_prefab.squad.members.iter().take(squad.current_members.into()) {
                let marine_transform = Transform::from_xyz(offset, 0.0, 0.0);
                let spawn_data = SpawnData {
                    snowflake: Snowflake::new(),
                    teamplayer: *teamplayer,
                    transform: marine_transform,
                };
                match *object_type {
                    ObjectType::Marine => {
                        commands.entity(entity).with_children(|child_builder| {
                            child_builder.spawn(MarineBundle::default().with_spawn_data(spawn_data));
                        });
                    },
                    _ => { },
                };
                offset += 0.75
            }
        });
    }
}

impl<S: StateData> Plugin for MarineSquadPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(self.state.clone())
            .with_system(Self::spawn_marine_squad)
        );
    }
}
