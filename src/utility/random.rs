use crate::mathfu::*;
use bevy::prelude::*;

pub trait Generator : Sized {
    type Seed;
    fn seeded(seed : Self::Seed) -> Self;
    fn cycle(&mut self) -> f32;
}

pub struct WichmannHill {
    s1 : i64,
    s2 : i64,
    s3 : i64,
}

impl Generator for WichmannHill {
    type Seed = f32;

    fn seeded(seed : f32) -> Self {
        Self {
            s1 : (seed.trunc() % 100.0 * 300.0) as i64,
            s2 : (seed.fract() * 100.0 * 300.0) as i64,
            s3 : (seed.trunc() % 100.0 * seed.fract() * 100.0 * 300.0) as i64,
        }
    }

    fn cycle(&mut self) -> f32 {
        self.s1 = 171 * self.s1 % 30269;
        self.s2 = 172 * self.s2 % 30307;
        self.s3 = 170 * self.s3 % 30323;

        (self.s1 as f32 / 30269.0 + self.s2 as f32 / 30307.0 + self.s3 as f32 / 30323.0) % 1.0
    }
}


#[derive(Debug, Clone, Copy)]
#[derive(Resource)]
pub struct Random<G : Generator = WichmannHill> {
    source : G,
}

impl<G : Generator> Random<G> {
    pub fn seeded(seed : G::Seed) -> Self {
        Self {
            source : G::seeded(seed)
        }
    }

    pub fn cycle(&mut self) -> f32 {
        self.source.cycle()
    }

    pub fn range(&mut self, low : f32, high : f32) -> f32 {
        d1::normalize_from_01(self.cycle(), low, high)
    }

    pub fn range_pog(&mut self, low : f32, high : f32) -> f32 {
        if self.boolean() {
            self.range(low, high)
        } else {
            -self.range(low, high)
        }
    }

    pub fn boolean(&mut self) -> bool {
        self.boolean_weight(0.5)
    }

    pub fn boolean_weight(&mut self, weight : f32) -> bool {
        self.cycle() < weight
    }

    pub fn range_i32(&mut self, low : i32, high : i32) -> i32 {
        self.range(low as f32 - 0.5, high as f32 + 0.5).round() as i32
    }

    pub fn item<'a, T>(&mut self, slice : &'a [T]) -> Option<&'a T> {
        if slice.len() == 0 { return None; }
        slice.get(self.range_i32(0, slice.len() as i32 - 1) as usize)
    }
}