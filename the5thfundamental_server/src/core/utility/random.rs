pub mod random{

    use mathfu::D1;
    #[derive(Copy, Clone)]
    pub struct Random {
        s1 : i32,
        s2 : i32,
        s3 : i32,
    }

    impl Random {
        /*pub fn new(s1 : i32, s2 : i32, s3 : i32) -> Self {
            Self {
                s1,
                s2,
                s3,
            }
        }*/
        /*
        [r, s1, s2, s3] = function(s1, s2, s3) is
            s1, s2, s3 should be random from 1 to 30,000. Use clock if available.
            s1 := mod(171 × s1, 30269)
            s2 := mod(172 × s2, 30307)
            s3 := mod(170 × s3, 30323)

            r := mod(s1/30269.0 + s2/30307.0 + s3/30323.0, 1)
        */

        pub fn cycle(&mut self) -> f32 {
            self.s1 = 171 * self.s1 % 30269;
            self.s2 = 172 * self.s2 % 30307;
            self.s3 = 170 * self.s3 % 30323;

            (self.s1 as f32 / 30269.0 + self.s2 as f32 / 30307.0 + self.s3 as f32 / 30323.0) % 1.0
        }


        pub fn boolean(&mut self) -> bool {
            self.cycle() > 0.5
        }

        pub fn range(&mut self, low : f32, high : f32) -> f32 {
            D1::normalize_from_01(self.cycle(), low.min(high), low.max(high))
        }

        pub fn range_pog(&mut self, low : f32, high : f32) -> f32 {
            if self.boolean() {
                self.range(low, high)
            } else {
                -self.range(low, high)
            }
        }
    }

    impl Default for Random{
        fn default() -> Self {
            Self {
                s1 : 4042,
                s2 : 911,
                s3 : 18342,
                /*s1 : 100,
                s2 : 100,
                s3 : 100,*/
            }
        }
    }
}