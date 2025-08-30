pub mod health;
pub mod weapon;

pub use health::*;
pub use weapon::*;

use bevy::{math::Vec3Swizzles, platform::collections::HashMap, prelude::*};
use xtrees::{Quad, QuadTree};
use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Event)]
pub struct ObjectKilledEvent(pub Entity);


#[derive(Debug, Default, Clone)]
#[derive(Resource)]
pub struct CombatWorld {
    pub layers: HashMap<TeamPlayer, QuadTree<Entity>>
}

impl CombatWorld {
    pub fn new(actors: &Commanders , map: &MapBounds) -> Self {
        let mut layers = HashMap::new();
        for a in actors.commanders.keys() {
            layers.insert(*a, QuadTree::new(Quad::new(0.0, 0.0, map.0.x as f32, map.0.y as f32)));
        }
        Self {
            layers,
        }
    }

    pub fn insert(&mut self, tp: TeamPlayer, ent: Entity, quad: Quad) {
        match self.layers.get_mut(&tp) {
            Some(x) => {
                x.insert(ent, quad);
            },
            None => { }
        }
    }

    pub fn clear_trees(&mut self) {
        for i in self.layers.values_mut() {
            i.clear();
        }
    }

    pub fn is_players(&self, e: Entity, player_id: TeamPlayer, team_players: Query<&TeamPlayer>) -> Result<bool, String> {
        match team_players.get(e) {
            Ok(x) => {
                Ok(x.team() == player_id.team() && x.player() == player_id.player())
            },
            Err(_) => {
                Err(String::from("Entity is not owned."))
            }
        }
    }

    pub fn is_ally(&self, e: Entity, player_id: TeamPlayer, team_players: Query<&TeamPlayer>) -> Result<bool, String> {
        match team_players.get(e) {
            Ok(x) => {
                Ok(x.team() == player_id.team() && x.player() != player_id.player())
            },
            Err(_) => {
                Err(String::from("Entity is not owned."))
            }
        }
    }

    pub fn is_players_or_ally(&self, e: Entity, player_id: TeamPlayer, team_players: Query<&TeamPlayer>) -> Result<bool, String> {
        match team_players.get(e) {
            Ok(x) => {
                Ok(x.team() == player_id.team())
            },
            Err(_) => {
                Err(String::from("Entity is not owned."))
            }
        }
    }

    pub fn is_enemy(&self, e: Entity, player_id: TeamPlayer, team_players: &Query<&TeamPlayer>) -> Result<bool, String> {
        match team_players.get(e) {
            Ok(x) => {
                Ok(x.team() != player_id.team())
            },
            Err(_) => {
                Err(String::from("Entity is not owned."))
            }
        }
    }

    pub fn search_targets(&self, id: TeamPlayer, position: Vec3, weapon: &Weapon) -> Vec<Entity> {
        let pos = position.xz();

        match weapon.target_force {
            TargetForce::Mine => { self.search_mine(id, pos, weapon.range) },
            TargetForce::Ally => { self.search_allies(id, pos, weapon.range) },
            TargetForce::Team => { self.search_mine_or_allies(id, pos, weapon.range) },
            TargetForce::Enemy => { self.search_enemies(id, pos, weapon.range) },
        }.iter().filter_map(|(e, target_pos)| (pos.distance(*target_pos) <= weapon.range).then_some(*e) ).collect()
    }

    fn search_mine(&self, id: TeamPlayer, position: Vec2, range: f32) -> Vec<(Entity, Vec2)> {
        self.layers.iter()
            .filter(|(_id, _)| _id.team() == id.team() && _id.player() == id.player())
            .map(|(_id, tree)| {
                tree.search(&Quad::new(position.x, position.y, range, range)).iter().map(|(entity, quad)| (*entity, Vec2::new(quad.x, quad.y))).collect::<Vec<_>>()
            }).flatten().collect()
    }

    fn search_allies(&self, id: TeamPlayer, position: Vec2, range: f32) -> Vec<(Entity, Vec2)> {
        self.layers.iter()
            .filter(|(_id, _)| _id.team() == id.team() && _id.player() != id.player())
            .map(|(_id, tree)| {
                tree.search(&Quad::new(position.x, position.y, range, range)).iter().map(|(entity, quad)| (*entity, Vec2::new(quad.x, quad.y))).collect::<Vec<_>>()
            }).flatten().collect()
    }

