use crate::prelude::*;
use alloc::vec::Vec;

///
/// An iterator over nibbles in the slice of bytes.
///
pub struct HalfIter<'it> {
    buf: &'it [u8],
    upper: bool,
    idx: usize,
}

impl<'it> HalfIter<'it> {
    ///
    /// Create a HalfIter from the first nibble in the slice.
    ///
    pub fn new(buf: &'it [u8]) -> Self {
        Self {
            buf,
            upper: true,
            idx: 0,
        }
    }
}

impl<'it> Iterator for HalfIter<'it> {
    type Item = u8;

    ///
    /// Take the next nibble from the slice.
    /// Upper then lower nibble, repeat.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.buf.len() {
            return None;
        }

        if self.upper {
            let value = unsafe { self.buf.get_unchecked(self.idx) } >> 4;
            self.upper = false;
            Some(value)
        } else {
            let value = unsafe { self.buf.get_unchecked(self.idx) } & 0x0F;
            self.upper = true;
            self.idx += 1;
            Some(value)
        }
    }
}

pub fn read_full_i128(buf: &[u8; 16]) -> i128 {
    // Reverse of
    // buf.push(HalfWord::Full((i >> 96) as u32));
    // buf.push(HalfWord::Full((i >> 64) as u32));
    // buf.push(HalfWord::Full((i >> 32) as u32));
    // buf.push(HalfWord::Full(i as u32));
    let word = ((u32::from_le_bytes([buf[3], buf[2], buf[1], buf[0]]) as u128) << 96)
        | ((u32::from_le_bytes([buf[7], buf[6], buf[5], buf[4]]) as u128) << 64)
        | ((u32::from_le_bytes([buf[11], buf[10], buf[9], buf[8]]) as u128) << 32)
        | u32::from_le_bytes([buf[15], buf[14], buf[13], buf[12]]) as u128;
    word as i128
}

pub fn read_full_i64(buf: &[u8; 8]) -> i64 {
    // Reverse of
    // buf.push(HalfWord::Full((i >> 32) as u32));
    // buf.push(HalfWord::Full(i as u32));
    let word = ((u32::from_le_bytes([buf[3], buf[2], buf[1], buf[0]]) as u64) << 32)
        | u32::from_le_bytes([buf[7], buf[6], buf[5], buf[4]]) as u64;
    word as i64
}

pub fn read_full_i32(buf: &[u8; 4]) -> i32 {
    // Reverse of
    // buf.push(HalfWord::Full(i as u32));
    let word = u32::from_le_bytes([buf[3], buf[2], buf[1], buf[0]]) as u32;
    word as i32
}

pub fn read_full_i16(buf: &[u8; 2]) -> i16 {
    // Reverse of
    // buf.push(HalfWord::Byte((i >> 8) as u8));
    // buf.push(HalfWord::Byte(i as u8));
    let word = u16::from_le_bytes([buf[1], buf[0]]) as u16;
    word as i16
}

pub fn read_full_i8(buf: &[u8; 1]) -> i8 {
    // Reverse of
    // buf.push(HalfWord::Byte(i as u8));
    buf[0] as i8
}

pub fn decode_i8<'it>(iter: &mut HalfIter<'it>, output: &mut Vec<i8>) -> Result<(), CodingError> {
    // No rows
    let Some(next_upper) = iter.next() else {
        return Ok(());
    };

    // Full 8 bit value
    let buf = [(next_upper << 4) | iter.next().ok_or(CodingError::NotEnoughBits)?];
    let value = read_full_i8(&buf);
    output.push(value);

    // One row
    let Some(next_upper) = iter.next() else {
        return Ok(());
    };

    // Delta encoded 16 bit value
    let buf = [
        (next_upper << 4) | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
    ];
    let delta = read_full_i16(&buf);
    let mut value = (value as i16 + delta) as i8;
    output.push(value);

    // Every thing is delta or delta-delta encoded from here on out
    while let Some(tag) = iter.next() {
        match tag {
            0b1001 => {
                // Start of column of next column
                break;
            }
            0b1111 => {
                // 2 bit pad, 10 samples of 3 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 3;
                let shift = 32 - padding - bit_width;
                for i in 0..10 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b111) as i16;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i16 + delta) as i8;
                    output.push(value);
                }
            }
            0b1110 => {
                // 5 samples of 6 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 6;
                let shift = 32 - padding - bit_width;
                for i in 0..5 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b11_1111) as i16;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i16 + delta) as i8;
                    output.push(value);
                }
            }
            0b1100 => {
                // 4 samples of 8 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 8;
                let shift = 32 - padding - bit_width;
                for i in 0..4 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b1111_1111) as i16;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i16 + delta) as i8;
                    output.push(value);
                }
            }
            0b1010 => {
                // 2 bit pad, 3 samples of 10 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 10;
                let shift = 32 - padding - bit_width;
                for i in 0..3 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b11_1111_1111) as i16;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i16 + delta) as i8;
                    output.push(value);
                }
            }
            0b1000 => {
                // 2 samples of 16 bit
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 16;
                let shift = 32 - padding - bit_width;
                for i in 0..2 {
                    let delta = (word >> (shift - bit_width * i)) as i16;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i16 + delta) as i8;
                    output.push(value);
                }
            }
            0b1011 => {
                // 1 sample of 32 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 32;
                let shift = 32 - padding - bit_width;
                for i in 0..1 {
                    let delta = (word >> (shift - bit_width * i)) as i16;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i16 + delta) as i8;
                    output.push(value);
                }
            }
            _ => {
                // Delta-Delta encoding, or not implemented
                todo!("Implement i8 delta-delta");
            }
        }
    }

    Ok(())
}

