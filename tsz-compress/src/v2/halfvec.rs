#![allow(dead_code)]

use core::mem::MaybeUninit;

use crate::prelude::*;
use crate::v2::consts::headers;
use alloc::vec::Vec;
pub use queue::*;

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
    pub fn finish<'a, I>(out: &mut Vec<u8>, word_lists: I)
    where
        I: Iterator<Item = &'a HalfVec> + Clone,
    {
        // We will be writing directly into the buffer since we know the capacity
        unsafe {
            // 2 nibbles per byte and len is in nibbles
            // Reserve enough space for the output
            let len = word_lists.clone().map(|w| w.len).sum::<usize>() / 2;
            let reserve_len = len + 1;
            let avail = out.capacity() - out.len();
            if avail < reserve_len {
                out.reserve_exact(reserve_len - avail);
            }
            let bytes = out.spare_capacity_mut();
            let mut idx = 0;

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
                        }
                        HalfWord::Byte(value) => {
                            // Use both nibbles from the byte
                            known_append(bytes, &mut idx, *value);
                        }
                        HalfWord::Full(value) => {
                            // Use both nibbles from the top of the full
                            known_append(bytes, &mut idx, (value >> 24) as u8);
                            // Use both nibbles from the top middle of the full
                            known_append(bytes, &mut idx, (value >> 16) as u8);
                            // Use both nibbles from the bottom middle of the full
                            known_append(bytes, &mut idx, (value >> 8) as u8);
                            // Use both nibbles from the bottom of the full
                            known_append(bytes, &mut idx, *value as u8);
                        }
                    }
                } else {
                    match word {
                        HalfWord::Half(value) => {
                            // Fill the lower nibble, the upper nibble is already filled
                            byte |= value & 0x0F;
                            known_append(bytes, &mut idx, byte);
                            // We are now on the upper nibble
                            upper = true;
                        }
                        HalfWord::Byte(value) => {
                            // Fill the lower nibble with the upper nibble of the value
                            byte |= value >> 4;
                            known_append(bytes, &mut idx, byte);
                            // Use the lower nibble from the value as the upper nibble
                            byte = value << 4;
                            // We are still on the lower nibble
                        }
                        HalfWord::Full(value) => {
                            // Fill the lower nibble with the upper nibble of the value
                            byte |= (value >> 28) as u8;
                            known_append(bytes, &mut idx, byte);
                            // Bits 28-20
                            byte = (value >> 20) as u8;
                            known_append(bytes, &mut idx, byte);
                            // Bits 20-12
                            byte = (value >> 12) as u8;
                            known_append(bytes, &mut idx, byte);
                            // Bits 12-4
                            byte = (value >> 4) as u8;
                            known_append(bytes, &mut idx, byte);

                            // Use the lower nibble from the full as the upper nibble
                            byte = (value << 4) as u8;
                            // We are still on the lower nibble
                        }
                    }
                }
            }

            if !upper {
                // We are on the lower nibble, so fill the upper nibble with headers::START_OF_COLUMN
                byte |= headers::START_OF_COLUMN;
                known_append(bytes, &mut idx, byte);
            }

            out.set_len(out.len() + idx);
        }
    }
}

/// Appends a value to a vector without checking the capacity.
#[inline(always)]
unsafe fn known_append(buf: &mut [MaybeUninit<u8>], idx: &mut usize, value: u8) {
    // SAFETY: We allocate at least one vector in the constructor and never remove it.
    (*buf.as_mut_ptr().add(*idx)).write(value);
    *idx += 1;
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
            assert_eq!(queue.len(), 128 + (i + 1) * 8);
            assert_eq!(queue.is_empty(), false);
        }

        queue.push(HalfWord::Half(15));
        assert!(queue.len() % 2 == 1);
        // End on the byte
        queue.push(HalfWord::Half(0));

        // Now every nibble is pushed together
        let mut bytes = Vec::new();
        HalfVec::finish(&mut bytes, [&queue].into_iter());
        assert_eq!(bytes.len(), (128 + 8 * 128 + 1 + 1) / 2);
    }

    #[test]
    fn can_push_with_header() {
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
            assert_eq!(queue.len(), 128 + (i + 1) * 8);
            assert_eq!(queue.is_empty(), false);
        }

        queue.push(HalfWord::Half(15));
        assert!(queue.len() % 2 == 1);
        // End on the byte
        queue.push(HalfWord::Half(0));

        // Now every nibble is pushed together
        let mut bytes = Vec::new();
        bytes.push(0xDE);
        bytes.push(0xAD);
        bytes.push(0xBE);
        bytes.push(0xEF);
        HalfVec::finish(&mut bytes, [&queue].into_iter());
        assert_eq!(bytes.len(), 4 + ((128 + 8 * 128 + 1 + 1) / 2));
        assert_eq!(bytes[0], 0xDE);
        assert_eq!(bytes[1], 0xAD);
        assert_eq!(bytes[2], 0xBE);
        assert_eq!(bytes[3], 0xEF);
    }
}

// Delta Tests
#[cfg(test)]
mod tests_emit_delta {

    use crate::prelude::halfvec::{HalfVec, HalfWord};

    use super::*;
    use bitvec::bits;
    use rand::Rng;

