pub struct CustomStack<V> {
    stack: Vec<V>
}

impl<V> CustomStack<V> {
    pub fn new() -> CustomStack<V> {
        CustomStack {
            stack: Vec::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> CustomStack<V> {
        CustomStack {
            stack: Vec::with_capacity(capacity)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push(&mut self , value: V) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<V> {
        self.stack.pop()
    }
}