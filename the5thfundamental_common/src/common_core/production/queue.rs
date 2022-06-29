use std::{ops::{Index, IndexMut}, time::Duration};
use bevy::{prelude::Component, utils::{HashSet, HashMap}};
use serde::{Serialize, Deserialize,};
use zipqueue::ZipQueue;
use crate::{StackData, ObjectType, SerdeComponent};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum ActiveQueue {
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
    pub timer : f64,
    pub zip_queue: ZipQueue<StackData>,
    pub buffer : HashSet<StackData>,
}

impl Default for Queue {
    fn default() -> Self {
        Self {
            timer: 0.0,
            zip_queue: ZipQueue::new(),
            buffer: HashSet::default(),
        }
    }
}

impl Queue {
    pub fn set_timer(&mut self, time : f64) {
        self.timer = time;
    }

    pub fn time_left(&self, timer : f64) -> f64 {
        self.timer.min(timer)
    }

    pub fn update(&mut self, delta : f64) {
        if self.timer > 0.0 {
            self.timer -= delta;
        }
    }

    pub fn is_ready(&self) -> bool {
        self.timer <= 0.0
    }

    pub fn advance(&mut self) -> Option<StackData> {
        let r = self.zip_queue.get_next_move();
        if let Some(sd) = self.zip_queue.get_next() {
            self.set_timer(sd.time_to_build.as_secs_f64());
        }
        r
    }

    pub fn enqueue(&mut self, stack_data: StackData) {
        if self.zip_queue.is_empty() {
            self.timer = stack_data.time_to_build.as_secs_f64();
        }
        self.zip_queue.raise_stack(stack_data, 1);
    }

    pub fn push_to_buffer(&mut self, stack_data: StackData) {
        self.buffer.insert(stack_data);
    }

    pub fn remove_from_buffer(&mut self, stack_data: &StackData) {
        self.buffer.remove(stack_data);
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Component)]
pub struct Queues {
    pub queues: HashMap<ActiveQueue, Queue>,
}

impl Queues {
    pub fn new() -> Self {
        Self {
            queues: HashMap::default(),
        }
    }

    pub fn count(&self) -> usize {
        self.queues.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queues.iter().fold(0, |a, (_, x)| a + x.zip_queue.spine().len()) == 0
    }

    pub fn push_data_to_queue(&mut self, queue: ActiveQueue, data: StackData) {
        if !self.queues.contains_key(&queue) {
            self.queues.insert(queue, Queue::default());
        }
        self.queues.get_mut(&queue).unwrap().zip_queue.push_stack(data);
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