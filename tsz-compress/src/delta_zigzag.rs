use crate::prelude::{BitBuffer, BitBufferSlice};

pub fn encode_delta_i8(values: &mut[i8], out: &mut BitBuffer) {
    if values.len() == 4 {
        // 4 samples of 8 bits
        // header is 1100
        for _ in 0..2 {
            out.push(true);
        }
        for _ in 0..2 {
            out.push(false);
        }
        for i in 0..4 {
            let value = values[i] as i8;
            let encoded = ((value << 1) ^ (value >> 7)) as u8;
            for j in 0..8 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 8 {
        // 8 samples of 4 bits
        // header is 1110
        for _ in 0..3 {
            out.push(true);
        }
        out.push(false);
        for i in 0..8 {
            let value = values[i] as i8;
            let encoded = ((value << 1) ^ (value >> 7)) as u8;
            for j in 0..4 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 10 {
        // 10 samples of 3 bits
        // header is 1111, padded with 00
        for _ in 0..4 {
            out.push(true);
        }
        for _ in 0..2 {
            out.push(false);
        }
        for i in 0..10 {
            let value = values[i] as i8;
            let encoded = ((value << 1) ^ (value >> 7)) as u8;
            for j in 0..3 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    }
}

pub fn encode_delta_i16(values: &mut[i16], out: &mut BitBuffer) {
    if values.len() == 2 {
        // 2 samples of 16 bits
        out.push(true);
        for _ in 0..3 {
            out.push(false);
        }
        for i in 0..2 {
            let value = values[i] as i16;
            let encoded = ((value << 1) ^ (value >> 15)) as u16;
            for j in 0..16 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 3 {
        // 3 samples of 10 bits
        out.push(true);
        out.push(false);
        out.push(true);
        for _ in 0..3 {
            out.push(false);
        }
        for i in 0..3 {
            let value = values[i] as i16;
            let encoded = ((value << 1) ^ (value >> 15)) as u16;
            for j in 0..10 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 4 {
        // 4 samples of 8 bits
        // header is 1100
        for _ in 0..2 {
            out.push(true);
        }
        for _ in 0..2 {
            out.push(false);
        }
        for i in 0..4 {
            let value = values[i] as i16;
            let encoded = ((value << 1) ^ (value >> 7)) as u16;
            for j in 0..8 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 8 {
        // 8 samples of 4 bits
        // header is 1110
        for _ in 0..3 {
            out.push(true);
        }
        out.push(false);
        for i in 0..8 {
            let value = values[i] as i16;
            let encoded = ((value << 1) ^ (value >> 7)) as u16;
            for j in 0..4 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 10 {
        // 10 samples of 3 bits
        // header is 1111, padded with 00
        for _ in 0..4 {
            out.push(true);
        }
        for _ in 0..2 {
            out.push(false);
        }
        for i in 0..10 {
            let value = values[i] as i8;
            let encoded = ((value << 1) ^ (value >> 7)) as u16;
            for j in 0..3 {
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
        for i in 0..2 {
            let value = values[i] as i32;
            let encoded = ((value << 1) ^ (value >> 31)) as u32;
            for j in 0..16 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 3 {
        // 3 samples of 10 bits
        out.push(true);
        out.push(false);
        out.push(true);
        for _ in 0..3 {
            out.push(false);
        }
        for i in 0..3 {
            let value = values[i] as i32;
            let encoded = ((value << 1) ^ (value >> 31)) as u32;
            for j in 0..10 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 4 {
        // 4 samples of 8 bits
        // header is 1100
        for _ in 0..2 {
            out.push(true);
        }
        for _ in 0..2 {
            out.push(false);
        }
        for i in 0..4 {
            let value = values[i] as i32;
            let encoded = ((value << 1) ^ (value >> 31)) as u32;
            for j in 0..8 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 8 {
        // 8 samples of 4 bits
        // header is 1110
        for _ in 0..3 {
            out.push(true);
        }
        out.push(false);
        for i in 0..8 {
            let value = values[i] as i32;
            let encoded = ((value << 1) ^ (value >> 31)) as u32;
            for j in 0..4 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 10 {
        // 10 samples of 3 bits
        // header is 1111, padded with 00
        for _ in 0..4 {
            out.push(true);
        }
        for _ in 0..2 {
            out.push(false);
        }
        for i in 0..10 {
            let value = values[i] as i32;
            let encoded = ((value << 1) ^ (value >> 31)) as u32;
            for j in 0..3 {
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
        for i in 0..2 {
            let value = values[i] as i64;
            let encoded = ((value << 1) ^ (value >> 63)) as u64;
            for j in 0..16 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 3 {
        // 3 samples of 10 bits
        out.push(true);
        out.push(false);
        out.push(true);
        for _ in 0..3 {
            out.push(false);
        }
        for i in 0..3 {
            let value = values[i] as i64;
            let encoded = ((value << 1) ^ (value >> 63)) as u64;
            for j in 0..10 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 4 {
        // 4 samples of 8 bits
        // header is 1100
        for _ in 0..2 {
            out.push(true);
        }
        for _ in 0..2 {
            out.push(false);
        }
        for i in 0..4 {
            let value = values[i] as i64;
            let encoded = ((value << 1) ^ (value >> 63)) as u64;
            for j in 0..8 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 8 {
        // 8 samples of 4 bits
        // header is 1110
        for _ in 0..3 {
            out.push(true);
        }
        out.push(false);
        for i in 0..8 {
            let value = values[i] as i64;
            let encoded = ((value << 1) ^ (value >> 63)) as u64;
            for j in 0..4 {
                out.push(encoded & (1 << j) != 0);
            }
        }
    } else if values.len() == 10 {
        // 10 samples of 3 bits
        // header is 1111, padded with 00
        for _ in 0..4 {
            out.push(true);
        }
        for _ in 0..2 {
            out.push(false);
        }
        for i in 0..10 {
            let value = values[i] as i64;
            let encoded = ((value << 1) ^ (value >> 63)) as u64;
            for j in 0..3 {
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
        // 4 samples of 8 bits
        // read 4 bits
        idx += 4;
        // for 4 samples
        for i in 0..4 {
            let mut value = 0;
            // read 8 bits
            for j in 0..8 {
                value |= (bits[idx + j] as u8) << j;
            }
            let result = ((value >> 1) as i8) ^ (-((value & 1) as i8));
            values[i] = result;
            idx += 8;
        }
        nvalues += 3;
    } else if !bits[3] {
        // 8 samples of 4 bits
        // read 4 bits
        idx += 4;
        // for 8 samples
        for i in 0..8 {
            let mut value = 0;
            // read 4 bits
            for j in 0..4 {
                value |= (bits[idx + j] as u8) << j;
            }
            let result = ((value >> 1) as i8) ^ (-((value & 1) as i8));
            values[i] = result;
            idx += 4;
        }
        nvalues += 8;
    } else {
        // 10 samples of 3 bits
        // read 6 bits
        idx += 6;
        // for 10 samples
        for i in 0..10 {
            let mut value = 0;
            // read 3 bits
            for j in 0..3 {
                value |= (bits[idx + j] as u8) << j;
            }
            let result = ((value >> 1) as i8) ^ (-((value & 1) as i8));
            values[i] = result;
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
) -> Result<([i16; 10], Option<&'_ BitBufferSlice>, i8), &'static str> {
    if !bits[0]{
        return Err("Not valid for delta decoding");
    }
    let mut idx = 0;
    let mut nvalues = 0;
    let mut values = [0i16; 10];
    if bits[1]{
        if !bits[2] {
            // 4 samples of 8 bits
            // read 4 bits
            idx += 4;
            // for 4 samples
            for i in 0..4 {
                let mut value = 0;
                // read 8 bits
                for j in 0..8 {
                    value |= (bits[idx + j] as u16) << j;
                }
                let result = ((value >> 1) as i16) ^ (-((value & 1) as i16));
                values[i] = result;
                idx += 8;
            }
            nvalues += 3;
        } else if !bits[3] {
            // 8 samples of 4 bits
            // read 4 bits
            idx += 4;
            // for 8 samples
            for i in 0..8 {
                let mut value = 0;
                // read 4 bits
                for j in 0..4 {
                    value |= (bits[idx + j] as u16) << j;
                }
                let result = ((value >> 1) as i16) ^ (-((value & 1) as i16));
                values[i] = result;
                idx += 4;
            }
            nvalues += 8;
        } else {
            // 10 samples of 3 bits
            // read 6 bits
            idx += 6;
            // for 10 samples
            for i in 0..10 {
                let mut value = 0;
                // read 3 bits
                for j in 0..3 {
                    value |= (bits[idx + j] as u16) << j;
                }
                let result = ((value >> 1) as i16) ^ (-((value & 1) as i16));
                values[i] = result;
                idx += 3;
            }
            nvalues += 10;
        }
    } else if bits[2] {
        // 3 samples of 10 bits
        // read 6 bits
        idx += 6;
        // for 3 samples
        for i in 0..3 {
            let mut value = 0;
            // read 10 bits
            for j in 0..10 {
                value |= (bits[idx+j] as u16) << j; 
            }
            let result = ((value >> 1) as i16) ^ (-((value&1) as i16));
            values[i] = result;
            idx += 10;
        }
        nvalues += 3;
    } else {
        // 2 samples of 16 bits
        // read 4 bits
        idx += 4;
        // for 2 samples
        for i in 0..2 {
            let mut value = 0;
            // read 16 bits
            for j in 0..16 {
                value |= (bits[idx+j] as u16) << j; 
            }
            let result = ((value >> 1) as i16) ^ (-((value&1) as i16));
            values[i] = result;
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
) -> Result<([i32; 10], Option<&'_ BitBufferSlice>, i8), &'static str> {
    let mut values = [0i32; 10];
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
) -> Result<([i64; 10], Option<&'_ BitBufferSlice>, i8), &'static str> {
    let mut values = [0i64; 10];
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
        // println!("Encoded: {}", bits);
        let (decoded, remaining, nvalues) = decode_delta_i8(&bits).unwrap();
        for i in 0..nvalues {
            // println!("{} {}", values[i as usize], decoded[i as usize]);
            assert_eq!(values[i as usize], decoded[i as usize]);
        }
        assert!(remaining.is_none());
    }

    fn encode_decode_i16(values: &mut[i16]) {
        let mut bits = BitBuffer::new();
        encode_delta_i16(values, &mut bits);
        // println!("Encoded: {}", bits);
        let (decoded, remaining, nvalues) = decode_delta_i16(&bits).unwrap();
        for i in 0..nvalues {
            // println!("{} {}", values[i as usize], decoded[i as usize]);
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
        for i in -4..4 {
            let mut values: [i8; 10] = [0; 10];
            for j in 0..10 {
                values[j] = i;
            }
            encode_decode_i8(&mut values[..]);
        }
        for i in -8..8 {
            let mut values: [i8; 8] = [0; 8];
            for j in 0..8 {
                values[j] = i;
            }
            encode_decode_i8(&mut values[..]);
        }
        for i in -128..=127 {
            let mut values: [i8; 4] = [0; 4];
            for j in 0..4 {
                values[j] = i;
            }
            encode_decode_i8(&mut values[..]);
        }
    }

    #[test]
    fn can_encode_decode_all_i16() {
        for i in -4..4 {
            let mut values: [i16; 10] = [0; 10];
            for j in 0..10 {
                values[j] = i;
            }
            encode_decode_i16(&mut values[..]);
        }
        for i in -8..8 {
            let mut values: [i16; 8] = [0; 8];
            for j in 0..8 {
                values[j] = i;
            }
            encode_decode_i16(&mut values[..]);
        }
        for i in -128..128 {
            let mut values: [i16; 4] = [0; 4];
            for j in 0..4 {
                values[j] = i;
            }
            encode_decode_i16(&mut values[..]);
        }
        for i in -512..512 {
            let mut values: [i16; 3] = [0; 3];
            for j in 0..3 {
                values[j] = i;
            }
            encode_decode_i16(&mut values[..]);
        }
        for i in -32768..=32767 {
            let mut values: [i16; 2] = [0; 2];
            for j in 0..2 {
                values[j] = i;
            }
            encode_decode_i16(&mut values[..]);
        }
    }

    #[test]
    fn can_encode_decode_all_i32() {
        for i in -4..4 {
            let mut values: [i32; 10] = [0; 10];
            for j in 0..10 {
                values[j] = i;
            }
            encode_decode_i32(&mut values[..]);
        }
        for i in -8..8 {
            let mut values: [i32; 8] = [0; 8];
            for j in 0..8 {
                values[j] = i;
            }
            encode_decode_i32(&mut values[..]);
        }
        for i in -128..128 {
            let mut values: [i32; 4] = [0; 4];
            for j in 0..4 {
                values[j] = i;
            }
            encode_decode_i32(&mut values[..]);
        }
        for i in -512..512 {
            let mut values: [i32; 3] = [0; 3];
            for j in 0..3 {
                values[j] = i;
            }
            encode_decode_i32(&mut values[..]);
        }
        for i in -32768..32768 {
            let mut values: [i32; 2] = [0; 2];
            for j in 0..2 {
                values[j] = i;
            }
            encode_decode_i32(&mut values[..]);
        }
    }

    #[test]
    fn can_encode_decode_all_i64() {
        for i in -4..4 {
            let mut values: [i64; 10] = [0; 10];
            for j in 0..10 {
                values[j] = i;
            }
            encode_decode_i64(&mut values[..]);
        }
        for i in -8..8 {
            let mut values: [i64; 8] = [0; 8];
            for j in 0..8 {
                values[j] = i;
            }
            encode_decode_i64(&mut values[..]);
        }
        for i in -128..128 {
            let mut values: [i64; 4] = [0; 4];
            for j in 0..4 {
                values[j] = i;
            }
            encode_decode_i64(&mut values[..]);
        }
        for i in -512..512 {
            let mut values: [i64; 3] = [0; 3];
            for j in 0..3 {
                values[j] = i;
            }
            encode_decode_i64(&mut values[..]);
        }
        for i in -32768..32768 {
            let mut values: [i64; 2] = [0; 2];
            for j in 0..2 {
                values[j] = i;
            }
            encode_decode_i64(&mut values[..]);
        }
    }
}
