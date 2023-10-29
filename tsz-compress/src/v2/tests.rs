use crate::prelude::*;
pub use decode::*;
pub use encode::*;
pub use queue::*;

#[cfg(test)]
mod tests_delta {

    use super::*;
    use bitvec::bits;
    use rand::Rng;

    // Delta i8
    // Helper function
    fn _can_encode_decode_delta_values_i8(
        values: &Vec<i8>,
        flush: bool,
    ) -> (usize, usize, usize, Vec<i8>) {
        // Case 5: Encode and decode 10 samples between [-4, 3] in 3 bits
        let mut queue: CompressionQueue<i8, 10> = CompressionQueue::new();
        for value in values {
            queue.push(*value);
        }
        let mut bits = BitBuffer::new();
        let num_emitted_samples = queue.emit_delta_bits(&mut bits, flush);
        let (decoded_values, decoded_size) = decode_delta_i8(&bits).unwrap();
        let decoded_values = decoded_values.to_vec();
        return (
            queue.len(),
            num_emitted_samples,
            decoded_size,
            decoded_values,
        );
    }

    #[test]
    fn can_encode_decode_delta_i8_sanity1() {
        // Case 4
        let values = vec![-3, 2, 0, 1, 2, -3, -1, -2, -4, -3];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i8(&values, false);
        assert_eq!(num_emitted_samples, 10);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i8_sanity2() {
        // Case 4, 5
        let values = vec![-4, 6, -8, 3, 2, -1, 7, 0, -5, 4];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i8(&values, false);
        assert_eq!(num_emitted_samples, 8);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i8_sanity3() {
        // Case 3, 4, 5
        let values = vec![-32, 115, -78, 56, 12, -127, 89, 43, -3, 101];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i8(&values, false);
        assert_eq!(num_emitted_samples, 4);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_random_i8() {
        // Random values in i8 range
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(i8::MIN..=i8::MAX));
            }
            let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
                _can_encode_decode_delta_values_i8(&random_vec, true);
            assert_eq!(queue_size, &random_vec.len() - num_emitted_samples);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..decoded_size]
            );
        }
    }

    #[test]
    fn can_encode_decode_delta_i8_flush_sanity() {
        let values = vec![-31, 11, -106, -75];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i8(&values, true);
        assert_eq!(num_emitted_samples, 4);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i8_flush_sanity2() {
        let values = vec![93, -127, -100];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i8(&values, true);

        // assert_eq!(num_emitted_samples, 4);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i8_flush_sanity3() {
        let values = vec![-55, 72];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i8(&values, true);

        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i8_flush_random() {
        for _ in 0..100000 {
            let mut rng = rand::thread_rng();
            let mut random_vec: Vec<i8> = Vec::with_capacity(10);
            // Number of samples in flush conditions
            let end_range = rng.gen_range(1..10);

            for _i in 0..=end_range {
                random_vec.push(rng.gen_range(i8::MIN..=i8::MAX));
            }
            let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
                _can_encode_decode_delta_values_i8(&random_vec, true);

            assert_eq!(queue_size, random_vec.len() - num_emitted_samples);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..decoded_size]
            );
        }
    }

    // Delta i16
    // Helper function
    fn _can_encode_decode_delta_values_i16(
        values: &Vec<i16>,
        flush: bool,
    ) -> (usize, usize, usize, Vec<i16>) {
        let mut queue: CompressionQueue<i16, 10> = CompressionQueue::new();
        for value in values {
            queue.push(*value);
        }
        let mut bits = BitBuffer::new();
        let num_emitted_samples = queue.emit_delta_bits(&mut bits, flush);
        let (decoded_values, decoded_size) = decode_delta_i16(&bits).unwrap();
        let decoded_values = decoded_values.to_vec();
        return (
            queue.len(),
            decoded_size,
            num_emitted_samples,
            decoded_values,
        );
    }

    #[test]
    fn can_encode_decode_delta_i16_sanity1() {
        // Case 5: Encode and decode 10 samples between [-4, 3] in 3 bits
        let values = vec![-3, 2, 0, 1, 2, -3, -1, -2, -4, -3];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i16(&values, false);
        assert_eq!(num_emitted_samples, 10);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i16_sanity2() {
        // Case 4 and 5: Encode and decode 10 samples between [-8, 7] in 3 bits
        let values = vec![-4, 6, -8, 3, 2, -1, 7, 0, -5, 4];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i16(&values, false);
        assert_eq!(num_emitted_samples, 8);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i16_sanity3() {
        // Case 3, 4, 5
        let values = vec![-32, 115, -78, 56, 12, -127, 89, 43, -3, 101];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i16(&values, false);
        assert_eq!(num_emitted_samples, 4);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i16_sanity4() {
        // Case 2, 3, 4, 5
        let values = vec![-256, 489, -123, 402, 67, -505, 311, 109, -412, 210];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i16(&values, false);
        assert_eq!(num_emitted_samples, 3);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i16_sanity5() {
        // Case 1, 2, 3, 4, 5
        let values = vec![
            -32768, 23456, -7891, 16042, 5678, -27600, 9123, 14567, -22222, 7890,
        ];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i16(&values, false);
        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_random_i16() {
        // Random values in i16 range
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(i16::MIN..=i16::MAX));
            }
            let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
                _can_encode_decode_delta_values_i16(&random_vec, true);
            assert_eq!(queue_size, &random_vec.len() - num_emitted_samples);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..decoded_size]
            );
        }
    }

    #[test]
    fn can_encode_decode_delta_i16_flush_sanity() {
        let values: Vec<i16> = vec![-8458, -11624, 15294, 27516];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i16(&values, true);
        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i16_flush_sanity2() {
        let values = vec![-8458, -11624, -100];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i16(&values, true);

        // assert_eq!(num_emitted_samples, 4);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i16_flush_sanity3() {
        let values = vec![-55, 72];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i16(&values, true);

        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i16_flush_random() {
        for _ in 0..100000 {
            let mut rng = rand::thread_rng();
            let mut random_vec: Vec<i16> = Vec::with_capacity(10);
            // Number of samples in flush conditions
            let end_range = rng.gen_range(1..10);

            for _i in 0..=end_range {
                random_vec.push(rng.gen_range(i16::MIN..=i16::MAX));
            }
            let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
                _can_encode_decode_delta_values_i16(&random_vec, true);

            assert_eq!(queue_size, random_vec.len() - num_emitted_samples);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..decoded_size]
            );
        }
    }

    // Delta i32
    // Helper function
    fn _can_encode_decode_delta_values_i32(
        values: &Vec<i32>,
        flush: bool,
    ) -> (usize, usize, usize, Vec<i32>) {
        let mut queue: CompressionQueue<i32, 10> = CompressionQueue::new();
        for value in values {
            queue.push(*value);
        }
        let mut bits = BitBuffer::new();
        let num_emitted_samples = queue.emit_delta_bits(&mut bits, flush);
        let (decoded_values, decoded_size) = decode_delta_i32(&bits).unwrap();
        let decoded_values = decoded_values.to_vec();
        return (
            queue.len(),
            decoded_size,
            num_emitted_samples,
            decoded_values,
        );
    }

    #[test]
    fn can_encode_decode_delta_i32_sanity1() {
        // Case 5: Encode and decode 10 samples between [-4, 3] in 3 bits
        let values = vec![-3, 2, 0, 1, 2, -3, -1, -2, -4, -3];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i32(&values, false);
        assert_eq!(num_emitted_samples, 10);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i32_sanity2() {
        // Case 4 and 5: Encode and decode 10 samples between [-8, 7] in 3 bits
        let values = vec![-4, 6, -8, 3, 2, -1, 7, 0, -5, 4];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i32(&values, false);
        assert_eq!(num_emitted_samples, 8);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i32_sanity3() {
        // Case 3, 4, 5
        let values = vec![-32, 115, -78, 56, 12, -127, 89, 43, -3, 101];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i32(&values, false);
        assert_eq!(num_emitted_samples, 4);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i32_sanity4() {
        // Case 2, 3, 4, 5
        let values = vec![-256, 489, -123, 402, 67, -505, 311, 109, -412, 210];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i32(&values, false);
        assert_eq!(num_emitted_samples, 3);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i32_sanity5() {
        // Case 1, 2, 3, 4, 5
        let values = vec![
            -32768, 23456, -7891, 16042, 5678, -27600, 9123, 14567, -22222, 7890,
        ];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i32(&values, false);
        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_random_i32() {
        // Random values in i32 range
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(i16::MIN as i32..=i16::MAX as i32));
            }
            let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
                _can_encode_decode_delta_values_i32(&random_vec, true);
            assert_eq!(queue_size, &random_vec.len() - num_emitted_samples);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..decoded_size]
            );
        }
    }

    #[test]
    fn can_encode_decode_delta_i32_flush_sanity() {
        let values: Vec<i32> = vec![-8458, -11624, 15294, 27516];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i32(&values, true);
        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i32_flush_sanity2() {
        let values = vec![-8458, -11624, -100];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i32(&values, true);

        // assert_eq!(num_emitted_samples, 4);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i32_flush_sanity3() {
        let values = vec![-55, 72];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i32(&values, true);

        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i32_flush_random() {
        for _ in 0..100000 {
            let mut rng = rand::thread_rng();
            let mut random_vec: Vec<i32> = Vec::with_capacity(10);
            // Number of samples in flush conditions
            let end_range = rng.gen_range(1..10);

            for _i in 0..=end_range {
                random_vec.push(rng.gen_range(i16::MIN as i32..=i16::MAX as i32));
            }
            let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
                _can_encode_decode_delta_values_i32(&random_vec, true);

            assert_eq!(queue_size, random_vec.len() - num_emitted_samples);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..decoded_size]
            );
        }
    }

    // Delta i64
    // Helper function
    fn _can_encode_decode_delta_values_i64(
        values: &Vec<i64>,
        flush: bool,
    ) -> (usize, usize, usize, Vec<i64>) {
        let mut queue: CompressionQueue<i64, 10> = CompressionQueue::new();
        for value in values {
            queue.push(*value);
        }
        let mut bits = BitBuffer::new();
        let num_emitted_samples = queue.emit_delta_bits(&mut bits, flush);
        let (decoded_values, decoded_size) = decode_delta_i64(&bits).unwrap();
        let decoded_values = decoded_values.to_vec();
        return (
            queue.len(),
            decoded_size,
            num_emitted_samples,
            decoded_values,
        );
    }

    #[test]
    fn can_encode_decode_delta_i64_sanity1() {
        // Case 5: Encode and decode 10 samples between [-4, 3] in 3 bits
        let values = vec![-3, 2, 0, 1, 2, -3, -1, -2, -4, -3];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i64(&values, false);
        assert_eq!(num_emitted_samples, 10);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i64_sanity2() {
        // Case 4 and 5: Encode and decode 10 samples between [-8, 7] in 3 bits
        let values = vec![-4, 6, -8, 3, 2, -1, 7, 0, -5, 4];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i64(&values, false);
        assert_eq!(num_emitted_samples, 8);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i64_sanity3() {
        // Case 3, 4, 5
        let values = vec![-32, 115, -78, 56, 12, -127, 89, 43, -3, 101];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i64(&values, false);
        assert_eq!(num_emitted_samples, 4);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i64_sanity4() {
        // Case 2, 3, 4, 5
        let values = vec![-256, 489, -123, 402, 67, -505, 311, 109, -412, 210];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i64(&values, false);
        assert_eq!(num_emitted_samples, 3);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i64_sanity5() {
        // Case 1, 2, 3, 4, 5
        let values = vec![
            -32768, 23456, -7891, 16042, 5678, -27600, 9123, 14567, -22222, 7890,
        ];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i64(&values, false);
        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_random_i64() {
        // Random values in i64 range
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(i16::MIN as i64..=i16::MAX as i64));
            }
            let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
                _can_encode_decode_delta_values_i64(&random_vec, true);
            assert_eq!(queue_size, &random_vec.len() - num_emitted_samples);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..decoded_size]
            );
        }
    }

    #[test]
    fn can_encode_decode_delta_i64_flush_sanity() {
        let values: Vec<i64> = vec![-8458, -11624, 15294, 27516];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i64(&values, true);
        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i64_flush_sanity2() {
        let values = vec![-8458, -11624, -100];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i64(&values, true);

        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i64_flush_sanity3() {
        let values = vec![-55, 72];
        let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
            _can_encode_decode_delta_values_i64(&values, true);

        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue_size, values.len() - num_emitted_samples);
        assert_eq!(
            values[..num_emitted_samples],
            decoded_values[..decoded_size]
        );
    }

    #[test]
    fn can_encode_decode_delta_i64_flush_random() {
        for _ in 0..100000 {
            let mut rng = rand::thread_rng();
            let mut random_vec: Vec<i64> = Vec::with_capacity(10);
            // Number of samples in flush conditions
            let end_range = rng.gen_range(1..10);

            for _i in 0..=end_range {
                random_vec.push(rng.gen_range(i16::MIN as i64..=i16::MAX as i64));
            }
            let (queue_size, num_emitted_samples, decoded_size, decoded_values) =
                _can_encode_decode_delta_values_i64(&random_vec, true);

            assert_eq!(queue_size, random_vec.len() - num_emitted_samples);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..decoded_size]
            );
        }
    }
}

