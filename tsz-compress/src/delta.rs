use crate::prelude::{BitBuffer, BitBufferSlice};
use core::ops::Range;

trait BitVectorOps {
    // In order to reduce unnecessary bitwise operations on value if value is considered i64 when it is i8, i16 and i32
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
fn push_n<const N: usize>(buf: &mut BitBuffer) {
    for _ in 0..N - 1 {
        buf.push(true);
    }
    buf.push(false);
}

pub fn encode_delta_i8(mut value: i8, out: &mut BitBuffer) {
    if value == 0 {
        push_n::<1>(out);
        return;
    }

    if value > 0 {
        value -= 1;
    }

    if (-8..8).contains(&value) {
        // write out 10
        push_n::<2>(out);

        // write out least significant 4 bits
        value.extend_bits(0..4, out);
    } else if (-64..64).contains(&value) {
        // write out 110
        push_n::<3>(out);

        // write out 110 and least significant 7 bits
        value.extend_bits(0..7, out);
    } else {
        // write out 1110
        push_n::<4>(out);

        // write out least significant 9 bits
        let value = value as i16;
        value.extend_bits(0..9, out);
    }
}

pub fn encode_delta_i16(mut value: i16, out: &mut BitBuffer) {
    if value == 0 {
        push_n::<1>(out);
        return;
    }

    if value > 0 {
        value -= 1;
    }

    if (-8..8).contains(&value) {
        // write out 10
        push_n::<2>(out);

        // write out least significant 4 bits
        value.extend_bits(0..4, out);
    } else if (-64..64).contains(&value) {
        // write out 110
        push_n::<3>(out);

        // write out least significant 7 bits
        value.extend_bits(0..7, out);
    } else if (-256..256).contains(&value) {
        // write out 1110
        push_n::<4>(out);

        // write out least significant 9 bits
        value.extend_bits(0..9, out);
    } else if (-2048..2048).contains(&value) {
        // write out 11110
        push_n::<5>(out);

        // write out least significant 12 bits
        value.extend_bits(0..12, out);
    } else if (-16384..16384).contains(&value) {
        // write out 111110
        push_n::<6>(out);

        // write out least significant 15 bits
        value.extend_bits(0..15, out);
    } else {
        // write out 1111110
        push_n::<7>(out);

        // write out least significant 18 bits
        let value = value as i32;
        value.extend_bits(0..18, out);
    }
}

pub fn encode_delta_i32(mut value: i32, out: &mut BitBuffer) {
    if value == 0 {
        push_n::<1>(out);
        return;
    }

    if value > 0 {
        value -= 1;
    }

    if (-8..8).contains(&value) {
        // write out 10
        push_n::<2>(out);

        // write out least significant 4 bits
        value.extend_bits(0..4, out);
    } else if (-64..64).contains(&value) {
        // write out 110
        push_n::<3>(out);

        // write out least significant 7 bits
        value.extend_bits(0..7, out);
    } else if (-256..256).contains(&value) {
        // write out 1110
        push_n::<4>(out);

        // write out least significant 9 bits
        value.extend_bits(0..9, out);
    } else if (-2048..2048).contains(&value) {
        // write out 11110
        push_n::<5>(out);

        // write out least significant 12 bits
        value.extend_bits(0..12, out);
    } else if (-16384..16384).contains(&value) {
        // write out 111110
        push_n::<6>(out);

        // write out least significant 15 bits
        value.extend_bits(0..15, out);
    } else if (-131072..131072).contains(&value) {
        // write out 1111110
        push_n::<7>(out);

        // write out least significant 18 bits
        value.extend_bits(0..18, out);
    } else {
        // write out 11111110
        push_n::<8>(out);

        // write out least significant 32 bits
        value.extend_bits(0..32, out);
    }
}

pub fn encode_delta_i64(mut value: i64, out: &mut BitBuffer) {
    if value == 0 {
        push_n::<1>(out);
        return;
    }

    if value > 0 {
        value -= 1;
    }

    if (-8..8).contains(&value) {
        // write out 10
        push_n::<2>(out);
        // write out least significant 4 bits
        value.extend_bits(0..4, out);
    } else if (-64..64).contains(&value) {
        // write out 110
        push_n::<3>(out);

        // write out least significant 7 bits
        value.extend_bits(0..7, out);
    } else if (-256..256).contains(&value) {
        // write out 1110
        push_n::<4>(out);

        // write out least significant 9 bits
        value.extend_bits(0..9, out);
    } else if (-2048..2048).contains(&value) {
        // write out 11110
        push_n::<5>(out);

        // write out least significant 12 bits
        value.extend_bits(0..12, out);
    } else if (-16384..16384).contains(&value) {
        // write out 111110
        push_n::<6>(out);

        // write out least significant 15 bits
        value.extend_bits(0..15, out);
    } else if (-131072..131072).contains(&value) {
        // write out 1111110
        push_n::<7>(out);

        // write out least significant 18 bits
        value.extend_bits(0..18, out);
    } else if (-(1 << 31)..(1 << 31)).contains(&value) {
        // write out 11111110
        push_n::<8>(out);

        // write out least significant 32 bits
        value.extend_bits(0..32, out);
    } else {
        // write out 11111111
        for _ in 0..8 {
            out.push(true);
        }

        // write out least significant 64 bits
        value.extend_bits(0..64, out);
    }
}

pub fn decode_delta_i8(
    bits: &'_ BitBufferSlice,
) -> Result<(i8, Option<&'_ BitBufferSlice>), &'static str> {
    if bits.is_empty() {
        return Err("Not enough bits to decode");
    }

    if !bits[0] {
        if bits.len() > 1 {
            return Ok((0, Some(&bits[1..])));
        } else {
            return Ok((0, None));
        }
    }

    let mut idx = 0;
    let mut value = 0;

    if !bits[1] {
        // read 2 bits
        idx += 2;

        // read 4 bits
        for i in 0..3 {
            value |= (bits[2 + i] as i8) << i;
        }
        // the top bit is the sign bit
        value -= (bits[2 + 3] as i8) << 3;
        idx += 4;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[2] {
        // read 3 bits
        idx += 3;

        // read 7 bits
        for i in 0..6 {
            value |= (bits[3 + i] as i8) << i;
        }
        // the top bit is the sign bit
        value -= (bits[3 + 6] as i8) << 6;
        idx += 7;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[3] {
        // read 4 bits
        idx += 4;

        // read 9 bits
        for i in 0..8 {
            value |= (bits[4 + i] as i8) << i;
        }
        idx += 9;
        if value >= 0 {
            if value == i8::MAX {
                return Err("Invalid encoding for i8");
            } else {
                value += 1;
            }
        }
    } else {
        return Err("Invalid encoding for i8");
    }

    if bits.len() > idx {
        Ok((value, Some(&bits[idx..])))
    } else {
        Ok((value, None))
    }
}

pub fn decode_delta_i16(
    bits: &'_ BitBufferSlice,
) -> Result<(i16, Option<&'_ BitBufferSlice>), &'static str> {
    if bits.is_empty() {
        return Err("Not enough bits to decode");
    }

    if !bits[0] {
        if bits.len() > 1 {
            return Ok((0, Some(&bits[1..])));
        } else {
            return Ok((0, None));
        }
    }

    let mut idx = 0;
    let mut value = 0;

    if !bits[1] {
        // read 2 bits
        idx += 2;

        // read 4 bits
        for i in 0..3 {
            value |= (bits[2 + i] as i16) << i;
        }
        // the top bit is the sign bit
        value -= (bits[2 + 3] as i16) << 3;
        idx += 4;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[2] {
        // read 3 bits
        idx += 3;

        // read 7 bits
        for i in 0..6 {
            value |= (bits[3 + i] as i16) << i;
        }
        // the top bit is the sign bit
        value -= (bits[3 + 6] as i16) << 6;
        idx += 7;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[3] {
        // read 4 bits
        idx += 4;

        // read 9 bits
        for i in 0..8 {
            value |= (bits[4 + i] as i16) << i;
        }
        // the top bit is the sign bit
        value -= (bits[4 + 8] as i16) << 8;
        idx += 9;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[4] {
        // read 5 bits
        idx += 5;

        // read 12 bits
        for i in 0..11 {
            value |= (bits[5 + i] as i16) << i;
        }
        // the top bit is the sign bit
        value -= (bits[5 + 11] as i16) << 11;
        idx += 12;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[5] {
        // read 6 bits
        idx += 6;

        // read 15 bits
        for i in 0..14 {
            value |= (bits[6 + i] as i16) << i;
        }
        // the top bit is the sign bit
        value -= (bits[6 + 14] as i16) << 14;
        idx += 15;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[6] {
        // read 7 bits
        idx += 7;

        // read 18 bits
        for i in 0..16 {
            value |= (bits[7 + i] as i16) << i;
        }
        idx += 18;
        if value >= 0 {
            if value == i16::MAX {
                return Err("Invalid encoding for i16");
            } else {
                value += 1;
            }
        }
    } else {
        return Err("Invalid encoding for i16");
    }

    if bits.len() > idx {
        Ok((value, Some(&bits[idx..])))
    } else {
        Ok((value, None))
    }
}

pub fn decode_delta_i32(
    bits: &'_ BitBufferSlice,
) -> Result<(i32, Option<&'_ BitBufferSlice>), &'static str> {
    if bits.is_empty() {
        return Err("Not enough bits to decode");
    }

    if !bits[0] {
        if bits.len() > 1 {
            return Ok((0, Some(&bits[1..])));
        } else {
            return Ok((0, None));
        }
    }

    let mut idx = 0;
    let mut value = 0;

    if !bits[1] {
        // read 2 bits
        idx += 2;

        // read 4 bits
        for i in 0..3 {
            value |= (bits[2 + i] as i32) << i;
        }
        // the top bit is the sign bit
        value -= (bits[2 + 3] as i32) << 3;
        idx += 4;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[2] {
        // read 3 bits
        idx += 3;

        // read 7 bits
        for i in 0..6 {
            value |= (bits[3 + i] as i32) << i;
        }
        // the top bit is the sign bit
        value -= (bits[3 + 6] as i32) << 6;
        idx += 7;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[3] {
        // read 4 bits
        idx += 4;

        // read 9 bits
        for i in 0..8 {
            value |= (bits[4 + i] as i32) << i;
        }
        // the top bit is the sign bit
        value -= (bits[4 + 8] as i32) << 8;
        idx += 9;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[4] {
        // read 5 bits
        idx += 5;

        // read 12 bits
        for i in 0..11 {
            value |= (bits[5 + i] as i32) << i;
        }
        // the top bit is the sign bit
        value -= (bits[5 + 11] as i32) << 11;
        idx += 12;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[5] {
        // read 6 bits
        idx += 6;

        // read 15 bits
        for i in 0..14 {
            value |= (bits[6 + i] as i32) << i;
        }
        // the top bit is the sign bit
        value -= (bits[6 + 14] as i32) << 14;
        idx += 15;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[6] {
        // read 7 bits
        idx += 7;

        // read 18 bits
        for i in 0..17 {
            value |= (bits[7 + i] as i32) << i;
        }
        // the top bit is the sign bit
        value -= (bits[7 + 17] as i32) << 17;
        idx += 18;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[7] {
        // read 8 bits
        idx += 8;

        for i in 0..32 {
            value |= (bits[8 + i] as i32) << i;
        }
        idx += 32;
        if value >= 0 {
            if value == i32::MAX {
                return Err("Invalid encoding for i32");
            } else {
                value += 1;
            }
        }
    } else {
        return Err("Invalid encoding for i32");
    }

    if bits.len() > idx {
        Ok((value, Some(&bits[idx..])))
    } else {
        Ok((value, None))
    }
}

pub fn decode_delta_i64(
    bits: &'_ BitBufferSlice,
) -> Result<(i64, Option<&'_ BitBufferSlice>), &'static str> {
    if bits.is_empty() {
        return Err("Not enough bits to decode");
    }

    if !bits[0] {
        if bits.len() > 1 {
            return Ok((0, Some(&bits[1..])));
        } else {
            return Ok((0, None));
        }
    }

    let mut idx = 0;
    let mut value = 0;

    if !bits[1] {
        // read 2 bits
        idx += 2;

        // read 4 bits
        for i in 0..3 {
            value |= (bits[2 + i] as i64) << i;
        }
        // the top bit is the sign bit
        value -= (bits[2 + 3] as i64) << 3;
        idx += 4;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[2] {
        // read 3 bits
        idx += 3;

        // read 7 bits
        for i in 0..6 {
            value |= (bits[3 + i] as i64) << i;
        }
        // the top bit is the sign bit
        value -= (bits[3 + 6] as i64) << 6;
        idx += 7;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[3] {
        // read 4 bits
        idx += 4;

        // read 9 bits
        for i in 0..8 {
            value |= (bits[4 + i] as i64) << i;
        }
        // the top bit is the sign bit
        value -= (bits[4 + 8] as i64) << 8;
        idx += 9;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[4] {
        // read 5 bits
        idx += 5;

        // read 12 bits
        for i in 0..11 {
            value |= (bits[5 + i] as i64) << i;
        }
        // the top bit is the sign bit
        value -= (bits[5 + 11] as i64) << 11;
        idx += 12;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[5] {
        // read 6 bits
        idx += 6;

        // read 15 bits
        for i in 0..14 {
            value |= (bits[6 + i] as i64) << i;
        }
        // the top bit is the sign bit
        value -= (bits[6 + 14] as i64) << 14;
        idx += 15;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[6] {
        // read 7 bits
        idx += 7;

        // read 18 bits
        for i in 0..17 {
            value |= (bits[7 + i] as i64) << i;
        }
        // the top bit is the sign bit
        value -= (bits[7 + 17] as i64) << 17;
        idx += 18;
        if value >= 0 {
            value += 1;
        }
    } else if !bits[7] {
        // read 8 bits
        idx += 8;

        // read 32 bits
        for i in 0..31 {
            value |= (bits[8 + i] as i64) << i;
        }
        // the top bit is the sign bit
        value -= (bits[8 + 31] as i64) << 31;
        idx += 32;
        if value >= 0 {
            value += 1;
        }
    } else {
        // read 8 bits
        idx += 8;

        // read 64 bits
        for i in 0..64 {
            value |= (bits[8 + i] as i64) << i;
        }
        idx += 64;
        if value >= 0 {
            if value == i64::MAX {
                return Err("Invalid encoding for i64");
            } else {
                value += 1;
            }
        }
    }

    if bits.len() > idx {
        Ok((value, Some(&bits[idx..])))
    } else {
        Ok((value, None))
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Add;
    use core::ops::Sub;
    use rand::Rng;

    use crate::svlq::*;
    use crate::uvlq::*;

    use super::*;

    fn encode_decode_i8(value: i8) {
        let mut bits = BitBuffer::new();
        encode_delta_i8(value, &mut bits);
        let (decoded, remaining) = decode_delta_i8(&bits).unwrap();
        assert_eq!(value, decoded);
        assert!(remaining.is_none());
    }

    fn encode_decode_i16(value: i16) {
        let mut bits = BitBuffer::new();
        encode_delta_i16(value, &mut bits);
        let (decoded, remaining) = decode_delta_i16(&bits).unwrap();
        assert_eq!(value, decoded);
        assert!(remaining.is_none());
    }

    fn encode_decode_i32(value: i32) {
        let mut bits = BitBuffer::new();
        encode_delta_i32(value, &mut bits);
        let (decoded, remaining) = decode_delta_i32(&bits).unwrap();
        assert_eq!(value, decoded);
        assert!(remaining.is_none());
    }

    fn encode_decode_i64(value: i64) {
        let mut bits = BitBuffer::new();
        encode_delta_i64(value, &mut bits);
        let (decoded, remaining) = decode_delta_i64(&bits).unwrap();
        assert_eq!(value, decoded);
        assert!(remaining.is_none());
    }

    #[test]
    fn can_encode_decode_all_i8() {
        for i in i8::MIN..=i8::MAX {
            encode_decode_i8(i);
        }
    }

    #[test]
    fn can_encode_decode_all_i16() {
        for i in i8::MIN..=i8::MAX {
            encode_decode_i16(i as i16);
        }

        for i in i16::MIN..=i16::MAX {
            encode_decode_i16(i);
        }
    }

    #[test]
    fn can_encode_decode_all_i32() {
        for i in i8::MIN..=i8::MAX {
            encode_decode_i32(i as i32);
        }

        for i in i16::MIN..=i16::MAX {
            encode_decode_i32(i as i32);
        }

        let mut rng = rand::thread_rng();

        // Randomly skip most of the values
        encode_decode_i32(i32::MIN);
        encode_decode_i32(0);
        encode_decode_i32(i32::MAX);
        for _ in 0..=10000 {
            let i = rng.gen_range(i32::MIN..=i32::MAX);
            encode_decode_i32(i);
        }
    }

    #[test]
    fn can_encode_decode_all_i64() {
        for i in i8::MIN..=i8::MAX {
            encode_decode_i64(i as i64);
        }

        for i in i16::MIN..=i16::MAX {
            encode_decode_i64(i as i64);
        }

        let mut rng = rand::thread_rng();

        // Randomly skip most of the values
        encode_decode_i64(i32::MIN as i64);
        encode_decode_i64(0);
        encode_decode_i64(i32::MAX as i64);
        for _ in 0..=10000 {
            let i = rng.gen_range(i32::MIN..=i32::MAX);
            encode_decode_i64(i as i64);
        }

        encode_decode_i64(i64::MIN);
        encode_decode_i64(0);
        encode_decode_i64(i64::MAX);
        for _ in 0..=100000 {
            let i = rng.gen_range(i64::MIN..=i64::MAX);
            encode_decode_i64(i);
        }
    }
}
