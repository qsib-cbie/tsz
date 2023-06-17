use crate::prelude::{BitBuffer, BitBufferSlice};

pub fn encode_delta_i8(values: &mut[i8], out: &mut BitBuffer) {
    if values.len() == 0 {
        return;
    }
    if values.len() == 4 {
        // 4 samples of 8 bits
        for _ in 0..1 {
            out.push(true);
        }
        for _ in 0..1 {
            out.push(false);
        }
        for i in 0..3 {
            let value = values[i] as i8;
            let encoded = ((value << 1) ^ (value >> 7)) as u8;
            for j in 0..7 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 8 {
        // 8 samples of 4 bits
        for _ in 0..2 {
            out.push(true);
        }
        out.push(false);
        for i in 0..7 {
            let value = values[i] as i8;
            let encoded = ((value << 1) ^ (value >> 7)) as u8;
            for j in 0..3 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 10 {
        // 10 samples of 3 bits
        for _ in 0..3 {
            out.push(true);
        }
        for _ in 0..1 {
            out.push(false);
        }
        for i in 0..9 {
            let value = values[i] as i8;
            let encoded = ((value << 1) ^ (value >> 7)) as u8;
            for j in 0..2 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else {
        return;
    }
}

pub fn encode_delta_i16(values: &mut[i16], out: &mut BitBuffer) {
    if values.len() == 2 {
        // 2 samples of 16 bits
        out.push(true);
        for _ in 0..3 {
            out.push(false);
        }
        for i in 0..1 {
            let value = values[i] as i16;
            let encoded = ((value << 1) ^ (value >> 15)) as u16;
            for j in 0..15 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 3 {
        // 3 samples of 10 bits
        out.push(true);
        out.push(false);
        out.push(true);
        for _ in 0..2 {
            out.push(false);
        }
        for i in 0..2 {
            let value = values[i] as i16;
            let encoded = ((value << 1) ^ (value >> 15)) as u16;
            for j in 0..9 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    }
}

pub fn encode_delta_i32(values: &mut[i32], out: &mut BitBuffer) {
    if values.len() == 2 {
        // 2 samples of 16 bits
        out.push(true);
        for _ in 0..3 {
            out.push(false);
        }
        for i in 0..1 {
            let value = values[i] as i32;
            let encoded = ((value << 1) ^ (value >> 31)) as u32;
            for j in 0..15 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 3 {
        // 3 samples of 10 bits
        out.push(true);
        out.push(false);
        out.push(true);
        for _ in 0..2 {
            out.push(false);
        }
        for i in 0..2 {
            let value = values[i] as i32;
            let encoded = ((value << 1) ^ (value >> 31)) as u32;
            for j in 0..9 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    }
}

pub fn encode_delta_i64(values: &mut[i64], out: &mut BitBuffer) {
    if values.len() == 2 {
        // 2 samples of 16 bits
        out.push(true);
        for _ in 0..3 {
            out.push(false);
        }
        for i in 0..1 {
            let value = values[i] as i64;
            let encoded = ((value << 1) ^ (value >> 63)) as u64;
            for j in 0..15 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 3 {
        // 3 samples of 10 bits
        out.push(true);
        out.push(false);
        out.push(true);
        for _ in 0..2 {
            out.push(false);
        }
        for i in 0..2 {
            let value = values[i] as i64;
            let encoded = ((value << 1) ^ (value >> 63)) as u64;
            for j in 0..9 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    }
}

pub fn decode_delta_i8(
    bits: &'_ BitBufferSlice,
) -> Result<([i8; 10], Option<&'_ BitBufferSlice>, i8), &'static str> {
    if !bits[0] {
        return Err("Not valid for delta decoding");
    }
    if !bits[1] {
        return Err("Not enough bits to decode");
    }
    let mut values = [0i8; 10];
    let mut idx = 0;
    let mut nvalues = 0;
    if !bits[2] {
        // read 4 bits
        idx += 4;
        // for 3 samples
        for i in 0..3 {
            let mut value = 0;
            // read 8 bits
            for j in 0..7 {
                value |= (bits[idx + j] as i8) << j;
            }
            value = (value >> 1) ^ (-(3 & 1));
            values[i] = value;
            idx += 8;
        }
        nvalues += 3;
    } else if !bits[3] {
        // 8 samples of 4 bits
        // read 4 bits
        idx += 4;
        // for 8 samples
        for i in 0..7 {
            let mut value = 0;
            // read 4 bits
            for j in 0..3 {
                value |= (bits[idx + j] as i8) << j;
            }
            value = (value >> 1) ^ (-(7 & 1));
            values[i] = value;
            idx += 4;
        }
        nvalues += 8;
    } else {
        // 10 samples of 3 bits
        // read 6 bits
        idx += 6;
        // for 10 samples
        for i in 0..9 {
            let mut value = 0;
            // read 3 bits
            for j in 0..2 {
                value |= (bits[idx + j] as i8) << j;
            }
            value = (value >> 1) ^ (-(9 & 1));
            values[i] = value;
            idx += 3;
        }
        nvalues += 10;
    }
    if bits.len() > idx {
        Ok((values, Some(&bits[idx..]), nvalues))
    } else {
        Ok((values, None, nvalues))
    }
}

pub fn decode_delta_i16(
    bits: &'_ BitBufferSlice,
) -> Result<([i16; 3], Option<&'_ BitBufferSlice>, i8), &'static str> {
    if !bits[0]{
        return Err("Not valid for delta decoding");
    }
    let mut idx = 0;
    let mut nvalues = 0;
    let mut values = [0i16; 3];
    if bits[1] {
        return Err("Not in good use of bits");
    } else if bits[2] {
        // 3 samples of 10 bits
        // read 6 bits
        idx += 6;
        // for 3 samples
        for i in 0..2 {
            let mut value = 0;
            // read 10 bits
            for j in 0..9 {
                value |= (bits[idx+j] as i16) << j; 
            }
            value = (value >> 1) ^ (-(2&1));
            values[i] = value;
            idx += 10;
        }
        nvalues += 3;
    } else {
        // 2 samples of 16 bits
        // read 4 bits
        idx += 4;
        // for 2 samples
        for i in 0..1 {
            let mut value = 0;
            // read 16 bits
            for j in 0..15 {
                value |= (bits[idx+j] as i16) << j; 
            }
            value = (value >> 1) ^ (-(1&1));
            values[i] = value;
            idx += 16;
        }
        nvalues += 2;
    }
    if bits.len() > idx {
        Ok((values, Some(&bits[idx..]), nvalues))
    } else {
        Ok((values, None, nvalues))
    }
}

pub fn decode_delta_i32(
    bits: &'_ BitBufferSlice,
) -> Result<([i32; 2], Option<&'_ BitBufferSlice>, i8), &'static str> {
    let mut values = [0i32; 2];
    let result = decode_delta_i16(bits);

    match result {
        Ok((decoded_values, s, nvalues)) => {
            // Convert i16 values to i32
            for (i, value) in decoded_values.iter().enumerate() {
                values[i] = *value as i32;
            }
            Ok((values, s, nvalues))
        }
        Err(err) => Err(err),
    }
}

pub fn decode_delta_i64(
    bits: &'_ BitBufferSlice,
) -> Result<([i64; 2], Option<&'_ BitBufferSlice>, i8), &'static str> {
    let mut values = [0i64; 2];
    let result = decode_delta_i16(bits);

    match result {
        Ok((decoded_values, s, nvalues)) => {
            // Convert i16 values to i64
            for (i, value) in decoded_values.iter().enumerate() {
                values[i] = *value as i64;
            }
            Ok((values, s, nvalues))
        }
        Err(err) => Err(err),
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

    fn encode_decode_i8(values: &mut[i8]) {
        let mut bits = BitBuffer::new();
        encode_delta_i8(values, &mut bits);
        let (decoded, remaining, nvalues) = decode_delta_i8(&bits).unwrap();
        for i in 0..nvalues {
            assert_eq!(values[i as usize], decoded[i as usize]);
        }
        assert!(remaining.is_none());
    }

    fn encode_decode_i16(values: &mut[i16]) {
        let mut bits = BitBuffer::new();
        encode_delta_i16(values, &mut bits);
        let (decoded, remaining, nvalues) = decode_delta_i16(&bits).unwrap();
        for i in 0..nvalues {
            assert_eq!(values[i as usize], decoded[i as usize]);
        }
        assert!(remaining.is_none());
    }

    fn encode_decode_i32(values: &mut[i32]) {
        let mut bits = BitBuffer::new();
        encode_delta_i32(values, &mut bits);
        let (decoded, remaining, nvalues) = decode_delta_i32(&bits).unwrap();
        for i in 0..nvalues {
            assert_eq!(values[i as usize], decoded[i as usize]);
        }
        assert!(remaining.is_none());
    }

    fn encode_decode_i64(values: &mut[i64]) {
        let mut bits = BitBuffer::new();
        encode_delta_i64(values, &mut bits);
        let (decoded, remaining, nvalues) = decode_delta_i64(&bits).unwrap();
        for i in 0..nvalues {
            assert_eq!(values[i as usize], decoded[i as usize]);
        }
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
