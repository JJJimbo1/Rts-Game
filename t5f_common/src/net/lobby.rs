use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use bimap::BiMap;
use crate::TeamPlayer;


#[derive(Debug, Clone)]
#[derive(Resource)]
pub struct Lobby {
    players: BiMap<ClientId, TeamPlayer>,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            players: BiMap::new(),
        }
    }

    pub fn insert_player(&mut self, id: ClientId, player: TeamPlayer,) {
        self.players.insert(id, player);
    }

    pub fn remove_player(&mut self, player: TeamPlayer) {
        self.players.remove_by_right(&player);
    }

    pub fn player_exists(&self, player: TeamPlayer) -> bool {
        self.players.contains_right(&player)
    }

    pub fn players_on_team(&self, player: TeamPlayer) -> u32 {
        self.players.right_values().filter(|p| p.team() == player.team()).count() as u32
    }

    pub fn players(&self) -> Vec<ClientId> {
        self.players.left_values().into_iter().cloned().collect()
    }

    pub fn players_except(&self, player: TeamPlayer) -> Vec<ClientId> {
        self.players.iter().filter_map(|(id, p)| (*p != player).then_some(*id)).collect()
    }
}