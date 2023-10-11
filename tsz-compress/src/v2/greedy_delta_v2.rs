extern crate std;

use std::collections::VecDeque;

const QUEUE_CAPACITY: usize = 10;

pub struct Queue<T> {
    data: VecDeque<T>,
    capacity: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            data: VecDeque::with_capacity(10),
            capacity: QUEUE_CAPACITY,
        }
    }

    pub fn is_full(&mut self) -> bool {
        if (self.data.len() == self.capacity) {
            return true;
        }
        return false;
    }

    pub fn push(&mut self, value: T) {
        self.data.push_back(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.data.pop_front();
    }
}
