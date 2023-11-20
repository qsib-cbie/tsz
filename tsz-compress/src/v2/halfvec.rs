#![allow(dead_code)]

use alloc::vec::Vec;

///
/// A dynamic vector of nibbles.
///
/// Pushing is fast. There is no pop.
/// All bits are pushed together during finish.
///
pub struct HalfVec {
    words: Vec<Vec<HalfWord>>,
    len: usize,
}

///
/// Bits collected into a single word as one byte or two bytes.
///
pub enum HalfWord {
    /// The bottom bits of the word are used.
    /// 0b00001111
    Half(u8),

    /// The top and bottom bits of the word are used.
    /// 0b11111111
    Byte(u8),

    /// All bits of the word are used.
    /// 0xffffffff
    Full(u32),
}

impl HalfWord {
    fn len(&self) -> usize {
        match self {
            HalfWord::Half(_) => 1,
            HalfWord::Byte(_) => 2,
            HalfWord::Full(_) => 8,
        }
    }
}

impl HalfVec {
    ///
    /// Creates an empty vector.
    ///
    pub fn new(capacity: usize) -> Self {
        let mut words = Vec::with_capacity(1);
        words.push(Vec::with_capacity(capacity));
        Self { words, len: 0 }
    }

    ///
    /// Returns the number of elements in the queue.
    ///
    pub const fn len(&self) -> usize {
        self.len
    }

    ///
    /// Returns true if the queue if there is
    /// nothing to pop from the queue.
    ///
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    ///
    /// Pushes a value into the queue,
    /// overwriting the oldest value if the queue is full.
    ///
    #[inline(always)]
    pub fn push(&mut self, value: HalfWord) {
        self.len += value.len();
        // SAFETY: We allocate at least one vector in the constructor and never remove it.
        unsafe { self.words.last_mut().unwrap_unchecked().push(value) }
    }

    ///
    /// Consumes another instance of the queue and appends its contents to this one.
    ///
    /// This is a fast operation that does not require copying. Half words are
    /// packed together during finish.
    ///
    pub fn extend(&mut self, other: HalfVec) {
        self.words.extend(other.words);
        self.len += other.len;
    }

    ///
    /// Flattens the queue into a single vector of bytes.
    ///
    pub fn finish(self) -> Vec<u8> {
        // We will be writing directly into the buffer since we know the capacity
        unsafe {
            // 2 nibbles per byte and len is in nibbles
            let expected_capacity = self.len / 2;
            let mut bytes = Vec::with_capacity(expected_capacity + 1);

            // Keep track of whether we are on the upper or lower nibble across word lists
            let mut upper = true;
            for words in self.words {
                let mut byte = 0u8;
                for word in words {
                    if upper {
                        match word {
                            HalfWord::Half(value) => {
                                // Shift the value into the upper nibble
                                byte = value << 4;
                                // We are now on the lower nibble
                                upper = false;
                                // println!("nibble: {:b}", value);
                            }
                            HalfWord::Byte(value) => {
                                // Use both nibbles from the byte
                                known_append(&mut bytes, value);
                            }
                            HalfWord::Full(value) => {
                                // Use both nibbles from the top of the full
                                known_append(&mut bytes, (value >> 24) as u8);
                                // Use both nibbles from the top middle of the full
                                known_append(&mut bytes, (value >> 16) as u8);
                                // Use both nibbles from the bottom middle of the full
                                known_append(&mut bytes, (value >> 8) as u8);
                                // Use both nibbles from the bottom of the full
                                known_append(&mut bytes, value as u8);
                                // println!("full upper: {:b}", value);
                            }
                        }
                    } else {
                        match word {
                            HalfWord::Half(value) => {
                                // Fill the lower nibble, the upper nibble is already filled
                                byte |= value;
                                known_append(&mut bytes, byte);
                                // We are now on the upper nibble
                                upper = true;
                                // println!("nibble: {:b}", value);
                            }
                            HalfWord::Byte(value) => {
                                // Fill the lower nibble with the upper nibble of the value
                                byte |= value >> 4;
                                known_append(&mut bytes, byte);
                                // Use the lower nibble from the value as the upper nibble
                                byte = (value << 4) as u8;
                                // We are still on the lower nibble
                            }
                            HalfWord::Full(value) => {
                                // println!("full lower: {:b}", value);
                                // Fill the lower nibble with the upper nibble of the value
                                byte |= (value >> 28) as u8;
                                known_append(&mut bytes, byte);
                                // Fill the upper nibble with the top middle nibble of the value
                                byte = (value >> 20) as u8;
                                // Use both nibbles from the top middle of the full
                                known_append(&mut bytes, byte);
                                // Fill the upper nibble with the bottom middle nibble of the value
                                byte = (value >> 12) as u8;
                                // Use both nibbles from the bottom middle of the full
                                known_append(&mut bytes, byte);
                                // Use the lower nibble from the full as the upper nibble
                                byte = (value << 4) as u8;
                                // We are still on the lower nibble
                            }
                        }
                    }
                }
            }
            bytes
        }
    }
}

/// Appends a value to a vector without checking the capacity.
#[inline(always)]
unsafe fn known_append(buf: &mut Vec<u8>, value: u8) {
    // SAFETY: We allocate at least one vector in the constructor and never remove it.
    buf.as_mut_ptr().add(buf.len()).write(value);
    buf.set_len(buf.len() + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_init() {
        let queue = HalfVec::new(128);
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn can_push() {
        let mut queue = HalfVec::new(128);
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.is_empty(), true);

        for i in 0..128 {
            queue.push(HalfWord::Half(i as u8));
            assert_eq!(queue.len(), i + 1);
            assert_eq!(queue.is_empty(), false);
        }

        for i in 0..128 {
            queue.push(HalfWord::Full(i as u32));
            assert_eq!(queue.len(), 128 + (i + 1) * 4);
            assert_eq!(queue.is_empty(), false);
        }

        queue.push(HalfWord::Half(15));
        assert!(queue.len() % 2 == 1);
        // End on the byte
        queue.push(HalfWord::Half(0));

        // Now every nibble is pushed together
        let flat = queue.finish();
        assert_eq!(flat.len(), (128 + 4 * 128 + 1 + 1) / 2);
    }

    // #[test]
    // fn fuzz() {
    //     use alloc::collections::VecDeque;
    //     use rand::Rng;

    //     let mut rng = rand::thread_rng();
    //     let mut std_queue: VecDeque<usize> = VecDeque::new();
    //     let mut queue: CompressionQueue<usize, 10> = CompressionQueue::new();

    //     for _ in 0..10000 {
    //         let value = rng.gen::<usize>() % 100;
    //         if rng.gen::<bool>() {
    //             std_queue.push_back(value);
    //             if queue.is_full() {
    //                 std_queue.pop_front();
    //             }
    //             queue.push(value);
    //         } else {
    //             assert_eq!(std_queue.pop_front(), queue.pop());
    //         }

    //         assert_eq!(std_queue.len(), queue.len());
    //         for (a, b) in std_queue.iter().zip(queue.iter()) {
    //             assert_eq!(*a, b);
    //         }
    //     }
    // }
}
