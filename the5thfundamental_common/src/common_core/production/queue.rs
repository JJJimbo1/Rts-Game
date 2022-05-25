use std::{ops::{Index, IndexMut}, time::Duration};
use bevy::{prelude::Component};
use serde::{Serialize, Deserialize,};
use zipqueue::ZipQueue;
use crate::{QueueData, StackData, ObjectType, SerdeComponent};



#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum ActiveQueue {
    // None,
    Structures,
    SupportStructures,
    Infantry,
    Vehicles,
    Aircraft,
    Watercraft,
    Technology,
    Transformation,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Queue {
    pub data: QueueData,
    pub zip_queue: ZipQueue<StackData>,
}

impl Default for Queue {
    fn default() -> Self {
        Self {
            data: QueueData::new(),
            zip_queue: ZipQueue::new(),
        }
    }
}

impl Queue {
    pub fn advance(&mut self) -> Option<StackData> {
        let r = self.zip_queue.get_next_move();
        if let Some(sd) = self.zip_queue.get_next() {
            self.data.set_timer(sd.time_to_build.as_secs_f64());
        }
        r
    }

    pub fn enqueue(&mut self, stack_data: StackData) {
        if self.zip_queue.is_empty() {
            self.data.timer = stack_data.time_to_build.as_secs_f64();
        }
        self.zip_queue.raise_stack(stack_data, 1);
    }

