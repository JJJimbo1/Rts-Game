use bevy::prelude::*;
use bevy_pathfinding::Path;
use bevy_rapier3d::prelude::{Collider, RigidBody, Velocity};
use serde::{Serialize, Deserialize};

use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[derive(Component)]
pub struct MarineSquad(Squad);

impl AssetId for MarineSquad {
    fn id(&self) -> &'static str {
        ObjectType::from(self.clone()).id()
    }
}

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

impl SerdeComponent for MarineSquad {
    fn saved(&self) -> Option<Self> {
        if self.0.members == self.0.max_members {
            None
        } else {
            Some(self.clone())
        }
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
    pub path_finder: GroundPathFinder,
    pub path: Path,
    pub controller: Controller,
    pub weapon_set: WeaponSet,
    pub team_player: TeamPlayer,
    pub selectable: Selectable,
    pub velocity: Velocity,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl MarineSquadBundle {
    pub fn with_spawn_data(mut self, spawn_data: ObjectSpawnEventData) -> Self {
        self.team_player = spawn_data.team_player;
        self.transform = spawn_data.transform;
        self
    }
}

impl From<MarineSquadPrefab> for MarineSquadBundle {
    fn from(prefab: MarineSquadPrefab) -> Self {
        Self {
            marine_squad: prefab.marine_squad,
            object_type: MarineSquad::default().into(),
            asset_type: MarineSquad::default().into(),
            snowflake: Snowflake::new(),
            health: prefab.health,
            path_finder: GroundPathFinder::default(),
            path: Path::default(),
            controller: prefab.controller,
            weapon_set: prefab.weapon_set,
            team_player: TeamPlayer::default(),
            selectable: Selectable::multiselect(),
            velocity: Velocity::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.real_collider.clone().unwrap(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl From<(SerdeMarineSquad, &MarineSquadPrefab)> for MarineSquadBundle {
    fn from((save, prefab): (SerdeMarineSquad, &MarineSquadPrefab)) -> Self {
        Self {
            marine_squad: save.marine_squad.unwrap_or_else(|| prefab.marine_squad.clone()),
            object_type: MarineSquad::default().into(),
            asset_type: MarineSquad::default().into(),
            snowflake: save.snowflake.unwrap_or_else(|| Snowflake::new()),
            health: save.health.unwrap_or(prefab.health),
            path_finder: save.path_finder.unwrap_or_default(),
            path: save.path.unwrap_or_default(),
            controller: save.controller.unwrap_or(prefab.controller),
            weapon_set: save.weapon_set.unwrap_or(prefab.weapon_set.clone()),
            team_player: save.team_player,
            velocity: save.velocity.unwrap_or(SerdeVelocity::default()).into(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: prefab.real_collider.clone().unwrap(),
            selectable: Selectable::multiselect(),
            transform: save.transform.into(),
            global_transform: GlobalTransform::default(),
        }
    }
}


#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct MarineSquadPrefab {
    pub stack: (ActiveQueue, StackData),
    pub marine_squad: MarineSquad,
    pub health: Health,
    pub controller: Controller,
    pub weapon_set: WeaponSet,
    pub collider_string: String,
    #[serde(skip)]
    pub real_collider: Option<Collider>,
}

impl MarineSquadPrefab {
    pub fn with_real_collider(mut self, collider: Collider) -> Self {
        self.real_collider = Some(collider);
        self
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SerdeMarineSquad {
    pub snowflake: Option<Snowflake>,
    pub marine_squad: Option<MarineSquad>,
    pub health: Option<Health>,
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
            marine_squad: object.1.saved(),
            health: object.2.saved(),
            path_finder: object.3.saved(),
            path: object.4.saved(),
            controller: object.5.saved(),
            weapon_set: object.6.saved(),
            velocity: SerdeVelocity::from(*object.7).saved(),
            team_player: *object.8,
            transform: (*object.9).into(),
        }
    }
}

pub fn marine_squad_spawn(
    mut resource_nodes: Query<(Entity, &TeamPlayer, &MarineSquad), Added<MarineSquad>>,
    mut commands: Commands,
) {
    resource_nodes.for_each_mut(|(entity, teamplayer, marine_squads)| {
        let mut offset: f32 = 0.0;
        for _ in marine_squads.0.member_ids.len()..marine_squads.0.members as usize {



            let marine_transform = Transform::from_xyz(offset, 0.0, 0.0);
            let spawn_data = ObjectSpawnEventData {
                snowflake: Snowflake::new(),
                object_type: ObjectType::ResourcePlatformUnclaimed,
                team_player: *teamplayer,
                transform: marine_transform,
            };
            commands.entity(entity).with_children(|child_builder| {
                child_builder.spawn_bundle(MarineBundle::default().with_spawn_data(spawn_data));
            });
            offset += 0.75
        }
    });
}