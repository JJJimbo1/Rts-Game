use bevy::{prelude::*, ecs::schedule::StateData};
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[derive(Component)]
pub struct MarineSquad;

impl From<MarineSquad> for ObjectType {
    fn from(_: MarineSquad) -> Self {
        ObjectType::MarineSquad
    }
}

impl From<MarineSquad> for AssetType {
    fn from(_: MarineSquad) -> Self {
        Self::Object(ObjectType::MarineSquad)
    }
}

#[derive(Clone)]
#[derive(Bundle)]
pub struct MarineSquadBundle {
    pub marine_squad: MarineSquad,
    pub object_type: ObjectType,
    pub asset_type: AssetType,
    pub snowflake: Snowflake,
    pub health: Health,
    pub squad: Squad,
    pub path_finder: GroundPathFinder,
    pub path: Path,
    pub controller: Controller,
    pub weapon_set: WeaponSet,
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

impl MarineSquadBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        self.snowflake = spawn_data.snowflake;
        self.team_player = spawn_data.teamplayer;
        self.transform = spawn_data.transform;
        self
    }
}

impl From<MarineSquadPrefab> for MarineSquadBundle {
    fn from(prefab: MarineSquadPrefab) -> Self {
        Self {
            marine_squad: MarineSquad,
            object_type: MarineSquad::default().into(),
            asset_type: MarineSquad::default().into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            squad: prefab.squad.into(),
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

impl From<(SerdeMarineSquad, &MarineSquadPrefab)> for MarineSquadBundle {
    fn from((save, prefab): (SerdeMarineSquad, &MarineSquadPrefab)) -> Self {
        Self {
            marine_squad: MarineSquad,
            object_type: MarineSquad::default().into(),
            asset_type: MarineSquad::default().into(),
            snowflake: save.snowflake.unwrap_or_else(|| Snowflake::new()),
            health: save.health.unwrap_or(prefab.health),
            squad: save.squad.unwrap_or_else(|| prefab.squad.clone()),
            path_finder: save.path_finder.unwrap_or_default(),
            path: save.path.unwrap_or_default(),
            controller: save.controller.unwrap_or(prefab.controller),
            weapon_set: save.weapon_set.unwrap_or(prefab.weapon_set.clone()),
            team_player: save.team_player,
            selectable: Selectable::multiselect(),
            velocity: save.velocity.unwrap_or(SerdeVelocity::default()).into(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.collider.clone(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}


#[derive(Clone)]
pub struct MarineSquadPrefab {
    pub health: Health,
    pub squad: Squad,
    pub controller: Controller,
    pub weapon_set: WeaponSet,
    pub collider: Collider,
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

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SerdeMarineSquad {
    pub snowflake: Option<Snowflake>,
    pub health: Option<Health>,
    pub squad: Option<Squad>,
    pub path_finder: Option<GroundPathFinder>,
    pub path: Option<Path>,
    pub controller: Option<Controller>,
    pub weapon_set: Option<WeaponSet>,
    pub velocity: Option<SerdeVelocity>,
    pub team_player: TeamPlayer,
    pub transform: SerdeTransform,
}

impl<'a> From<SerdeMarineSquadQuery<'a>> for SerdeMarineSquad {
    fn from(object: SerdeMarineSquadQuery) -> Self {
        Self {
            snowflake: Some(*object.0),
            // marine_squad: None,
            health: object.2.saved(),
            squad: object.3.saved(),
            path_finder: object.4.saved(),
            path: object.5.saved(),
            controller: object.6.saved(),
            weapon_set: object.7.saved(),
            velocity: SerdeVelocity::from(*object.8).saved(),
            team_player: *object.9,
            transform: (*object.10).into(),
        }
    }
}

impl From<SerdeMarineSquad> for ObjectSpawnEvent {
    fn from(value: SerdeMarineSquad) -> Self {
        Self(ObjectSpawnEventData{
            object_type: ObjectType::MarineSquad,
            snowflake: Snowflake::new(),
            teamplayer: value.team_player,
            transform: value.transform.into(),
        })
    }
}

pub struct MarineSquadPlugin<T: StateData> {
    state: T,
}

impl<T: StateData> MarineSquadPlugin<T> {
    pub fn spawn_marine_squad(
        mut spawn_events: EventReader<ObjectSpawnEvent>,
        prefabs: Res<ObjectPrefabs>,
        mut identifiers: ResMut<Identifiers>,
        mut new_marine_squads: Query<(Entity, &TeamPlayer, &mut Squad), Added<MarineSquad>>,
        mut commands: Commands,
    ) {
    
        for event in spawn_events.iter() {
            if event.0.object_type != ObjectType::MarineSquad { continue; }
            let entity = commands.spawn(MarineSquadBundle::from(prefabs.marine_squad_prefab.clone()).with_spawn_data(event.0)).id();
            identifiers.insert(event.0.snowflake, entity);
        }
        new_marine_squads.for_each_mut(|(entity, teamplayer, squad)| {
            let mut offset: f32 = 0.0;
            for (object_type, _) in prefabs.marine_squad_prefab.squad.members.iter().take(squad.current_members.into()) {
                let marine_transform = Transform::from_xyz(offset, 0.0, 0.0);
                let spawn_data = ObjectSpawnEventData {
                    object_type: *object_type,
                    snowflake: Snowflake::new(),
                    teamplayer: *teamplayer,
                    transform: marine_transform,
                };
                match spawn_data.object_type {
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

impl<T: StateData> Plugin for MarineSquadPlugin<T> {
    fn build(&self, app: &mut App) {
        
    }
}
