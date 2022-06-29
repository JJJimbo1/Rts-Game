
use bevy::{prelude::*, utils::HashSet, math::Vec3Swizzles};
use bevy_pathfinding::PathFinder;
use simple_random::Random;

use crate::{Snowflake, MobileObject};


// #[derive(Debug, Clone)]
// pub struct MoveCommand{
//     pub position : Vec2,
//     pub units : Vec<Snowflake>,
// }
// #[derive(Debug, Clone)]
// pub struct AttackCommand{
//     pub target : Snowflake,
//     pub units : Vec<Snowflake>,
// }

#[derive(Debug, Clone)]
pub struct UnitCommand {
    pub units: HashSet<Entity>,
    pub command_type: UnitCommandType,
}

#[derive(Debug, Copy, Clone)]
pub enum UnitCommandType {
    Move(Vec2),
    Attack(Entity),
}

// impl Default for ActorCommand {
//     fn default() -> Self {
//         Self::None
//     }
// }

// pub fn process_commands(
//     mut command_reader: EventReader<UnitCommand>,
//     mut rand : ResMut<Random>,
//     mut pathfinders: Query<(Entity, &Transform, &mut PathFinder, &mut MobileObject)>,
//     // mut weapons: Query<(Entity, &mut WeaponSet)>,

// ) {
//     for command in command_reader.iter() {
//         match command.command_type {
//             UnitCommandType::Move(destination) => {
//                 let spread = (command.units.len() as f32).sqrt() * 2.0;
//                 pathfinders.iter_mut().filter(|(entity, _, _, _)| command.units.contains(entity)).for_each(|(_, transform, mut pathfinder, mut mobile_object)| {
//                     pathfinder.start = transform.translation.xz();
//                     pathfinder.end = destination + Vec2::new(rand.range(-spread, spread), rand.range(-spread, spread));
//                     mobile_object.follow = true;
//                     mobile_object.pursuant = None;
//                 });
//             },
//             UnitCommandType::Attack(target) => {
//                 pathfinders.iter_mut().filter(|(entity, _, _, _)| command.units.contains(entity)).for_each(|(_, _, _, mut mobile_object)| {
//                     mobile_object.pursuant = Some(target);
//                 });
//             }
//         }
//     }
// }