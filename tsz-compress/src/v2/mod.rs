use crate::prelude::*;
use core::ops::Range;

pub mod queue;
pub use queue::*;

#[derive(Debug)]
pub enum CodingError {
    NotEnoughBits,
    InvalidBits,
}

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
extend_bitsi!(i128);

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

/**
Delta Compression
*/
pub trait EmitDeltaBits<T> {
    /// Emits bits according to the most efficient case of Delta Compression.
    /// Returns the number of elements popped from the queue.
    fn emit_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize;
}

impl EmitDeltaBits<i64> for CompressionQueue<i64, 10> {
    #[allow(unused)]
    fn emit_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;

        let queue_length = self.len();

        // Check flush conditions
        if flush {
            // Can not emit with any case of delta compression if queue is empty or contains 1 sample.
            if self.is_empty() || self.len() == 1 {
                return 0;
            }

            // Can not emit with case v of delta compression if number of samples < 10
            if self.len() < 10 {
                three = false;
            }

            // Can not emit with case iv of delta compression if number of samples < 8.
            if self.len() < 8 {
                four = false;
            }

            // Can not emit with case iii of delta compression if number of samples < 4
            if self.len() < 4 {
                eight = false;
            }
            // Can not emit with case ii of delta compression if number of samples < 3
            if self.len() < 3 {
                ten = false;
            }
            // Can not emit with case ii of delta compression if number of samples < 2
            if self.len() < 2 {
                sixteen = false;
            }
        }

        self.iter().enumerate().for_each(|(index, value)| {
            if (index <= 2 && !(-32768..=32767).contains(&value)) {
                sixteen = false;
            }
            if (index <= 3 && !(-512..=511).contains(&value)) {
                ten = false;
            }
            if (index <= 4 && !(-128..=127).contains(&value)) {
                eight = false;
            }
            if (index <= 8 && !(-8..=7).contains(&value)) {
                four = false;
            }
            if (index <= 10 && !(-4..=3).contains(&value)) {
                three = false;
            }
        });

        // Emit according to priority of cases
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

impl EmitDeltaBits<i32> for CompressionQueue<i32, 10> {
    #[allow(unused)]
    fn emit_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;

        let queue_length = self.len();

        // Check flush conditions
        if flush {
            // Can not emit with any case of delta compression if queue is empty or contains 1 sample.
            if self.is_empty() || self.len() == 1 {
                return 0;
            }

            // Can not emit with case v of delta compression if number of samples < 10
            if self.len() < 10 {
                three = false;
            }

            // Can not emit with case iv of delta compression if number of samples < 8.
            if self.len() < 8 {
                four = false;
            }

            // Can not emit with case iii of delta compression if number of samples < 4
            if self.len() < 4 {
                eight = false;
            }
            // Can not emit with case ii of delta compression if number of samples < 3
            if self.len() < 3 {
                ten = false;
            }
            // Can not emit with case ii of delta compression if number of samples < 2
            if self.len() < 2 {
                sixteen = false;
            }
        }

        self.iter().enumerate().for_each(|(index, value)| {
            if (index <= 2 && !(-32768..=32767).contains(&value)) {
                sixteen = false;
            }
            if (index <= 3 && !(-512..=511).contains(&value)) {
                ten = false;
            }
            if (index <= 4 && !(-128..=127).contains(&value)) {
                eight = false;
            }
            if (index <= 8 && !(-8..=7).contains(&value)) {
                four = false;
            }
            if (index <= 10 && !(-4..=3).contains(&value)) {
                three = false;
            }
        });

        // Emit according to priority of cases
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

impl EmitDeltaBits<i16> for CompressionQueue<i16, 10> {
    #[allow(unused)]
    fn emit_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;

        let queue_length = self.len();

        // Check flush conditions
        if flush {
            // Can not emit with any case of delta compression if queue is empty or contains 1 sample.
            if self.is_empty() || self.len() == 1 {
                return 0;
            }

            // Can not emit with case v of delta compression if number of samples < 10
            if self.len() < 10 {
                three = false;
            }

            // Can not emit with case iv of delta compression if number of samples < 8.
            if self.len() < 8 {
                four = false;
            }

            // Can not emit with case iii of delta compression if number of samples < 4
            if self.len() < 4 {
                eight = false;
            }
            // Can not emit with case ii of delta compression if number of samples < 3
            if self.len() < 3 {
                ten = false;
            }
            // Can not emit with case ii of delta compression if number of samples < 2
            if self.len() < 2 {
                sixteen = false;
            }
        }

