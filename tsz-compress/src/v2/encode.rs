use num_traits::PrimInt;

use crate::prelude::*;

use super::halfvec::{HalfVec, HalfWord};

trait Bits: PrimInt {
    const BITS: usize;

    fn zigzag_bits(self) -> Self {
        // ZigZag Encoding
        let z = (self << 1) ^ (self >> Self::BITS - 1);
        z
    }

    fn zigzag_bit_masked(self, mask: Self) -> Self {
        // Mask bottom bits
        self.zigzag_bits() & mask
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
        word |= values[i]
            .zigzag_bit_masked(mask)
            .to_u32()
            .unwrap_unchecked();
        word <<= 3;
    }
    word |= values[9]
        .zigzag_bit_masked(mask)
        .to_u32()
        .unwrap_unchecked();

    // Push a full 32 bit word
    buf.push(HalfWord::Full(word));
}

#[inline(always)]
unsafe fn push_four_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
    const N: usize = 8;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(0b1110));
    let mut word: u32 = 0;
    let mask = T::from(0b1111).unwrap_unchecked();
    let values = q.pop_n::<N>().unwrap_unchecked();
    for i in 0..N1 {
        word |= values[i]
            .zigzag_bit_masked(mask)
            .to_u32()
            .unwrap_unchecked();
        word <<= 4;
    }
    word |= values[N1]
        .zigzag_bit_masked(mask)
        .to_u32()
        .unwrap_unchecked();
    buf.push(HalfWord::Full(word));
}

#[inline(always)]
unsafe fn push_eight_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
    const N: usize = 4;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(0b1100));
    let mut word: u32 = 0;
    let mask = T::from(0b11111111).unwrap_unchecked();
    let values = q.pop_n::<N>().unwrap_unchecked();
    for i in 0..N1 {
        word |= values[i]
            .zigzag_bit_masked(mask)
            .to_u32()
            .unwrap_unchecked();
        word <<= 8;
    }
    word |= values[N1]
        .zigzag_bit_masked(mask)
        .to_u32()
        .unwrap_unchecked();
    buf.push(HalfWord::Full(word));
}

#[inline(always)]
unsafe fn push_ten_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
    const N: usize = 3;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(0b1010));
    let mut word: u32 = 0b00 << 10;
    let mask = T::from(0b1111111111).unwrap_unchecked();
    let values = q.pop_n::<N>().unwrap_unchecked();
    for i in 0..N1 {
        word |= values[i]
            .zigzag_bit_masked(mask)
            .to_u32()
            .unwrap_unchecked();
        word <<= 10;
    }
    word |= values[N1]
        .zigzag_bit_masked(mask)
        .to_u32()
        .unwrap_unchecked();
    buf.push(HalfWord::Full(word));
}

#[inline(always)]
unsafe fn push_sixteen_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
    const N: usize = 2;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(0b1000));
    let mut word: u32 = 0b00 << 10;
    let mask = T::from(0xffff).unwrap_unchecked();
    let values = q.pop_n::<N>().unwrap_unchecked();
    for i in 0..N1 {
        word |= values[i]
            .zigzag_bit_masked(mask)
            .to_u32()
            .unwrap_unchecked();
        word <<= 16;
    }
    word |= values[N1]
        .zigzag_bit_masked(mask)
        .to_u32()
        .unwrap_unchecked();
    buf.push(HalfWord::Full(word));
}

#[inline(always)]
unsafe fn push_thirty_two_bits<T: PrimInt + Bits>(
    q: &mut CompressionQueue<T, 10>,
    buf: &mut HalfVec,
) {
    buf.push(HalfWord::Half(0b1011));
    let value = q.pop().unwrap_unchecked();
    let word = value.zigzag_bits().to_u32().unwrap_unchecked();
    buf.push(HalfWord::Full(word));
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
        let mut fits = [true; 6];

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
            unsafe {
                push_eight_bits(self, out);
            }
            return 4;
        } else if fits[3] {
            unsafe {
                push_ten_bits(self, out);
            }
            return 3;
        } else if fits[4] {
            unsafe {
                push_sixteen_bits(self, out);
            }
            return 2;
        } else if fits[5] {
            unsafe {
                push_thirty_two_bits(self, out);
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
        let mut fits = [true; 5];

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
            unsafe {
                push_eight_bits(self, out);
            }
            return 4;
        } else if fits[3] {
            unsafe {
                push_ten_bits(self, out);
            }
            return 3;
        } else if fits[4] {
            unsafe {
                push_sixteen_bits(self, out);
            }
            return 2;
        }
        0
    }
}

