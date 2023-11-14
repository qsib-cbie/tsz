#![allow(dead_code)]
use core::mem::MaybeUninit;

use num_traits::PrimInt;

///
/// A statically sized ring-buffer queue used
/// while compressing a column.
///
pub struct CompressionQueue<T, const N: usize> {
    buf: [MaybeUninit<T>; N],
    front: usize,
    len: usize,
}

impl<T: PrimInt, const N: usize> CompressionQueue<T, N> {
    ///
    /// Creates an empty queue.
    ///
    pub const fn new() -> Self {
        CompressionQueue {
            buf: [MaybeUninit::uninit(); N],
            front: 0,
            len: 0,
        }
    }

    ///
    /// Returns the number of elements in the queue.
    ///
    pub const fn len(&self) -> usize {
        self.len
    }

    ///
    /// Returns true if the queue if the queue would
    /// have to overwrite an element to push a new value.
    ///
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    ///
    /// Returns true if the queue if there is
    /// nothing to pop from the queue.
    ///
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    ///
    /// Pushes a value into the queue,
    /// overwriting the oldest value if the queue is full.
    ///
    pub fn push(&mut self, value: T) {
        let index = (self.front + self.len) % N;
        unsafe { self.write(index, value) };
        if !self.is_full() {
            self.len += 1;
        } else {
            self.front = (self.front + 1) % N;
        }
    }

    ///
    /// Pops the oldest value from the queue,
    /// returning None if the queue is empty.
    ///
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let value = unsafe { self.at(self.front) };
        self.front = (self.front + 1) % N;
        self.len -= 1;
        Some(value)
    }

    ///
    /// Pop N values from the queue at once,
    /// returning None if the queue is empty.
    ///
    #[inline(always)]
    pub fn pop_n<const M: usize>(&mut self) -> Option<[T; M]> {
        if self.len < M {
            return None;
        }

        let mut values: [T; M] = [T::zero(); M];
        for i in 0..M {
            let index = (self.front + i) % N;
            unsafe { values[i] = self.at(index) };
        }

        self.front = (self.front + M) % N;
        self.len -= M;

        Some(values)
    }

    ///
    /// Creates an iterator over the elements of the queue.
    /// The iterator will yield the oldest element first.
    ///
    pub fn iter(&self) -> CompressionQueueIter<T, N> {
        CompressionQueueIter {
            queue: self,
            index: 0,
        }
    }

    ///
    /// Internal use accessor to an initialized value.
    ///
    /// # Safety
    /// This function is unsafe because it assumes that
    /// the index is inbounds and initialized.
    ///
    unsafe fn at(&self, index: usize) -> T {
        self.buf.get_unchecked(index).assume_init()
    }

    ///
    /// Internal use mutable accessor to an initialized value.
    ///
    /// # Safety
    /// This function is unsafe because it assumes that
    /// the index is inbounds. It does *not* drop the value
    /// at the index if it was initialized; therefore, T must
    /// be trivially dropable.
    ///
    unsafe fn write(&mut self, index: usize, t: T) {
        self.buf.get_unchecked_mut(index).write(t);
    }
}

///
/// An iterator over the elements of a CompressionQueue.
/// The iterator will yield the oldest element first.
///
pub struct CompressionQueueIter<'a, T, const N: usize> {
    queue: &'a CompressionQueue<T, N>,
    index: usize,
}

impl<'a, T: Copy, const N: usize> Iterator for CompressionQueueIter<'a, T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.len {
            return None;
        }

        let index = (self.queue.front + self.index) % N;
        self.index += 1;
        Some(unsafe { self.queue.at(index) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.queue.len - self.index;
        (remaining, Some(remaining))
    }
}

impl<'a, T: Copy, const N: usize> ExactSizeIterator for CompressionQueueIter<'a, T, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_init() {
        let queue: CompressionQueue<usize, 10> = CompressionQueue::new();
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.is_full(), false);
        assert_eq!(queue.is_empty(), true);
        queue.iter().for_each(|_| panic!("should be empty"));
    }

    #[test]
    fn is_empty_or_full() {
        let mut queue: CompressionQueue<usize, 4> = CompressionQueue::new();
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.is_full(), false);

        // push 4 values, queue should be full
        for i in 0..4 {
            queue.push(i);
            assert_eq!(queue.len(), i + 1);
            assert_eq!(queue.is_empty(), false);
            assert_eq!(queue.is_full(), i == 3);
        }

        // iterate over the values, they should be 0..4
        for x in queue.iter().enumerate() {
            assert_eq!(x.0, x.1);
        }

        // pop 4 values, queue should be empty
        for i in 0..4 {
            assert_eq!(queue.pop(), Some(i));
            assert_eq!(queue.len(), 3 - i);
            assert_eq!(queue.is_empty(), (i == 3));
            assert_eq!(queue.is_full(), false);
        }
    }

    #[test]
    fn can_overwrite() {
        let mut queue: CompressionQueue<usize, 4> = CompressionQueue::new();

        // push 4 values, queue should be full
        for i in 0..4 {
            queue.push(i);
            assert_eq!(queue.len(), i + 1);
            assert_eq!(queue.is_empty(), false);
            assert_eq!(queue.is_full(), i == 3);
        }

        // keep pushing, queue should still be full
        for i in 4..8 {
            queue.push(i);
            assert_eq!(queue.len(), 4);
            assert_eq!(queue.is_empty(), false);
            assert_eq!(queue.is_full(), true);
        }

        // iterate over the values, they should be 4..8
        for (i, value) in queue.iter().enumerate() {
            assert_eq!(value, i + 4);
        }

        // pop the values, they should be 4..8
        for j in 0..4 {
            assert_eq!(queue.pop(), Some(j + 4));
            assert_eq!(queue.len(), 3 - j);
            assert_eq!(queue.is_empty(), (j == 3));
            assert_eq!(queue.is_full(), false);
        }
    }

    #[test]
    fn fuzz() {
        use alloc::collections::VecDeque;
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut std_queue: VecDeque<usize> = VecDeque::new();
        let mut queue: CompressionQueue<usize, 10> = CompressionQueue::new();

        for _ in 0..10000 {
            let value = rng.gen::<usize>() % 100;
            if rng.gen::<bool>() {
                std_queue.push_back(value);
                if queue.is_full() {
                    std_queue.pop_front();
                }
                queue.push(value);
            } else {
                assert_eq!(std_queue.pop_front(), queue.pop());
            }

            assert_eq!(std_queue.len(), queue.len());
            for (a, b) in std_queue.iter().zip(queue.iter()) {
                assert_eq!(*a, b);
            }
        }
    }
}
