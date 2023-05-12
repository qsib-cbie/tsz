use crate::prelude::{BitBuffer, BitBufferSlice};

///
/// An unsigned variable-length quantity.
///
pub struct Uvlq {
    pub bits: BitBuffer,
}

///
/// A reference to bits in a Uvlq.
///
pub struct UvlqRef<'a>(pub &'a BitBufferSlice);

///
/// Construct a Uvlq from an unsigned.
/// Construct an unsigned from a Uvlq.
///
/// When constructing a Uvlq:
/// * Take the minimum trailing bits of the input value,
/// starting from the MSB that is set.
/// * At least one Uvlq byte is always produced.
///
/// When constructing an unsigned:
/// * Fails if the Uvlq is longer than the bit width of the output type.
///
macro_rules! impl_unsigned_uvlq {
    ($unsigned:ident) => {
        impl From<$unsigned> for Uvlq {
            fn from(value: $unsigned) -> Self {
                let mut bits = BitBuffer::with_capacity(8);
                let mut value = value;
                while value > 0 {
                    let mut byte = (value & 0x7f) as u8;
                    value >>= 7;
                    if value > 0 {
                        byte |= 0x80;
                    }
                    byte = byte.reverse_bits();
                    for _ in 0..8 {
                        bits.push(byte & 1 == 1);
                        byte >>= 1;
                    }
                }
                if bits.len() == 0 {
                    for _ in 0..8 {
                        bits.push(false);
                    }
                }
                Self { bits }
            }
        }

        impl TryFrom<Uvlq> for $unsigned {
            type Error = ();

            fn try_from(value: Uvlq) -> Result<Self, Self::Error> {
                let mut out: Self = 0;
                let mut out_idx = 0;
                for (vlq_idx, vlq_byte) in value.bits.chunks_exact(8).enumerate() {
                    let extra_bits = if out_idx + 7 > Self::BITS {
                        let extra = (out_idx + 7 - Self::BITS) as usize;
                        let overflow = vlq_byte.iter().skip(1).take(extra).any(|b| *b);
                        if overflow {
                            return Err(());
                        }
                        extra
                    } else {
                        0
                    };

                    let mut val: Self = 0;
                    for bit in vlq_byte.iter().skip(1 + extra_bits) {
                        val <<= 1;
                        if *bit {
                            if out_idx >= Self::BITS {
                                return Err(());
                            }
                            val |= 1;
                        }
                        out_idx += 1;
                    }
                    out |= val << (7 * vlq_idx);
                }
                Ok(out)
            }
        }
    };
}

macro_rules! impl_unsigned_uvlq_ref {
    ($unsigned:ident) => {
        impl TryFrom<UvlqRef<'_>> for ($unsigned, usize) {
            type Error = &'static str;

            fn try_from(value: UvlqRef) -> Result<Self, Self::Error> {
                let mut out: $unsigned = 0;
                let mut out_idx = 0;
                let mut consumed = 0;
                for (vlq_idx, vlq_byte) in value.0.chunks_exact(8).enumerate() {
                    consumed += 8;
                    let extra_bits = if out_idx + 7 > $unsigned::BITS {
                        let extra = (out_idx + 7 - $unsigned::BITS) as usize;
                        let overflow = vlq_byte.iter().skip(1).take(extra).any(|b| *b);
                        if overflow {
                            return Err("Unsigned VLQ bit overflow");
                        }
                        extra
                    } else {
                        0
                    };

                    let mut val: $unsigned = 0;
                    for bit in vlq_byte.iter().skip(1 + extra_bits) {
                        val <<= 1;
                        if *bit {
                            if out_idx >= $unsigned::BITS {
                                return Err("Unsigned VLQ bit overflow");
                            }
                            val |= 1;
                        }
                        out_idx += 1;
                    }
                    out |= val << (7 * vlq_idx);

                    if !vlq_byte[0] {
                        break;
                    }
                }
                Ok((out, consumed))
            }
        }
    };
}

