use crate::prelude::*;
use core::ops::Range;

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

// Delta Encoding
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

        if queue_length == 1 {
            let num_emitted = self.emit_delta_delta_bits(out, flush);
            return num_emitted;
        }

        // Check flush conditions
        if flush {
            // Can not emit with any case of delta compression if queue is empty
            if self.is_empty() {
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
        } else {
            let num_emitted = self.emit_delta_delta_bits(out, flush);
            return num_emitted;
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

        if queue_length == 1 {
            let num_emitted = self.emit_delta_delta_bits(out, flush);
            return num_emitted;
        }

        // Check flush conditions
        if flush {
            // Can not emit with any case of delta compression if queue is empty
            if self.is_empty() {
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
        } else {
            let num_emitted = self.emit_delta_delta_bits(out, flush);
            return num_emitted;
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

        if queue_length == 1 {
            let num_emitted = self.emit_delta_delta_bits(out, flush);
            return num_emitted;
        }

        // Check flush conditions
        if flush {
            // Can not emit with any case of delta compression if queue is empty.
            if self.is_empty() {
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
        } else {
            let num_emitted = self.emit_delta_delta_bits(out, flush);
            return num_emitted;
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

        extern crate std;
        use std::{print, println};

        if queue_length == 1 {
            let num_emitted = self.emit_delta_delta_bits(out, flush);
            return num_emitted;
        }

        // Check flush conditions
        if flush {
            // Can not emit with any case of delta compression if queue is empty
            if self.is_empty() {
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
        } else {
            let num_emitted = self.emit_delta_delta_bits(out, flush);
            return num_emitted;
        }
        0
    }
}

// Delta-Delta Encoding
pub trait EmitDeltaDeltaBits<T> {
    /// Emits bits according to the most efficient case of Delta Compression.
    /// Returns the number of elements popped from the queue.
    fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize;
}

impl EmitDeltaDeltaBits<i64> for CompressionQueue<i64, 10> {
    fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let num_values = if flush { self.len() } else { 10 };
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
        let num_values = if flush { self.len() } else { 10 };
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
        let num_values = if flush { self.len() } else { 10 };
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
        let num_values = if flush { self.len() } else { 10 };
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
