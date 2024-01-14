use bevy::{math::Vec3Swizzles, prelude::*};
use t5f_utility::mathfu::d2::distance_magnitude;
use crate::*;

#[derive(Debug, Clone, Copy)]
#[derive(Event)]
pub struct ObjectKilledEvent(pub Entity);

#[derive(Default)]
pub struct CombatPlugin;

impl CombatPlugin {
    fn targeting_system(
        teamplayer_world: Res<TeamPlayerWorld>,
        transforms: Query<&Transform>,
        mut query: Query<(&Transform, &mut PathFinder, &mut Navigator, &mut WeaponSet, &TeamPlayer,)>,
    ) {

        //TODO: Make sure weapons can only target the target if they are able to.
        query.for_each_mut(|(transform, mut pathfinder, mut navigator, mut weapon_set, teamplayer)| {
            match navigator.pursue {
                Some(target) => {
                    if let Ok(target_transform) = transforms.get(target) {
                        let dir = Vec2::new(transform.translation.x - target_transform.translation.x, transform.translation.z - target_transform.translation.z).normalize() * weapon_set.closing_range;
                        if distance_magnitude((transform.translation.x, transform.translation.z), (target_transform.translation.x, target_transform.translation.z)) > dir.length_squared() {
                            let start = transform.translation.xz();
                            let end = target_transform.translation.xz() + dir;
                            pathfinder.set_trip((start, end));
                        }

                        for weapon in weapon_set.weapons.iter_mut() {
                            if distance_magnitude((transform.translation.x, transform.translation.z), (target_transform.translation.x, target_transform.translation.z)) < weapon.range.powi(2) {
                                weapon.target = Target::ManualTarget(target);
                            } else if let Target::AutoTarget(_) = weapon.target {

                            } else {
                                weapon.target = Target::None
                            }
                        }
                    } else {
                        navigator.pursue = None;
                    }
                },
                None => {
                    for weapon in weapon_set.weapons.iter_mut() {
                        if let Target::ManualTarget(_) = weapon.target {
                            weapon.target = Target::None;
                        } else if let Target::AutoTarget(target) = weapon.target {
                            if let Ok(target_transform) = transforms.get(target) {
                                if distance_magnitude((transform.translation.x, transform.translation.z), (target_transform.translation.x, target_transform.translation.z)) > weapon.range.powi(2) {
                                    weapon.target = Target::None;
                                }
                            } else {
                                weapon.target = Target::None;
                            }
                        }
                        if let Target::None = weapon.target {
                            let targets = teamplayer_world.search_targets(*teamplayer, transform.translation, weapon);
                            if let Some(e) = targets.first() {
                                weapon.target = Target::AutoTarget(*e);
                            }
                        }
                    }
                }
            }
        });
    }

    fn weapons_system(
        time : Res<Time>,
        mut weapons : Query<&mut WeaponSet>,
        mut healths : Query<&mut Health>
    ) {
        weapons.for_each_mut(|mut wep| {
            for weapon in wep.weapons.iter_mut() {
                if weapon.cooldown > 0.0 {
                    weapon.cooldown -= time.delta_seconds();
                }
                if weapon.cooldown > 0.0 {
                    continue;
                }
                println!("{:?}", weapon.target.get_target());
                if let Some(mut health) = weapon.target.get_target().and_then(|target| healths.get_mut(target).ok()) {
                    health.damage(weapon.damage, weapon.damage_types);
                    weapon.cooldown = weapon.fire_rate;
                }
            }
        });
    }

    fn health_system(
        mut objects_killed_writer: EventWriter<ObjectKilledEvent>,
        query: Query<(Entity, &Health)>,
        mut commands: Commands,
    ) {
        query.for_each(|(entity, health)| {
            if health.is_dead() {
                if let Some(entity_commands) = commands.get_entity(entity) {
                    entity_commands.despawn_recursive();
                }
                objects_killed_writer.send(ObjectKilledEvent(entity));
            }
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct CombatSystems;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {

        app
            .add_systems(Update, (
                Self::targeting_system.after(CommandSystems),
                Self::weapons_system.after(Self::targeting_system),
                Self::health_system.after(Self::weapons_system),
            ).in_set(CombatSystems))
        ;
    }
}
