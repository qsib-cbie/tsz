use num_traits::{One, PrimInt};

use crate::prelude::*;
use core::ops::Range;

use super::halfvec::{HalfVec, HalfWord};

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

trait Bits: PrimInt {
    const BITS: usize;

    fn zigzag_bits(self, mask: Self) -> Self {
        // ZigZag Encoding
        let z = (self << 1) ^ (self >> Self::BITS - 1);
        // Bottom bits
        z & mask
    }
}

impl Bits for i16 {
    const BITS: usize = 16;
}

impl Bits for i32 {
    const BITS: usize = 32;
}

#[inline(always)]
unsafe fn push_three_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
    // Push a header nibble
    buf.push(HalfWord::Half(0b1111));

    // Prepare the rest of the header
    let mut word: u32 = 0b00 << 3;

    // Take the bottom 3 bits
    let mask = T::from(0b111).unwrap_unchecked();
    let values = q.pop_n::<10>().unwrap_unchecked();
    for i in 0..9 {
        word |= values[i].zigzag_bits(mask).to_u32().unwrap_unchecked();
        word <<= 3;
    }
    word |= values[9].zigzag_bits(mask).to_u32().unwrap_unchecked();

    // Push a full 32 bit word
    buf.push(HalfWord::Full(word));
}

#[inline(always)]
unsafe fn push_four_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
    buf.push(HalfWord::Half(0b1110));
    let mut word: u32 = 0;
    let mask = T::from(0b1111).unwrap_unchecked();
    let values = q.pop_n::<8>().unwrap_unchecked();
    for i in 0..8 {
        word |= values[i].zigzag_bits(mask).to_u32().unwrap_unchecked();
        word <<= 3;
    }
    word |= values[8].zigzag_bits(mask).to_u32().unwrap_unchecked();

    // Push a full 32 bit word
    buf.push(HalfWord::Full(word));
}

#[inline]
fn push_header_pad_eight_bits(buf: &mut BitBuffer) {
    // for _ in 0..2 {
    //     buf.push(true);
    // }
    // for _ in 0..2 {
    //     buf.push(false);
    // }
    let bits = BitBufferSlice::from_element(&0b0011);
    buf.extend_from_bitslice(&bits[..4]);
}

#[inline]
fn push_header_pad_ten_bits(buf: &mut BitBuffer) {
    // buf.push(true);
    // buf.push(false);
    // buf.push(true);
    // for _ in 0..3 {
    //     buf.push(false);
    // }
    let bits = BitBufferSlice::from_element(&0b000101);
    buf.extend_from_bitslice(&bits[..6]);
}

#[inline]
fn push_header_pad_sixteen_bits(buf: &mut BitBuffer) {
    // buf.push(true);
    // for _ in 0..3 {
    //     buf.push(false);
    // }
    let bits = BitBufferSlice::from_element(&0b0001);
    buf.extend_from_bitslice(&bits[..4]);
}

#[inline]
fn push_header_pad_thirty_two_bits(buf: &mut BitBuffer) {
    // buf.push(true);
    // buf.push(false);
    // for _ in 0..3 {
    //     buf.push(true);
    // }
    // buf.push(false);
    let bits = BitBufferSlice::from_element(&0b011101);
    buf.extend_from_bitslice(&bits[..6]);
}

#[inline]
fn push_header_pad_sixty_five_bits(buf: &mut BitBuffer) {
    // buf.push(true);
    // buf.push(false);
    // for _ in 0..4 {
    //     buf.push(true);
    // }
    let bits = BitBufferSlice::from_element(&0b111101);
    buf.extend_from_bitslice(&bits[..6]);
}

pub trait EmitDeltaBits<T> {
    /// Emits bits according to the most efficient case of Delta Compression.
    /// Returns the number of elements popped from the queue.
    fn emit_delta_bits(&mut self, out: &mut HalfVec, flush: bool) -> usize;
}

