#[allow(dead_code)]
pub mod lobby {

    use std::{
        fmt,
        net::{
            SocketAddr, IpAddr,
        },
    };
    use log::*;
    use multi_map::MultiMap;
    //#[derive(Eq, PartialEq, Hash)]
    pub enum ID {
        Address(SocketAddr),
        Number(u8),
    }

    use super::super::{
        Player, PlayerCounter,
    };

    #[derive(Default)]
    pub struct Lobby {
        host : Option<SocketAddr>,
        players : MultiMap<SocketAddr, u8, Player>,
        player_counter : PlayerCounter,
        ban_list : Vec<IpAddr>,
    }

    impl Lobby {
        pub fn add_player(&mut self, addr : SocketAddr) -> Result<(), LobbyError> {
            if self.host.is_none() {
                self.host = Some(addr);
            }
            //info!("{}", addr);
            match self.player_counter.allocate_next_player() {
                Ok(x) => {
                    self.players.insert(addr, x.id(), x);
                    return Ok(());
                },
                Err(_) => {
                    return Err(LobbyError::MaxPlayersReached)
                }
            };
        }

        pub fn remove_player(&mut self, id : ID) -> Result<(), LobbyError> {
            match id {
                ID::Address(addr) => {
                    match self.players.remove(&addr) {
                        Some(_) => {
                            return Ok(());
                        },
                        None => {
                            return Err(LobbyError::PlayerDoesNotExist);
                        }
                    }
                },
                ID::Number(y) => {
                    match self.players.remove_alt(&y) {
                        Some(_) => {
                            return Ok(());
                        },
                        None => {
                            return Err(LobbyError::PlayerDoesNotExist);
                        }
                    }
                }
            };
        }

        pub fn contains(&self, id : ID) -> bool {
            match id {
                ID::Address(x) => {
                    self.players.get(&x).is_some()
                },
                ID::Number(x) => {
                    self.players.get_alt(&x).is_some()
                }
            }
        }

        pub fn ban(&mut self, addr : IpAddr) {
            self.ban_list.push(addr);
        }

        pub fn banned(&self, addr : IpAddr) -> bool {
            for i in self.ban_list.iter() {
                if i == &addr {
                    return true
                }
            }
            return false;
        }

        pub fn allowed_to_join(&self, addr : SocketAddr) -> Result<(), LobbyError> {
            match (self.banned(addr.ip()), self.contains(ID::Address(addr))) {
                (true, _) => {
                    Err(LobbyError::PlayerIsBanned)
                },
                (_, true) => {
                    Err(LobbyError::DuplicatePlayerAddress)
                },
                (false, false) => {
                    Ok(())
                }
            }
        }

        pub fn players(&self) -> &MultiMap<SocketAddr, u8, Player> {
            &self.players
        }

        pub fn get_player(&self, id : ID) -> Option<&Player> {
            match id {
                ID::Address(x) => {
                    self.players.get(&x)
                }
                ID::Number(x) => {
                    self.players.get_alt(&x)
                }
            }
        }

        pub fn player_by_id_mut(&mut self, id : u8) -> Option<&mut Player> {
            self.players.get_mut_alt(&id)
        }
    }

    pub enum LobbyError {
        MaxPlayersReached,
        PlayerDoesNotExist,
        PlayerIsBanned,
        DuplicatePlayerAddress,
    }

    impl fmt::Display for LobbyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                LobbyError::MaxPlayersReached => {
                    write!(f, "Maximum number of players has been reached.")
                },
                LobbyError::PlayerDoesNotExist => {
                    write!(f, "Player does not exist.")
                },
                LobbyError::PlayerIsBanned => {
                    write!(f, "Player has been banned from the server")
                },
                LobbyError::DuplicatePlayerAddress => {
                    write!(f, "Player is already connected to server.")
                }
            }
        }
    }
}