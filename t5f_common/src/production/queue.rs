use std::time::Duration;
use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
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
        self.queues.iter().fold(0, |a, (_, x)| a + x.zip_queue.spine().len() + x.buffer.spine().len()) == 0
    }
}

impl From<(&AssetQueues, &HashMap<ObjectType, (ActiveQueue, StackData)>)> for Queues {
    fn from((prefab, stacks): (&AssetQueues, &HashMap<ObjectType, (ActiveQueue, StackData)>)) -> Self {
        let mut queues = Queues::new();
        for s in prefab.objects.iter() {
            let (active, data) = &stacks[s];
            queues.queues.entry(*active).or_default().stacks.push(data.clone());
        }
        queues
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Queue {
    pub timer: f64,
    pub stacks: Vec<StackData>,
    pub zip_queue: ZipQueue<StackData>,
    pub buffer: ZipQueue<StackData>,
}

impl Default for Queue {
    fn default() -> Self {
        Self {
            timer: 0.0,
            stacks: Vec::new(),
            zip_queue: ZipQueue::new(),
            buffer: ZipQueue::new(),
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

    pub fn next(&self) -> Option<StackData> {
        self.zip_queue.next()
    }

    pub fn advance(&mut self) -> Option<StackData> {
        let r = self.zip_queue.next_move();
        if let Some(sd) = self.zip_queue.next() {
            self.set_timer(sd.time_to_build.as_secs_f64());
        }
        r
    }

    pub fn enqueue(&mut self, stack_data: StackData) {
        if self.zip_queue.is_empty() {
            self.timer = stack_data.time_to_build.as_secs_f64();
        }
        self.zip_queue.push(stack_data);
    }

    pub fn push_to_buffer(&mut self, stack_data: StackData) {
        self.buffer.push(stack_data);
    }

    pub fn remove_from_buffer(&mut self, stack_data: &StackData) {
        self.buffer.remove(stack_data);
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ZipQueue<S: Eq + Clone> {
    spine: Vec<S>,
}

impl<S: Eq + Clone> ZipQueue<S> {
    pub fn new() -> Self {
        Self {
            spine : Vec::new(),
        }
    }

    pub fn with_capacity(capacity : usize) -> Self {
        Self {
            spine : Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, stack : S) {
        self.spine.push(stack);
    }

    pub fn remove(&mut self, stack : &S) {
        let Some(mut i) = self.spine.iter().rev().position(|s| s == stack) else { return; };
        i = self.spine.len() - i - 1;
        self.spine.remove(i);
    }

    pub fn remove_all(&mut self, stack : &S) {
        self.spine.retain(|s| s != stack);
    }

    pub fn next(&self) -> Option<S> {
        self.spine.first().cloned()
    }

    pub fn next_move(&mut self) -> Option<S> {
        if self.spine.len() > 0 {
            return Some(self.spine.remove(0))
        }
        None
    }

    pub fn spine(&self) -> &Vec<S> {
        &self.spine
    }

    pub fn spine_mut(&mut self) -> &mut Vec<S> {
        &mut self.spine
    }

    pub fn height(&self, stack : &S) -> usize {
        self.spine.iter().filter(|s| *s == stack).count()
    }

    pub fn is_empty(&self) -> bool {
        self.spine.len() == 0
    }

    pub fn clear(&mut self) {
        self.spine.clear();
    }
}

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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct StackData {
    pub object: ObjectType,
    pub time_to_build: Duration,
    pub cost: u128,
    pub buffered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueObject {
    pub cost: u128,
    pub time_to_build: Duration,
}


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct AssetQueues {
    pub objects: Vec<ObjectType>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemSet)]
pub struct QueueSystem;
