pub use systems::*;
mod systems {

    use std::collections::HashMap;

    use bevy::prelude::*;
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

    fn resource_adder_system(time : Res<Time>, mut actors : ResMut<Actors>, query : Query<(&TeamPlayer, &ResourceProvider)>) {
        let mut add : HashMap<TeamPlayer, (u32, f64)> = HashMap::new();
        for a in actors.actors.iter() {
            add.insert(*a.0, (0, 0.0));
        }
        query.for_each(|(tp, res)| {
            if let Some(x) = add.get_mut(tp) {
                x.0 += 1;
                x.1 += res.strength;
            }
        });
        for actor in actors.actors.iter_mut() {
            let mut to_add = add[actor.0];
            to_add.1 *= time.delta_seconds() as f64;
            actor.1.economy.add_resources(to_add);
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct InitRequests {
        pub requests : Vec<(ObjectType, String, InstantiationData, Option<Entity>)>,
        pub with_entities_requests : Vec<(ObjectType, String, InstantiationData, Entity)>,
    }

    impl InitRequests {
        pub fn new() -> Self {
            Self{
                requests : Vec::new(),
                with_entities_requests : Vec::new(),
            }
        }

        pub fn request(&mut self, otype : ObjectType, id : String, data : InstantiationData, entity : Option<Entity>) {
            self.requests.push((otype, id, data, entity));
        }
    }

    fn queue_system(time : Res<Time>, mut actors : ResMut<Actors>, mut inits : ResMut<InitRequests>, mut query : Query<(&Transform, &TeamPlayer, &mut Queues)>) {
        query.for_each_mut(|(tran, tp, mut que)| {
            match &mut que.building_queue {
                Some(x) => {
                    if let Some(sd) = x.get_next() {
                        if let Some(a) = actors.actors.get_mut(tp) {
                            let cost_this_frame = sd.cost as f64 / sd.time_to_build.as_secs_f64() * x.data().time(time.delta_seconds() as f64);
                            if a.economy.remove_resources(cost_this_frame) {
                            // println!("{}", x.height(&sd));
                                if x.data_mut().update(time.delta_seconds() as f64) {
                                    match x.get_next_move() {
                                        Some(sd) => {
                                            x.data_mut().buffer.push(sd);
                                        },
                                        None => { }
                                    }
                                    match x.get_next() {
                                        Some(sd) => {
                                            x.data_mut().set_timer(sd.time_to_build.as_secs_f64());
                                        },
                                        None => { }
                                    }
                                }
                            }
                        }
                    }
                },
                None => { }
            }
            match &mut que.unit_queue {
                Some(x) => {
                    if let Some(sd) = x.get_next() {
                        if let Some(a) = actors.actors.get_mut(tp) {
                            let cost_this_frame = sd.cost as f64 / sd.time_to_build.as_secs_f64() * x.data().time(time.delta_seconds() as f64);
                            if a.economy.remove_resources(cost_this_frame) {
                                if x.data_mut().update(time.delta_seconds() as f64) {
                                    match x.get_next_move() {
                                        Some(sd) => {
                                            inits.request(sd.object_type, sd.id, InstantiationData{
                                                transform : tran.clone(),
                                                spawn_point : x.data().spawn_point,
                                                end_point : x.data().end_point,
                                                team_player : *tp,
                                                multiplayer : false,
                                                had_identifier : false,
                                            }, None);
                                            // println!("{}", inits.requests.len());
                                        },
                                        None => { }
                                    }
                                    match x.get_next() {
                                        Some(sd) => {
                                            x.data_mut().set_timer(sd.time_to_build.as_secs_f64());
                                        },
                                        None => { }
                                    }
                                }
                            }
                        }
                    }
                },
                None => { }
            }
        })
    }
}