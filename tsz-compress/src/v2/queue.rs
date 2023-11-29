use super::encode::Bits;

///
/// A statically sized ring-buffer queue used
/// while compressing a column.
///
/// The absolute max size of this buffer is 16 elements.
///
#[derive(Debug)]
pub struct CompressionQueue<const N: usize> {
    zigzag: [u32; 16],
    bitcount: [usize; 16],
    front: usize,
    len: usize,
}

impl<const N: usize> CompressionQueue<N> {
    ///
    /// Creates an empty queue.
    ///
    pub const fn new() -> Self {
        assert!(N <= 16);
        CompressionQueue {
            zigzag: [0; 16],
            bitcount: [0; 16],
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
        self.len >= N
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
    pub fn push<T: Bits + Sized>(&mut self, value: T) {
        let index = (self.front + self.len) % 16;
        unsafe { self.write(index, value) };
        if self.len < 16 {
            self.len += 1;
        } else {
            self.front = (self.front + 1) % 16;
        }
    }

    ///
    /// Pops the oldest value from the queue,
    /// returning None if the queue is empty.
    ///
    pub fn pop(&mut self) -> Option<u32> {
        if self.is_empty() {
            return None;
        }

        let value = unsafe { self.value_at(self.front) };
        self.front = (self.front + 1) % 16;
        self.len -= 1;
        Some(value)
    }

    ///
    /// Pop N values from the queue at once,
    /// values may not be meaningful if the queue is
    /// not of length N.
    ///
    #[inline(always)]
    pub fn pop_n<const M: usize>(&mut self) -> [u32; M] {
        let mut values: [u32; M] = [0; M];
        for i in 0..M {
            let index = (self.front + i) % 16;
            unsafe {
                *values.get_unchecked_mut(i) = self.value_at(index);
            }
        }
        self.front = (self.front + M) % 16;
        self.len -= M;
        values
    }

    ///
    /// Peak N values from the queue at once,
    /// values may not be meaningful if the queue is
    /// not of length N.
    ///
    #[inline(always)]
    pub fn peak_bitcounts<const M: usize>(&mut self) -> [usize; M] {
        let mut values: [usize; M] = [0; M];
        for i in 0..M {
            let index = (self.front + i) % 16;
            unsafe {
                *values.get_unchecked_mut(i) = self.count_at(index);
            }
        }
        values
    }

    ///
    /// Internal use accessor to an initialized value.
    ///
    /// # Safety
    /// This function is unsafe because it assumes that
    /// the index is inbounds and initialized.
    ///
    unsafe fn value_at(&self, index: usize) -> u32 {
        *self.zigzag.get_unchecked(index)
    }

    ///
    /// Internal use accessor to an initialized value.
    ///
    /// # Safety
    /// This function is unsafe because it assumes that
    /// the index is inbounds and initialized.
    ///
    unsafe fn count_at(&self, index: usize) -> usize {
        *self.bitcount.get_unchecked(index)
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
    unsafe fn write<T: Bits + Sized>(&mut self, index: usize, t: T) {
        let (zbits, zcount) = t.zigzag_bits();
        *self.zigzag.get_unchecked_mut(index) = zbits;
        *self.bitcount.get_unchecked_mut(index) = zcount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_init() {
        let queue: CompressionQueue<10> = CompressionQueue::new();
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.is_full(), false);
        assert_eq!(queue.is_empty(), true);
    }

    // #[test]
    // fn is_empty_or_full() {
    //     let mut queue: CompressionQueue<usize, 4> = CompressionQueue::new();
    //     assert_eq!(queue.len(), 0);
    //     assert_eq!(queue.is_empty(), true);
    //     assert_eq!(queue.is_full(), false);

    //     // push 4 values, queue should be full
    //     for i in 0..4 {
    //         queue.push(i);
    //         assert_eq!(queue.len(), i + 1);
    //         assert_eq!(queue.is_empty(), false);
    //         assert_eq!(queue.is_full(), i == 3);
    //     }

    //     // iterate over the values, they should be 0..4
    //     for x in queue.iter().enumerate() {
    //         assert_eq!(x.0, x.1);
    //     }

    //     // pop 4 values, queue should be empty
    //     for i in 0..4 {
    //         assert_eq!(queue.pop(), Some(i));
    //         assert_eq!(queue.len(), 3 - i);
    //         assert_eq!(queue.is_empty(), (i == 3));
    //         assert_eq!(queue.is_full(), false);
    //     }
    // }

    // #[test]
    // fn can_overwrite() {
    //     let mut queue: CompressionQueue<usize, 4> = CompressionQueue::new();

    //     // push 4 values, queue should be full
    //     for i in 0..4 {
    //         queue.push(i);
    //         assert_eq!(queue.len(), i + 1);
    //         assert_eq!(queue.is_empty(), false);
    //         assert_eq!(queue.is_full(), i == 3);
    //     }

    //     // keep pushing, queue should still be full
    //     for i in 4..8 {
    //         queue.push(i);
    //         assert_eq!(queue.len(), i + 1);
    //         assert_eq!(queue.is_empty(), false);
    //         assert_eq!(queue.is_full(), true);
    //     }

    //     // keep pushing, queue should still be full and start overwriting
    //     for i in 8..20 {
    //         queue.push(i);
    //         assert_eq!(queue.len(), (i + 1).min(16));
    //         assert_eq!(queue.is_empty(), false);
    //         assert_eq!(queue.is_full(), true);
    //     }

    //     // iterate over the values, they should be 4..20
    //     for (i, value) in queue.iter().enumerate() {
    //         assert_eq!(value, i + 4);
    //     }

    //     // pop the values, they should be 16..20
    //     for j in 0..4 {
    //         assert_eq!(queue.pop(), Some(j + 4));
    //         assert_eq!(queue.len(), 15 - j);
    //         assert_eq!(queue.is_empty(), false);
    //         assert_eq!(queue.is_full(), true);
    //     }

    //     // pop another 8 values, then the queue will start to empty
    //     queue.pop_n::<8>();
    //     assert_eq!(queue.len(), 4);
    //     assert_eq!(queue.is_empty(), false);
    //     assert_eq!(queue.is_full(), true);

    //     // pop the remaining 4 values, then the queue will be empty
    //     for j in 0..4 {
    //         assert_eq!(queue.pop(), Some(j + 16));
    //         assert_eq!(queue.len(), 3 - j);
    //         assert_eq!(queue.is_empty(), (j == 3));
    //         assert_eq!(queue.is_full(), false);
    //     }
    // }

    #[test]
    fn fuzz() {
        use alloc::collections::VecDeque;
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut std_queue: VecDeque<u32> = VecDeque::new();
        let mut queue: CompressionQueue<10> = CompressionQueue::new();
        for _ in 0..10000 {
            let value = rng.gen::<i32>();
            let zig_zag_value = value.zigzag();
            if rng.gen::<bool>() {
                std_queue.push_back(zig_zag_value);
                if queue.len() == 16 {
                    assert_eq!(std_queue.pop_front(), queue.pop());
                }
                queue.push(value);
            } else {
                assert_eq!(std_queue.pop_front(), queue.pop());
            }
            assert_eq!(std_queue.len(), queue.len());

            for _ in 0..queue.len() {
                assert_eq!(std_queue.pop_front(), queue.pop());
            }
        }
    }
}
