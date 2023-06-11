use crate::prelude::{BitBuffer, BitBufferSlice};

pub fn encode_delta_i8(mut value: i8, out: &mut BitBuffer) {
    
}

pub fn encode_delta_i16(mut value: i16, out: &mut BitBuffer) {
    
}

pub fn encode_delta_i32(mut value: i32, out: &mut BitBuffer) {
    
}

pub fn encode_delta_i64(mut value: i64, out: &mut BitBuffer) {
    
}

pub fn decode_delta_i8(
    bits: &'_ BitBufferSlice,
) -> Result<(i8, Option<&'_ BitBufferSlice>), &'static str> {
    
}

pub fn decode_delta_i16(
    bits: &'_ BitBufferSlice,
) -> Result<(i16, Option<&'_ BitBufferSlice>), &'static str> {
    
}

pub fn decode_delta_i32(
    bits: &'_ BitBufferSlice,
) -> Result<(i32, Option<&'_ BitBufferSlice>), &'static str> {
    
}

pub fn decode_delta_i64(
    bits: &'_ BitBufferSlice,
) -> Result<(i64, Option<&'_ BitBufferSlice>), &'static str> {
    
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