    /// Compares two HalfVec for equality.
    impl PartialEq for HalfVec {
        fn eq(&self, other: &Self) -> bool {
            self.words == other.words && self.len() == other.len()
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

    // Helped function
    fn _emit_delta_i32(values: Vec<i32>) -> HalfVec {
        // Create queue
        let mut queue: CompressionQueue<10> = CompressionQueue::new();

        // Push values into queue
        for value in &values {
            queue.push(*value);
        }

        // Initialize bit buffer
        let mut bits = HalfVec::new(8);

        // Encode
        queue.emit_delta_bits(&mut bits);

        bits
    }

    #[test]
    fn test_emit_delta_i32_sanity1() {
        // Case 7: Encode 10 samples between [-4, 3] in 3 bits
        let values = vec![-3, 2, 0, 1, 2, -3, -1, -2, -4, -3];

        let encoded_halfvec = _emit_delta_i32(values);

        // Initialize expected bit buffer
        let mut expected_halfvec = HalfVec::new(8);

        // Expecting 10 samples of 3 bits
        expected_halfvec.push(HalfWord::Half(headers::THREE_BITS_TEN_SAMPLES));

        // Values: [-3, 2, 0, 1, 2, -3, -1, -2, -4, -3]
        // Zigzag values: [5, 4, 0, 2, 4, 5, 1, 3, 7, 5]
        // Binary of zigzag values: 00 101 100 000 010 100 101 001 011 111 101
        let full_value = 0x2C0A_52FD;
        expected_halfvec.push(HalfWord::Full(full_value));

        // Assert equality
        assert_eq!(encoded_halfvec, expected_halfvec);
    }

    #[test]
    fn test_emit_delta_i32_sanity2() {
        // Case 6 and 7: Encode 5 samples between [-32, 31] in 6 bits
        let values = vec![-32, 31, 16, 1, 2];

        let encoded_halfvec = _emit_delta_i32(values);

        // Initialize expected bit buffer
        let mut expected_halfvec = HalfVec::new(8);

        // Expecting 10 samples of 3 bits
        expected_halfvec.push(HalfWord::Half(headers::SIX_BITS_FIVE_SAMPLES));

        // Values: [-32, 31, 16, 1, 2]
        // Zigzag values: [63, 62, 32, 2, 4]
        // Binary of zigzag values: 111111 111110 100000 000010 000100
        let full_value = 0x3FFA_0084;
        expected_halfvec.push(HalfWord::Full(full_value));

        // Expected length
        assert_eq!(encoded_halfvec, expected_halfvec);
    }

    #[test]
    fn test_emit_delta_i32_sanity3() {
        // Case 5, 6 and 7: Encode 4 samples between [-128, 127] in 8 bits
        let values = vec![-128, 127, 64, 1];

        let encoded_halfvec = _emit_delta_i32(values);

        // Initialize expected bit buffer
        let mut expected_halfvec = HalfVec::new(8);

        // Expecting 10 samples of 3 bits
        expected_halfvec.push(HalfWord::Half(headers::EIGHT_BITS_FOUR_SAMPLES));

        // Values: [-128, 127, 64, 1]
        // Zigzag values: [255, 254, 128, 2]
        // Binary of zigzag values: 11111111 11111110 10000000 00000010
        let full_value = 0xFFFE_8002;
        expected_halfvec.push(HalfWord::Full(full_value));

        // Expected length
        assert_eq!(encoded_halfvec, expected_halfvec);
    }

    #[test]
    fn test_emit_delta_i32_sanity4() {
        // Case 4, 5, 6 and 7: Encode 3 samples between [-512, 511] in 10 bits
        let values = vec![-512, 511, 256];

        let encoded_halfvec = _emit_delta_i32(values);

        // Initialize expected bit buffer
        let mut expected_halfvec = HalfVec::new(8);

        // Expecting 10 samples of 3 bits
        expected_halfvec.push(HalfWord::Half(headers::TEN_BITS_THREE_SAMPLES));

        // Values: [-512, 511, 256]
        // Zigzag values: [1023, 1022, 512]
        // Binary of zigzag values: 1111111111 1111111110 1000000000
        let full_value = 0x3FFF_FA00;
        expected_halfvec.push(HalfWord::Full(full_value));

        // Expected length
        assert_eq!(encoded_halfvec, expected_halfvec);
    }

    #[test]
    fn test_emit_delta_i32_sanity5() {
        // Case 3, 4, 5, 6 and 7: Encode 2 samples between [-32768, 32767] in 16 bits
        let values = vec![-32768, 32767];

        let encoded_halfvec = _emit_delta_i32(values);

        // Initialize expected bit buffer
        let mut expected_halfvec = HalfVec::new(8);

        // Expecting 10 samples of 3 bits
        expected_halfvec.push(HalfWord::Half(headers::SIXTEEN_BITS_TWO_SAMPLES));

        // Values: [-32768, 32767]
        // Zigzag values: [65535, 65534]
        // Binary of zigzag values: 1111111111111111 1111111111111110
        let full_value = 0xFFFF_FFFE;
        expected_halfvec.push(HalfWord::Full(full_value));

        // Expected length
        assert_eq!(encoded_halfvec, expected_halfvec);
    }

    #[test]
    fn test_emit_delta_i32_sanity6() {
        // Case 2, 3, 4, 5, 6 and 7: Encode 2 samples between [-32768, 32767] in 16 bits
        let values = vec![i32::MIN / 4, 1, 1, 1];

        let encoded_halfvec = _emit_delta_i32(values);

        // Initialize expected bit buffer
        let mut expected_halfvec = HalfVec::new(8);

        // Expecting 10 samples of 3 bits
        expected_halfvec.push(HalfWord::Half(headers::THIRTY_TWO_BITS_ONE_SAMPLE));

        // Values: vec![i32::MIN / 4]
        // Zigzag values: [-1 * i32::MIN / 2]
        // Binary of zigzag values:
        let full_value = 0x3FFFFFFF;
        expected_halfvec.push(HalfWord::Full(full_value));

        // Expected length
        assert_eq!(encoded_halfvec, expected_halfvec);
    }
}