#[cfg(test)]
mod test_delta_delta {

    use super::*;
    use bitvec::bits;
    use rand::Rng;

    #[test]
    fn can_encode_decode_delta_delta_i8() {
        let values: [i8; 10] = [-128, -64, -32, -16, -8, 7, 15, 31, 63, 127];
        let mut queue: CompressionQueue<i8, 10> = CompressionQueue::new();
        for value in values {
            queue.push(value);
        }
        let mut bits = BitBuffer::new();
        let out = &mut bits;
        let num_emitted_values = queue.emit_delta_delta_bits(out, false);
        let (decoded_values, decoded_size) = decode_delta_delta_i8(out).unwrap();
        assert_eq!(values[..num_emitted_values], decoded_values[..decoded_size]);
    }

    #[test]
    fn can_encode_decode_delta_delta_i16() {
        let values: [i16; 10] = [
            -32768, -16384, -8192, -4096, -2048, 2047, 4095, 8191, 16383, 32767,
        ];
        let mut queue: CompressionQueue<i16, 10> = CompressionQueue::new();
        for value in values {
            queue.push(value);
        }
        let mut bits = BitBuffer::new();
        let out = &mut bits;
        let num_emitted_values = queue.emit_delta_delta_bits(out, false);
        let (decoded_values, decoded_size) = decode_delta_delta_i16(out).unwrap();
        assert_eq!(values[..num_emitted_values], decoded_values[..decoded_size]);
    }

