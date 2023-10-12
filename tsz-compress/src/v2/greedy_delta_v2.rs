extern crate std;

use std::collections::VecDeque;

pub struct Queue<T, const N: usize> {
    pub(crate) data: VecDeque<T>,
}

impl<T, const N: usize> Queue<T, N> {
    pub fn new() -> Self {
        Queue {
            data: VecDeque::with_capacity(N),
        }
    }

    pub fn is_full(&mut self) -> bool {
        if self.data.len() >= N {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_push_pop() {
        const SIZE: usize = 10;
        let mut queue: Queue<usize, SIZE> = Queue::new();
        let pushed_value = 8;
        queue.push(pushed_value);
        let popped_value = queue.pop().unwrap();
        assert_eq!(pushed_value, popped_value);
    }
    #[test]
    fn test_is_full() {
        const SIZE: usize = 10;
        let mut queue: Queue<usize, SIZE> = Queue::new();
        for value in 0..SIZE {
            queue.push(value);
        }
        assert!(queue.is_full());
    }

    #[test]
    fn test_is_not_full() {
        const SIZE: usize = 10;
        let mut queue: Queue<usize, SIZE> = Queue::new();
        for value in 0..SIZE - 1 {
            queue.push(value);
        }
        assert!(!queue.is_full());
    }
}
