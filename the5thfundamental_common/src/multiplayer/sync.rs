pub use sync::*;
mod sync {

    pub enum NetMode {
        Single,
        Client,
        Server,
        ///Server + Client
        Host,
    }

    impl NetMode {
        pub fn is_singleplayer(&self) -> bool {
            match self {
                NetMode::Single => true,
                NetMode::Client => false,
                NetMode::Server => false,
                NetMode::Host => false,
            }
        }

        pub fn is_multiplayer(&self) -> bool {
            !self.is_singleplayer()
        }
    }
}