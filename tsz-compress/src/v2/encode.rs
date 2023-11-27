use core::fmt::Binary;

use num_traits::PrimInt;

use crate::prelude::*;

use super::halfvec::{HalfVec, HalfWord};

trait Bits: PrimInt + Binary {
    const BITS: usize;

    /// Language limitations prevent us from writing simple math expressions
    /// ((self << 1) ^ (self >> Self::BITS - 1)) as u32
    fn zigzag_bits(self) -> u32;

    fn zigzag_bit_masked(self, mask: u32) -> u32 {
        // Mask bottom bits
        let r = self.zigzag_bits() & mask;
        // println!("r: {:b}", r);
        r
    }
}

impl Bits for i8 {
    const BITS: usize = 8;
    /// Language limitations prevent us from writing simple math expressions
    #[inline(always)]
    fn zigzag_bits(self) -> u32 {
        ((self << 1) ^ (self >> Self::BITS - 1)) as u32
    }
}

impl Bits for i16 {
    const BITS: usize = 16;
    /// Language limitations prevent us from writing simple math expressions
    #[inline(always)]
    fn zigzag_bits(self) -> u32 {
        ((self << 1) ^ (self >> Self::BITS - 1)) as u32
    }
}

impl Bits for i32 {
    const BITS: usize = 32;
    /// Language limitations prevent us from writing simple math expressions
    #[inline(always)]
    fn zigzag_bits(self) -> u32 {
        ((self << 1) ^ (self >> Self::BITS - 1)) as u32
    }
}

#[inline(always)]
unsafe fn push_three_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
    const N: usize = 10;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(0b1111));
    let mut word: u32 = 0;
    let mask = 0b111_u32;
    let values = q.pop_n::<N>().unwrap_unchecked();
    for i in 0..N1 {
        word |= values[i].zigzag_bit_masked(mask);
        word <<= 3;
    }
    word |= values[N1].zigzag_bit_masked(mask);
    word <<= 2; // Nibble alignment
    buf.push(HalfWord::Full(word));
}

// #[inline(always)]
// unsafe fn push_four_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
//     const N: usize = 8;
//     const N1: usize = N - 1;
//     buf.push(HalfWord::Half(0b1110));
//     let mut word: u32 = 0;
//     let mask = 0b1111_u32;
//     let values = q.pop_n::<N>().unwrap_unchecked();
//     for i in 0..N1 {
//         word |= values[i].zigzag_bit_masked(mask);
//         word <<= 4;
//     }
//     word |= values[N1].zigzag_bit_masked(mask);
//     buf.push(HalfWord::Full(word));
// }

#[inline(always)]
unsafe fn push_six_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
    const N: usize = 5;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(0b1110));
    let mut word: u32 = 0;
    let mask = 0b11_1111_u32;
    let values = q.pop_n::<N>().unwrap_unchecked();
    for i in 0..N1 {
        word |= values[i].zigzag_bit_masked(mask);
        word <<= 6;
    }
    word |= values[N1].zigzag_bit_masked(mask);
    word <<= 2; // Nibble alignment
    buf.push(HalfWord::Full(word));
}

#[inline(always)]
unsafe fn push_eight_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
    const N: usize = 4;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(0b1100));
    let mut word: u32 = 0;
    let mask = 0b1111_1111_u32;
    let values = q.pop_n::<N>().unwrap_unchecked();
    for i in 0..N1 {
        word |= values[i].zigzag_bit_masked(mask);
        word <<= 8;
    }
    word |= values[N1].zigzag_bit_masked(mask);
    buf.push(HalfWord::Full(word));
}

#[inline(always)]
unsafe fn push_ten_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
    const N: usize = 3;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(0b1010));
    let mut word: u32 = 0b00 << 10;
    let mask = 0b11_1111_1111_u32;
    let values = q.pop_n::<N>().unwrap_unchecked();
    for i in 0..N1 {
        word |= values[i].zigzag_bit_masked(mask);
        word <<= 10;
    }
    word |= values[N1].zigzag_bit_masked(mask);
    word <<= 2; // Nibble alignment
    buf.push(HalfWord::Full(word));
}