impl EmitDeltaBits<i32> for CompressionQueue<i32, 10> {
    #[allow(unused)]
    fn emit_delta_bits(&mut self, out: &mut HalfVec, flush: bool) -> usize {
        let queue_length = self.len();
        let mut fits = [true; 7];

        // Check flush conditions
        if flush {
            // Can not emit with any case of delta compression if queue is empty
            if self.is_empty() {
                return 0;
            }

            // Can not emit with case v of delta compression if number of samples < 10
            if self.len() < 10 {
                fits[0] = false;
            }

            // Can not emit with case iv of delta compression if number of samples < 8.
            if self.len() < 8 {
                fits[1] = false;
            }

            // Can not emit with case iii of delta compression if number of samples < 4
            if self.len() < 4 {
                fits[2] = false;
            }
            // Can not emit with case ii of delta compression if number of samples < 3
            if self.len() < 3 {
                fits[3] = false;
            }
            // Can not emit with case ii of delta compression if number of samples < 2
            if self.len() < 2 {
                fits[4] = false;
            }
        }

        for (index, value) in self.iter().enumerate() {
            if (index < 2 && !(-32768..=32767).contains(&value)) {
                fits[..5].iter_mut().for_each(|x| *x = false);
                break;
            }
            if (index < 3 && !(-512..=511).contains(&value)) {
                fits[..4].iter_mut().for_each(|x| *x = false);
                break;
            }
            if (index < 4 && !(-128..=127).contains(&value)) {
                fits[..3].iter_mut().for_each(|x| *x = false);
                break;
            }
            if (index < 8 && !(-8..=7).contains(&value)) {
                fits[..2].iter_mut().for_each(|x| *x = false);
                break;
            }
            if (index < 10 && !(-4..=3).contains(&value)) {
                fits[..1].iter_mut().for_each(|x| *x = false);
                break;
            }
        }

        // Emit according to priority of cases
        if fits[0] {
            unsafe {
                push_three_bits(self, out);
            }
            return 10;
        } else if fits[1] {
            unsafe {
                push_four_bits(self, out);
            }

            return 8;
        } else if fits[2] {
            push_header_pad_eight_bits(out);
            for _ in 0..4 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32); // ZigZag Encoding
                    value.extend_bits(0..8, out);
                }
            }
            return 4;
        } else if fits[3] {
            push_header_pad_ten_bits(out);
            for _ in 0..3 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32); // ZigZag Encoding
                    value.extend_bits(0..10, out);
                }
            }
            return 3;
        } else if fits[4] {
            push_header_pad_sixteen_bits(out);
            for _ in 0..2 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32); // ZigZag Encoding
                    value.extend_bits(0..16, out);
                }
            }
            return 2;
        } else if fits[5] {
            push_header_pad_thirty_two_bits(out);
            for _ in 0..1 {
                if let Some(value) = self.pop() {
                    let value = value as i32;
                    let value = (value << 1i32) ^ (value >> 31i32); // ZigZag Encoding
                    value.extend_bits(0..32, out);
                }
            }
            return 1;
        }
        0
    }
}

