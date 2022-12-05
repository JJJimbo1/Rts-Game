use std::hash::Hash;
use indexmap::IndexMap;

///The ZipQueue is a data structure made up of slots of type S which are zipped together to create a spine.
#[derive(Debug, Default, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ZipQueue<S : Eq + Hash + Clone> {
    stacks : IndexMap<S, (usize, Vec<usize>)>,
    spine : Vec<S>,
}

impl<S : Eq + Hash + Clone> ZipQueue<S> {
    ///Creates a new stackqueue with the supplied data.
    pub fn new() -> Self {
        Self {
            stacks : IndexMap::new(),
            spine : Vec::new(),
        }
    }

    ///Creates a new stackqueue with the supplied capacity.
    pub fn with_capacity(capacity : usize) -> Self {
        Self {
            stacks : IndexMap::with_capacity(capacity),
            spine : Vec::new(),
        }
    }

    ///Creates a new stack in the stackqueue.
    pub fn push_stack(&mut self, stack : S) {
        self.stacks.insert(stack, (0, Vec::new()));
    }

    ///Removes the stack from the stackqueue. Does nothing if the stack does not exist.
    pub fn remove_stack(&mut self, stack : &S) {
        if !self.stacks.contains_key(stack) { return; }
        self.clear_stack(stack);
        self.stacks.remove(stack);
    }

    ///Sets the stack height to zero. Does nothing if the stack does not exist.
    pub fn clear_stack(&mut self, stack : &S) {
        if !self.stacks.contains_key(stack) { return; }
        self.lower_stack(stack, self.stacks[stack].0);
    }

    ///Increments the stack height, placing an instance of the stack type at the back of the queue. Does nothing if the stack does not exist.
    pub fn raise_stack(&mut self, stack : S, amount : usize) {
        if !self.stacks.contains_key(&stack) { return; }
        for _ in 0..amount {
            let len = self.spine.len();
            self.spine.push(stack.clone());
            self.stacks.entry(stack.clone()).and_modify(|u| {u.0 += 1; u.1.push(len)});
        }
    }

    ///Decrements the stack height, removes it from the spine and shifts everything above it down.
    pub fn lower_stack(&mut self, stack : &S, amount : usize) {
        for _ in 0..amount {
            let mut index : Option<usize> = None;
            self.stacks.entry(stack.clone()).and_modify(|u| {
                if u.0 > 0 {
                    u.0 -= 1;
                    if let Some(x) = u.1.last() {
                        index = Some(*x);
                    }
                    u.1.remove(u.0);
                }
            });
            if let Some(x) = index {
                self.spine.remove(x);
                for u in self.stacks.values_mut() {
                    for j in u.1.iter_mut() {
                        if *j > x  {
                            *j -= 1;
                        }
                    }
                }
            } else {
                break;
            }
        }
    }

    ///Gets the item in the front of queue without moving the queue.
    pub fn get_next(&self) -> Option<S> {
        self.spine.first().cloned()
    }

    ///Gets the item in front of the queue and moves the queue.
    pub fn get_next_move(&mut self) -> Option<S> {
        let s = self.spine.first().cloned();
        for u in self.stacks.values_mut() {
            let mut to_remove : Option<usize> = None;
            for j in 0..u.1.len() {
                if u.1[j] == 0 {
                    u.0 -= 1;
                    to_remove = Some(u.1[j]);
                } else {
                    u.1[j] -= 1;
                }
            }
            if let Some(x) = to_remove {
                u.1.remove(x);
            }
        }
        if self.spine.len() > 0 {
            self.spine.remove(0);
        }
        s
    }

    ///Returns a reference to the internal spine.
    pub fn spine(&self) -> &Vec<S> {
        &self.spine
    }

    pub fn contains_stack(&self, stack : &S) -> bool {
        self.stacks.contains_key(stack)
    }

    pub fn height(&self, stack : &S) -> usize {
        self.stacks.get(stack).map_or(0, |s| s.0)
    }


    ///Returns a vec of all the stacks.
    pub fn stacks(&self) -> Vec<&S> {
        self.stacks.keys().collect()
    }

    ///Returns true if the internal spine is empty.
    pub fn is_empty(&self) -> bool {
        self.spine.len() == 0
    }

    ///Clears the internal spine.
    pub fn clear(&mut self) {
        for i in self.stacks.values_mut() {
            i.0 = 0;
            i.1.clear();
        }
        self.spine.clear();
    }
}