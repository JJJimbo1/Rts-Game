pub use systems::*;
mod systems {

    use std::collections::HashMap;

    use bevy::prelude::*;
    use snowflake::ProcessUniqueId;
    use crate::*;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
    pub enum EconomySystems {
        ResourceAdderSystem,
        QueueSystem,
    }

    pub fn economy_system_set(set : SystemSet) -> SystemSet {
        set.label(CommonSystemSets::Economy)
            .with_system(resource_adder_system.label(EconomySystems::ResourceAdderSystem))
            .with_system(queue_system.label(EconomySystems::QueueSystem).after(EconomySystems::ResourceAdderSystem))
    }

    fn resource_adder_system(
        time : Res<Time>,
        mut actors : ResMut<Actors>,
        query : Query<(&TeamPlayer, &EconomicObject)>
    ) {
        let mut add : HashMap<TeamPlayer, (u32, f64)> = HashMap::new();
        for a in actors.actors.iter() {
            add.insert(*a.0, (0, 0.0));
        }
        query.for_each(|(tp, res)| {
            if let Some(x) = add.get_mut(tp) {
                x.0 += 1;
                x.1 += res.resource_gen - res.resource_drain;
            }
        });
        for actor in actors.actors.iter_mut() {
            let mut to_add = add[actor.0];
            to_add.1 *= time.delta_seconds() as f64;
            actor.1.economy.add_resources(to_add);
        }
    }

    // #[derive(Debug, Clone, Default)]
    // pub struct InitRequests {
    //     pub requests : Vec<(ObjectType, String, InstantiationData, Option<Entity>)>,
    //     pub with_entities_requests : Vec<(ObjectType, String, InstantiationData, Entity)>,
    // }

    // impl InitRequests {
    //     pub fn new() -> Self {
    //         Self{
    //             requests : Vec::new(),
    //             with_entities_requests : Vec::new(),
    //         }
    //     }

    //     pub fn request(&mut self, otype : ObjectType, id : String, data : InstantiationData, entity : Option<Entity>) {
    //         self.requests.push((otype, id, data, entity));
    //     }
    // }

    fn queue_system(
        mut spawn_events: EventWriter<ObjectSpawnEvent>,
        time : Res<Time>,
        mut actors : ResMut<Actors>,
        // mut inits : ResMut<InitRequests>,
        mut query : Query<(&Transform, &TeamPlayer, &mut Queues)>
    ) {
        query.for_each_mut(|(tranform, team_player, mut queues)| {
            if let Some(actor) = actors.actors.get_mut(team_player) {
                for i in 0..queues.count() {
                    let queue = &mut queues[i];
                    if let Some(x) = actor.tick_queue(queue, time.delta_seconds_f64()) {
                        if x.buffered {
                            queue.push_buffer(x);
                        } else {
                            let spawn_data = ObjectSpawnEventData { snow_flake: Snowflake(ProcessUniqueId::new()), object_type: x.object_type, team_player: *team_player, transform: *tranform};
                            spawn_events.send(ObjectSpawnEvent(spawn_data));
                        }
                    }
                }
            }
        })
    }
}