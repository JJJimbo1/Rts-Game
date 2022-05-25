pub use buffer::*;

mod buffer {
    use bevy::prelude::Component;
    use serde::{Deserialize, Serialize};
    use snowflake::ProcessUniqueId;

    use crate::{SMALL_BUFFER_SIZE, MEDIUM_BUFFER_SIZE, LARGE_BUFFER_SIZE};

    #[derive(Debug, Clone, Copy)]
    pub struct LimitedBuffer<const B: usize> {
        message: [u8; B],
    }

    impl<const U: usize> From<LimitedBuffer<U>> for String {
        fn from(buffer: LimitedBuffer<U>) -> Self {
            let mut s = String::with_capacity(buffer.message.len());
            for c in buffer.message {
                if c != 0 {
                    s.push(c as char);
                }
            }
            s
        }
    }

    impl<const U: usize> From<String> for LimitedBuffer<U> {
        fn from(s: String) -> Self {
            let mut b = [0; U];
            for c in s.to_string().char_indices() {
                if c.0 > U { break; }
                b[c.0] = c.1 as u8;
            }
            Self { message: b }
        }
    }

    pub type SmallBuffer = LimitedBuffer<SMALL_BUFFER_SIZE>;
    pub type MediumBuffer = LimitedBuffer<MEDIUM_BUFFER_SIZE>;
    pub type LargeBuffer = LimitedBuffer<LARGE_BUFFER_SIZE>;

    // #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[derive(Serialize, Deserialize)]
    #[derive(Component)]
    pub struct Snowflake(pub ProcessUniqueId);

    impl Snowflake {
        pub fn new() -> Self {
            Self(ProcessUniqueId::new())
        }
    }
}