        // Check case range conditions
        self.iter().enumerate().for_each(|(index, value)| {
            if (index <= 2 && !(-32768..=32767).contains(&value)) {
                sixteen = false;
            }
            if (index <= 3 && !(-512..=511).contains(&value)) {
                ten = false;
            }
            if (index <= 4 && !(-128..=127).contains(&value)) {
                eight = false;
            }
            if (index <= 8 && !(-8..=7).contains(&value)) {
                four = false;
            }
            if (index <= 10 && !(-4..=3).contains(&value)) {
                three = false;
            }
        });

        // Emit according to priority of cases
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
                    let value = value as i32;
                    let value = (value << 1i16) ^ (value >> 15i16); // ZigZag Encoding
                    value.extend_bits(0..16, out);
                }
            }
            return 2;
        }
        0
    }
}

impl EmitDeltaBits<i8> for CompressionQueue<i8, 10> {
    #[allow(unused)]
    fn emit_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;

        let queue_length = self.len();

        // Check flush conditions
        if flush {
            // Can not emit with any case of delta compression if queue is empty or contains 1 sample.
            if self.is_empty() || self.len() == 1 {
                return 0;
            }
            // Can not emit with case v of delta compression if number of samples < 10
            if self.len() < 10 {
                three = false;
            }

            // Can not emit with case iv of delta compression if number of samples < 8.
            if self.len() < 8 {
                four = false;
            }

            // Can not emit with case iii of delta compression if number of samples < 4
            if self.len() < 4 {
                eight = false;
            }

            // Can not emit with case ii of delta compression if number of samples < 3
            if self.len() < 3 {
                ten = false;
            }

            // Can not emit with case ii of delta compression if number of samples < 2
            if self.len() < 2 {
                sixteen = false;
            }
        }

        // Check case range conditions

        self.iter().enumerate().for_each(|(index, value)| {
            // Can not emit with case iii if a sample between indices [0, 4] contains values out of case iii range [-128, 127]
            if (index <= 4 && !(-128..=127).contains(&value)) {
                eight = false;
            }

            // Can not emit with case iv if a sample between indices [0, 8] contains values out of case iv range [-8, 7]
            if (index <= 8 && !(-8..=7).contains(&value)) {
                four = false;
            }

            // Can not emit with case v if a sample between indices [0, 10] (entire queue) contains values out of case v range [-4, 3]
            if (index <= 10 && !(-4..=3).contains(&value)) {
                three = false;
            }

            // i8 can not have values out of [-128, 127] range, no need to check case i or ii.
        });

        // Emit according to priority of cases
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
                    let value = value as i16;
                    let value = (value << 1i8) ^ (value >> 7i8); // ZigZag Encoding
                    value.extend_bits(0..8, out);
                }
            }
            return 4;
        // If there are only three samples of i8 to emit
        } else if ten {
            push_header_pad_ten_bits(out);
            for _ in 0..3 {
                if let Some(value) = self.pop() {
                    let value = value as i16;
                    let value = (value << 1i8) ^ (value >> 7i8); // ZigZag Encoding
                    value.extend_bits(0..10, out);
                }
            }
            return 3;
            // If there are only two samples of i8 to emit
        } else if sixteen {
            push_header_pad_sixteen_bits(out);
            for _ in 0..2 {
                if let Some(value) = self.pop() {
                    let value = value as i16;
                    let value = (value << 1i8) ^ (value >> 7i8); // ZigZag Encoding
                    value.extend_bits(0..16, out);
                }
            }
            return 2;
        }
        0
    }
}

