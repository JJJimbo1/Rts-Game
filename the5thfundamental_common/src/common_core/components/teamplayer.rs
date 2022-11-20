pub use teamplayer::*;
mod teamplayer {

    use std::collections::HashMap;

    use bevy::{ecs::component::Component, math::{Vec2, Vec3}, prelude::{Entity, Query, Res, Resource}};

    use serde::{
        Serialize, Deserialize,
    };
    use xtrees::quad::*;

    use crate::{Actors, MapBounds, TargetForce, Weapon};

    #[derive(Debug, Default, Clone, Copy, Hash, Eq)]
    #[derive(Serialize, Deserialize)]
    #[derive(Component)]
    pub struct TeamPlayer {
        pub team : usize,
        pub player : usize,
    }

    impl TeamPlayer {
        pub fn new(team : usize, player : usize) -> Self {
            Self {
                team,
                player,
            }
        }

        pub fn team(&self) -> usize {
            self.team
        }

        pub fn player(&self) -> usize {
            self.player
        }

        pub fn reassign(&mut self, team : usize, player : usize) {
            self.team = team;
            self.player = player;
        }
    }

    impl PartialEq for TeamPlayer {
        fn eq(&self, other: &TeamPlayer) -> bool {
            self.team == other.team && self.player == other.player
        }
    }

    #[derive(Debug, Default, Clone)]
    #[derive(Resource)]
    pub struct TeamPlayerWorld {
        pub layers : HashMap<TeamPlayer, QuadTree<Entity>>
    }

    impl TeamPlayerWorld {
        pub fn new(actors : &Actors , map : &MapBounds) -> Self {
            let mut tpw = Self {
                layers : HashMap::new(),
            };
            for a in actors.actors.keys() {
                tpw.layers.insert(*a, QuadTree::new(Quad::new(0.0, 0.0, map.0.x as f32, map.0.y as f32), 17, 8));
            }
            tpw
        }

        pub fn sort(tps : Query<&TeamPlayer>) -> Vec<Vec<usize>> {
            let mut team_capacity : usize = 0;

            tps.for_each(|tp| {
                if tp.team() + 1 > team_capacity {
                    team_capacity = tp.team() + 1;
                }
            });

            let mut player_capacities : Vec<usize> = Vec::with_capacity(team_capacity);

            for i in 0..player_capacities.capacity() {
                player_capacities.insert(i, 0);
            }

            tps.for_each(|tp| {
                if tp.player() + 1 > player_capacities[tp.team()] {
                    player_capacities[tp.team()] = tp.player() + 1;
                }
            });

            let mut team_player_capacities : Vec<Vec<usize>> = Vec::<Vec<usize>>::with_capacity(team_capacity);

            for i in 0..team_player_capacities.capacity() {
                team_player_capacities.insert(i, Vec::<usize>::with_capacity(player_capacities[i]));
                for j in 0..team_player_capacities[i].capacity() {
                    team_player_capacities[i].insert(j, 0)
                }
            }

            return team_player_capacities;
        }

