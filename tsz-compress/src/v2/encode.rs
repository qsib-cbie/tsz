use core::fmt::Binary;
use num_traits::PrimInt;

use crate::prelude::*;
use crate::v2::consts::headers;

use super::halfvec::{HalfVec, HalfWord};

pub trait Bits: PrimInt + Binary {
    const BITS: usize;

    /// Language limitations prevent us from writing simple math expressions
    /// ((self << 1) ^ (self >> Self::BITS - 1)) as u32
    fn zigzag(self) -> usize;

    /// Return the zigzag encoding and number of bits required to represent the value
    #[inline(always)]
    fn zigzag_bits(self) -> (usize, usize) {
        let zbits = self.zigzag();
        (zbits, (usize::BITS - zbits.leading_zeros()) as usize)
    }
}

impl Bits for i8 {
    const BITS: usize = 8;

    #[inline(always)]
    fn zigzag(self) -> usize {
        ((self << 1) ^ (self >> Self::BITS - 1)) as u8 as usize
    }
}

impl Bits for i16 {
    const BITS: usize = 16;

    #[inline(always)]
    fn zigzag(self) -> usize {
        ((self << 1) ^ (self >> Self::BITS - 1)) as u16 as usize
    }
}

impl Bits for i32 {
    const BITS: usize = 32;

    #[inline(always)]
    fn zigzag(self) -> usize {
        ((self << 1) ^ (self >> Self::BITS - 1)) as u32 as usize
    }
}

#[cfg(target_pointer_width = "64")]
impl Bits for i64 {
    const BITS: usize = 64;

    #[inline(always)]
    fn zigzag(self) -> usize {
        ((self << 1) ^ (self >> Self::BITS - 1)) as u64 as usize
    }
}

#[inline(always)]
fn push_three_bits(q: &mut CompressionQueue<10>, buf: &mut HalfVec) {
    const N: usize = 10;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(headers::THREE_BITS_TEN_SAMPLES));
    let mut word: usize = 0;
    let values = q.pop_n::<N>();
    for i in 0..N1 {
        word |= values[i];
        word <<= 3;
    }
    word |= values[N1];
    buf.push(HalfWord::Full(word as u32));
}

#[inline(always)]
fn push_six_bits(q: &mut CompressionQueue<10>, buf: &mut HalfVec) {
    const N: usize = 5;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(headers::SIX_BITS_FIVE_SAMPLES));
    let mut word: usize = 0;
    let values = q.pop_n::<N>();
    for i in 0..N1 {
        word |= values[i];
        word <<= 6;
    }
    word |= values[N1];
    buf.push(HalfWord::Full(word as u32));
}

#[inline(always)]
fn push_eight_bits(q: &mut CompressionQueue<10>, buf: &mut HalfVec) {
    const N: usize = 4;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(headers::EIGHT_BITS_FOUR_SAMPLES));
    let mut word: usize = 0;
    let values = q.pop_n::<N>();
    for i in 0..N1 {
        word |= values[i];
        word <<= 8;
    }
    word |= values[N1];
    buf.push(HalfWord::Full(word as u32));
}

#[inline(always)]
fn push_ten_bits(q: &mut CompressionQueue<10>, buf: &mut HalfVec) {
    const N: usize = 3;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(headers::TEN_BITS_THREE_SAMPLES));
    let mut word: usize = 0b00 << 10;
    let values = q.pop_n::<N>();
    for i in 0..N1 {
        word |= values[i];
        word <<= 10;
    }
    word |= values[N1];
    buf.push(HalfWord::Full(word as u32));
}

#[inline(always)]
fn push_sixteen_bits(q: &mut CompressionQueue<10>, buf: &mut HalfVec) {
    const N: usize = 2;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(headers::SIXTEEN_BITS_TWO_SAMPLES));
    let mut word: usize = 0b00 << 10;
    let values = q.pop_n::<N>();
    for i in 0..N1 {
        word |= values[i];
        word <<= 16;
    }
    word |= values[N1];
    buf.push(HalfWord::Full(word as u32));
}

#[inline(always)]
unsafe fn push_32_or_64_bits(q: &mut CompressionQueue<10>, buf: &mut HalfVec) {
    let value = q.pop().unwrap_unchecked();
    if value <= u32::MAX as usize {
        buf.push(HalfWord::Half(headers::THIRTY_TWO_BITS_ONE_SAMPLE));
    } else {
        buf.push(HalfWord::Half(headers::SIXTY_FOUR_BITS_ONE_SAMPLE));
        buf.push(HalfWord::Full((value >> 32) as u32));
    }
    buf.push(HalfWord::Full(value as u32));
}

pub trait EmitDeltaBits {
    /// Emits bits according to the most efficient case of Delta Compression.
    /// Returns the number of elements popped from the queue.
    fn emit_delta_bits(&mut self, out: &mut HalfVec) -> usize;
    fn flush_delta_bits(&mut self, out: &mut HalfVec) -> usize;
}

