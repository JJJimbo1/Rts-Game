use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Copy, Hash, Eq)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct TeamPlayer {
    pub team: usize,
    pub player: usize,
}

impl TeamPlayer {

    pub const PLAYER_ID: TeamPlayer = TeamPlayer { team: 1, player: 0};

    pub fn new(team: usize, player: usize) -> Self {
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

    pub fn reassign(&mut self, team: usize, player: usize) {
        self.team = team;
        self.player = player;
    }
}

impl PartialEq for TeamPlayer {
    fn eq(&self, other: &TeamPlayer) -> bool {
        self.team == other.team && self.player == other.player
    }
}