    #[test]
    fn can_encode_decode_delta_delta_i32() {
        let values: [i32; 10] = [
            -2147483648,
            -1073741824,
            -536870912,
            -268435456,
            -134217728,
            134217727,
            268435455,
            536870911,
            1073741823,
            2147483647,
        ];
        let mut queue: CompressionQueue<i32, 10> = CompressionQueue::new();
        for value in values {
            queue.push(value);
        }
        let mut bits = BitBuffer::new();
        let out = &mut bits;
        let num_emitted_values = queue.emit_delta_delta_bits(out, false);
        let (decoded_values, decoded_size) = decode_delta_delta_i32(out).unwrap();
        assert_eq!(values[..num_emitted_values], decoded_values[..decoded_size]);
    }

    #[test]
    fn can_encode_decode_delta_delta_i64() {
        let values: [i64; 10] = [
            -9223372036854775808,
            -4611686018427387904,
            -2305843009213693952,
            -1152921504606846976,
            -576460752303423488,
            576460752303423487,
            1152921504606846975,
            2305843009213693951,
            4611686018427387903,
            9223372036854775807,
        ];
        let mut queue: CompressionQueue<i64, 10> = CompressionQueue::new();
        for value in values {
            queue.push(value);
        }
        let mut bits = BitBuffer::new();
        let out = &mut bits;
        let num_emitted_values = queue.emit_delta_delta_bits(out, false);
        let (decoded_values, decoded_size) = decode_delta_delta_i64(out).unwrap();
        assert_eq!(values[..num_emitted_values], decoded_values[..decoded_size]);
    }