    pub fn push_buffer(&mut self, stack_data: StackData) {
        self.data.buffer.push(stack_data);
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Queues {
    pub structures_queue : Option<Queue>,
    pub support_structures_queue : Option<Queue>,
    pub infantry_queue: Option<Queue>,
    pub vehicle_queue: Option<Queue>,
    pub aircraft_queue: Option<Queue>,
    pub watercraft_queue: Option<Queue>,
    pub technology_queue : Option<Queue>,
    pub transformation_queue : Option<Queue>,
}

impl Queues {
    pub fn new() -> Self {
        Self {
            structures_queue : None,
            support_structures_queue : None,
            infantry_queue: None,
            vehicle_queue: None,
            aircraft_queue: None,
            watercraft_queue: None,
            technology_queue : None,
            transformation_queue : None,
        }
    }

    pub fn count(&self) -> usize {
        self.structures_queue.is_some() as usize
        + self.support_structures_queue.is_some() as usize
        + self.infantry_queue.is_some() as usize
        + self.vehicle_queue.is_some() as usize
        + self.aircraft_queue.is_some() as usize
        + self.watercraft_queue.is_some() as usize
        + self.technology_queue.is_some() as usize
        + self.transformation_queue.is_some() as usize
    }

    pub fn is_empty(&self) -> bool {
        if let Some(x) = &self.structures_queue { if !x.zip_queue.is_empty() { return false; } }
        if let Some(x) = &self.support_structures_queue { if !x.zip_queue.is_empty() { return false; } }
        if let Some(x) = &self.infantry_queue { if !x.zip_queue.is_empty() { return false; } }
        if let Some(x) = &self.vehicle_queue { if !x.zip_queue.is_empty() { return false; } }
        if let Some(x) = &self.aircraft_queue { if !x.zip_queue.is_empty() { return false; } }
        if let Some(x) = &self.watercraft_queue { if !x.zip_queue.is_empty() { return false; } }
        if let Some(x) = &self.technology_queue { if !x.zip_queue.is_empty() { return false; } }
        if let Some(x) = &self.transformation_queue { if !x.zip_queue.is_empty() { return false; } }
        true
    }

    pub fn get(&self, queue: ActiveQueue) -> Option<&Queue> {
        match queue {
            ActiveQueue::Structures => { self.structures_queue.as_ref() },
            ActiveQueue::SupportStructures => { self.support_structures_queue.as_ref() },
            ActiveQueue::Infantry => { self.infantry_queue.as_ref() },
            ActiveQueue::Vehicles => { self.vehicle_queue.as_ref() },
            ActiveQueue::Aircraft => { self.aircraft_queue.as_ref() },
            ActiveQueue::Watercraft => { self.watercraft_queue.as_ref() },
            ActiveQueue::Technology => { self.technology_queue.as_ref() },
            ActiveQueue::Transformation => { self.transformation_queue.as_ref() },
        }
    }

    pub fn get_mut(&mut self, queue: ActiveQueue) -> Option<&mut Queue> {
        match queue {
            ActiveQueue::Structures => { self.structures_queue.as_mut() },
            ActiveQueue::SupportStructures => { self.support_structures_queue.as_mut() },
            ActiveQueue::Infantry => { self.infantry_queue.as_mut() },
            ActiveQueue::Vehicles => { self.vehicle_queue.as_mut() },
            ActiveQueue::Aircraft => { self.aircraft_queue.as_mut() },
            ActiveQueue::Watercraft => { self.watercraft_queue.as_mut() },
            ActiveQueue::Technology => { self.technology_queue.as_mut() },
            ActiveQueue::Transformation => { self.transformation_queue.as_mut() },
        }
    }

    pub fn push_data_to_queue(&mut self, queue: ActiveQueue, data: StackData) {
        match queue {
            ActiveQueue::Structures => { self.structures_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
            ActiveQueue::SupportStructures => { self.support_structures_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
            ActiveQueue::Infantry => { self.infantry_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
            ActiveQueue::Vehicles => { self.vehicle_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
            ActiveQueue::Aircraft => { self.aircraft_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
            ActiveQueue::Watercraft => { self.watercraft_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
            ActiveQueue::Technology => { self.technology_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
            ActiveQueue::Transformation => { self.transformation_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
        }
    }

    // pub fn remove_from_buffer(&mut self, queue: ActiveQueue, data: StackData) {
    //     match queue {
    //         ActiveQueue::Structures => { self.structures_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
    //         ActiveQueue::SupportStructures => { self.support_structures_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
    //         ActiveQueue::Infantry => { self.infantry_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
    //         ActiveQueue::Vehicles => { self.vehicle_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
    //         ActiveQueue::Aircraft => { self.aircraft_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
    //         ActiveQueue::Watercraft => { self.watercraft_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
    //         ActiveQueue::Technology => { self.technology_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
    //         ActiveQueue::Transformation => { self.transformation_queue.get_or_insert(Queue::default()).zip_queue.push_stack(data) },
    //     }
    // }

}

impl Index<usize> for Queues {
    type Output = Queue;

    fn index(&self, index: usize) -> &Self::Output {
        let mut count = 0;
        if self.structures_queue.is_some() { if index == count { return self.structures_queue.as_ref().unwrap(); } else { count += 1; }}
        if self.support_structures_queue.is_some() { if index == count { return self.support_structures_queue.as_ref().unwrap(); } else { count += 1; }}
        if self.infantry_queue.is_some() { if index == count { return self.infantry_queue.as_ref().unwrap(); } else { count += 1; }}
        if self.vehicle_queue.is_some() { if index == count { return self.vehicle_queue.as_ref().unwrap(); } else { count += 1; }}
        if self.aircraft_queue.is_some() { if index == count { return self.aircraft_queue.as_ref().unwrap(); } else { count += 1; }}
        if self.watercraft_queue.is_some() { if index == count { return self.watercraft_queue.as_ref().unwrap(); } else { count += 1; }}
        if self.technology_queue.is_some() { if index == count { return self.technology_queue.as_ref().unwrap(); } else { count += 1; }}
        if self.transformation_queue.is_some() { if index == count { return self.transformation_queue.as_ref().unwrap(); }}
        panic!();
    }
}

impl IndexMut<usize> for Queues {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut count = 0;
        if index > self.count().into() { panic!(); }
        if self.structures_queue.is_some() { if index == count { return self.structures_queue.as_mut().unwrap(); } else { count += 1; }}
        if self.support_structures_queue.is_some() { if index == count { return self.support_structures_queue.as_mut().unwrap(); } else { count += 1; }}
        if self.infantry_queue.is_some() { if index == count { return self.infantry_queue.as_mut().unwrap(); } else { count += 1; }}
        if self.vehicle_queue.is_some() { if index == count { return self.vehicle_queue.as_mut().unwrap(); } else { count += 1; }}
        if self.aircraft_queue.is_some() { if index == count { return self.aircraft_queue.as_mut().unwrap(); } else { count += 1; }}
        if self.watercraft_queue.is_some() { if index == count { return self.watercraft_queue.as_mut().unwrap(); } else { count += 1; }}
        if self.technology_queue.is_some() { if index == count { return self.technology_queue.as_mut().unwrap(); } else { count += 1; }}
        if self.transformation_queue.is_some() { if index == count { return self.transformation_queue.as_mut().unwrap(); } else { panic!() }}
        panic!();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueObject {
    pub cost : u128,
    pub time_to_build : Duration,
}


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct PrefabQueues {
    pub objects: Vec<ObjectType>
}

impl SerdeComponent for Queues {
    fn saved(&self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(self.clone())
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct MasterQueue {
//     pub queues : HashMap::<ObjectType, Queues>,
//     pub building_uis : HashMap::<String, BuildingUIData>,
//     pub building_prefabs : HashMap::<String, BuildingPrefab>,
//     pub unit_uis : HashMap::<String, UnitUIData>,
//     pub unit_prefabs : HashMap::<String, UnitPrefab>,
// }

// impl MasterQueue {
//     pub fn new() -> Self {
//         Self {
//             queues : HashMap::new(),
//             building_uis : HashMap::new(),
//             building_prefabs : HashMap::new(),
//             unit_uis : HashMap::new(),
//             unit_prefabs : HashMap::new(),
//         }
//     }
//     pub fn load_all(&mut self, objects : &QLoader<GameObject, ()>) {
//         let mut queues = HashMap::<ObjectType, Queues>::new();
//         let mut building_uis = HashMap::<String, BuildingUIData>::new();
//         let mut building_prefabs = HashMap::<String, BuildingPrefab>::new();
//         let mut unit_uis = HashMap::<String, UnitUIData>::new();
//         let mut unit_prefabs = HashMap::<String, UnitPrefab>::new();

//         for (id, object) in objects.iter() {
//             if id.chars().count() > SMALL_BUFFER_SIZE { log::error!("'{}' is longer than {} letters", id, SMALL_BUFFER_SIZE); continue; }
//             match object.object_type {
//                 ObjectType::Building => {
//                     let bp = BuildingPrefab(object.clone());
//                     building_prefabs.insert(id.clone(), bp);

//                     if let Ok(bud) = BuildingUIData::try_from(object.clone()) {
//                         building_uis.insert(id.clone(), bud);
//                     }
//                 },
//                 ObjectType::Unit => {
//                     let up = UnitPrefab(object.clone());
//                     unit_prefabs.insert(id.clone(), up);

//                     if let Ok(uud) = UnitUIData::try_from(object.clone()) {
//                         unit_uis.insert(id.clone(), uud);
//                     }
//                 }
//             }
//             let mut lqueues : Option<Queues> = None;

//             // if let Some(_co) = &object.constructor {
//             //     //TODO: x does nothing. I don't know what I want it to do.
//             //     let data = QueueData {
//             //         timer : 0.0,
//             //         spawn_point : None,
//             //         end_point : None,
//             //         buffer : Vec::new(),
//             //     };
//             //     match lqueues.as_mut() {
//             //         Some(q) => {
//             //             q.structures_queue = Some(ZipQueue::<StackData, QueueData>::new(data));
//             //         },
//             //         None => {
//             //             lqueues = Some(Queues::new(id.to_owned()));
//             //             lqueues.as_mut().unwrap().structures_queue = Some(ZipQueue::<StackData, QueueData>::new(data));
//             //         }
//             //     }
//             // }

//             // if let Some(t) = &object.trainer {
//             //     let data = QueueData {
//             //         timer : 0.0,
//             //         spawn_point : Some(t.spawn_point),
//             //         end_point : Some(t.end_point),
//             //         buffer : Vec::new(),
//             //     };
//             //     match lqueues.as_mut() {
//             //         Some(q) => {
//             //             q.support_queue = Some(ZipQueue::<StackData, QueueData>::new(data));
//             //         },
//             //         None => {
//             //             lqueues = Some(Queues::new(id.to_owned()));
//             //             lqueues.as_mut().unwrap().support_queue = Some(ZipQueue::<StackData, QueueData>::new(data));
//             //         }
//             //     }
//             // }

//             if let Some(x) = lqueues {
//                 queues.insert(id.clone(), x);
//             }
//         }

//         // for (id, object) in objects.iter() {
//         //     if let Some(q) = queues.get_mut(id) {
//         //         if let Some(co) = &object.constructor {
//         //             for s in co.buildings.iter() {
//         //                 let ob = building_prefabs.get(s).unwrap();
//         //                 let sd = StackData::try_from(ob.0.clone()).unwrap();
//         //                 q.structures_queue.as_mut().unwrap().push_stack(sd);
//         //             }
//         //         }
//         //         if let Some(t) = &object.trainer {
//         //             for s in t.trainies.iter() {
//         //                 let ob = unit_prefabs.get(s).unwrap();
//         //                 let sd = StackData::try_from(ob.0.clone()).unwrap();
//         //                 q.support_queue.as_mut().unwrap().push_stack(sd);
//         //             }
//         //         }
//         //     }
//         // }

//         self.queues = queues;
//         self.building_uis = building_uis;
//         self.building_prefabs = building_prefabs;
//         self.unit_uis = unit_uis;
//         self.unit_prefabs = unit_prefabs;
//     }

//     pub fn loaded_all(mut self, objects : &QLoader<GameObject, ()>) -> Self {
//         self.load_all(objects);
//         self
//     }

//     pub fn get(&self, name : &str) -> Option<&Queues> {
//         self.queues.get(name)
//     }

//     pub fn get_mut(&mut self, name : &str) -> Option<&mut Queues> {
//         self.queues.get_mut(name)
//     }
// }