#[inline(always)]
unsafe fn push_sixteen_bits<T: PrimInt + Bits>(q: &mut CompressionQueue<T, 10>, buf: &mut HalfVec) {
    const N: usize = 2;
    const N1: usize = N - 1;
    buf.push(HalfWord::Half(0b1000));
    let mut word: u32 = 0b00 << 10;
    let mask = 0xffffu32;
    let values = q.pop_n::<N>().unwrap_unchecked();
    for i in 0..N1 {
        word |= values[i].zigzag_bit_masked(mask);
        word <<= 16;
    }
    word |= values[N1].zigzag_bit_masked(mask);
    buf.push(HalfWord::Full(word));
}

#[inline(always)]
unsafe fn push_thirty_two_bits<T: PrimInt + Bits>(
    q: &mut CompressionQueue<T, 10>,
    buf: &mut HalfVec,
) {
    buf.push(HalfWord::Half(0b1011));
    let value = q.pop().unwrap_unchecked();
    let word = value.zigzag_bits();
    buf.push(HalfWord::Full(word));
}

// Todo: How can we implement this?
// #[inline(always)]
// unsafe fn push_sixty_four_bits<T: PrimInt + Bits>();

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
        }

        for (index, value) in self.iter().enumerate() {
            if (index < 2 && !(-32768..=32767).contains(&value)) {
                fits[..=4].fill(false);
                break;
            }
            if (index < 3 && !(-512..=511).contains(&value)) {
                fits[3] = false;
            }
            if (index < 4 && !(-128..=127).contains(&value)) {
                fits[2] = false;
            }
            if (index < 5 && !(-32..=31).contains(&value)) {
                fits[1] = false;
            }
            if (index < 10 && !(-4..=3).contains(&value)) {
                fits[0] = false;
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
                push_six_bits(self, out);
            }
            return 5;
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
        }

        for (index, value) in self.iter().enumerate() {
            // println!("value: {}", value);
            if (index < 3 && !(-512..=511).contains(&value)) {
                fits[..=3].fill(false);
                break;
            }
            if (index < 4 && !(-128..=127).contains(&value)) {
                fits[2] = false;
            }
            if (index < 5 && !(-32..=31).contains(&value)) {
                fits[1] = false;
            }
            if (index < 10 && !(-4..=3).contains(&value)) {
                fits[0] = false;
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
                push_six_bits(self, out);
            }
            return 5;
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

impl EmitDeltaBits<i8> for CompressionQueue<i8, 10> {
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
        }

        for (index, value) in self.iter().enumerate() {
            if (index < 4 && !(-128..=127).contains(&value)) {
                fits[2] = false;
            }
            if (index < 5 && !(-32..=31).contains(&value)) {
                fits[1] = false;
            }
            if (index < 10 && !(-4..=3).contains(&value)) {
                fits[0] = false;
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
                push_six_bits(self, out);
            }
            return 5;
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

// Delta-Delta Encoding
pub trait EmitDeltaDeltaBits<T> {
    /// Emits bits according to the most efficient case of Delta Compression.
    /// Returns the number of elements popped from the queue.
    fn emit_delta_delta_bits(&mut self, out: &mut HalfVec) -> usize;
}

fn emit_popped_values32<const N: usize>(values: &[i32; N], out: &mut HalfVec) {
    for value in values {
        match value {
            0 => out.push(HalfWord::Half(0b0000)),
            -1 => out.push(HalfWord::Half(0b0001)),
            -16..=15 => {
                let zigzag = value.zigzag_bit_masked(0b1_1111) as u8;
                out.push(HalfWord::Byte(0b0010_0000 | zigzag));
            }
            -256..=255 => {
                let zigzag = value.zigzag_bit_masked(0b1_1111_1111) as u16;
                out.push(HalfWord::Half(0b0100 | (zigzag >> 8) as u8));
                out.push(HalfWord::Byte(zigzag as u8));
            }
            -32678..=32767 => {
                let zigzag = value.zigzag_bit_masked(0b1111_1111_1111_1111) as u16;
                out.push(HalfWord::Half(0b0110));
                out.push(HalfWord::Byte((zigzag >> 8) as u8));
                out.push(HalfWord::Byte(zigzag as u8));
            }
            _ => {
                let zigzag = value.zigzag_bits();
                out.push(HalfWord::Half(0b0111));
                out.push(HalfWord::Full(zigzag));
            }
        }
    }
}

fn emit_popped_values32_q(q: &mut CompressionQueue<i32, 2>, out: &mut HalfVec) {
    while !q.is_empty() {
        let value = unsafe { q.pop().unwrap_unchecked() };
        match value {
            0 => out.push(HalfWord::Half(0b0000)),
            -1 => out.push(HalfWord::Half(0b0001)),
            -16..=15 => {
                let zigzag = value.zigzag_bit_masked(0b1_1111) as u8;
                out.push(HalfWord::Byte(0b0010_0000 | zigzag));
            }
            -256..=255 => {
                let zigzag = value.zigzag_bit_masked(0b1_1111_1111) as u16;
                out.push(HalfWord::Half(0b0100 | (zigzag >> 8) as u8));
                out.push(HalfWord::Byte(zigzag as u8));
            }
            -32678..=32767 => {
                let zigzag = value.zigzag_bit_masked(0b1111_1111_1111_1111) as u16;
                out.push(HalfWord::Half(0b0110));
                out.push(HalfWord::Byte((zigzag >> 8) as u8));
                out.push(HalfWord::Byte(zigzag as u8));
            }
            _ => {
                let zigzag = value.zigzag_bits();
                out.push(HalfWord::Half(0b0111));
                out.push(HalfWord::Full(zigzag));
            }
        }
    }
}

impl EmitDeltaDeltaBits<i32> for CompressionQueue<i32, 2> {
    fn emit_delta_delta_bits(&mut self, out: &mut HalfVec) -> usize {
        match self.len() {
            2 => {
                let values = unsafe { self.pop_n::<2>().unwrap_unchecked() };
                emit_popped_values32(&values, out);
                return 2;
            }
            _ => {
                let len = self.len();
                emit_popped_values32_q(self, out);
                return len;
            }
        }
    }
}

fn emit_popped_values16<const N: usize>(values: &[i16; N], out: &mut HalfVec) {
    for value in values {
        match value {
            0 => out.push(HalfWord::Half(0b0000)),
            -1 => out.push(HalfWord::Half(0b0001)),
            -16..=15 => {
                let zigzag = value.zigzag_bit_masked(0b1_1111) as u8;
                out.push(HalfWord::Byte(0b0010_0000 | zigzag));
            }
            -256..=255 => {
                let zigzag = value.zigzag_bit_masked(0b1_1111_1111) as u16;
                out.push(HalfWord::Half(0b0100 | (zigzag >> 8) as u8));
                out.push(HalfWord::Byte(zigzag as u8));
            }
            -32678..=32767 => {
                let zigzag = value.zigzag_bit_masked(0b1111_1111_1111_1111) as u16;
                out.push(HalfWord::Half(0b0110));
                out.push(HalfWord::Byte((zigzag >> 8) as u8));
                out.push(HalfWord::Byte(zigzag as u8));
            }
            _ => {
                let zigzag = value.zigzag_bits();
                out.push(HalfWord::Half(0b0111));
                out.push(HalfWord::Full(zigzag));
            }
        }
    }
}

fn emit_popped_values16_q(q: &mut CompressionQueue<i16, 2>, out: &mut HalfVec) {
    while !q.is_empty() {
        let value = unsafe { q.pop().unwrap_unchecked() };
        match value {
            0 => out.push(HalfWord::Half(0b0000)),
            -1 => out.push(HalfWord::Half(0b0001)),
            -16..=15 => {
                let zigzag = value.zigzag_bit_masked(0b1_1111) as u8;
                out.push(HalfWord::Byte(0b0010_0000 | zigzag));
            }
            -256..=255 => {
                let zigzag = value.zigzag_bit_masked(0b1_1111_1111) as u16;
                out.push(HalfWord::Half(0b0100 | (zigzag >> 8) as u8));
                out.push(HalfWord::Byte(zigzag as u8));
            }
            -32678..=32767 => {
                let zigzag = value.zigzag_bit_masked(0b1111_1111_1111_1111) as u16;
                out.push(HalfWord::Half(0b0110));
                out.push(HalfWord::Byte((zigzag >> 8) as u8));
                out.push(HalfWord::Byte(zigzag as u8));
            }
            _ => {
                let zigzag = value.zigzag_bits();
                out.push(HalfWord::Half(0b0111));
                out.push(HalfWord::Full(zigzag));
            }
        }
    }
}

impl EmitDeltaDeltaBits<i16> for CompressionQueue<i16, 2> {
    fn emit_delta_delta_bits(&mut self, out: &mut HalfVec) -> usize {
        match self.len() {
            2 => {
                let values = unsafe { self.pop_n::<2>().unwrap_unchecked() };
                emit_popped_values16(&values, out);
                return 2;
            }
            _ => {
                let len = self.len();
                emit_popped_values16_q(self, out);
                return len;
            }
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
