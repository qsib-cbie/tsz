use crate::prelude::*;
use core::ops::Range;

pub mod queue;
pub use queue::*;

trait BitVectorOps {
    fn extend_bits(self, bit_range: Range<usize>, buf: &mut BitBuffer);
}

macro_rules! extend_bitsi {
    ($i:ident) => {
        impl BitVectorOps for $i {
            #[inline(always)]
            fn extend_bits(self, bit_range: Range<usize>, buf: &mut BitBuffer) {
                bit_range.for_each(|x| buf.push(self & (1 << x) != 0));
            }
        }
    };
}

extend_bitsi!(i8);
extend_bitsi!(i16);
extend_bitsi!(i32);
extend_bitsi!(i64);

#[inline]
fn push_header_pad_three_bits(buf: &mut BitBuffer) {
    for _ in 0..4 {
        buf.push(true);
    }
    for _ in 0..2 {
        buf.push(false);
    }
}

#[inline]
fn push_header_pad_four_bits(buf: &mut BitBuffer) {
    for _ in 0..3 {
        buf.push(true);
    }
    buf.push(false);
}

#[inline]
fn push_header_pad_eight_bits(buf: &mut BitBuffer) {
    for _ in 0..2 {
        buf.push(true);
    }
    for _ in 0..2 {
        buf.push(false);
    }
}

#[inline]
fn push_header_pad_ten_bits(buf: &mut BitBuffer) {
    buf.push(true);
    buf.push(false);
    buf.push(true);
    for _ in 0..3 {
        buf.push(false);
    }
}

#[inline]
fn push_header_pad_sixteen_bits(buf: &mut BitBuffer) {
    buf.push(true);
    for _ in 0..3 {
        buf.push(false);
    }
}

pub trait EmitBits<T> {
    /// Emits bits according to the most efficient case of Delta Compression.
    /// Returns the number of elements popped from the queue.
    fn emit_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize;
}

impl EmitBits<i64> for CompressionQueue<i64, 10> {
    #[allow(unused)]
    fn emit_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut iter = self.iter();
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;
        while let Some(value) = iter.next() {
            let remaining = iter.size_hint().0;
            let index = 10 - remaining;

            if ((0..=2).contains(&index) && !(-32768..=32767).contains(&value)) {
                sixteen = false;
            }
            if ((0..=3).contains(&index) && !(-512..=511).contains(&value)) {
                ten = false;
            }
            if ((0..=4).contains(&index) && !(-128..=127).contains(&value)) {
                eight = false;
            }
            if ((0..=8).contains(&index) && !(-8..=7).contains(&value)) {
                four = false;
            }
            if ((0..=10).contains(&index) && !(-4..=3).contains(&value)) {
                three = false;
            }
        }

        if three {
            push_header_pad_three_bits(out);
            for _ in 0..10 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i64) ^ (value >> 63i64); // ZigZag Encoding
                    value.extend_bits(0..3, out);
                }
            }
            return 10;
        } else if four {
            push_header_pad_four_bits(out);
            for _ in 0..8 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i64) ^ (value >> 63i64); // ZigZag Encoding
                    value.extend_bits(0..4, out);
                }
            }
            return 8;
        } else if eight {
            push_header_pad_eight_bits(out);
            for _ in 0..4 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i64) ^ (value >> 63i64); // ZigZag Encoding
                    value.extend_bits(0..8, out);
                }
            }
            return 4;
        } else if ten {
            push_header_pad_ten_bits(out);
            for _ in 0..3 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i64) ^ (value >> 63i64); // ZigZag Encoding
                    value.extend_bits(0..10, out);
                }
            }
            return 3;
        } else if sixteen {
            push_header_pad_sixteen_bits(out);
            for _ in 0..2 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i64) ^ (value >> 63i64); // ZigZag Encoding
                    value.extend_bits(0..16, out);
                }
            }
            return 2;
        }
        0
    }
}

impl EmitBits<i32> for CompressionQueue<i32, 10> {
    #[allow(unused)]
    fn emit_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut iter = self.iter();
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;
        while let Some(value) = iter.next() {
            let remaining = iter.size_hint().0;
            let index = 10 - remaining;

            if ((0..=2).contains(&index) && !(-32768..=32767).contains(&value)) {
                sixteen = false;
            }
            if ((0..=3).contains(&index) && !(-512..=511).contains(&value)) {
                ten = false;
            }
            if ((0..=4).contains(&index) && !(-128..=127).contains(&value)) {
                eight = false;
            }
            if ((0..=8).contains(&index) && !(-8..=7).contains(&value)) {
                four = false;
            }
            if ((0..=10).contains(&index) && !(-4..=3).contains(&value)) {
                three = false;
            }
        }