pub fn decode_delta_i64(bits: &'_ BitBufferSlice) -> Result<([i64; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i64; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut value: i64 = 0;

    if !bits[0] {
        return Err(CodingError::InvalidBits);
    }

    // Case 1: 00
    if !bits[1] && !bits[2] {
        // Skipping pad 0
        for i in (4..36).step_by(16) {
            for j in 0..16 {
                value |= (bits[i + j] as i64) << j;
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
                value |= (bits[i + j] as i64) << j;
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
                value |= (bits[i + j] as i64) << j;
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
                value |= (bits[i + j] as i64) << j;
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
                value |= (bits[i + j] as i64) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    } else {
        return Err(CodingError::InvalidBits);
    }
    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_i32(bits: &'_ BitBufferSlice) -> Result<([i32; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i32; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut value: i32 = 0;

    if !bits[0] {
        return Err(CodingError::InvalidBits);
    }

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
        return Err(CodingError::InvalidBits);
    }
    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_i16(bits: &'_ BitBufferSlice) -> Result<([i16; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i16; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut value: i16 = 0;

    if !bits[0] {
        return Err(CodingError::InvalidBits);
    }

    // Case 1: 00
    if !bits[1] && !bits[2] {
        // Skipping pad
        let mut value: i32 = value as i32;
        for i in (4..36).step_by(16) {
            for j in 0..16 {
                value |= (bits[i + j] as i32) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value as i16;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 2: 01
    else if !bits[1] && bits[2] {
        // Skipping pad 000
        for i in (6..36).step_by(10) {
            for j in 0..10 {
                value |= (bits[i + j] as i16) << j;
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
                value |= (bits[i + j] as i16) << j;
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
                value |= (bits[i + j] as i16) << j;
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
                value |= (bits[i + j] as i16) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    } else {
        return Err(CodingError::InvalidBits);
    }
    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_i8(bits: &'_ BitBufferSlice) -> Result<([i8; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i8; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut value: i8 = 0;

    if !bits[0] {
        return Err(CodingError::InvalidBits);
    }

    // Case 1: 00
    if !bits[1] && !bits[2] {
        // Skipping pad 0
        let mut value = value as i16;
        for i in (4..36).step_by(16) {
            for j in 0..16 {
                value |= (bits[i + j] as i16) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value as i8;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 2: 01
    else if !bits[1] && bits[2] {
        // Skipping pad 000
        let mut value = value as i16;
        for i in (6..36).step_by(10) {
            for j in 0..10 {
                value |= (bits[i + j] as i16) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value as i8;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 3: 10
    else if bits[1] && !bits[2] {
        // Skipping pad 0
        let mut value = value as i16;
        for i in (4..36).step_by(8) {
            for j in 0..8 {
                value |= (bits[i + j] as i16) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value as i8;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 4: 110
    else if bits[1] && bits[2] && !bits[3] {
        for i in (4..36).step_by(4) {
            for j in 0..4 {
                value |= (bits[i + j] as i8) << j;
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
                value |= (bits[i + j] as i8) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    } else {
        return Err(CodingError::InvalidBits);
    }
    return Ok((decoded_buffer, decoded_buffer_index));
}

/**
Delta Delta Compression
*/
pub trait EmitDeltaDeltaBits<T> {
    /// Emits bits according to the most efficient case of Delta Compression.
    /// Returns the number of elements popped from the queue.
    fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize;
}

impl EmitDeltaDeltaBits<i64> for CompressionQueue<i64, 10> {
    fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let num_values = self.len();
        for _ in 0..num_values {
            if let Some(value) = self.pop() {
                out.push(false);
                if value == 0 {
                    // Write out 00
                    out.push(false);
                    out.push(false);

                    let value = (value << 1i64) ^ (value >> 63i64);
                    out.push(value & (1 << 0) != 0);
                } else if (-16..16).contains(&value) {
                    // Write out 01
                    out.push(false);
                    out.push(true);
                    let value = (value << 1i64) ^ (value >> 63i64);
                    value.extend_bits(0..5, out);
                } else if (-256..=255).contains(&value) {
                    // Write out 10
                    out.push(true);
                    out.push(false);

                    // ZigZag encoding
                    let value = (value << 1i64) ^ (value >> 63i64);

                    value.extend_bits(0..9, out);
                } else if (-16384..=16383).contains(&value) {
                    // Write out 110
                    out.push(true);
                    out.push(true);
                    out.push(false);

                    // ZigZag encoding
                    let value = (value << 1i64) ^ (value >> 63i64);

                    value.extend_bits(0..16, out);
                } else {
                    // Write out 111
                    out.push(true);
                    out.push(true);
                    out.push(true);

                    let value = value as i128;

                    // ZigZag Encoding
                    let value = (value << 1i128) ^ (value >> 127i128);

                    // Write out least significant 64 bits
                    value.extend_bits(0..64, out);
                }
            }
        }
        num_values
    }
}

impl EmitDeltaDeltaBits<i32> for CompressionQueue<i32, 10> {
    fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let num_values = self.len();
        for _ in 0..num_values {
            if let Some(value) = self.pop() {
                out.push(false);
                if value == 0 {
                    // Write out 00
                    out.push(false);
                    out.push(false);

                    let value = (value << 1i32) ^ (value >> 31i32);
                    out.push(value & (1 << 0) != 0);
                } else if (-16..16).contains(&value) {
                    // Write out 01
                    out.push(false);
                    out.push(true);
                    let value = (value << 1i32) ^ (value >> 31i32);
                    value.extend_bits(0..5, out);
                } else if (-256..=255).contains(&value) {
                    // Write out 10
                    out.push(true);
                    out.push(false);

                    // ZigZag encoding
                    let value = (value << 1i32) ^ (value >> 31i32);

                    value.extend_bits(0..9, out);
                } else if (-16384..=16383).contains(&value) {
                    // Write out 110
                    out.push(true);
                    out.push(true);
                    out.push(false);

                    // ZigZag encoding
                    let value = (value << 1i32) ^ (value >> 31i32);

                    value.extend_bits(0..16, out);
                } else {
                    // Write out 111
                    out.push(true);
                    out.push(true);
                    out.push(true);

                    let value = value as i64;

                    // ZigZag Encoding
                    let value = (value << 1i64) ^ (value >> 63i16);

                    // Write out least significant 64 bits
                    value.extend_bits(0..64, out);
                }
            }
        }
        num_values
    }
}

impl EmitDeltaDeltaBits<i16> for CompressionQueue<i16, 10> {
    fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let num_values = self.len();
        for _ in 0..num_values {
            if let Some(value) = self.pop() {
                out.push(false);
                if value == 0 {
                    // Write out 00
                    out.push(false);
                    out.push(false);

                    let value = (value << 1i16) ^ (value >> 15i16);
                    out.push(value & (1 << 0) != 0);
                } else if (-16..16).contains(&value) {
                    // Write out 01
                    out.push(false);
                    out.push(true);
                    let value = (value << 1i16) ^ (value >> 15i16);
                    value.extend_bits(0..5, out);
                } else if (-256..=255).contains(&value) {
                    // Write out 10
                    out.push(true);
                    out.push(false);

                    let value = value as i16;

                    // ZigZag encoding
                    let value = (value << 1i16) ^ (value >> 15i16);

                    value.extend_bits(0..9, out);
                } else if (-16384..=16383).contains(&value) {
                    // Write out 110
                    out.push(true);
                    out.push(true);
                    out.push(false);

                    // ZigZag encoding
                    let value = (value << 1i16) ^ (value >> 15i16);

                    value.extend_bits(0..16, out);
                } else {
                    // Write out 111
                    out.push(true);
                    out.push(true);
                    out.push(true);

                    let value = value as i64;

                    // ZigZag Encoding
                    let value = (value << 1i64) ^ (value >> 63i16);

                    // Write out least significant 64 bits
                    value.extend_bits(0..64, out);
                }
            }
        }
        num_values
    }
}

impl EmitDeltaDeltaBits<i8> for CompressionQueue<i8, 10> {
    fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let num_values = self.len();
        for _ in 0..num_values {
            if let Some(value) = self.pop() {
                out.push(false);
                if value == 0 {
                    // Write out 00
                    out.push(false);
                    out.push(false);

                    let value = (value << 1i8) ^ (value >> 7i8);
                    out.push(value & (1 << 0) != 0);
                } else if (-16..16).contains(&value) {
                    // Write out 01
                    out.push(false);
                    out.push(true);
                    let value = (value << 1i8) ^ (value >> 7i8);
                    value.extend_bits(0..5, out);
                } else if (-128..=127).contains(&value) {
                    // Write out 10
                    out.push(true);
                    out.push(false);

                    let value = value as i16;

                    // ZigZag encoding
                    let value = (value << 1i16) ^ (value >> 15i16);

                    value.extend_bits(0..9, out);
                }
            }
        }
        num_values
    }
}

pub fn decode_delta_delta_i8(bits: &'_ BitBufferSlice) -> Result<([i8; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i8; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut idx = 0;
    while idx < bits.len() {
        if bits[idx] {
            return Err(CodingError::InvalidBits);
        }
        let mut value = 0;
        if !bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            value = bits[idx] as i8;
            let value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 1;
        } else if !bits[idx + 1] && bits[idx + 2] {
            idx += 3;
            for i in 0..5 {
                value |= (bits[idx + i] as i8) << i;
            }
            let value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 5;
        } else if bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            let mut value: i16 = 0;
            for i in 0..9 {
                value |= (bits[idx + i] as i16) << i;
            }
            // ZigZag decoding
            let value = (value >> 1) ^ -(value & 1);

            decoded_buffer[decoded_buffer_index] = value as i8;
            decoded_buffer_index += 1;
            idx += 9;
        } else {
            return Err(CodingError::InvalidBits);
        }
    }
    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_delta_i16(bits: &'_ BitBufferSlice) -> Result<([i16; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i16; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut idx = 0;
    while idx < bits.len() {
        if bits[idx] {
            return Err(CodingError::InvalidBits);
        }
        let mut value = 0;
        if !bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            value = bits[idx] as i16;
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 1;
        } else if !bits[idx + 1] && bits[idx + 2] {
            idx += 3;
            for i in 0..5 {
                value |= (bits[idx + i] as i16) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 5;
        } else if bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            for i in 0..9 {
                value |= (bits[idx + i] as i16) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 9;
        } else if bits[idx + 1] && bits[idx + 2] && !bits[idx + 3] {
            idx += 4;
            for i in 0..16 {
                value |= (bits[idx + i] as i16) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 16;
        } else if bits[idx + 1] && bits[idx + 2] && bits[idx + 3] {
            idx += 4;
            let mut value: i64 = 0;
            for i in 0..64 {
                value |= (bits[idx + i] as i64) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value as i16;
            decoded_buffer_index += 1;
            idx += 64;
        } else {
            return Err(CodingError::InvalidBits);
        }
    }

    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_delta_i32(bits: &'_ BitBufferSlice) -> Result<([i32; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i32; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut idx = 0;
    while idx < bits.len() {
        if bits[idx] {
            return Err(CodingError::InvalidBits);
        }
        let mut value = 0;
        if !bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            value = bits[idx] as i32;
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 1;
        } else if !bits[idx + 1] && bits[idx + 2] {
            idx += 3;
            for i in 0..5 {
                value |= (bits[idx + i] as i32) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 5;
        } else if bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            for i in 0..9 {
                value |= (bits[idx + i] as i32) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 9;
        } else if bits[idx + 1] && bits[idx + 2] && !bits[idx + 3] {
            idx += 4;
            for i in 0..16 {
                value |= (bits[idx + i] as i32) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 16;
        } else if bits[idx + 1] && bits[idx + 2] && bits[idx + 3] {
            idx += 4;
            let mut value: i64 = 0;
            for i in 0..64 {
                value |= (bits[idx + i] as i64) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value as i32;
            decoded_buffer_index += 1;
            idx += 64;
        } else {
            return Err(CodingError::InvalidBits);
        }
    }

    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_delta_i64(bits: &'_ BitBufferSlice) -> Result<([i64; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i64; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut idx = 0;
    while idx < bits.len() {
        if bits[idx] {
            return Err(CodingError::InvalidBits);
        }
        let mut value = 0;
        if !bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            value = bits[idx] as i64;
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 1;
        } else if !bits[idx + 1] && bits[idx + 2] {
            idx += 3;
            for i in 0..5 {
                value |= (bits[idx + i] as i64) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 5;
        } else if bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            for i in 0..9 {
                value |= (bits[idx + i] as i64) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 9;
        } else if bits[idx + 1] && bits[idx + 2] && !bits[idx + 3] {
            idx += 4;
            for i in 0..16 {
                value |= (bits[idx + i] as i64) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 16;
        } else if bits[idx + 1] && bits[idx + 2] && bits[idx + 3] {
            idx += 4;
            let mut value: i128 = 0;
            for i in 0..64 {
                value |= (bits[idx + i] as i128) << i;
            }
            let value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value as i64;
            decoded_buffer_index += 1;
            idx += 64;
        } else {
            return Err(CodingError::InvalidBits);
        }
    }
    return Ok((decoded_buffer, decoded_buffer_index));
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
