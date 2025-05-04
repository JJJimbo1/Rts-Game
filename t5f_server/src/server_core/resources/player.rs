pub mod player {
    #[derive(Debug, Clone, Copy)]
    pub struct Player {
        id : u8,
    }

    impl Player {
        pub fn new(id : u8) -> Self {
            Self {
                id,
            }
        }

        pub fn id(&self) -> u8 {
            self.id
        }
    }
}