        if three {
            push_header_pad_three_bits(out);
            for _ in 0..10 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32); // ZigZag Encoding
                    value.extend_bits(0..3, out);
                }
            }
            return 10;
        } else if four {
            push_header_pad_four_bits(out);
            for _ in 0..8 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32); // ZigZag Encoding
                    value.extend_bits(0..4, out);
                }
            }
            return 8;
        } else if eight {
            push_header_pad_eight_bits(out);
            for _ in 0..4 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32); // ZigZag Encoding
                    value.extend_bits(0..8, out);
                }
            }
            return 4;
        } else if ten {
            push_header_pad_ten_bits(out);
            for _ in 0..3 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32); // ZigZag Encoding
                    value.extend_bits(0..10, out);
                }
            }
            return 3;
        } else if sixteen {
            push_header_pad_sixteen_bits(out);
            for _ in 0..2 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32); // ZigZag Encoding
                    value.extend_bits(0..16, out);
                }
            }
            return 2;
        }
        0
    }
}

impl EmitBits<i16> for CompressionQueue<i16, 10> {
    #[allow(unused)]
    fn emit_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut iter = self.iter();
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;
        while let Some(value) = iter.next() {
            let remaining = iter.size_hint().0;
            let index = 10 - remaining;

            if ((0..=3).contains(&index) && !(-512..=511).contains(&value)) {
                ten = false;
            }
            if ((0..=4).contains(&index) && !(-128..=127).contains(&value)) {
                eight = false;
            }
            if ((0..=8).contains(&index) && !(-8..=7).contains(&value)) {
                four = false;
            }
            if ((0..=10).contains(&index) && !(-4..=3).contains(&value)) {
                three = false;
            }
        }

        if three {
            push_header_pad_three_bits(out);
            for _ in 0..10 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16); // ZigZag Encoding
                    value.extend_bits(0..3, out);
                }
            }
            return 10;
        } else if four {
            push_header_pad_four_bits(out);
            for _ in 0..8 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16); // ZigZag Encoding
                    value.extend_bits(0..4, out);
                }
            }
            return 8;
        } else if eight {
            push_header_pad_eight_bits(out);
            for _ in 0..4 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16); // ZigZag Encoding
                    value.extend_bits(0..8, out);
                }
            }
            return 4;
        } else if ten {
            push_header_pad_ten_bits(out);
            for _ in 0..3 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16); // ZigZag Encoding
                    value.extend_bits(0..10, out);
                }
            }
            return 3;
        } else if sixteen {
            push_header_pad_sixteen_bits(out);
            for _ in 0..2 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16); // ZigZag Encoding
                    value.extend_bits(0..16, out);
                }
            }
            return 2;
        }
        0
    }
}

impl EmitBits<i8> for CompressionQueue<i8, 10> {
    #[allow(unused)]
    fn emit_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut iter = self.iter();
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;
        while let Some(value) = iter.next() {
            let remaining = iter.size_hint().0;
            let index = 10 - remaining;

            if ((0..=8).contains(&index) && !(-8..=7).contains(&value)) {
                four = false;
            }
            if ((0..=10).contains(&index) && !(-4..=3).contains(&value)) {
                three = false;
            }
        }

        if three {
            push_header_pad_three_bits(out);
            for _ in 0..10 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i8) ^ (value >> 7i8); // ZigZag Encoding
                    value.extend_bits(0..3, out);
                }
            }
            return 10;
        } else if four {
            push_header_pad_four_bits(out);
            for _ in 0..8 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i8) ^ (value >> 7i8); // ZigZag Encoding
                    value.extend_bits(0..4, out);
                }
            }
            return 8;
        } else if eight {
            push_header_pad_eight_bits(out);
            for _ in 0..4 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i8) ^ (value >> 7i8); // ZigZag Encoding
                    value.extend_bits(0..8, out);
                }
            }
            return 4;
        } else if ten {
            push_header_pad_ten_bits(out);
            for _ in 0..3 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i8) ^ (value >> 7i8); // ZigZag Encoding
                    value.extend_bits(0..10, out);
                }
            }
            return 3;
        } else if sixteen {
            push_header_pad_sixteen_bits(out);
            for _ in 0..2 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i8) ^ (value >> 7i8); // ZigZag Encoding
                    value.extend_bits(0..16, out);
                }
            }
            return 2;
        }
        0
    }
}

