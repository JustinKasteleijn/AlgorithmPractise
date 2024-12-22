pub struct CustomQueue<V> {
    queue: Vec<V>,
}

impl<V> CustomQueue<V> {
    pub fn new() -> CustomQueue<V> {
        CustomQueue { queue: Vec::new() }
    }
    pub fn with_capacity(n: usize) -> CustomQueue<V> {
        CustomQueue { queue: Vec::with_capacity(n) }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn enqueue(&mut self, v: V) {
        self.queue.push(v);
    }

    pub fn dequeue(&mut self) -> Option<V> {
        if !self.queue.is_empty() {
            Some(self.queue.swap_remove(0))
        } else {
            None
        }
    }
}