    fn search_mine_or_allies(&self, id: TeamPlayer, position: Vec2, range: f32) -> Vec<(Entity, Vec2)> {
        self.layers.iter()
            .filter(|(_id, _)| _id.team() == id.team())
            .map(|(_id, tree)| {
                tree.search(&Quad::new(position.x, position.y, range, range)).iter().map(|(entity, quad)| (*entity, Vec2::new(quad.x, quad.y))).collect::<Vec<_>>()
            }).flatten().collect()
    }

    fn search_enemies(&self, id: TeamPlayer, position: Vec2, range: f32) -> Vec<(Entity, Vec2)> {
        self.layers.iter()
            .filter(|(_id, _)| _id.team() != id.team())
            .map(|(_id, tree)| {
                tree.search(&Quad::new(position.x, position.y, range, range)).iter().map(|(entity, quad)| (*entity, Vec2::new(quad.x, quad.y))).collect::<Vec<_>>()
            }).flatten().collect()
    }
}

#[derive(Default)]
pub struct CombatPlugin;

impl CombatPlugin {
    fn targeting_system(
        teamplayer_world: Res<CombatWorld>,
        transforms: Query<&Transform>,
        mut query: Query<(&Transform, &mut PathFinder, &mut Navigator, &mut WeaponSet, &TeamPlayer,)>,
    ) {
        //TODO: Make sure weapons can only target the target if they are able to.
        query.iter_mut().for_each(|(transform, mut pathfinder, mut navigator, mut weapon_set, teamplayer)| {
            match navigator.pursue {
                Some(target) => {
                    if let Ok(target_transform) = transforms.get(target) {
                        let pos = transform.translation.xz();
                        let target_pos = target_transform.translation.xz();
                        if pos.distance(target_pos) > weapon_set.closing_range {
                            let start = transform.translation.xz();
                            let end = target_transform.translation.xz() + (pos - target_pos).normalize() * weapon_set.closing_range;
                            pathfinder.set_trip((start, end));
                        }

                        for weapon in weapon_set.weapons.iter_mut() {
                            if pos.distance(target_pos) > weapon.range {
                                weapon.target = Target::ManualTarget(target);
                            } else if let Target::AutoTarget(_) = weapon.target {

                            } else {
                                weapon.target = Target::None
                            }
                        }
                    } else {
                        navigator.pursue = None;
                    }
                },
                None => {
                    for weapon in weapon_set.weapons.iter_mut() {
                        if let Target::ManualTarget(_) = weapon.target {
                            weapon.target = Target::None;
                        } else if let Target::AutoTarget(target) = weapon.target {
                            if let Ok(target_transform) = transforms.get(target) {
                                if transform.translation.xz().distance(target_transform.translation.xz()) > weapon.range {
                                    weapon.target = Target::None;
                                }
                            } else {
                                weapon.target = Target::None;
                            }
                        }
                        if let Target::None = weapon.target {
                            let targets = teamplayer_world.search_targets(*teamplayer, transform.translation, weapon);
                            if let Some(e) = targets.first() {
                                weapon.target = Target::AutoTarget(*e);
                            }
                        }
                    }
                }
            }
        });
    }

    fn weapons_system(
        time: Res<Time>,
        mut weapons: Query<&mut WeaponSet>,
        mut healths: Query<&mut Health>
    ) {
        weapons.iter_mut().for_each(|mut wep| {
            for weapon in wep.weapons.iter_mut() {
                if weapon.cooldown > 0.0 {
                    weapon.cooldown -= time.delta_secs();
                }
                if weapon.cooldown > 0.0 {
                    continue;
                }
                if let Some(mut health) = weapon.target.get_target().and_then(|target| healths.get_mut(target).ok()) {
                    health.damage(weapon.damage, weapon.damage_types);
                    weapon.cooldown = weapon.fire_rate;
                }
            }
        });
    }

    fn health_system(
        mut objects_killed_writer: EventWriter<ObjectKilledEvent>,
        query: Query<(Entity, &Health)>,
        mut commands: Commands,
    ) {
        query.iter().for_each(|(entity, health)| {
            if health.is_dead() {
                if let Ok(mut entity_commands) = commands.get_entity(entity) {
                    entity_commands.despawn();
                }
                objects_killed_writer.write(ObjectKilledEvent(entity));
            }
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct CombatSystems;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {

        app
            .add_systems(Update, (
                Self::targeting_system.after(CommandSystems),
                Self::weapons_system.after(Self::targeting_system),
                Self::health_system.after(Self::weapons_system),
            ).in_set(CombatSystems))
        ;
    }
}