pub fn decode_i16<'it>(iter: &mut HalfIter<'it>, output: &mut Vec<i16>) -> Result<(), CodingError> {
    // No rows
    let Some(next_upper) = iter.next() else {
        return Ok(());
    };

    // Full 16 bit value
    let buf = [
        (next_upper << 4) | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
    ];
    let value = read_full_i16(&buf);
    output.push(value);

    // One row
    let Some(next_upper) = iter.next() else {
        return Ok(());
    };

    // Delta encoded 32 bit value
    let buf = [
        (next_upper << 4) | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
    ];
    let delta = read_full_i32(&buf);
    let mut value = (value as i32 + delta) as i16;
    output.push(value);

    while let Some(tag) = iter.next() {
        match tag {
            0b1001 => {
                // Start of column of next column
                break;
            }
            0b1111 => {
                // 2 bit pad, 10 samples of 3 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 3;
                let shift = 32 - padding - bit_width;
                for i in 0..10 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b111) as i32;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i32 + delta) as i16;
                    output.push(value);
                }
            }
            0b1110 => {
                // 5 samples of 6 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 6;
                let shift = 32 - padding - bit_width;
                for i in 0..5 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b11_1111) as i32;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i32 + delta) as i16;
                    output.push(value);
                }
            }
            0b1100 => {
                // 4 samples of 8 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 8;
                let shift = 32 - padding - bit_width;
                for i in 0..4 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b1111_1111) as i32;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i32 + delta) as i16;
                    output.push(value);
                }
            }
            0b1010 => {
                // 2 bit pad, 3 samples of 10 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 10;
                let shift = 32 - padding - bit_width;
                for i in 0..3 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b11_1111_1111) as i32;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i32 + delta) as i16;
                    output.push(value);
                }
            }
            0b1000 => {
                // 2 samples of 16 bit
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 16;
                let shift = 32 - padding - bit_width;
                for i in 0..2 {
                    let delta = (word >> (shift - bit_width * i)) as i32;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i32 + delta) as i16;
                    output.push(value);
                }
            }
            0b1011 => {
                // 1 sample of 32 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 32;
                let shift = 32 - padding - bit_width;
                for i in 0..1 {
                    let delta = (word >> (shift - bit_width * i)) as i32;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i32 + delta) as i16;
                    output.push(value);
                }
            }
            _ => {
                // Delta-Delta encoding, or not implemented
                todo!("Implement i16 delta-delta");
            }
        }
    }

    Ok(())
}

