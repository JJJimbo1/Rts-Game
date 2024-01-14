use bevy_rapier3d::dynamics::Velocity;
use t5f_common::*;

pub trait Slim where Self: Sized {
    fn slim(&self) -> Option<Self>;
}

impl Slim for Health {
    fn slim(&self) -> Option<Self> {
        if self.is_full_health() {
            None
        } else {
            Some(*self)
        }
    }
}

impl Slim for WeaponSet {
    fn slim(&self) -> Option<Self> {
        let mut cooled = true;
        for weapon in self.weapons.iter() {
            if weapon.cooldown > 0.0 {
                cooled = false;
            }
        }
        if self.no_targets().unwrap_or(true) && cooled {
            None
        } else {
            Some(self.clone())
        }
    }
}

impl Slim for Navigator {
    fn slim(&self) -> Option<Self> {
        self.pursue.is_some().then_some(*self)
    }
}

impl Slim for PathFinder {
    fn slim(&self) -> Option<Self> {
        match self {
            Self::Idle => None,
            _ => Some(self.clone()),
        }
    }
}

impl Slim for Snowflake {
    fn slim(&self) -> Option<Self> {
        //TODO: Something here.
        Some(*self)
    }
}

impl Slim for Squad {
    fn slim(&self) -> Option<Self> {
        None
    }
}

impl Slim for Queues {
    fn slim(&self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(self.clone())
        }
    }
}

impl Slim for Velocity {
    fn slim(&self) -> Option<Self> {
        Some(self.clone())
    }
}