pub fn decode_delta(bits: &'_ BitBufferSlice) -> Result<[i32; 10], &'static str> {
    if bits.is_empty() {
        return Err("Not enough bits to decode");
    }
    let mut decoded_buffer: [i32; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut value: i32 = 0;
    if bits[0] {
        // Case 1: 00
        if !bits[1] && !bits[2] {
            // Skipping pad 0
            for i in (4..36).step_by(16) {
                for j in 0..16 {
                    value |= (bits[i + j] as i32) << j;
                }
                value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                decoded_buffer[decoded_buffer_index] = value;
                decoded_buffer_index += 1;
                value = 0;
            }
        }
        // Case 2: 01
        else if !bits[1] && bits[2] {
            // Skipping pad 000
            for i in (6..36).step_by(10) {
                for j in 0..10 {
                    value |= (bits[i + j] as i32) << j;
                }
                value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                decoded_buffer[decoded_buffer_index] = value;
                decoded_buffer_index += 1;
                value = 0;
            }
        }
        // Case 3: 10
        else if bits[1] && !bits[2] {
            // Skipping pad 0
            for i in (4..36).step_by(8) {
                for j in 0..8 {
                    value |= (bits[i + j] as i32) << j;
                }
                value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                decoded_buffer[decoded_buffer_index] = value;
                decoded_buffer_index += 1;
                value = 0;
            }
        }
        // Case 4: 110
        else if bits[1] && bits[2] && !bits[3] {
            for i in (4..36).step_by(4) {
                for j in 0..4 {
                    value |= (bits[i + j] as i32) << j;
                }
                value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                decoded_buffer[decoded_buffer_index] = value;
                decoded_buffer_index += 1;
                value = 0;
            }
        }
        // Case 5: 111
        else if bits[1] && bits[2] && bits[3] {
            // Skipping pad 00
            for i in (6..36).step_by(3) {
                for j in 0..3 {
                    value |= (bits[i + j] as i32) << j;
                }
                value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                decoded_buffer[decoded_buffer_index] = value;
                decoded_buffer_index += 1;
                value = 0;
            }
        } else {
            return Err("Invalid encoding");
        }

        return Ok(decoded_buffer);
    } else if !bits[0] {
        // Delta-Delta Decompression
        todo!()
    } else {
        return Err("Invalid encoding for delta or delta-delta compression.");
    }
}

///
/// High-level interface for compression.
///
pub trait TszCompressV2 {
    /// The type of the row to compress.
    type T: Copy;

    ///
    /// Lazily compress a row.
    ///
    fn compress(&mut self, row: Self::T);

    ///
    /// The number of bits that have been compressed.
    /// This is an estimate, as the last few samples may have been emitted are estimated.
    ///
    fn len(&self) -> usize;

    ///
    /// Return an estimate of bits per column value as the number of
    /// compressed bits / count of column values compressed / columns per row.
    ///
    fn bit_rate(&self) -> usize;

    ///
    /// Finish compression and return the compressed data.
    ///
    fn finish(self) -> BitBuffer;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn can_encode_decode_case1() {
        // Case 1: Encode and decode 3 samples between [-32768, 32767] in 16 bits
        let values: [i32; 10] = [-32768, 30000, -512, -128, 511, 80, 7, -2, 2, 3];
        let mut queue: CompressionQueue<i32, 10> = CompressionQueue::new();
        for value in values {
            queue.push(value);
        }

        let mut bits = BitBuffer::new();
        let num_emitted_samples = queue.emit_bits(&mut bits, true);
        let decoded_values = decode_delta(&bits).unwrap();
        assert_eq!(num_emitted_samples, 2);
        assert_eq!(queue.len(), 8);
        assert_eq!(values[..2], decoded_values[..2]);
    }
    #[test]
    fn can_encode_decode_case2() {
        // Case 2: Encode and decode 3 samples between [-512, 511] in 10 bits
        let values: [i32; 10] = [-3, 499, -512, -128, 511, 80, 7, -2, 2, 3];
        let mut queue: CompressionQueue<i32, 10> = CompressionQueue::new();
        for value in values {
            queue.push(value);
        }
        let mut bits = BitBuffer::new();
        let num_emitted_samples = queue.emit_bits(&mut bits, true);
        let decoded_values = decode_delta(&bits).unwrap();
        assert_eq!(num_emitted_samples, 3);
        assert_eq!(queue.len(), 7);
        assert_eq!(values[..3], decoded_values[..3]);
    }

    #[test]
    fn can_encode_decode_case3() {
        // Case 3: Encode and decode 4 samples between [-128, 127] in 8 bits
        let values: [i32; 10] = [-3, 100, 0, -128, 127, 80, 7, -2, 2, 3];
        let mut queue: CompressionQueue<i32, 10> = CompressionQueue::new();
        for value in values {
            queue.push(value);
        }

        let mut bits = BitBuffer::new();
        let num_emitted_samples = queue.emit_bits(&mut bits, true);
        let decoded_values = decode_delta(&bits).unwrap();
        assert_eq!(num_emitted_samples, 4);
        assert_eq!(queue.len(), 6);
        assert_eq!(values[..4], decoded_values[..4]);
    }

    #[test]
    fn can_encode_decode_case4() {
        // Case 4: Encode and decode 8 samples between [-8, 7] in 4 bits
        let values: [i32; 10] = [-3, 2, 0, 1, -8, 6, -7, -2, 2, 3];
        let mut queue: CompressionQueue<i32, 10> = CompressionQueue::new();
        for value in values {
            queue.push(value);
        }

        let mut bits = BitBuffer::new();
        let num_emitted_samples = queue.emit_bits(&mut bits, true);
        let decoded_values = decode_delta(&bits).unwrap();
        assert_eq!(num_emitted_samples, 8);
        assert_eq!(queue.len(), 2);
        assert_eq!(values[..8], decoded_values[..8]);
    }

    #[test]
    fn can_encode_decode_case5() {
        // Case 5: Encode and decode 10 samples between [-4, 3] in 3 bits
        let values: [i32; 10] = [-3, 2, 0, 1, 2, -3, -1, -2, -4, -3];
        let mut queue: CompressionQueue<i32, 10> = CompressionQueue::new();
        for value in values {
            queue.push(value);
        }
        let mut bits = BitBuffer::new();
        let num_emitted_samples = queue.emit_bits(&mut bits, true);
        let decoded_values = decode_delta(&bits).unwrap();
        assert_eq!(num_emitted_samples, 10);
        assert_eq!(queue.len(), 0);
        assert_eq!(values, decoded_values);
    }

    fn _can_encode_decode_values(values: &Vec<i32>) -> (usize, [i32; 10]) {
        // Helper function
        if values.len() != 10 {
            println!("Vec size should be 10");
        }
        let mut queue: CompressionQueue<i32, 10> = CompressionQueue::new();
        for value in values {
            queue.push(*value);
        }
        let mut bits = BitBuffer::new();
        let num_emitted_samples = queue.emit_bits(&mut bits, true);
        let decoded_values = decode_delta(&bits).unwrap();
        return (num_emitted_samples, decoded_values);
    }

    #[test]
    fn can_encode_decode_all() {
        for i in (-32768..=32758).step_by(10) {
            let values: Vec<i32> = (i..i + 10).collect::<Vec<_>>();
            let (num_emitted_samples, decoded_values) = _can_encode_decode_values(&values);
            assert_eq!(
                values[..num_emitted_samples],
                decoded_values[..num_emitted_samples]
            );
        }
    }

    #[test]
    fn can_encode_decode_random_three_bits_values() {
        // Random values in range of case 5
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec: Vec<i32> = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(-4..=3));
            }
            let (num_emitted_samples, decoded_values) = _can_encode_decode_values(&random_vec);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..num_emitted_samples]
            );
        }
    }

    #[test]
    fn can_encode_decode_random_four_bits_values() {
        // Random values in range of case 4 & 5
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec: Vec<i32> = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(-8..=7));
            }
            let (num_emitted_samples, decoded_values) = _can_encode_decode_values(&random_vec);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..num_emitted_samples]
            );
        }
    }

    #[test]
    fn can_encode_decode_random_eight_bits_values() {
        // Random values in range of case 3, 4 & 5
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec: Vec<i32> = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(-128..=127));
            }
            let (num_emitted_samples, decoded_values) = _can_encode_decode_values(&random_vec);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..num_emitted_samples]
            );
        }
    }

    #[test]
    fn can_encode_decode_random_ten_bits_values() {
        // Random values in range of case 2, 3, 4 & 5
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec: Vec<i32> = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(-512..=511));
            }
            let (num_emitted_samples, decoded_values) = _can_encode_decode_values(&random_vec);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..num_emitted_samples]
            );
        }
    }

    #[test]
    fn can_encode_decode_random_sixteen_bits_values() {
        // Random values in range of case 1, 2, 3, 4 & 5
        let mut rng = rand::thread_rng();
        for _ in 0..=100000 {
            let mut random_vec: Vec<i32> = Vec::with_capacity(10);
            for _i in 0..10 {
                random_vec.push(rng.gen_range(-32768..=32767));
            }
            let (num_emitted_samples, decoded_values) = _can_encode_decode_values(&random_vec);
            assert_eq!(
                random_vec[..num_emitted_samples],
                decoded_values[..num_emitted_samples]
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
