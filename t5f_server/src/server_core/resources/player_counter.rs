#[allow(dead_code)]
pub mod player_counter {
    use std::fmt;

    use super::super::Player;

    type Result<T> = std::result::Result<T, PlayerCounterError>;
    type Max = u8;

    pub struct PlayerCounter {
        player_ids : [bool; 255],
    }

    impl PlayerCounter {
        pub fn new() -> Self {
            Self::default()
        }

        fn next_available_player_id(&self) -> Result<usize> {
            for i in 0..self.player_ids.len() {
                if self.player_ids[i] == false {
                    return Ok(i);
                }
            }
            Err(PlayerCounterError)
        }

        pub fn allocate_next_player(&mut self) -> Result<Player> {
            match self.next_available_player_id() {
                Ok(x) => {
                    self.player_ids[x] = true;
                    Ok(Player::new(x as u8))
                },
                Err(e) => Err(e)
            }
        }

        pub fn deallocate_id(&mut self, id : u8) {
            match self.player_ids.get_mut(id as usize) {
                Some(x) => {
                    *x = false;
                },
                None => {}
            };
        }

        ///Returns the number of slots taken and open, respectively.
        pub fn slots(&self) -> (u16, u16) {
            let mut taken : u16 = 0;
            for i in 0..self.player_ids.len() {
                if self.player_ids[i] == true {
                    taken += 1;
                }
            };
            (taken, 256 - taken)
        }
    }

    impl Default for PlayerCounter {
        fn default() -> Self {
            Self {
                player_ids : [false; 255]
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct PlayerCounterError;

    impl fmt::Display for PlayerCounterError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "No player slots available.")
        }
    }
}