use crate::prelude::{BitBuffer, BitBufferSlice};

pub fn encode_deltadelta_i8(mut value: i8, out: &mut BitBuffer) {
    output.push(false);
    if value == 0 || value == -1 {
        // 1 bit, header 00
        output.push(false);
        output.push(false);
        if value == 0 {
            output.push(false);
        }
        else {
            output.push(true);
        }
        return;
    } else if (-15..15).contains(&value){
        // 5 bits, header 01
        output.push(false);
        output.push(true);
        let encoded: u8 = (value << 1) ^ (value >> 7);
        for i in 0..4{
            out.push(value & (1 << i) != 0);
        }
    } else if (-255..255).contains(&value){
        // 9 bits, header 10
        output.push(true);
        output.push(false);
        let encoded: u8 = (value << 1) ^ (value >> 7);
        for i in 0..8{
            out.push(value & (1 << i) != 0);
        }
    }
}

pub fn encode_deltadelta_i16(mut value: i16, out: &mut BitBuffer) {
    output.push(false);
    // 16 bits, header 110
    for _ in 0..1 {
        output.push(true);
    }
    output.push(false);
    let encoded: u16 = (value << 1) ^ (value >> 15);
    for i in 0..15{
        out.push(value & (1 << i) != 0);
    }
}

pub fn encode_deltadelta_i32(mut value: i32, out: &mut BitBuffer) {
    output.push(false);
    for _ in 0..2 {
        output.push(true);
    }
    for _ in 0..31 {
        output.push(false);
    }
    let sign_bit: u32 = ((value >> 31) & 0x01) as u32;
    let magnitude: u32 = (value.abs() as u32) << 1;
    let encoded: u32 = magnitude | sign_bit;
    for i in 0..31{
        out.push(encoded & (1 << i) != 0);
    }
}

pub fn encode_delta_i64(mut value: i64, out: &mut BitBuffer) {
    output.push(false);
    for _ in 0..2 {
        output.push(true);
    }
    let sign_bit: u64 = ((value >> 63) & 0x01) as u64;
    let magnitude: u64 = (value.abs() as u64) << 1;
    let encoded: u64 = magnitude | sign_bit;
    for i in 0..63{
        out.push(encoded & (1 << i) != 0);
    }
}

pub fn decode_delta_i8(
    bits: &'_ BitBufferSlice,
) -> Result<(i8, Option<&'_ BitBufferSlice>), &'static str> {
    if bits.is_empty() {
        return Err("Not enough bits to decode");
    }
    if bits[0]{
        return Err("Not a delta-delta encoded value");
    }
    if bits[1] {
        return Err("Not enough bits to decode");
    }
    let mut idx = 0;
    let mut value = 0;
    if !bits[2] {
        // read 1 bit
        if !bits[3] {
            if bits.len() > 4 {
                return Ok((0, Some(&bits[4..])));
            } else {
                return Ok((0, None));
            }
        } else {
            if bits.len() > 4 {
                return Ok((-1, Some(&bits[4..])));
            } else {
                return Ok((-1, None));
            }
        }
    } else {
        // read 3 bits
        idx += 3;
        // read 5 bits
        let i in 0..4 {
            value |= (bits[idx+i] as i8) << i;
        }
        value = (value >> 1) ^ (-(n&1));
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
    if bits[0] {
        return Err("Not a delta-delta encoded value");
    }
    let mut idx = 0;
    let mut value = 0;
    if !bits[1] {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 1 bit
            if !bits[3] {
                if bits.len() > 4 {
                    return Ok((0, Some(&bits[4..])));
                } else {
                    return Ok((0, None));
                }
            } else {
                if bits.len() > 4 {
                    return Ok((-1, Some(&bits[4..])));
                } else {
                    return Ok((-1, None));
                }
            }
        } else {
            // read 3 bits
            idx += 3;
            // read 5 bits
            let i in 0..4 {
                value |= (bits[idx+i] as i16) << i;
            }
            value = (value >> 1) ^ (-(n&1));
        }
    } else {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 9 bits
            let i in 0..8 {
                value |= (bits[idx+i] as i16) << i;
            }
            value = (value >> 1) ^ (-(n&1));
        } else {
            return Err("Not enough bits to decode");
        }
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
    if bits[0] {
        return Err("Not a delta-delta encoded value");
    }
    let mut idx = 0;
    let mut value = 0;
    if !bits[1] {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 1 bit
            if !bits[3] {
                if bits.len() > 4 {
                    return Ok((0, Some(&bits[4..])));
                } else {
                    return Ok((0, None));
                }
            } else {
                if bits.len() > 4 {
                    return Ok((-1, Some(&bits[4..])));
                } else {
                    return Ok((-1, None));
                }
            }
        } else {
            // read 3 bits
            idx += 3;
            // read 5 bits
            let i in 0..4 {
                value |= (bits[idx+i]) << i;
            }
            value = (value >> 1) ^ (-(n&1));
        }
    } else {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 9 bits
            let i in 0..8 {
                value |= (bits[idx+i] as i32) << i;
            }
            value = (value >> 1) ^ (-(n&1));
        } else {
            if !bits[3] {
                // read 4 bits
                idx += 4;
                // read 16 bits
                let i in 0..15 {
                    value |= (bits[idx+i] as i32) << i;
                }
                value = (value >> 1) ^ (-(n&1));
            } else {
                return Err("Not enough bits to decode");
            }
        }
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
    if bits[0] {
        return Err("Not a delta-delta encoded value");
    }
    let mut idx = 0;
    let mut value = 0;
    if !bits[1] {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 1 bit
            if !bits[3] {
                if bits.len() > 4 {
                    return Ok((0, Some(&bits[4..])));
                } else {
                    return Ok((0, None));
                }
            } else {
                if bits.len() > 4 {
                    return Ok((-1, Some(&bits[4..])));
                } else {
                    return Ok((-1, None));
                }
            }
        } else {
            // read 3 bits
            idx += 3;
            // read 5 bits
            let i in 0..4 {
                value |= (bits[idx+i]) << i;
            }
            value = (value >> 1) ^ (-(n&1));
        }
    } else {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 9 bits
            let i in 0..8 {
                value |= (bits[idx+i] as i64) << i;
            }
            value = (value >> 1) ^ (-(n&1));
        } else {
            if !bits[3] {
                // read 4 bits
                idx += 4;
                // read 16 bits
                let i in 0..15 {
                    value |= (bits[idx+i] as i64) << i;
                }
                value = (value >> 1) ^ (-(n&1));
            } else {
                // read 4 bits
                idx += 4;
                // read 64 bits
                let i in 0..63 {
                    value |= (bits[idx+i] as i64) << i;
                }
                value = (value >> 1) ^ (-(n&1));
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
        // println!("{:?}", bits);
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