impl_unsigned_uvlq!(u8);
impl_unsigned_uvlq!(u16);
impl_unsigned_uvlq!(u32);
impl_unsigned_uvlq!(u64);
impl_unsigned_uvlq!(u128);

impl_unsigned_uvlq_ref!(u8);
impl_unsigned_uvlq_ref!(u16);
impl_unsigned_uvlq_ref!(u32);
impl_unsigned_uvlq_ref!(u64);
impl_unsigned_uvlq_ref!(u128);

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{prelude::BitBuffer, uvlq::UvlqRef};

    #[test]
    fn can_encode_0() {
        let uvlq = super::Uvlq::from(0u32);
        assert_eq!(uvlq.bits.len(), 8);
        assert_eq!(uvlq.bits[0], false);
        assert_eq!(uvlq.bits[1], false);
        assert_eq!(uvlq.bits[2], false);
        assert_eq!(uvlq.bits[3], false);
        assert_eq!(uvlq.bits[4], false);
        assert_eq!(uvlq.bits[5], false);
        assert_eq!(uvlq.bits[6], false);
        assert_eq!(uvlq.bits[7], false);

        let uvlq = super::Uvlq::from(0u64);
        assert_eq!(uvlq.bits.len(), 8);
        assert_eq!(uvlq.bits[0], false);
        assert_eq!(uvlq.bits[1], false);
        assert_eq!(uvlq.bits[2], false);
        assert_eq!(uvlq.bits[3], false);
        assert_eq!(uvlq.bits[4], false);
        assert_eq!(uvlq.bits[5], false);
        assert_eq!(uvlq.bits[6], false);
        assert_eq!(uvlq.bits[7], false);
    }

    #[test]
    fn can_encode_1() {
        let uvlq = super::Uvlq::from(1u32);
        assert_eq!(uvlq.bits.len(), 8);
        assert_eq!(uvlq.bits[0], false);
        assert_eq!(uvlq.bits[1], false);
        assert_eq!(uvlq.bits[2], false);
        assert_eq!(uvlq.bits[3], false);
        assert_eq!(uvlq.bits[4], false);
        assert_eq!(uvlq.bits[5], false);
        assert_eq!(uvlq.bits[6], false);
        assert_eq!(uvlq.bits[7], true);

        let uvlq = super::Uvlq::from(1u64);
        assert_eq!(uvlq.bits.len(), 8);
        assert_eq!(uvlq.bits[0], false);
        assert_eq!(uvlq.bits[1], false);
        assert_eq!(uvlq.bits[2], false);
        assert_eq!(uvlq.bits[3], false);
        assert_eq!(uvlq.bits[4], false);
        assert_eq!(uvlq.bits[5], false);
        assert_eq!(uvlq.bits[6], false);
        assert_eq!(uvlq.bits[7], true);
    }

    #[test]
    fn can_encode_127() {
        let uvlq = super::Uvlq::from(127u32);
        assert_eq!(uvlq.bits.len(), 8);
        assert_eq!(uvlq.bits[0], false);
        assert_eq!(uvlq.bits[1], true);
        assert_eq!(uvlq.bits[2], true);
        assert_eq!(uvlq.bits[3], true);
        assert_eq!(uvlq.bits[4], true);
        assert_eq!(uvlq.bits[5], true);
        assert_eq!(uvlq.bits[6], true);
        assert_eq!(uvlq.bits[7], true);

        let uvlq = super::Uvlq::from(127u64);
        assert_eq!(uvlq.bits.len(), 8);
        assert_eq!(uvlq.bits[0], false);
        assert_eq!(uvlq.bits[1], true);
        assert_eq!(uvlq.bits[2], true);
        assert_eq!(uvlq.bits[3], true);
        assert_eq!(uvlq.bits[4], true);
        assert_eq!(uvlq.bits[5], true);
        assert_eq!(uvlq.bits[6], true);
        assert_eq!(uvlq.bits[7], true);
    }

    #[test]
    fn can_encode_128() {
        let uvlq = super::Uvlq::from(128u32);
        assert_eq!(uvlq.bits.len(), 16);
        assert_eq!(uvlq.bits[0], true);
        assert_eq!(uvlq.bits[1], false);
        assert_eq!(uvlq.bits[2], false);
        assert_eq!(uvlq.bits[3], false);
        assert_eq!(uvlq.bits[4], false);
        assert_eq!(uvlq.bits[5], false);
        assert_eq!(uvlq.bits[6], false);
        assert_eq!(uvlq.bits[7], false);
        assert_eq!(uvlq.bits[8], false);
        assert_eq!(uvlq.bits[9], false);
        assert_eq!(uvlq.bits[10], false);
        assert_eq!(uvlq.bits[11], false);
        assert_eq!(uvlq.bits[12], false);
        assert_eq!(uvlq.bits[13], false);
        assert_eq!(uvlq.bits[14], false);
        assert_eq!(uvlq.bits[15], true);

        let uvlq = super::Uvlq::from(128u64);
        assert_eq!(uvlq.bits.len(), 16);
        assert_eq!(uvlq.bits[0], true);
        assert_eq!(uvlq.bits[1], false);
        assert_eq!(uvlq.bits[2], false);
        assert_eq!(uvlq.bits[3], false);
        assert_eq!(uvlq.bits[4], false);
        assert_eq!(uvlq.bits[5], false);
        assert_eq!(uvlq.bits[6], false);
        assert_eq!(uvlq.bits[7], false);
        assert_eq!(uvlq.bits[8], false);
        assert_eq!(uvlq.bits[9], false);
        assert_eq!(uvlq.bits[10], false);
        assert_eq!(uvlq.bits[11], false);
        assert_eq!(uvlq.bits[12], false);
        assert_eq!(uvlq.bits[13], false);
        assert_eq!(uvlq.bits[14], false);
        assert_eq!(uvlq.bits[15], true);
    }

    #[test]
    fn can_encode_33333() {
        let uvlq = super::Uvlq::from(33333u32);
        // 33333 = 0b10 0000100 0110101
        // vlq = 0b1 bottom 7, 0b1 next bottom 7, 0b0 remaining 7
        assert_eq!(uvlq.bits.len(), 24);
        let str = uvlq
            .bits
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>();
        let expected = "101101011000010000000010".to_string();
        assert_eq!(str, expected);

        let uvlq = super::Uvlq::from(33333u64);
        // 33333 = 0b10 0000100 0110101
        // vlq = 0b1 bottom 7, 0b1 next bottom 7, 0b0 remaining 7
        assert_eq!(uvlq.bits.len(), 24);
        let str = uvlq
            .bits
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>();
        let expected = "101101011000010000000010".to_string();
        assert_eq!(str, expected);
    }

    #[test]
    fn can_decode_0() {
        // u8
        let uvlq = super::Uvlq::from(0u32);
        let byte = u8::try_from(uvlq).unwrap();
        assert_eq!(byte, 0);

        // u16
        let uvlq = super::Uvlq::from(0u32);
        let byte = u16::try_from(uvlq).unwrap();
        assert_eq!(byte, 0);

        // u32
        let uvlq = super::Uvlq::from(0u32);
        let byte = u32::try_from(uvlq).unwrap();
        assert_eq!(byte, 0);

        // u64
        let uvlq = super::Uvlq::from(0u32);
        let byte = u64::try_from(uvlq).unwrap();
        assert_eq!(byte, 0);
    }

    #[test]
    fn can_decode_1() {
        // u8
        let uvlq = super::Uvlq::from(1u64);
        let byte = u8::try_from(uvlq).unwrap();
        assert_eq!(byte, 1);

        // u16
        let uvlq = super::Uvlq::from(1u64);
        let byte = u16::try_from(uvlq).unwrap();
        assert_eq!(byte, 1);

        // u32
        let uvlq = super::Uvlq::from(1u64);
        let byte = u32::try_from(uvlq).unwrap();
        assert_eq!(byte, 1);

        // u64
        let uvlq = super::Uvlq::from(1u64);
        let byte = u64::try_from(uvlq).unwrap();
        assert_eq!(byte, 1);
    }

    #[test]
    fn can_decode_127() {
        let uvlq = super::Uvlq::from(127u32);
        let byte = u8::try_from(uvlq).unwrap();
        assert_eq!(byte, 127);

        let uvlq = super::Uvlq::from(127u32);
        let byte = u16::try_from(uvlq).unwrap();
        assert_eq!(byte, 127);

        let uvlq = super::Uvlq::from(127u32);
        let byte = u32::try_from(uvlq).unwrap();
        assert_eq!(byte, 127);

        let uvlq = super::Uvlq::from(127u32);
        let byte = u64::try_from(uvlq).unwrap();
        assert_eq!(byte, 127);
    }

    #[test]
    fn can_decode_128() {
        let uvlq = super::Uvlq::from(128u64);
        let byte = u8::try_from(uvlq).unwrap();
        assert_eq!(byte, 128);

        let uvlq = super::Uvlq::from(128u64);
        let byte = u16::try_from(uvlq).unwrap();
        assert_eq!(byte, 128);

        let uvlq = super::Uvlq::from(128u64);
        let byte = u32::try_from(uvlq).unwrap();
        assert_eq!(byte, 128);

        let uvlq = super::Uvlq::from(128u64);
        let byte = u64::try_from(uvlq).unwrap();
        assert_eq!(byte, 128);
    }

    #[test]
    fn can_decode_255() {
        let uvlq = super::Uvlq::from(255u32);
        let byte = u8::try_from(uvlq).unwrap();
        assert_eq!(byte, 255);

        let uvlq = super::Uvlq::from(255u32);
        let byte = u16::try_from(uvlq).unwrap();
        assert_eq!(byte, 255);

        let uvlq = super::Uvlq::from(255u32);
        let byte = u32::try_from(uvlq).unwrap();
        assert_eq!(byte, 255);

        let uvlq = super::Uvlq::from(255u32);
        let byte = u64::try_from(uvlq).unwrap();
        assert_eq!(byte, 255);
    }

    #[test]
    fn cant_decode_256_as_u8() {
        let uvlq = super::Uvlq::from(256u64);
        let byte = u8::try_from(uvlq);
        assert!(byte.is_err());
    }

    #[test]
    fn can_decode_256_above_u8() {
        let uvlq = super::Uvlq::from(256u32);
        let value = u16::try_from(uvlq).unwrap();
        assert_eq!(value, 256);

        let uvlq = super::Uvlq::from(256u32);
        let value = u32::try_from(uvlq).unwrap();
        assert_eq!(value, 256);

        let uvlq = super::Uvlq::from(256u32);
        let value = u64::try_from(uvlq).unwrap();
        assert_eq!(value, 256);
    }

    #[test]
    fn cant_decode_33333_as_u8() {
        let uvlq = super::Uvlq::from(33333u64);
        let byte = u8::try_from(uvlq);
        assert!(byte.is_err());
    }

    #[test]
    fn can_decode_33333_above_u8() {
        let uvlq = super::Uvlq::from(33333u64);
        let value = u16::try_from(uvlq).unwrap();
        assert_eq!(value, 33333);

        let uvlq = super::Uvlq::from(33333u64);
        let value = u32::try_from(uvlq).unwrap();
        assert_eq!(value, 33333);

        let uvlq = super::Uvlq::from(33333u64);
        let value = u64::try_from(uvlq).unwrap();
        assert_eq!(value, 33333);
    }

    #[test]
    fn can_convert_epoch_s() {
        let s = 1675528564;
        let uvlq = super::Uvlq::from(s);
        let value = u32::try_from(uvlq).unwrap();
        assert_eq!(value, s as u32);

        let uvlq = super::Uvlq::from(s);
        let value = u64::try_from(uvlq).unwrap();
        assert_eq!(value, s);
    }

    #[test]
    fn can_convert_epoch_ms() {
        let ms = 1675528564000;
        let uvlq = super::Uvlq::from(ms);
        let value = u32::try_from(uvlq);
        assert!(value.is_err());

        let uvlq = super::Uvlq::from(ms);
        let value = u64::try_from(uvlq).unwrap();
        assert_eq!(value, ms);
    }

    #[test]
    fn can_convert_epoch_us() {
        let us = 1675528564123456;
        let uvlq = super::Uvlq::from(us);
        let value = u32::try_from(uvlq);
        assert!(value.is_err());

        let uvlq = super::Uvlq::from(us);
        let value = u64::try_from(uvlq).unwrap();
        assert_eq!(value, us);
    }

    #[test]
    fn can_convert_all_u8() {
        for i in u8::MIN..=u8::MAX {
            // println!("i: {}", i);
            let svlq = super::Uvlq::from(i);
            let value = u8::try_from(svlq).unwrap();
            assert_eq!(value, i);
        }
    }

    #[test]
    fn can_convert_all_u16() {
        for i in u8::MIN..=u8::MAX {
            // println!("i: {}", i);
            let svlq = super::Uvlq::from(i);
            let value = u16::try_from(svlq).unwrap();
            assert_eq!(value, i as u16);
        }

        for i in u16::MIN..=u16::MAX {
            // println!("i: {}", i);
            let svlq = super::Uvlq::from(i);
            let value = u16::try_from(svlq).unwrap();
            assert_eq!(value, i);
        }
    }

    #[test]
    fn can_encode_all_u16() {
        let mut bits = BitBuffer::new();

        for i in u16::MIN..=u16::MAX {
            let svlq = super::Uvlq::from(i);
            bits.extend(svlq.bits.iter());
        }

        let mut bits = bits.as_bitslice();
        for i in u16::MIN..=u16::MAX {
            let (vu16, vu16_bits) = <(u16, usize)>::try_from(UvlqRef(bits)).unwrap();
            bits = &bits[vu16_bits..];
            assert_eq!(vu16, i);
        }
    }

    #[test]
    fn can_convert_all_u32() {
        for i in u8::MIN..=u8::MAX {
            // println!("i: {}", i);
            let svlq = super::Uvlq::from(i);
            let value = u32::try_from(svlq).unwrap();
            assert_eq!(value, i as u32);
        }

        for i in u16::MIN..=u16::MAX {
            // println!("i: {}", i);
            let svlq = super::Uvlq::from(i);
            let value = u32::try_from(svlq).unwrap();
            assert_eq!(value, i as u32);
        }

        let mut rng = rand::thread_rng();
        for _ in 0..100000 {
            let i = rng.gen_range(u32::MIN..=u32::MAX);
            let svlq = super::Uvlq::from(i);
            let value = u32::try_from(svlq).unwrap();
            assert_eq!(value, i as u32);
        }
    }

    #[test]
    fn can_convert_all_u64() {
        for i in u8::MIN..=u8::MAX {
            println!("i: {}", i);
            let svlq = super::Uvlq::from(i);
            let value = u64::try_from(svlq).unwrap();
            assert_eq!(value, i as u64);
        }

        for i in u16::MIN..=u16::MAX {
            println!("i: {}", i);
            let svlq = super::Uvlq::from(i);
            let value = u64::try_from(svlq).unwrap();
            assert_eq!(value, i as u64);
        }

        let mut rng = rand::thread_rng();
        for _ in 0..100000 {
            let i = rng.gen_range(u32::MIN..=u32::MAX);
            let svlq = super::Uvlq::from(i);
            let value = u64::try_from(svlq).unwrap();
            assert_eq!(value, i as u64);
        }

        for _ in 0..100000 {
            let i = rng.gen_range(u64::MIN..=u64::MAX);
            let svlq = super::Uvlq::from(i);
            let value = u64::try_from(svlq).unwrap();
            assert_eq!(value, i);
        }
    }
}
