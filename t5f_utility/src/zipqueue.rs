#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Default, Clone)]
pub struct ZipQueue<S : Eq + Clone> {
    spine : Vec<S>,
}

impl<S : Eq + Clone> ZipQueue<S> {
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