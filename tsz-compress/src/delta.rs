use bitvec::prelude::*;

pub fn encode_delta_i8(mut value: i8, out: &mut BitVec) {
    if value == 0 {
        out.push(false);
        return;
    }

    if value > 0 {
        value -= 1;
    }

    if (-8..8).contains(&value) {
        // write out 10
        out.push(true);
        out.push(false);

        // write out least significant 4 bits
        for i in 0..4 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-64..64).contains(&value) {
        // write out 110
        for _ in 0..2 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 7 bits
        for i in 0..7 {
            out.push(value & (1 << i) != 0);
        }
    } else {
        // write out 1110
        for _ in 0..3 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 9 bits
        let value = value as i16;
        for i in 0..9 {
            out.push(value & (1 << i) != 0);
        }
    }
}

pub fn encode_delta_i16(mut value: i16, out: &mut BitVec) {
    if value == 0 {
        out.push(false);
        return;
    }

    if value > 0 {
        value -= 1;
    }

    if (-8..8).contains(&value) {
        // write out 10
        out.push(true);
        out.push(false);

        // write out least significant 4 bits
        for i in 0..4 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-64..64).contains(&value) {
        // write out 110
        for _ in 0..2 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 7 bits
        for i in 0..7 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-256..256).contains(&value) {
        // write out 1110
        for _ in 0..3 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 9 bits
        for i in 0..9 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-2048..2048).contains(&value) {
        // write out 11110
        for _ in 0..4 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 12 bits
        for i in 0..12 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-16384..16384).contains(&value) {
        // write out 111110
        for _ in 0..5 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 15 bits
        for i in 0..15 {
            out.push(value & (1 << i) != 0);
        }
    } else {
        // write out 1111110
        for _ in 0..6 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 18 bits
        let value = value as i32;
        for i in 0..18 {
            out.push(value & (1 << i) != 0);
        }
    }
}

pub fn encode_delta_i32(mut value: i32, out: &mut BitVec) {
    if value == 0 {
        out.push(false);
        return;
    }

    if value > 0 {
        value -= 1;
    }

    if (-8..8).contains(&value) {
        // write out 10
        out.push(true);
        out.push(false);

        // write out least significant 4 bits
        for i in 0..4 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-64..64).contains(&value) {
        // write out 110
        for _ in 0..2 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 7 bits
        for i in 0..7 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-256..256).contains(&value) {
        // write out 1110
        for _ in 0..3 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 9 bits
        for i in 0..9 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-2048..2048).contains(&value) {
        // write out 11110
        for _ in 0..4 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 12 bits
        for i in 0..12 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-16384..16384).contains(&value) {
        // write out 111110
        for _ in 0..5 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 15 bits
        for i in 0..15 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-131072..131072).contains(&value) {
        // write out 1111110
        for _ in 0..6 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 18 bits
        for i in 0..18 {
            out.push(value & (1 << i) != 0);
        }
    } else {
        // write out 11111110
        for _ in 0..7 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 32 bits
        for i in 0..32 {
            out.push(value & (1 << i) != 0);
        }
    }
}

pub fn encode_delta_i64(mut value: i64, out: &mut BitVec) {
    if value == 0 {
        out.push(false);
        return;
    }

    if value > 0 {
        value -= 1;
    }

    if (-8..8).contains(&value) {
        // write out 10
        out.push(true);
        out.push(false);

        // write out least significant 4 bits
        for i in 0..4 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-64..64).contains(&value) {
        // write out 110
        for _ in 0..2 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 7 bits
        for i in 0..7 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-256..256).contains(&value) {
        // write out 1110
        for _ in 0..3 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 9 bits
        for i in 0..9 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-2048..2048).contains(&value) {
        // write out 11110
        for _ in 0..4 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 12 bits
        for i in 0..12 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-16384..16384).contains(&value) {
        // write out 111110
        for _ in 0..5 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 15 bits
        for i in 0..15 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-131072..131072).contains(&value) {
        // write out 1111110
        for _ in 0..6 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 18 bits
        for i in 0..18 {
            out.push(value & (1 << i) != 0);
        }
    } else if (-(1 << 31)..(1 << 31)).contains(&value) {
        // write out 11111110
        for _ in 0..7 {
            out.push(true);
        }
        out.push(false);

        // write out least significant 32 bits
        for i in 0..32 {
            out.push(value & (1 << i) != 0);
        }
    } else {
        // write out 11111111
        for _ in 0..8 {
            out.push(true);
        }

        // write out least significant 64 bits
        for i in 0..64 {
            out.push(value & (1 << i) != 0);
        }
    }
}

pub fn decode_delta_i8(bits: &'_ BitSlice) -> Result<(i8, Option<&'_ BitSlice>), &'static str> {
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

pub fn decode_delta_i16(bits: &'_ BitSlice) -> Result<(i16, Option<&'_ BitSlice>), &'static str> {
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

pub fn decode_delta_i32(bits: &'_ BitSlice) -> Result<(i32, Option<&'_ BitSlice>), &'static str> {
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

pub fn decode_delta_i64(bits: &'_ BitSlice) -> Result<(i64, Option<&'_ BitSlice>), &'static str> {
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
        let mut bits: BitVec<_, Lsb0> = BitVec::new();
        encode_delta_i8(value, &mut bits);
        let (decoded, remaining) = decode_delta_i8(&bits).unwrap();
        assert_eq!(value, decoded);
        assert!(remaining.is_none());
    }

    fn encode_decode_i16(value: i16) {
        let mut bits = BitVec::new();
        encode_delta_i16(value, &mut bits);
        let (decoded, remaining) = decode_delta_i16(&bits).unwrap();
        assert_eq!(value, decoded);
        assert!(remaining.is_none());
    }

    fn encode_decode_i32(value: i32) {
        let mut bits = BitVec::new();
        encode_delta_i32(value, &mut bits);
        // println!("{:?}", bits);
        let (decoded, remaining) = decode_delta_i32(&bits).unwrap();
        assert_eq!(value, decoded);
        assert!(remaining.is_none());
    }

    fn encode_decode_i64(value: i64) {
        let mut bits = BitVec::new();
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