impl EmitDeltaBits for CompressionQueue<10> {
    #[inline(always)]
    fn emit_delta_bits(&mut self, out: &mut HalfVec) -> usize {
        let mut fits = [true; 5];

        // Check if the values will fit in the cases
        let values = self.peak_bitcounts::<10>();
        for (index, bits_required) in values.into_iter().enumerate() {
            if (index < 2) & (bits_required > 16) {
                fits[4] = false;
            }
            if (index < 3) & (bits_required > 10) {
                fits[3] = false;
            }
            if (index < 4) & (bits_required > 8) {
                fits[2] = false;
            }
            if (index < 5) & (bits_required > 6) {
                fits[1] = false;
            }
            if (index < 10) & (bits_required > 3) {
                fits[0] = false;
            }
        }

        // Emit according to priority of cases
        if fits[0] {
            push_three_bits(self, out);
            return 10;
        } else if fits[1] {
            push_six_bits(self, out);
            return 5;
        } else if fits[2] {
            push_eight_bits(self, out);
            return 4;
        } else if fits[3] {
            push_ten_bits(self, out);
            return 3;
        } else if fits[4] {
            push_sixteen_bits(self, out);
            return 2;
        } else {
            unsafe {
                push_32_or_64_bits(self, out);
            }
            return 1;
        }
    }

    #[inline(always)]
    fn flush_delta_bits(&mut self, out: &mut HalfVec) -> usize {
        let mut fits = [true; 5];

        // Can not emit with any case of delta compression if queue is empty
        if self.is_empty() {
            return 0;
        }

        // Can not emit with case v of delta compression if number of samples < 10
        if self.len() < 10 {
            fits[0] = false;
        }

        // Can not emit with case iv of delta compression if number of samples < 5.
        if self.len() < 5 {
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

        // Check if the values will fit in the cases
        let values = self.peak_bitcounts::<10>();
        for (index, bits_required) in values.into_iter().enumerate() {
            if (index < 2) & (bits_required > 16) {
                fits[4] = false;
            }
            if (index < 3) & (bits_required > 10) {
                fits[3] = false;
            }
            if (index < 4) & (bits_required > 8) {
                fits[2] = false;
            }
            if (index < 5) & (bits_required > 6) {
                fits[1] = false;
            }
            if (index < 10) & (bits_required > 3) {
                fits[0] = false;
            }
        }

        // Emit according to priority of cases
        if fits[0] {
            push_three_bits(self, out);
            return 10;
        } else if fits[1] {
            push_six_bits(self, out);
            return 5;
        } else if fits[2] {
            push_eight_bits(self, out);
            return 4;
        } else if fits[3] {
            push_ten_bits(self, out);
            return 3;
        } else if fits[4] {
            push_sixteen_bits(self, out);
            return 2;
        } else {
            unsafe {
                push_32_or_64_bits(self, out);
            }
            return 1;
        }
    }
}

// Delta-Delta Encoding
pub trait EmitDeltaDeltaBits {
    /// Emits bits according to the most efficient case of Delta Compression.
    /// Returns the number of elements popped from the queue.
    fn emit_delta_delta_bits(&mut self, out: &mut HalfVec) -> usize;
}

fn emit_popped_values<const N: usize>(
    bitcounts: &[usize; N],
    values: &[usize; N],
    out: &mut HalfVec,
) {
    for (bits, value) in bitcounts.into_iter().zip(values.into_iter()) {
        match bits {
            0 => out.push(HalfWord::Half(0b0000)),
            // -1 => out.push(HalfWord::Half(0b0001)),
            1..=5 => {
                let zigzag = (value & 0b1_1111) as u8;
                out.push(HalfWord::Byte(0b0010_0000 | zigzag));
            }
            6..=9 => {
                let zigzag = (value & 0b1_1111_1111) as u16;
                out.push(HalfWord::Half(0b0100 | (zigzag >> 8) as u8));
                out.push(HalfWord::Byte(zigzag as u8));
            }
            10..=16 => {
                let zigzag = (value & 0b1111_1111_1111_1111) as u16;
                out.push(HalfWord::Half(0b0110));
                out.push(HalfWord::Byte((zigzag >> 8) as u8));
                out.push(HalfWord::Byte(zigzag as u8));
            }
            _ => {
                out.push(HalfWord::Half(0b0111));
                out.push(HalfWord::Full(*value as u32));
            }
        }
    }
}

impl EmitDeltaDeltaBits for CompressionQueue<2> {
    fn emit_delta_delta_bits(&mut self, out: &mut HalfVec) -> usize {
        match self.len() {
            2 => {
                let bitcounts = self.peak_bitcounts::<2>();
                let values = self.pop_n::<2>();
                emit_popped_values(&bitcounts, &values, out);
                return 2;
            }
            1 => {
                let bitcounts = self.peak_bitcounts::<1>();
                let values = self.pop_n::<1>();
                emit_popped_values(&bitcounts, &values, out);
                return 1;
            }
            _ => return 0,
        }
    }
}

pub fn write_i128_bits(buf: &mut HalfVec, i: i128) {
    let i = i as u128;
    buf.push(HalfWord::Full((i >> 96) as u32));
    buf.push(HalfWord::Full((i >> 64) as u32));
    buf.push(HalfWord::Full((i >> 32) as u32));
    buf.push(HalfWord::Full(i as u32));
}

pub fn write_i64_bits(buf: &mut HalfVec, i: i64) {
    let i = i as u64;
    buf.push(HalfWord::Full((i >> 32) as u32));
    buf.push(HalfWord::Full(i as u32));
}

pub fn write_i32_bits(buf: &mut HalfVec, i: i32) {
    buf.push(HalfWord::Full(i as u32));
}

pub fn write_i16_bits(buf: &mut HalfVec, i: i16) {
    let i = i as u16;
    buf.push(HalfWord::Byte((i >> 8) as u8));
    buf.push(HalfWord::Byte(i as u8));
}

pub fn write_i8_bits(buf: &mut HalfVec, i: i8) {
    buf.push(HalfWord::Byte(i as u8));
}