    #[test]
    fn can_encode_decode_random_delta_delta_i8() {
        // Random values in i8 range
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec: Vec<i8> = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(i8::MIN..i8::MAX));
            }
            let mut queue: CompressionQueue<i8, 10> = CompressionQueue::new();
            for value in &random_vec {
                queue.push(*value);
            }
            let mut bits = BitBuffer::new();
            let out = &mut bits;
            let num_emitted_values = queue.emit_delta_delta_bits(out, false);
            let (decoded_values, decoded_size) = decode_delta_delta_i8(out).unwrap();
            assert_eq!(
                random_vec[..num_emitted_values],
                decoded_values[..decoded_size]
            );
        }
    }

    #[test]
    fn can_encode_decode_random_delta_delta_i16() {
        // Random values in i16 range
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec: Vec<i16> = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(i16::MIN..i16::MAX));
            }
            let mut queue: CompressionQueue<i16, 10> = CompressionQueue::new();
            for value in &random_vec {
                queue.push(*value);
            }
            let mut bits = BitBuffer::new();
            let out = &mut bits;
            let num_emitted_values = queue.emit_delta_delta_bits(out, false);
            let (decoded_values, decoded_size) = decode_delta_delta_i16(out).unwrap();
            assert_eq!(
                random_vec[..num_emitted_values],
                decoded_values[..decoded_size]
            );
        }
    }
    #[test]
    fn can_encode_decode_random_delta_delta_i32() {
        // Random values in i32 range
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec: Vec<i32> = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(i32::MIN..i32::MAX));
            }
            let mut queue: CompressionQueue<i32, 10> = CompressionQueue::new();
            for value in &random_vec {
                queue.push(*value);
            }
            let mut bits = BitBuffer::new();
            let out = &mut bits;
            let num_emitted_values = queue.emit_delta_delta_bits(out, false);
            let (decoded_values, decoded_size) = decode_delta_delta_i32(out).unwrap();
            assert_eq!(
                random_vec[..num_emitted_values],
                decoded_values[..decoded_size]
            );
        }
    }
    #[test]
    fn can_encode_decode_random_delta_delta_i64() {
        // Random values in i64 range
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec: Vec<i64> = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(i64::MIN..i64::MAX));
            }
            let mut queue: CompressionQueue<i64, 10> = CompressionQueue::new();
            for value in &random_vec {
                queue.push(*value);
            }
            let mut bits = BitBuffer::new();
            let out = &mut bits;
            let num_emitted_values = queue.emit_delta_delta_bits(out, false);
            let (decoded_values, decoded_size) = decode_delta_delta_i64(out).unwrap();
            assert_eq!(
                random_vec[..num_emitted_values],
                decoded_values[..decoded_size]
            );
        }
    }

    #[test]
    fn can_impl_compress() {
        #[derive(Copy, Clone)]
        struct TestRow {
            a: i32,
            b: i64,
        }
        struct CompressorImpl {
            a_queue: CompressionQueue<i32, 10>,
            b_queue: CompressionQueue<i64, 10>,
        }

        impl TszCompressV2 for CompressorImpl {
            type T = TestRow;
            fn compress(&mut self, row: TestRow) {
                self.a_queue.push(row.a);
                self.b_queue.push(row.b);
            }

            fn len(&self) -> usize {
                self.a_queue.len() + self.b_queue.len()
            }

            fn bit_rate(&self) -> usize {
                0
            }

            fn finish(self) -> BitBuffer {
                BitBuffer::new()
            }
        }
    }
}
