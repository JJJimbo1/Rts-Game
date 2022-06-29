pub use weapon::*;
mod weapon {

    use bevy::{prelude::{Component, Entity}, reflect::Reflect};
    use serde::{
        Serialize, Deserialize,
    };

    use crate::{Snowflake, SerdeComponent};


    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum Target {
        AutoTarget(Entity),
        ManualTarget(Entity),
        None
    }

    impl Target {
        pub fn get_target(&self) -> Option<Entity> {
            match self {
                Self::AutoTarget(e) => { Some(*e) },
                Self::ManualTarget(e) => { Some(*e) },
                Self::None => { None }
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

    #[derive(Debug, Clone, Copy)]
    #[derive(Serialize, Deserialize)]
    pub struct DamageTypes {
        pub kinetic : f32,
        pub fire : f32,
        pub energy : f32,
        pub sonic : f32,
        pub explosive : f32,
        pub shock : f32,
        pub radioactivity : f32,
    }

    #[derive(Debug, Clone, Copy)]
    #[derive(Serialize, Deserialize)]
    pub struct Weapon {
        pub target : Target,
        pub target_force : TargetForce,
        pub target_type : TargetType,
        pub range : f32,
        pub damage : f32,
        pub damage_types : DamageTypes,
        pub fire_rate : f32,
        // #[serde(skip)]
        pub cooldown : f32,
    }

    #[derive(Debug, Clone)]
    #[derive(Serialize, Deserialize)]
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

    impl SerdeComponent for WeaponSet {
        fn saved(&self) -> Option<Self> {
            let mut cooled = true;
            for weapon in self.weapons.iter() {
                if weapon.cooldown > 0.0 {
                    cooled = false;
                }
            }
            if self.no_targets() && cooled {
                None
            } else {
                Some(self.clone())
            }
        }
    }
}