        pub fn insert(&mut self, tp : TeamPlayer, ent : Entity, quad : Quad) {
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

        pub fn is_owned(&self, e : Entity, team_players : Query<&TeamPlayer>) -> bool {
            team_players.get(e).is_ok()
        }

        pub fn is_mine(&self, e : Entity, player_id : TeamPlayer, team_players : Query<&TeamPlayer>) -> Result<bool, String> {
            match team_players.get(e) {
                Ok(x) => {
                    Ok(x.team() == player_id.team() && x.player() == player_id.player())
                },
                Err(_) => {
                    Err(String::from("Entity is not owned."))
                }
            }
        }

        pub fn is_ally(&self, e : Entity, player_id : TeamPlayer, team_players : Query<&TeamPlayer>) -> Result<bool, String> {
            match team_players.get(e) {
                Ok(x) => {
                    Ok(x.team() == player_id.team() && x.player() != player_id.player())
                },
                Err(_) => {
                    Err(String::from("Entity is not owned."))
                }
            }
        }

        pub fn is_mine_or_ally(&self, e : Entity, player_id : TeamPlayer, team_players : Query<&TeamPlayer>) -> Result<bool, String> {
            match team_players.get(e) {
                Ok(x) => {
                    Ok(x.team() == player_id.team())
                },
                Err(_) => {
                    Err(String::from("Entity is not owned."))
                }
            }
        }

        pub fn is_enemy(&self, e : Entity, player_id : TeamPlayer, team_players : &Query<&TeamPlayer>) -> Result<bool, String> {
            match team_players.get(e) {
                Ok(x) => {
                    Ok(x.team() != player_id.team())
                },
                Err(_) => {
                    Err(String::from("Entity is not owned."))
                }
            }
        }

        pub fn search_targets(&self, id : TeamPlayer, position : Vec3, weapon : &Weapon) -> Vec<Entity> {
            let pos = Vec2::new(position.x, position.z);
            match weapon.target_force {
                TargetForce::Mine => { self.search_mine(id, pos, weapon.range) },
                TargetForce::Ally => { self.search_allies(id, pos, weapon.range) },
                TargetForce::MineOrAlly => { self.search_mine_or_allies(id, pos, weapon.range) },
                TargetForce::Enemy => { self.search_enemies(id, pos, weapon.range) },
            }.iter().filter_map(|(e, pos)| if mathfu::D2::distance_magnitude((position.x, position.z), (pos.x, pos.y)) <= weapon.range.powi(2) { Some(*e) } else { None}).collect()

            // results
        }

        pub fn search_mine(&self, id : TeamPlayer, position : Vec2, range : f32) -> Vec<(Entity, Vec2)> {
            let mut results : Vec<(Entity, Vec2)> = Vec::new();
            for i in self.layers.iter() {
                if i.0.team() == id.team() && i.0.player() == id.player() {
                    for sr in i.1.search_simple(&Quad::new(position.x, position.y, range, range)).iter() {
                        results.push((sr.0, Vec2::new(sr.1.x, sr.1.y)));
                    }
                }
            }
            results
        }

        pub fn search_allies(&self, id : TeamPlayer, position : Vec2, range : f32) -> Vec<(Entity, Vec2)> {
            let mut results : Vec<(Entity, Vec2)> = Vec::new();
            for i in self.layers.iter() {
                if i.0.team() == id.team() && i.0.player() != id.player() {
                    for sr in i.1.search_simple(&Quad::new(position.x, position.y, range, range)).iter() {
                        results.push((sr.0, Vec2::new(sr.1.x, sr.1.y)));
                    }
                }
            }
            results
        }

        pub fn search_mine_or_allies(&self, id : TeamPlayer, position : Vec2, range : f32) -> Vec<(Entity, Vec2)> {
            let mut results : Vec<(Entity, Vec2)> = Vec::new();
            for i in self.layers.iter() {
                if i.0.team() == id.team() {
                    for sr in i.1.search_simple(&Quad::new(position.x, position.y, range, range)).iter() {
                        results.push((sr.0, Vec2::new(sr.1.x, sr.1.y)));
                    }
                }
            }
            results
        }

        pub fn search_enemies(&self, id : TeamPlayer, position : Vec2, range : f32) -> Vec<(Entity, Vec2)> {
            let mut results : Vec<(Entity, Vec2)> = Vec::new();
            for i in self.layers.iter() {
                if i.0.team() != id.team() {
                    for sr in i.1.search_simple(&Quad::new(position.x, position.y, range, range)).iter() {
                        results.push((sr.0, Vec2::new(sr.1.x, sr.1.y)));
                    }
                }
            }
            results
        }

        pub fn player_count(team_players : Query<&TeamPlayer>) -> usize {
            let layers = Self::sort(team_players);
            let mut count : usize = 0;

            for i in 0..layers.len() {
                count += layers[i].len();
            }

            count
        }

        pub fn under_player_count(team_player : TeamPlayer, tps : Query<&TeamPlayer>) -> usize {
            let mut count : usize = 0;

            tps.for_each(|tp| {
                if *tp == team_player {
                    count += 1;
                }
            });

            count
        }

        // pub fn under_player_values<'a, N : Component>(team_player : TeamPlayer, query : Query<(&TeamPlayer, &N)>) -> Vec<&'a N> {
        //     let mut results : Vec<&'a N> = Vec::with_capacity(query.iter().count() / 2);

        //     query.for_each(|(tp, item)| {
        //         if *tp == team_player {
        //             results.push();
        //         }
        //     });

        //     results
        // }
    }
}