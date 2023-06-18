use crate::prelude::{BitBuffer, BitBufferSlice};

pub fn encode_deltadelta_i8(value: i8, out: &mut BitBuffer) {
    out.push(false);
    if value == 0 || value == -1 {
        // 1 bit, header 00
        out.push(false);
        out.push(false);
        if value == 0 {
            out.push(false);
        }
        else {
            out.push(true);
        }
        return;
    } else if (-15..15).contains(&value){
        // 5 bits, header 01
        out.push(false);
        out.push(true);
        let encoded: u8 = ((value << 1) ^ (value >> 7)) as u8;
        for i in 0..5{
            out.push(encoded & (1 << i) != 0);
        }
    } else {
        // 9 bits, header 10
        out.push(true);
        out.push(false);
        let encoded: u8 = ((value << 1) ^ (value >> 7)) as u8;
        for i in 0..8{
            out.push(encoded & (1 << i) != 0);
        }
        out.push(false);
    }
}

pub fn encode_deltadelta_i16(value: i16, out: &mut BitBuffer) {
    out.push(false);
    if value == 0 || value == -1 {
        // 1 bit, header 00
        out.push(false);
        out.push(false);
        if value == 0 {
            out.push(false);
        }
        else {
            out.push(true);
        }
        return;
    } else if (-15..15).contains(&value){
        // 5 bits, header 01
        out.push(false);
        out.push(true);
        let encoded: u16 = ((value << 1) ^ (value >> 15)) as u16;
        for i in 0..5{
            out.push(encoded & (1 << i) != 0);
        }
    } else if (-255..255).contains(&value){
        // 9 bits, header 10
        out.push(true);
        out.push(false);
        let encoded: u16 = ((value << 1) ^ (value >> 15)) as u16;
        for i in 0..9{
            out.push(encoded & (1 << i) != 0);
        }
    } else {
        // 16 bits, header 110
        for _ in 0..2 {
            out.push(true);
        }
        out.push(false);
        let encoded: u16 = ((value << 1) ^ (value >> 15)) as u16;
        for i in 0..16{
            out.push(encoded & (1 << i) != 0);
        }
    }
}

pub fn encode_deltadelta_i32(value: i32, out: &mut BitBuffer) {
    out.push(false);
    if value == 0 || value == -1 {
        // 1 bit, header 00
        out.push(false);
        out.push(false);
        if value == 0 {
            out.push(false);
        }
        else {
            out.push(true);
        }
        return;
    } else if (-15..15).contains(&value){
        // 5 bits, header 01
        out.push(false);
        out.push(true);
        let encoded: u32 = ((value << 1) ^ (value >> 31)) as u32;
        for i in 0..5{
            out.push(encoded & (1 << i) != 0);
        }
    } else if (-255..255).contains(&value){
        // 9 bits, header 10
        out.push(true);
        out.push(false);
        let encoded: u32 = ((value << 1) ^ (value >> 31)) as u32;
        for i in 0..9{
            out.push(encoded & (1 << i) != 0);
        }
    } else if (-32768..32768).contains(&value){
        // 16 bits, header 110
        for _ in 0..2 {
            out.push(true);
        }
        out.push(false);
        let encoded: u32 = ((value << 1) ^ (value >> 31)) as u32;
        for i in 0..16{
            out.push(encoded & (1 << i) != 0);
        }
    } else {
        for _ in 0..3 {
            out.push(true);
        }
        for _ in 0..32 {
            out.push(false);
        }
        let encoded: u32 = ((value << 1) ^ (value >> 31)) as u32;
        for i in 0..32{
            out.push(encoded & (1 << i) != 0);
        }
    }
}