impl EmitDeltaBits<i16> for CompressionQueue<i16, 10> {
    #[allow(unused)]
    fn emit_delta_bits(&mut self, out: &mut HalfVec, flush: bool) -> usize {
        let queue_length = self.len();
        let mut fits = [true; 7];

        // Check flush conditions
        if flush {
            // Can not emit with any case of delta compression if queue is empty
            if self.is_empty() {
                return 0;
            }

            // Can not emit with case v of delta compression if number of samples < 10
            if self.len() < 10 {
                fits[0] = false;
            }

            // Can not emit with case iv of delta compression if number of samples < 8.
            if self.len() < 8 {
                fits[1] = false;
            }

            // Can not emit with case iii of delta compression if number of samples < 4
            if self.len() < 4 {
                fits[2] = false;
            }
            // Can not emit with case ii of delta compression if number of samples < 3
            if self.len() < 3 {
                fits[3] = false;
            }
            // Can not emit with case ii of delta compression if number of samples < 2
            if self.len() < 2 {
                fits[4] = false;
            }
        }

        for (index, value) in self.iter().enumerate() {
            if (index < 3 && !(-512..=511).contains(&value)) {
                fits[..4].iter_mut().for_each(|x| *x = false);
                break;
            }
            if (index < 4 && !(-128..=127).contains(&value)) {
                fits[..3].iter_mut().for_each(|x| *x = false);
                break;
            }
            if (index < 8 && !(-8..=7).contains(&value)) {
                fits[..2].iter_mut().for_each(|x| *x = false);
                break;
            }
            if (index < 10 && !(-4..=3).contains(&value)) {
                fits[..1].iter_mut().for_each(|x| *x = false);
                break;
            }
        }

        // Emit according to priority of cases
        if fits[0] {
            push_header_pad_three_bits(out);
            for _ in 0..10 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16); // ZigZag Encoding
                    value.extend_bits(0..3, out);
                }
            }
            return 10;
        } else if fits[1] {
            push_header_pad_four_bits(out);
            for _ in 0..8 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16); // ZigZag Encoding
                    value.extend_bits(0..4, out);
                }
            }
            return 8;
        } else if fits[2] {
            push_header_pad_eight_bits(out);
            for _ in 0..4 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16); // ZigZag Encoding
                    value.extend_bits(0..8, out);
                }
            }
            return 4;
        } else if fits[3] {
            push_header_pad_ten_bits(out);
            for _ in 0..3 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16); // ZigZag Encoding
                    value.extend_bits(0..10, out);
                }
            }
            return 3;
        } else if fits[4] {
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

// impl EmitDeltaBits<i8> for CompressionQueue<i8, 10> {
//     #[allow(unused)]
//     fn emit_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
//         let queue_length = self.len();
//         let mut fits = [true; 3];

//         // Check flush conditions
//         if flush {
//             // Can not emit with any case of delta compression if queue is empty
//             if self.is_empty() {
//                 return 0;
//             }

//             // Can not emit with case v of delta compression if number of samples < 10
//             if self.len() < 10 {
//                 fits[0] = false;
//             }

//             // Can not emit with case iv of delta compression if number of samples < 8.
//             if self.len() < 8 {
//                 fits[1] = false;
//             }

//             // Can not emit with case iii of delta compression if number of samples < 4
//             if self.len() < 4 {
//                 fits[2] = false;
//             }
//         }

//         for (index, value) in self.iter().enumerate() {
//             if (index < 4 && !(-128..=127).contains(&value)) {
//                 fits[..3].iter_mut().for_each(|x| *x = false);
//                 break;
//             }
//             if (index < 8 && !(-8..=7).contains(&value)) {
//                 fits[..2].iter_mut().for_each(|x| *x = false);
//                 break;
//             }
//             if (index < 10 && !(-4..=3).contains(&value)) {
//                 fits[..1].iter_mut().for_each(|x| *x = false);
//                 break;
//             }
//         }

//         // Emit according to priority of cases
//         if fits[0] {
//             push_header_pad_three_bits(out);
//             for _ in 0..10 {
//                 if let Some(value) = self.pop() {
//                     let value = (value << 1i8) ^ (value >> 7i8); // ZigZag Encoding
//                     value.extend_bits(0..3, out);
//                 }
//             }
//             return 10;
//         } else if fits[1] {
//             push_header_pad_four_bits(out);
//             for _ in 0..8 {
//                 if let Some(value) = self.pop() {
//                     let value = (value << 1i8) ^ (value >> 7i8); // ZigZag Encoding
//                     value.extend_bits(0..4, out);
//                 }
//             }
//             return 8;
//         } else if fits[2] {
//             push_header_pad_eight_bits(out);
//             for _ in 0..4 {
//                 if let Some(value) = self.pop() {
//                     let value = value as i16;
//                     let value = (value << 1i8) ^ (value >> 7i8); // ZigZag Encoding
//                     value.extend_bits(0..8, out);
//                 }
//             }
//             return 4;
//         }
//         0
//     }
// }

// Delta-Delta Encoding
pub trait EmitDeltaDeltaBits<T> {
    /// Emits bits according to the most efficient case of Delta Compression.
    /// Returns the number of elements popped from the queue.
    fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize;
}

// impl EmitDeltaDeltaBits<i128> for CompressionQueue<i128, 10> {
//     fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
//         let num_values = if flush { self.len() } else { 10 };
//         for _ in 0..num_values {
//             if let Some(value) = self.pop() {
//                 out.push(false);
//                 if value == 0 {
//                     // Write out 00
//                     out.push(false);
//                     out.push(false);

//                     let value = (value << 1i64) ^ (value >> 63i64);
//                     out.push(value & (1 << 0) != 0);
//                 } else if (-16..16).contains(&value) {
//                     // Write out 01
//                     out.push(false);
//                     out.push(true);
//                     let value = (value << 1i64) ^ (value >> 63i64);
//                     value.extend_bits(0..5, out);
//                 } else if (-256..=255).contains(&value) {
//                     // Write out 10
//                     out.push(true);
//                     out.push(false);

//                     // ZigZag encoding
//                     let value = (value << 1i64) ^ (value >> 63i64);
//                     value.extend_bits(0..9, out);
//                 } else if (-16384..=16383).contains(&value) {
//                     // Write out 110
//                     out.push(true);
//                     out.push(true);
//                     out.push(false);

//                     // ZigZag encoding
//                     let value = (value << 1i64) ^ (value >> 63i64);

//                     value.extend_bits(0..16, out);
//                 } else {
//                     // Write out 111
//                     out.push(true);
//                     out.push(true);
//                     out.push(true);

//                     let value = value as i128;

//                     // ZigZag Encoding
//                     let value = (value << 1i128) ^ (value >> 127i128);

//                     // Write out least significant 64 bits
//                     value.extend_bits(0..64, out);
//                 }
//             }
//         }
//         num_values
//     }
// }

// impl EmitDeltaDeltaBits<i64> for CompressionQueue<i64, 10> {
//     fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
//         let num_values = if flush { self.len() } else { 10 };
//         for _ in 0..num_values {
//             if let Some(value) = self.pop() {
//                 out.push(false);
//                 if value == 0 {
//                     // Write out 00
//                     out.push(false);
//                     out.push(false);

//                     let value = (value << 1i64) ^ (value >> 63i64);
//                     out.push(value & (1 << 0) != 0);
//                 } else if (-16..16).contains(&value) {
//                     // Write out 01
//                     out.push(false);
//                     out.push(true);
//                     let value = (value << 1i64) ^ (value >> 63i64);
//                     value.extend_bits(0..5, out);
//                 } else if (-256..=255).contains(&value) {
//                     // Write out 10
//                     out.push(true);
//                     out.push(false);

//                     // ZigZag encoding
//                     let value = (value << 1i64) ^ (value >> 63i64);

//                     value.extend_bits(0..9, out);
//                 } else if (-16384..=16383).contains(&value) {
//                     // Write out 110
//                     out.push(true);
//                     out.push(true);
//                     out.push(false);

//                     // ZigZag encoding
//                     let value = (value << 1i64) ^ (value >> 63i64);

//                     value.extend_bits(0..16, out);
//                 } else {
//                     // Write out 111
//                     out.push(true);
//                     out.push(true);
//                     out.push(true);

//                     let value = value as i128;

//                     // ZigZag Encoding
//                     let value = (value << 1i128) ^ (value >> 127i128);

//                     // Write out least significant 64 bits
//                     value.extend_bits(0..64, out);
//                 }
//             }
//         }
//         num_values
//     }
// }

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

// impl EmitDeltaDeltaBits<i8> for CompressionQueue<i8, 10> {
//     fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
//         let num_values = if flush { self.len() } else { 10 };
//         for _ in 0..num_values {
//             if let Some(value) = self.pop() {
//                 out.push(false);
//                 if value == 0 {
//                     // Write out 00
//                     out.push(false);
//                     out.push(false);

//                     let value = (value << 1i8) ^ (value >> 7i8);
//                     out.push(value & (1 << 0) != 0);
//                 } else if (-16..16).contains(&value) {
//                     // Write out 01
//                     out.push(false);
//                     out.push(true);
//                     let value = (value << 1i8) ^ (value >> 7i8);
//                     value.extend_bits(0..5, out);
//                 } else if (-128..=127).contains(&value) {
//                     // Write out 10
//                     out.push(true);
//                     out.push(false);

//                     let value = value as i16;

//                     // ZigZag encoding
//                     let value = (value << 1i16) ^ (value >> 15i16);

//                     value.extend_bits(0..9, out);
//                 }
//             }
//         }
//         num_values
//     }
// }
