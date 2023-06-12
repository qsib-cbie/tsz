use crate::prelude::{BitBuffer, BitBufferSlice};

pub fn encode_delta_i8(values: &mut[i8], out: &mut BitBuffer) {
    if values.len() == 0 {
        return Err("Not enough values to encode");
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
            let val = values[i] as i8;
            let encoded: ((value << 1) ^ (value >> 127)) as u8;
            for i in 0..7 {
                out.push(encoded & (1 << i) != 0);
            }
        }
    } else if values.len() == 8 {
        // 8 samples of 4 bits
        for _ in 0..2 {
            out.push(true);
        }
        out.push(false);
        for i in 0..7 {
            let val = values[i] as i8;
            let encoded: ((value << 1) ^ (value >> 7)) as u8;
            for i in 0..3 {
                out.push(encoded & (1 << i) != 0);
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
            let val = values[i] as i8;
            let encoded: ((value << 1) ^ (value >> 7)) as u8;
            for i in 0..2 {
                out.push(encoded & (1 << i) != 0);
            }
        }
    } else {
        return Err("Unsupported number of samples");
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
            let encoded: ((value << 1) ^ (value >> 15)) as u16;
            for i in 0..15 {
                out.push(encoded & (1 << i) != 0);
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
            let encoded: ((value << 1) ^ (value >> 15)) as u16;
            for i in 0..9 {
                out.push(encoded & (1 << i) != 0);
            }
        }
    }
}

pub fn decode_delta_i8(
    bits: &'_ BitBufferSlice,
) -> Result<(&'mut [i8], Option<&'_ BitBufferSlice>), &'static str> {
    if !bits[0]{
        return Err("Not valid for delta decoding");
    }
    if !bits[1] {
        return Err("Not enough bits to decode");
    }
    let mut idx = 0;
    if !bits[2] {
        let mut values = [0i8; 4];
        // read 4 bits
        idx += 4;
        // for 4 samples
        for i in 0..3 {
            let mut value = 0;
            // read 8 bits
            for j in 0..7 {
                value |= (bits[idx+j]) << i; 
            }
            value = (value >> 1) ^ (-(n&1));
            values[i] = value;
        }
    } else if !bits[3] {
        // 8 samples of 4 bits
        let mut values = [0i8; 8];
        // read 4 bits
        idx += 4;
        // for 8 samples
        for i in 0..7 {
            let mut value = 0;
            // read 4 bits
            for j in 0..3 {
                value |= (bits[idx+j]) << i; 
            }
            value = (value >> 1) ^ (-(n&1));
            values[i] = value;
        }
    } else {
        // 10 samples of 3 bits
        let mut values = [0i8; 10];
        // read 6 bits
        idx += 6;
        // for 10 samples
        for i in 0..9 {
            let mut value = 0;
            // read 3 bits
            for j in 0..2 {
                value |= (bits[idx+j]) << i; 
            }
            value = (value >> 1) ^ (-(n&1));
            values[i] = value;
        }
    }
    if bits.len() > idx {
        Ok((values, Some(&bits[idx..])))
    } else {
        Ok((values, None))
    }
 }

pub fn decode_delta_i16(
    bits: &'_ BitBufferSlice,
) -> Result<(&'mut [i16], Option<&'_ BitBufferSlice>), &'static str> {
    
}

pub fn decode_delta_i32(
    bits: &'_ BitBufferSlice,
) -> Result<(&'mut [i32], Option<&'_ BitBufferSlice>), &'static str> {
    
}

pub fn decode_delta_i64(
    bits: &'_ BitBufferSlice,
) -> Result<(&'mut [i64], Option<&'_ BitBufferSlice>), &'static str> {
    
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
