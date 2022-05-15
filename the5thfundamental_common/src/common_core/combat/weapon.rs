pub use weapon::*;
mod weapon {

    use bevy::prelude::Component;
    use serde::{
        Serialize, Deserialize,
    };

    use crate::SnowFlake;


    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum Target {
        AutoTarget(SnowFlake),
        ManualTarget(SnowFlake),
        None
    }

    impl Target {
        pub fn get_target(&self) -> Option<SnowFlake> {
            match self {
                Self::AutoTarget(sf) => {
                    Some(*sf)
                },
                Self::ManualTarget(sf) => {
                    Some(*sf)
                },
                Self::None => {
                    None
                }
            }
        }
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum TargetForce {
        Mine,
        Ally,
        MineOrAlly,
        Enemy,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum TargetType {
        Air,
        Ground,
        Universal,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub struct DamageTypes {
        pub kinetic : f32,
        pub fire : f32,
        pub energy : f32,
        pub sonic : f32,
        pub explosive : f32,
        pub shock : f32,
        pub radioactivity : f32,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub struct Weapon {
        pub target : Target,
        pub target_force : TargetForce,
        pub target_type : TargetType,
        pub range : f32,
        pub damage : f32,
        pub damage_types : DamageTypes,
        pub fire_rate : f32,
        #[serde(skip)]
        pub fire_time : f32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[derive(Component)]
    pub struct WeaponSet {
        ///This value should be slightly lower (~*0.98) than a weapons range.
        pub closing_range : f32,
        pub weapons : Vec<Weapon>,
    }

    impl WeaponSet {
        pub fn max_range(&self) -> f32 {
            self.weapons.iter().map(|w| w.range).fold(0.0, |m, v| v.max(m))
        }
        pub fn min_range(&self) -> f32 {
            if self.weapons.len() == 0 { return 0.0; }
            self.weapons.iter().map(|w| w.range).fold(f32::MAX, |m, v| v.min(m))
        }
        pub fn no_targets(&self) -> bool {
            for w in self.weapons.iter() {
                match w.target {
                    Target::None => { },
                    _ => { return false; }
                }
            }
            true
        }
    }
}