pub use buffer::*;

mod buffer {
    use bevy::prelude::Component;
    use serde::{Deserialize, Serialize};
    use snowflake::ProcessUniqueId;

    use crate::{SMALL_BUFFER_SIZE, MEDIUM_BUFFER_SIZE, LARGE_BUFFER_SIZE};

    pub type SmallBuffer = [u8; SMALL_BUFFER_SIZE];
    pub type MediumBuffer = [u8; MEDIUM_BUFFER_SIZE];
    pub type LargeBuffer = [u8; LARGE_BUFFER_SIZE];

    pub struct LimitedBuffer;

    impl LimitedBuffer {
        pub fn new_small() -> SmallBuffer {
            [0; SMALL_BUFFER_SIZE]
        }
        pub fn new_medium() -> MediumBuffer {
            [0; MEDIUM_BUFFER_SIZE]
        }
        pub fn new_large() -> LargeBuffer {
            [0; LARGE_BUFFER_SIZE]
        }
        pub fn small_from_string(string : &str) -> SmallBuffer {
            let mut b = [0; SMALL_BUFFER_SIZE];
            for c in string.to_string().char_indices() {
                if c.0 > SMALL_BUFFER_SIZE { break; }
                b[c.0] = c.1 as u8;
            }
            b
        }
        pub fn medium_from_string(string : &str) -> MediumBuffer {
            let mut b = [0; MEDIUM_BUFFER_SIZE];
            for c in string.to_string().char_indices() {
                if c.0 > MEDIUM_BUFFER_SIZE { break; }
                b[c.0] = c.1 as u8;
            }
            b
        }
        pub fn large_from_string(string : &str) -> LargeBuffer {
            let mut b = [0; LARGE_BUFFER_SIZE];
            for c in string.to_string().char_indices() {
                if c.0 > LARGE_BUFFER_SIZE { break; }
                b[c.0] = c.1 as u8;
            }
            b
        }
        pub fn to_string(buffer : &[u8]) -> String {
            let mut s = String::with_capacity(buffer.len());
            for c in buffer {
                if *c != 0 {
                    s.push(*c as char);
                }
            }
            s
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    #[derive(Serialize, Deserialize)]
    #[derive(Component)]
    pub struct SnowFlake(pub ProcessUniqueId);
}