#![allow(dead_code)]

use alloc::vec::Vec;

///
/// A dynamic vector of nibbles.
///
/// Pushing is fast. There is no pop.
/// All bits are pushed together during finish.
///
#[derive(Debug)]
pub struct HalfVec {
    words: Vec<HalfWord>,
    len: usize,
}

///
/// Bits collected into a single word as one byte or two bytes.
///
#[derive(Debug)]
pub enum HalfWord {
    /// The bottom bits of the word are used.
    /// 0b0000_1111
    Half(u8),

    /// The top and bottom bits of the word are used.
    /// 0b1111_1111
    Byte(u8),

    /// All bits of the word are used.
    /// 0xffff_ffff
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
        Self {
            words: Vec::with_capacity(capacity),
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
    /// Returns true if the queue if there is
    /// nothing to pop from the queue.
    ///
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    ///
    /// Clears the queue, removing all values.
    /// The queue will be empty after this call completes, but it may not be zero-capacity.
    ///
    pub fn clear(&mut self) {
        self.len = 0;
        self.words.clear();
    }

    ///
    /// Pushes a value into the queue,
    /// overwriting the oldest value if the queue is full.
    ///
    #[inline(always)]
    pub fn push(&mut self, value: HalfWord) {
        self.len += value.len();
        self.words.push(value);
    }

    ///
    /// Flattens the queue into a single vector of bytes.
    ///
    pub fn finish<'a, I>(word_lists: I) -> Vec<u8>
    where
        I: Iterator<Item = &'a HalfVec> + Clone,
    {
        // We will be writing directly into the buffer since we know the capacity
        unsafe {
            // 2 nibbles per byte and len is in nibbles
            let len = word_lists.clone().map(|w| w.len).sum::<usize>() / 2;
            let mut bytes = Vec::with_capacity(len + 1);

            // Keep track of whether we are on the upper or lower nibble across word lists
            let mut upper = true;
            let mut byte = 0u8;

            // Iterate over all the words in all the word lists
            for word in word_lists.flat_map(|w| w.words.iter()) {
                if upper {
                    match word {
                        HalfWord::Half(value) => {
                            // Shift the value into the upper nibble
                            byte = value << 4;
                            // We are now on the lower nibble
                            upper = false;
                            // println!("upper nibble: {:b}", value);
                        }
                        HalfWord::Byte(value) => {
                            // Use both nibbles from the byte
                            known_append(&mut bytes, *value);
                            // println!("upper byte: {:b}", value);
                        }
                        HalfWord::Full(value) => {
                            // Use both nibbles from the top of the full
                            known_append(&mut bytes, (value >> 24) as u8);
                            // Use both nibbles from the top middle of the full
                            known_append(&mut bytes, (value >> 16) as u8);
                            // Use both nibbles from the bottom middle of the full
                            known_append(&mut bytes, (value >> 8) as u8);
                            // Use both nibbles from the bottom of the full
                            known_append(&mut bytes, *value as u8);
                            // println!("upper full: {:b}", value);
                        }
                    }
                } else {
                    match word {
                        HalfWord::Half(value) => {
                            // Fill the lower nibble, the upper nibble is already filled
                            byte |= value & 0x0F;
                            known_append(&mut bytes, byte);
                            // We are now on the upper nibble
                            upper = true;
                            // println!("lower nibble: {:b}", value);
                        }
                        HalfWord::Byte(value) => {
                            // Fill the lower nibble with the upper nibble of the value
                            byte |= value >> 4;
                            known_append(&mut bytes, byte);
                            // Use the lower nibble from the value as the upper nibble
                            byte = (value << 4) as u8;
                            // We are still on the lower nibble
                            // println!("lower byte: {:b}", value);
                        }
                        HalfWord::Full(value) => {
                            // println!("lower full: {:b}", value);
                            // Fill the lower nibble with the upper nibble of the value
                            byte |= (value >> 28) as u8;
                            known_append(&mut bytes, byte);
                            // Bits 28-20
                            byte = (value >> 20) as u8;
                            known_append(&mut bytes, byte);
                            // Bits 20-12
                            byte = (value >> 12) as u8;
                            known_append(&mut bytes, byte);
                            // Bits 12-4
                            byte = (value >> 4) as u8;
                            known_append(&mut bytes, byte);

                            // Use the lower nibble from the full as the upper nibble
                            byte = (value << 4) as u8;
                            // We are still on the lower nibble
                        }
                    }
                }
            }

            if !upper {
                // We are on the lower nibble, so fill the upper nibble with 0b1001
                byte |= 0b1001;
                known_append(&mut bytes, byte);
            }

            bytes
        }
    }
}

/// Compares two HalfVec for equality.
impl PartialEq for HalfVec {
    fn eq(&self, other: &Self) -> bool {
        self.words == other.words && self.len == other.len
    }
}

/// Compares two HalfWord for equality.
impl PartialEq for HalfWord {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HalfWord::Half(value1), HalfWord::Half(value2)) => value1 == value2,
            (HalfWord::Byte(value1), HalfWord::Byte(value2)) => value1 == value2,
            (HalfWord::Full(value1), HalfWord::Full(value2)) => value1 == value2,
            _ => false,
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
        let flat = HalfVec::finish(&[&queue]);
        assert_eq!(flat.len(), (128 + 4 * 128 + 1 + 1) / 2);
    }
}