pub fn encode_deltadelta_i64(value: i64, out: &mut BitBuffer) {
    out.push(false);
    if value == 0 || value == -1 {
        // 1 bit, header 00
        out.push(false);
        out.push(false);
        if value == 0 {
            out.push(false);
        }
        else {
            out.push(true);
        }
        return;
    } else if (-15..15).contains(&value){
        // 5 bits, header 01
        out.push(false);
        out.push(true);
        let encoded: u64 = ((value << 1) ^ (value >> 63)) as u64;
        for i in 0..5{
            out.push(encoded & (1 << i) != 0);
        }
    } else if (-255..255).contains(&value){
        // 9 bits, header 10
        out.push(true);
        out.push(false);
        let encoded: u64 = ((value << 1) ^ (value >> 63)) as u64;
        for i in 0..9{
            out.push(encoded & (1 << i) != 0);
        }
    } else if (-32768..32768).contains(&value){
        // 16 bits, header 110
        for _ in 0..2 {
            out.push(true);
        }
        out.push(false);
        let encoded: u64 = ((value << 1) ^ (value >> 63)) as u64;
        for i in 0..16{
            out.push(encoded & (1 << i) != 0);
        }
    } else {
        for _ in 0..3 {
            out.push(true);
        }
        let encoded: u64 = ((value << 1) ^ (value >> 63)) as u64;
        for i in 0..64{
            out.push(encoded & (1 << i) != 0);
        }
    }
}

pub fn decode_deltadelta_i8(
    bits: &'_ BitBufferSlice,
) -> Result<(i8, Option<&'_ BitBufferSlice>), &'static str> {
    if bits.is_empty() {
        return Err("Not enough bits to decode");
    }
    if bits[0]{
        return Err("Not a delta-delta encoded value");
    }
    let mut idx = 0;
    let mut value = 0;
    let mut result = 0;
    if !bits[1]{
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
            for i in 0..5 {
                value |= (bits[idx+i] as u8) << i;
            }
            result += ((value >> 1) as i8) ^ (-((value&1) as i8));
            idx += 5;
        }
    } else {
        if bits[2]{
            return Err ("Exceeds the range");
        } else {
            // read 3 bits
            idx += 3;
            // read 8 bits
            for i in 0..8 {
                value |= (bits[idx+i] as u8) << i;
            }
            result += ((value >> 1) as i8) ^ (-((value&1) as i8));
            idx += 9;
        }
    }
    if bits.len() > idx {
        Ok((result, Some(&bits[idx..])))
    } else {
        Ok((result, None))
    }
}

pub fn decode_deltadelta_i16(
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
    let mut result = 0;
    if !bits[1] {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 1 bit
            idx += 1;
            if !bits[3] {
                if bits.len() > 4 {
                    return Ok((0, Some(&bits[idx..])));
                } else {
                    return Ok((0, None));
                }
            } else {
                if bits.len() > 4 {
                    return Ok((-1, Some(&bits[idx..])));
                } else {
                    return Ok((-1, None));
                }
            }
        } else {
            // read 3 bits
            idx += 3;
            // read 5 bits
            for i in 0..5 {
                value |= (bits[idx+i] as u16) << i;
            }
            result += ((value >> 1) as i16) ^ (-((value&1) as i16));
            idx += 5;
        }
    } else {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 9 bits
            for i in 0..9 {
                value |= (bits[idx+i] as u16) << i;
            }
            result += ((value >> 1) as i16) ^ (-((value&1) as i16));
            idx += 9;
        } else {
            // read 4 bits
            idx += 4;
            // read 16 bits
            for i in 0..16 {
                value |= (bits[idx+i] as u16) << i;
            }
            result += ((value >> 1) as i16) ^ (-((value&1) as i16));
            idx += 16;
        }
    }
    if bits.len() > idx {
        Ok((result, Some(&bits[idx..])))
    } else {
        Ok((result, None))
    }
}

