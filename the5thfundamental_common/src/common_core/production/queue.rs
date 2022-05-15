pub use queue::*;
mod queue {

    use std::collections::HashMap;

    use bevy::{prelude::Component};
    use serde::{Serialize, Deserialize,};

    use zipqueue::ZipQueue;
    use qloader::*;
    use crate::{BuildingPrefab, BuildingUIData, GameObject, ObjectType, QueueData, SMALL_BUFFER_SIZE, StackData, UnitPrefab, UnitUIData};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[derive(Component)]
    pub struct Queues {
        pub name : String,
        pub building_queue : Option<ZipQueue<StackData, QueueData>>,
        pub unit_queue : Option<ZipQueue<StackData, QueueData>>,
        pub tech_queue : Option<ZipQueue<StackData, QueueData>>,
        pub trans_queue : Option<ZipQueue<StackData, QueueData>>,
    }

    impl Queues {
        pub fn new(name : String) -> Self {
            Self {
                name,
                building_queue : None,
                unit_queue : None,
                tech_queue : None,
                trans_queue : None,
            }
        }

        pub fn count(&self) -> u8 {
            u8::from(self.building_queue.is_some()) + u8::from(self.unit_queue.is_some())
        }

        pub fn is_empty(&self) -> bool {
            if let Some(x) = &self.building_queue { if !x.is_empty() { return false; } }
            if let Some(x) = &self.unit_queue { if !x.is_empty() { return false; } }
            if let Some(x) = &self.tech_queue { if !x.is_empty() { return false; } }
            if let Some(x) = &self.trans_queue { if !x.is_empty() { return false; } }
            true
        }
    }

    #[derive(Debug, Clone)]
    pub struct MasterQueue {
        pub queues : HashMap::<String, Queues>,
        pub building_uis : HashMap::<String, BuildingUIData>,
        pub building_prefabs : HashMap::<String, BuildingPrefab>,
        pub unit_uis : HashMap::<String, UnitUIData>,
        pub unit_prefabs : HashMap::<String, UnitPrefab>,
    }

    impl MasterQueue {
        pub fn new() -> Self {
            Self {
                queues : HashMap::new(),
                building_uis : HashMap::new(),
                building_prefabs : HashMap::new(),
                unit_uis : HashMap::new(),
                unit_prefabs : HashMap::new(),
            }
        }
        pub fn load_all(&mut self, objects : &QLoader<GameObject, ()>) {
            let mut queues = HashMap::<String, Queues>::new();
            let mut building_uis = HashMap::<String, BuildingUIData>::new();
            let mut building_prefabs = HashMap::<String, BuildingPrefab>::new();
            let mut unit_uis = HashMap::<String, UnitUIData>::new();
            let mut unit_prefabs = HashMap::<String, UnitPrefab>::new();

            for (id, object) in objects.iter() {
                if id.chars().count() > SMALL_BUFFER_SIZE { log::error!("'{}' is longer than {} letters", id, SMALL_BUFFER_SIZE); continue; }
                match object.object_type {
                    ObjectType::Building => {
                        let bp = BuildingPrefab(object.clone());
                        building_prefabs.insert(id.clone(), bp);

                        if let Ok(bud) = BuildingUIData::try_from(object.clone()) {
                            building_uis.insert(id.clone(), bud);
                        }
                    },
                    ObjectType::Unit => {
                        let up = UnitPrefab(object.clone());
                        unit_prefabs.insert(id.clone(), up);

                        if let Ok(uud) = UnitUIData::try_from(object.clone()) {
                            unit_uis.insert(id.clone(), uud);
                        }
                    }
                }
                let mut lqueues : Option<Queues> = None;

                if let Some(_co) = &object.constructor {
                    //TODO: x does nothing. I don't know what I want it to do.
                    let data = QueueData {
                        timer : 0.0,
                        spawn_point : None,
                        end_point : None,
                        buffer : Vec::new(),
                    };
                    match lqueues.as_mut() {
                        Some(q) => {
                            q.building_queue = Some(ZipQueue::<StackData, QueueData>::new(data));
                        },
                        None => {
                            lqueues = Some(Queues::new(id.to_owned()));
                            lqueues.as_mut().unwrap().building_queue = Some(ZipQueue::<StackData, QueueData>::new(data));
                        }
                    }
                }

                if let Some(t) = &object.trainer {
                    let data = QueueData {
                        timer : 0.0,
                        spawn_point : Some(t.spawn_point),
                        end_point : Some(t.end_point),
                        buffer : Vec::new(),
                    };
                    match lqueues.as_mut() {
                        Some(q) => {
                            q.unit_queue = Some(ZipQueue::<StackData, QueueData>::new(data));
                        },
                        None => {
                            lqueues = Some(Queues::new(id.to_owned()));
                            lqueues.as_mut().unwrap().unit_queue = Some(ZipQueue::<StackData, QueueData>::new(data));
                        }
                    }
                }

                if let Some(x) = lqueues {
                    queues.insert(id.clone(), x);
                }
            }

            for (id, object) in objects.iter() {
                if let Some(q) = queues.get_mut(id) {
                    if let Some(co) = &object.constructor {
                        for s in co.buildings.iter() {
                            let ob = building_prefabs.get(s).unwrap();
                            let sd = StackData::try_from(ob.0.clone()).unwrap();
                            q.building_queue.as_mut().unwrap().push_stack(sd);
                        }
                    }
                    if let Some(t) = &object.trainer {
                        for s in t.trainies.iter() {
                            let ob = unit_prefabs.get(s).unwrap();
                            let sd = StackData::try_from(ob.0.clone()).unwrap();
                            q.unit_queue.as_mut().unwrap().push_stack(sd);
                        }
                    }
                }
            }

            self.queues = queues;
            self.building_uis = building_uis;
            self.building_prefabs = building_prefabs;
            self.unit_uis = unit_uis;
            self.unit_prefabs = unit_prefabs;
        }

        pub fn loaded_all(mut self, objects : &QLoader<GameObject, ()>) -> Self {
            self.load_all(objects);
            self
        }

        pub fn get(&self, name : &str) -> Option<&Queues> {
            self.queues.get(name)
        }

        pub fn get_mut(&mut self, name : &str) -> Option<&mut Queues> {
            self.queues.get_mut(name)
        }
    }
}