// Delta-Delta Encoding
// pub trait EmitDeltaDeltaBits<T> {
//     /// Emits bits according to the most efficient case of Delta Compression.
//     /// Returns the number of elements popped from the queue.
//     fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize;
// }

// impl EmitDeltaDeltaBits<i32> for CompressionQueue<i32, 10> {
//     fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
//         let num_values = if flush { self.len() } else { 10 };
//         for _ in 0..num_values {
//             if let Some(value) = self.pop() {
//                 out.push(false);
//                 if value == 0 {
//                     // Write out 00
//                     out.push(false);
//                     out.push(false);

//                     let value = (value << 1i32) ^ (value >> 31i32);
//                     out.push(value & (1 << 0) != 0);
//                 } else if (-16..16).contains(&value) {
//                     // Write out 01
//                     out.push(false);
//                     out.push(true);
//                     let value = (value << 1i32) ^ (value >> 31i32);
//                     value.extend_bits(0..5, out);
//                 } else if (-256..=255).contains(&value) {
//                     // Write out 10
//                     out.push(true);
//                     out.push(false);

//                     // ZigZag encoding
//                     let value = (value << 1i32) ^ (value >> 31i32);

//                     value.extend_bits(0..9, out);
//                 } else if (-16384..=16383).contains(&value) {
//                     // Write out 110
//                     out.push(true);
//                     out.push(true);
//                     out.push(false);

//                     // ZigZag encoding
//                     let value = (value << 1i32) ^ (value >> 31i32);

//                     value.extend_bits(0..16, out);
//                 } else {
//                     // Write out 111
//                     out.push(true);
//                     out.push(true);
//                     out.push(true);

//                     let value = value as i64;

//                     // ZigZag Encoding
//                     let value = (value << 1i64) ^ (value >> 63i16);

//                     // Write out least significant 64 bits
//                     value.extend_bits(0..64, out);
//                 }
//             }
//         }
//         num_values
//     }
// }

// impl EmitDeltaDeltaBits<i16> for CompressionQueue<i16, 10> {
//     fn emit_delta_delta_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
//         let num_values = if flush { self.len() } else { 10 };
//         for _ in 0..num_values {
//             if let Some(value) = self.pop() {
//                 out.push(false);
//                 if value == 0 {
//                     // Write out 00
//                     out.push(false);
//                     out.push(false);

//                     let value = (value << 1i16) ^ (value >> 15i16);
//                     out.push(value & (1 << 0) != 0);
//                 } else if (-16..16).contains(&value) {
//                     // Write out 01
//                     out.push(false);
//                     out.push(true);
//                     let value = (value << 1i16) ^ (value >> 15i16);
//                     value.extend_bits(0..5, out);
//                 } else if (-256..=255).contains(&value) {
//                     // Write out 10
//                     out.push(true);
//                     out.push(false);

//                     let value = value as i16;

//                     // ZigZag encoding
//                     let value = (value << 1i16) ^ (value >> 15i16);

//                     value.extend_bits(0..9, out);
//                 } else if (-16384..=16383).contains(&value) {
//                     // Write out 110
//                     out.push(true);
//                     out.push(true);
//                     out.push(false);

//                     // ZigZag encoding
//                     let value = (value << 1i16) ^ (value >> 15i16);

//                     value.extend_bits(0..16, out);
//                 } else {
//                     // Write out 111
//                     out.push(true);
//                     out.push(true);
//                     out.push(true);

//                     let value = value as i64;

//                     // ZigZag Encoding
//                     let value = (value << 1i64) ^ (value >> 63i16);

//                     // Write out least significant 64 bits
//                     value.extend_bits(0..64, out);
//                 }
//             }
//         }
//         num_values
//     }
// }