pub fn decode_deltadelta_i32(
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
    let mut result = 0;
    if !bits[1] {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 1 bit
            idx += 1;
            if !bits[3] {
                if bits.len() > 4 {
                    return Ok((0, Some(&bits[idx..])));
                } else {
                    return Ok((0, None));
                }
            } else {
                if bits.len() > 4 {
                    return Ok((-1, Some(&bits[idx..])));
                } else {
                    return Ok((-1, None));
                }
            }
        } else {
            // read 3 bits
            idx += 3;
            // read 5 bits
            for i in 0..5 {
                value |= (bits[idx+i] as u32) << i;
            }
            result += ((value >> 1) as i32) ^ (-((value&1) as i32));
            idx += 5;
        }
    } else {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 9 bits
            for i in 0..9 {
                value |= (bits[idx+i] as u32) << i;
            }
            result += ((value >> 1) as i32) ^ (-((value&1) as i32));
            idx += 9;
        } else {
            if !bits[3]{
                // read 4 bits
                idx += 4;
                // read 16 bits
                for i in 0..16 {
                    value |= (bits[idx+i] as u32) << i;
                }
                result += ((value >> 1) as i32) ^ (-((value&1) as i32));
                idx += 16;
            } else {
                // read 36 bits
                idx += 36;
                // read 32 bits
                for i in 0..32{
                    value |= (bits[idx+i] as u32) << i;
                }
                result += ((value >> 1) as i32) ^ (-((value&1) as i32));
                idx += 32;
            }
        }
    }
    if bits.len() > idx {
        Ok((result, Some(&bits[idx..])))
    } else {
        Ok((result, None))
    }
}

pub fn decode_deltadelta_i64(
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
    let mut result = 0;
    if !bits[1] {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 1 bit
            idx += 1;
            if !bits[3] {
                if bits.len() > 4 {
                    return Ok((0, Some(&bits[idx..])));
                } else {
                    return Ok((0, None));
                }
            } else {
                if bits.len() > 4 {
                    return Ok((-1, Some(&bits[idx..])));
                } else {
                    return Ok((-1, None));
                }
            }
        } else {
            // read 3 bits
            idx += 3;
            // read 5 bits
            for i in 0..5 {
                value |= (bits[idx+i] as u64) << i;
            }
            result += ((value >> 1) as i64) ^ (-((value&1) as i64));
            idx += 5;
        }
    } else {
        if !bits[2] {
            // read 3 bits
            idx += 3;
            // read 9 bits
            for i in 0..9 {
                value |= (bits[idx+i] as u64) << i;
            }
            result += ((value >> 1) as i64) ^ (-((value&1) as i64));
            idx += 9;
        } else {
            if !bits[3]{
                // read 4 bits
                idx += 4;
                // read 16 bits
                for i in 0..16 {
                    value |= (bits[idx+i] as u64) << i;
                }
                result += ((value >> 1) as i64) ^ (-((value&1) as i64));
                idx += 16;
            } else {
                // read 4 bits
                idx += 4;
                // read 64 bits
                for i in 0..64{
                    value |= (bits[idx+i] as u64) << i;
                }
                result += ((value >> 1) as i64) ^ (-((value&1) as i64));
                idx += 64;
            }
        }
    }
    if bits.len() > idx {
        Ok((result, Some(&bits[idx..])))
    } else {
        Ok((result, None))
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
        // println!("value: {}", value);
        encode_deltadelta_i8(value, &mut bits);
        // println!("bits: {}", bits);
        let (decoded, remaining) = decode_deltadelta_i8(&bits).unwrap();
        assert_eq!(value, decoded);
        assert!(remaining.is_none());
    }

    fn encode_decode_i16(value: i16) {
        let mut bits = BitBuffer::new();
        // println!("value: {}", value);
        encode_deltadelta_i16(value, &mut bits);
        // println!("bits: {}", bits);
        let (decoded, remaining) = decode_deltadelta_i16(&bits).unwrap();
        assert_eq!(value, decoded);
        assert!(remaining.is_none());
    }

    fn encode_decode_i32(value: i32) {
        let mut bits = BitBuffer::new();
        // println!("value: {}", value);
        encode_deltadelta_i32(value, &mut bits);
        // println!("bits: {}", bits);
        let (decoded, remaining) = decode_deltadelta_i32(&bits).unwrap();
        assert_eq!(value, decoded);
        assert!(remaining.is_none());
    }

    fn encode_decode_i64(value: i64) {
        let mut bits = BitBuffer::new();
        // println!("value: {}", value);
        encode_deltadelta_i64(value, &mut bits);
        // println!("bits: {}", bits);
        let (decoded, remaining) = decode_deltadelta_i64(&bits).unwrap();
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
