use bitvec::prelude::*;

///
/// An unsigned variable-length quantity.
///
pub struct Uvlq {
    pub(crate) bits: BitVec,
}

///
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
                let mut bits = BitVec::with_capacity(8);
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
                    // #[cfg(test)]
                    // {
                    //     println!("out_idx: {:?} vlq_byte: {:?}", out_idx, vlq_byte);
                    // }

                    let extra_bits = if out_idx + 7 > Self::BITS {
                        let extra = (out_idx + 7 - Self::BITS) as usize;
                        // #[cfg(test)]
                        // {
                        //     println!("extra: {:?} overflow: {:?}", extra, vlq_byte.iter().skip(1).take(extra).map(|b| *b).collect::<Vec<_>>());
                        // }

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
                        // #[cfg(test)]
                        // {
                        //     println!("out_idx: {:?} val: {:b} bit: {:?}", out_idx, val, bit);
                        // }

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

impl_unsigned_uvlq!(u8);
impl_unsigned_uvlq!(u16);
impl_unsigned_uvlq!(u32);
impl_unsigned_uvlq!(u64);

#[cfg(test)]
mod tests {
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
}