pub fn decode_i32<'it>(iter: &mut HalfIter<'it>, output: &mut Vec<i32>) -> Result<(), CodingError> {
    // No rows
    let Some(next_upper) = iter.next() else {
        return Ok(());
    };

    // Full 32 bit value
    let buf = [
        (next_upper << 4) | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
    ];
    let value = read_full_i32(&buf);
    output.push(value);

    // One row
    let Some(next_upper) = iter.next() else {
        return Ok(());
    };

    // Delta encoded 64 bit value
    let buf = [
        (next_upper << 4) | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
    ];
    let delta = read_full_i64(&buf);
    let mut value = (value as i64 + delta) as i32;
    output.push(value);

    while let Some(tag) = iter.next() {
        match tag {
            0b1001 => {
                // Start of column of next column
                break;
            }
            0b1111 => {
                // 2 bit pad, 10 samples of 3 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 3;
                let shift = 32 - padding - bit_width;
                for i in 0..10 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b111) as i64;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i64 + delta) as i32;
                    output.push(value);
                }
            }
            0b1110 => {
                // 5 samples of 6 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 6;
                let shift = 32 - padding - bit_width;
                for i in 0..5 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b11_1111) as i64;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i64 + delta) as i32;
                    output.push(value);
                }
            }
            0b1100 => {
                // 4 samples of 8 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 8;
                let shift = 32 - padding - bit_width;
                for i in 0..4 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b1111) as i64;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i64 + delta) as i32;
                    output.push(value);
                }
            }

            0b1010 => {
                // 2 bit pad, 3 samples of 10 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 10;
                let shift = 32 - padding - bit_width;
                for i in 0..3 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b11_1111_1111) as i64;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i64 + delta) as i32;
                    output.push(value);
                }
            }
            0b1000 => {
                // 2 samples of 16 bit
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 16;
                let shift = 32 - padding - bit_width;
                for i in 0..2 {
                    let delta = ((word >> (shift - bit_width * i)) & 0xffff) as i64;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i64 + delta) as i32;
                    output.push(value);
                }
            }
            0b1011 => {
                // 1 sample of 32 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 32;
                let shift = 32 - padding - bit_width;
                for i in 0..1 {
                    let delta = (word >> (shift - bit_width * i)) as i64;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i64 + delta) as i32;
                    output.push(value);
                }
            }
            _ => {
                // Delta-Delta encoding, or not implemented
                todo!("Implement i32 delta-delta");
            }
        }
    }

    Ok(())
}

pub fn decode_i64<'it>(iter: &mut HalfIter<'it>, output: &mut Vec<i64>) -> Result<(), CodingError> {
    // No rows
    let Some(next_upper) = iter.next() else {
        return Ok(());
    };

    // Full 64 bit value
    let buf = [
        (next_upper << 4) | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
    ];
    let value = read_full_i64(&buf);
    output.push(value);

    // One row
    let Some(next_upper) = iter.next() else {
        return Ok(());
    };

    // Delta encoded 128 bit value
    let buf = [
        (next_upper << 4) | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
        (iter.next().ok_or(CodingError::NotEnoughBits)? << 4)
            | iter.next().ok_or(CodingError::NotEnoughBits)?,
    ];
    let delta = read_full_i128(&buf);
    let mut value = (value as i128 + delta) as i64;
    output.push(value);

    while let Some(tag) = iter.next() {
        match tag {
            0b1001 => {
                // Start of column of next column
                break;
            }
            0b1111 => {
                // 2 bit pad, 10 samples of 3 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 3;
                let shift = 32 - padding - bit_width;
                for i in 0..10 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b111) as i128;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i128 + delta) as i64;
                    output.push(value);
                }
            }
            0b1110 => {
                // 5 samples of 6 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 6;
                let shift = 32 - padding - bit_width;
                for i in 0..5 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b11_1111) as i128;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i128 + delta) as i64;
                    output.push(value);
                }
            }
            0b1100 => {
                // 4 samples of 8 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 8;
                let shift = 32 - padding - bit_width;
                for i in 0..4 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b1111) as i128;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i128 + delta) as i64;
                    output.push(value);
                }
            }

            0b1010 => {
                // 2 bit pad, 3 samples of 10 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 2;
                let bit_width = 10;
                let shift = 32 - padding - bit_width;
                for i in 0..3 {
                    let delta = ((word >> (shift - bit_width * i)) & 0b11_1111_1111) as i128;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i128 + delta) as i64;
                    output.push(value);
                }
            }
            0b1000 => {
                // 2 samples of 16 bit
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 16;
                let shift = 32 - padding - bit_width;
                for i in 0..2 {
                    let delta = ((word >> (shift - bit_width * i)) & 0xffff) as i128;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i128 + delta) as i64;
                    output.push(value);
                }
            }
            0b1011 => {
                // 1 sample of 32 bits
                let mut word: u32 = 0;
                for _ in 0..7 {
                    let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                    word |= half as u32;
                    word <<= 4;
                }
                let half = iter.next().ok_or(CodingError::NotEnoughBits)?;
                word |= half as u32;

                let padding = 0;
                let bit_width = 32;
                let shift = 32 - padding - bit_width;
                for i in 0..1 {
                    let delta = (word >> (shift - bit_width * i)) as i128;
                    let delta = (delta >> 1) ^ -(delta & 1);
                    value = (value as i128 + delta) as i64;
                    output.push(value);
                }
            }
            _ => {
                // Delta-Delta encoding, or not implemented
                todo!("Implement i64 delta-delta");
            }
        }
    }

    Ok(())
}
