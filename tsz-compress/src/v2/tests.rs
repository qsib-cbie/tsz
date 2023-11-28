use crate::prelude::*;
pub use decode::*;
pub use encode::*;
pub use queue::*;

// Delta-Delta Tests
#[cfg(test)]
mod tests_delta {

    use crate::prelude::halfvec::{HalfVec, HalfWord};

    use super::*;
    use bitvec::bits;
    use rand::Rng;

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

        return bits;
    }

    #[test]
    fn test_emit_delta_i32_sanity1() {
        // Case 7: Encode 10 samples between [-4, 3] in 3 bits
        let values = vec![-3, 2, 0, 1, 2, -3, -1, -2, -4, -3];

        let encoded_halfvec = _emit_delta_i32(values);

        // Initialize expected bit buffer
        let mut expected_halfvec = HalfVec::new(8);

        // Expecting 10 samples of 3 bits
        expected_halfvec.push(HalfWord::Half(0b1111));

        // Values: [-3, 2, 0, 1, 2, -3, -1, -2, -4, -3]
        // Zigzag values: [5, 4, 0, 2, 4, 5, 1, 3, 7, 5]
        // Binary of zigzag values: 00 101 100 000 010 100 101 001 011 111 101
        let full_value = 0b00101100000010100101001011111101;
        expected_halfvec.push(HalfWord::Full(full_value));

        // Expected length
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
        expected_halfvec.push(HalfWord::Half(0b1110));

        // Values: [-32, 31, 16, 1, 2]
        // Zigzag values: [63, 62, 32, 2, 4]
        // Binary of zigzag values: 111111 111110 100000 000010 000100
        let full_value = 0b00111111111110100000000010000100;
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
        expected_halfvec.push(HalfWord::Half(0b1100));

        // Values: [-128, 127, 64, 1]
        // Zigzag values: [255, 254, 128, 2]
        // Binary of zigzag values: 11111111 11111110 10000000 00000010
        let full_value = 0b11111111111111101000000000000010;
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
        expected_halfvec.push(HalfWord::Half(0b1010));

        // Values: [-512, 511, 256]
        // Zigzag values: [1023, 1022, 512]
        // Binary of zigzag values: 1111111111 1111111110 1000000000
        let full_value = 0b00111111111111111111101000000000;
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
        expected_halfvec.push(HalfWord::Half(0b1000));

        // Values: [-32768, 32767]
        // Zigzag values: [65535, 65534]
        // Binary of zigzag values: 1111111111111111 1111111111111110
        let full_value = 0b11111111111111111111111111111110;
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
        expected_halfvec.push(HalfWord::Half(0b1011));

        // Values: vec![i32::MIN / 4]
        // Zigzag values: [-1 * i32::MIN / 2]
        // Binary of zigzag values:
        let full_value = 0b00111111111111111111111111111111;
        expected_halfvec.push(HalfWord::Full(full_value));

        // Expected length
        assert_eq!(encoded_halfvec, expected_halfvec);
    }
}
