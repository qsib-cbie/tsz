use bitvec::prelude::*;
use core::cmp::max;

///
/// An signed variable-length quantity.
///
pub struct Svlq {
    pub bits: BitVec,
}

///
/// A reference to bits in an Svlq.
///
pub struct SvlqRef<'a>(pub &'a BitSlice);

///
/// Construct a Svlq from an signed.
/// Construct an signed from a svlq.
///
/// When constructing a Svlq:
/// * The first bit is 1 if the value is negative, 0 if positive.
/// * Take the minimum trailing bits of the magnitude of the input value,
///     starting from the MSB that is set.
/// * At least one Svlq byte is always produced.
///
/// When constructing an signed:
/// * Fails if the Svlq is longer than the bit width of the output type.
///
macro_rules! impl_signed_svlq {
    ($signed:ident) => {
        impl From<$signed> for Svlq {
            fn from(value: $signed) -> Self {
                let mut bits = BitVec::with_capacity(8);
                let mut value = if value < 0 {
                    bits.push(true);
                    -(value as i128) as u128
                } else if value > 0 {
                    bits.push(false);
                    value as u128
                } else {
                    for _ in 0..8 {
                        bits.push(false);
                    }
                    return Self { bits };
                };

                // First byte is 6 bits, the rest are 7 bits
                let mut byte = (value & 0b00111111) as u8;
                value >>= 6;
                if value > 0 {
                    // VLQ continuation bit is set to the 7th bit on the first byte
                    byte |= (1 << 6);
                }
                byte = byte.reverse_bits();
                for i in 1..8 {
                    bits.push((byte >> i) & 1 == 1);
                }

                // Subsequent bytes are handled the same way as unsigned VLQ
                while value > 0 {
                    let mut byte = (value & 0x7f) as u8;
                    value >>= 7;
                    if value > 0 {
                        // VLQ continuation bit is set to the 8th bit on all other bytes
                        byte |= (1 << 7);
                    }
                    byte = byte.reverse_bits();
                    for _ in 0..8 {
                        bits.push(byte & 1 == 1);
                        byte >>= 1;
                    }
                }

                debug_assert!(bits.len() >= 8);
                debug_assert!(bits.len() % 8 == 0);

                Self { bits }
            }
        }

        impl TryFrom<Svlq> for $signed {
            type Error = ();

            fn try_from(value: Svlq) -> Result<Self, Self::Error> {
                let mut out: u128 = 0;
                let mut out_idx = 0;
                let mut negative = None;
                let bits: usize = (Self::BITS - 1) as usize; // Signed bit in primitive can't be parsed from unsigned VLQ storage
                for (vlq_idx, vlq_byte) in value.bits.chunks_exact(8).enumerate() {
                    let mut vlq_byte = &vlq_byte[..8];
                    let bits_to_read = if negative.is_none() {
                        if vlq_byte[0] {
                            negative = Some(true);
                        } else {
                            negative = Some(false);
                        }
                        vlq_byte = &vlq_byte[1..];
                        6
                    } else {
                        7
                    };

                    let extra_bits = if out_idx + bits_to_read > bits {
                        let mut extra = (out_idx + bits_to_read - bits) as usize;

                        // For signed VLQ, there is a special case for one more value in magnitude
                        // allowed, only when the value is negative. This is because the magnitude
                        // is stored as the two's complement of the value, ie, [-128, 127]

                        if negative.unwrap_or(false) {
                            extra = max(extra as isize - 1, 0) as usize;
                        }

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
                            if out_idx > bits {
                                return Err(());
                            }
                            val |= 1;
                        }
                        out_idx += 1;
                    }

                    if vlq_idx == 0 {
                        out = val as u128;
                    } else {
                        let shift = (7 * (vlq_idx - 1)) + 6;
                        out |= (val as u128) << shift;
                    }
                }

                let out = if negative.unwrap_or(false) {
                    -(out as i128)
                } else {
                    out as i128
                };

                if out < Self::MIN as i128 || out > Self::MAX as i128 {
                    return Err(());
                }

                Ok(out as Self)
            }
        }
    };
}

impl_signed_svlq!(i8);
impl_signed_svlq!(i16);
impl_signed_svlq!(i32);
impl_signed_svlq!(i64);
impl_signed_svlq!(i128);

macro_rules! impl_signed_svlq_ref {
    ($signed:ident) => {
        impl TryFrom<SvlqRef<'_>> for ($signed, usize) {
            type Error = &'static str;

            fn try_from(value: SvlqRef) -> Result<Self, Self::Error> {
                let mut out: u128 = 0;
                let mut out_idx = 0;
                let mut negative = None;
                let mut consumed = 0;
                let bits: usize = ($signed::BITS - 1) as usize; // Signed bit in primitive can't be parsed from unsigned VLQ storage
                for (vlq_idx, vlq_byte) in value.0.chunks_exact(8).enumerate() {
                    consumed += 8;
                    let mut vlq_byte = &vlq_byte[..8];
                    let bits_to_read = if negative.is_none() {
                        if vlq_byte[0] {
                            negative = Some(true);
                        } else {
                            negative = Some(false);
                        }
                        vlq_byte = &vlq_byte[1..];
                        6
                    } else {
                        7
                    };

                    let extra_bits = if out_idx + bits_to_read > bits {
                        let mut extra = (out_idx + bits_to_read - bits) as usize;

                        // For signed VLQ, there is a special case for one more value in magnitude
                        // allowed, only when the value is negative. This is because the magnitude
                        // is stored as the two's complement of the value, ie, [-128, 127]

                        if negative.unwrap_or(false) {
                            extra = max(extra as isize - 1, 0) as usize;
                        }

                        let overflow = vlq_byte.iter().skip(1).take(extra).any(|b| *b);
                        if overflow {
                            return Err("Signed VLQ bit overflow");
                        }

                        extra
                    } else {
                        0
                    };

                    let mut val: $signed = 0;
                    for bit in vlq_byte.iter().skip(1 + extra_bits) {
                        val <<= 1;
                        if *bit {
                            if out_idx >= bits {
                                return Err("Signed VLQ bit overflow");
                            }
                            val |= 1;
                        }
                        out_idx += 1;
                    }

                    if vlq_idx == 0 {
                        out = val as u128;
                    } else {
                        let shift = (7 * (vlq_idx - 1)) + 6;
                        out |= (val as u128) << shift;
                    }
                    if !vlq_byte[0] {
                        break;
                    }
                }

                let out = if negative.unwrap_or(false) {
                    -(out as i128)
                } else {
                    out as i128
                };

                if out < $signed::MIN as i128 || out > $signed::MAX as i128 {
                    return Err("Signed VLQ value out of bit range");
                }

                Ok((out as $signed, consumed))
            }
        }
    };
}

impl_signed_svlq_ref!(i8);
impl_signed_svlq_ref!(i16);
impl_signed_svlq_ref!(i32);
impl_signed_svlq_ref!(i64);
impl_signed_svlq_ref!(i128);

#[cfg(test)]
mod tests {
    use rand::Rng;

    #[test]
    fn can_encode_0() {
        let svlq = super::Svlq::from(0i8);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], false);

        let svlq = super::Svlq::from(0i16);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], false);

        let svlq = super::Svlq::from(0i32);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], false);

        let svlq = super::Svlq::from(0i64);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], false);
    }

    #[test]
    fn can_encode_1() {
        let svlq = super::Svlq::from(1i8);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], true);

        let svlq = super::Svlq::from(1i16);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], true);

        let svlq = super::Svlq::from(1i32);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], true);

        let svlq = super::Svlq::from(1i64);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], true);
    }

    #[test]
    fn can_encode_neg_1() {
        let svlq = super::Svlq::from(-1i8);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], true);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], true);

        let svlq = super::Svlq::from(-1i16);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], true);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], true);

        let svlq = super::Svlq::from(-1i32);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], true);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], true);

        let svlq = super::Svlq::from(-1i64);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], true);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], true);
    }

    #[test]
    fn can_encode_63() {
        let svlq = super::Svlq::from(63i8);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], true);
        assert_eq!(svlq.bits[3], true);
        assert_eq!(svlq.bits[4], true);
        assert_eq!(svlq.bits[5], true);
        assert_eq!(svlq.bits[6], true);
        assert_eq!(svlq.bits[7], true);

        let svlq = super::Svlq::from(63i16);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], true);
        assert_eq!(svlq.bits[3], true);
        assert_eq!(svlq.bits[4], true);
        assert_eq!(svlq.bits[5], true);
        assert_eq!(svlq.bits[6], true);
        assert_eq!(svlq.bits[7], true);

        let svlq = super::Svlq::from(63i32);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], true);
        assert_eq!(svlq.bits[3], true);
        assert_eq!(svlq.bits[4], true);
        assert_eq!(svlq.bits[5], true);
        assert_eq!(svlq.bits[6], true);
        assert_eq!(svlq.bits[7], true);

        let svlq = super::Svlq::from(63i64);
        assert_eq!(svlq.bits.len(), 8);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], false);
        assert_eq!(svlq.bits[2], true);
        assert_eq!(svlq.bits[3], true);
        assert_eq!(svlq.bits[4], true);
        assert_eq!(svlq.bits[5], true);
        assert_eq!(svlq.bits[6], true);
        assert_eq!(svlq.bits[7], true);
    }

    #[test]
    fn can_encode_128() {
        // 0b1000 0000
        // 0b0100 0000 0b0000 0010
        let svlq = super::Svlq::from(128i16);
        assert_eq!(svlq.bits.len(), 16);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], true);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], false);
        assert_eq!(svlq.bits[8], false);
        assert_eq!(svlq.bits[9], false);
        assert_eq!(svlq.bits[10], false);
        assert_eq!(svlq.bits[11], false);
        assert_eq!(svlq.bits[12], false);
        assert_eq!(svlq.bits[13], false);
        assert_eq!(svlq.bits[14], true);
        assert_eq!(svlq.bits[15], false);

        let svlq = super::Svlq::from(128i32);
        assert_eq!(svlq.bits.len(), 16);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], true);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], false);
        assert_eq!(svlq.bits[8], false);
        assert_eq!(svlq.bits[9], false);
        assert_eq!(svlq.bits[10], false);
        assert_eq!(svlq.bits[11], false);
        assert_eq!(svlq.bits[12], false);
        assert_eq!(svlq.bits[13], false);
        assert_eq!(svlq.bits[14], true);
        assert_eq!(svlq.bits[15], false);

        let svlq = super::Svlq::from(128i64);
        assert_eq!(svlq.bits.len(), 16);
        assert_eq!(svlq.bits[0], false);
        assert_eq!(svlq.bits[1], true);
        assert_eq!(svlq.bits[2], false);
        assert_eq!(svlq.bits[3], false);
        assert_eq!(svlq.bits[4], false);
        assert_eq!(svlq.bits[5], false);
        assert_eq!(svlq.bits[6], false);
        assert_eq!(svlq.bits[7], false);
        assert_eq!(svlq.bits[8], false);
        assert_eq!(svlq.bits[9], false);
        assert_eq!(svlq.bits[10], false);
        assert_eq!(svlq.bits[11], false);
        assert_eq!(svlq.bits[12], false);
        assert_eq!(svlq.bits[13], false);
        assert_eq!(svlq.bits[14], true);
        assert_eq!(svlq.bits[15], false);
    }

    #[test]
    fn can_encode_33333() {
        let svlq = super::Svlq::from(33333i32);
        // 33333 = 0b100 0001000 110101
        // vlq = 0b0 0b1 bottom 6, 0b1 next bottom 7, 0b0 remaining 7
        assert_eq!(svlq.bits.len(), 24);
        let str = svlq
            .bits
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>();
        let expected = "011101011000100000000100".to_string();
        assert_eq!(str, expected);

        let svlq = super::Svlq::from(33333i64);
        // 33333 = 0b10 0000100 0110101
        // vlq = 0b1 bottom 7, 0b1 next bottom 7, 0b0 remaining 7
        assert_eq!(svlq.bits.len(), 24);
        let str = svlq
            .bits
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>();
        let expected = "011101011000100000000100".to_string();
        assert_eq!(str, expected);
    }

    #[test]
    fn can_encode_neg_33333() {
        let svlq = super::Svlq::from(-33333i32);
        // 33333 = 0b100 0001000 110101
        // vlq = 0b1 0b1 bottom 6, 0b1 next bottom 7, 0b0 remaining 7
        assert_eq!(svlq.bits.len(), 24);
        let str = svlq
            .bits
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>();
        let expected = "111101011000100000000100".to_string();
        assert_eq!(str, expected);

        let svlq = super::Svlq::from(-33333i64);
        // 33333 = 0b10 0000100 0110101
        // vlq = 0b1 bottom 7, 0b1 next bottom 7, 0b0 remaining 7
        assert_eq!(svlq.bits.len(), 24);
        let str = svlq
            .bits
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>();
        let expected = "111101011000100000000100".to_string();
        assert_eq!(str, expected);
    }

    #[test]
    fn can_decode_0() {
        // i8
        let svlq = super::Svlq::from(0i8);
        let byte = i8::try_from(svlq).unwrap();
        assert_eq!(byte, 0);

        // i16
        let svlq = super::Svlq::from(0i16);
        let byte = i16::try_from(svlq).unwrap();
        assert_eq!(byte, 0);

        // i32
        let svlq = super::Svlq::from(0i32);
        let byte = i32::try_from(svlq).unwrap();
        assert_eq!(byte, 0);

        // i64
        let svlq = super::Svlq::from(0i64);
        let byte = i64::try_from(svlq).unwrap();
        assert_eq!(byte, 0);
    }

    #[test]
    fn can_decode_1() {
        // i8
        let svlq = super::Svlq::from(-1i32);
        let byte = i8::try_from(svlq).unwrap();
        assert_eq!(byte, -1);

        // i16
        let svlq = super::Svlq::from(-1i32);
        let byte = i16::try_from(svlq).unwrap();
        assert_eq!(byte, -1);

        // i32
        let svlq = super::Svlq::from(-1i32);
        let byte = i32::try_from(svlq).unwrap();
        assert_eq!(byte, -1);

        // i64
        let svlq = super::Svlq::from(-1i32);
        let byte = i64::try_from(svlq).unwrap();
        assert_eq!(byte, -1);
    }

    #[test]
    fn can_decode_neg_1() {
        // i8
        let svlq = super::Svlq::from(-1i32);
        let byte = i8::try_from(svlq).unwrap();
        assert_eq!(byte, -1);

        // i16
        let svlq = super::Svlq::from(-1i32);
        let byte = i16::try_from(svlq).unwrap();
        assert_eq!(byte, -1);

        // i32
        let svlq = super::Svlq::from(-1i32);
        let byte = i32::try_from(svlq).unwrap();
        assert_eq!(byte, -1);

        // i64
        let svlq = super::Svlq::from(-1i32);
        let byte = i64::try_from(svlq).unwrap();
        assert_eq!(byte, -1);
    }

    #[test]
    fn can_decode_127() {
        let svlq = super::Svlq::from(127i32);
        let byte = i8::try_from(svlq).unwrap();
        assert_eq!(byte, 127);

        let svlq = super::Svlq::from(127i32);
        let byte = i16::try_from(svlq).unwrap();
        assert_eq!(byte, 127);

        let svlq = super::Svlq::from(127i32);
        let byte = i32::try_from(svlq).unwrap();
        assert_eq!(byte, 127);

        let svlq = super::Svlq::from(127i32);
        let byte = i64::try_from(svlq).unwrap();
        assert_eq!(byte, 127);
    }

    #[test]
    fn can_decode_128() {
        let svlq = super::Svlq::from(128i64);
        let byte = i8::try_from(svlq);
        assert!(byte.is_err());

        let svlq = super::Svlq::from(128i64);
        let byte = i16::try_from(svlq).unwrap();
        assert_eq!(byte, 128);

        let svlq = super::Svlq::from(128i64);
        let byte = i32::try_from(svlq).unwrap();
        assert_eq!(byte, 128);

        let svlq = super::Svlq::from(128i64);
        let byte = i64::try_from(svlq).unwrap();
        assert_eq!(byte, 128);
    }

    #[test]
    fn can_decode_neg_128() {
        let svlq = super::Svlq::from(-128i64);
        let byte = i8::try_from(svlq).unwrap();
        assert_eq!(byte, -128);

        let svlq = super::Svlq::from(-128i64);
        let byte = i16::try_from(svlq).unwrap();
        assert_eq!(byte, -128);

        let svlq = super::Svlq::from(-128i64);
        let byte = i32::try_from(svlq).unwrap();
        assert_eq!(byte, -128);

        let svlq = super::Svlq::from(-128i64);
        let byte = i64::try_from(svlq).unwrap();
        assert_eq!(byte, -128);
    }

    #[test]
    fn can_decode_255() {
        let svlq = super::Svlq::from(255i32);
        let byte = i8::try_from(svlq);
        assert!(byte.is_err());

        let svlq = super::Svlq::from(255i32);
        let byte = i16::try_from(svlq).unwrap();
        assert_eq!(byte, 255);

        let svlq = super::Svlq::from(255i32);
        let byte = i32::try_from(svlq).unwrap();
        assert_eq!(byte, 255);

        let svlq = super::Svlq::from(255i32);
        let byte = i64::try_from(svlq).unwrap();
        assert_eq!(byte, 255);
    }

    #[test]
    fn can_convert_epoch_s() {
        let s = 1675528564;
        let svlq = super::Svlq::from(s);
        let value = i32::try_from(svlq).unwrap();
        assert_eq!(value, s as i32);

        let svlq = super::Svlq::from(s);
        let value = i64::try_from(svlq).unwrap();
        assert_eq!(value, s);
    }

    #[test]
    fn can_convert_epoch_ms() {
        let ms = 1675528564000;
        let svlq = super::Svlq::from(ms);
        let value = i32::try_from(svlq);
        assert!(value.is_err());

        let svlq = super::Svlq::from(ms);
        let value = i64::try_from(svlq).unwrap();
        assert_eq!(value, ms);
    }

    #[test]
    fn can_convert_epoch_us() {
        let us = 1675528564123456;
        let svlq = super::Svlq::from(us);
        let value = i32::try_from(svlq);
        assert!(value.is_err());

        let svlq = super::Svlq::from(us);
        let value = i64::try_from(svlq).unwrap();
        assert_eq!(value, us);
    }

    #[test]
    fn can_convert_all_i8() {
        for i in -128..=127 {
            // println!("i: {}", i);
            let svlq = super::Svlq::from(i);
            let value = i8::try_from(svlq).unwrap();
            assert_eq!(value, i);
        }
    }

    #[test]
    fn can_convert_all_i16() {
        for i in -128..=127 {
            // println!("i: {}", i);
            let svlq = super::Svlq::from(i);
            let value = i16::try_from(svlq).unwrap();
            assert_eq!(value, i);
        }

        for i in -32768..=32767 {
            // println!("i: {}", i);
            let svlq = super::Svlq::from(i);
            let value = i16::try_from(svlq).unwrap();
            assert_eq!(value, i);
        }
    }

    #[test]
    fn can_convert_all_i32() {
        for i in i8::MIN..=i8::MAX {
            // println!("i: {}", i);
            let svlq = super::Svlq::from(i);
            let value = i32::try_from(svlq).unwrap();
            assert_eq!(value, i as i32);
        }

        for i in i16::MIN..=i16::MAX {
            // println!("i: {}", i);
            let svlq = super::Svlq::from(i);
            let value = i32::try_from(svlq).unwrap();
            assert_eq!(value, i as i32);
        }

        // for i in i32::MIN..=i32::MAX {
        //     if i % 1000000 == 0 {
        //         println!("i: {}", i);
        //     }
        //     let svlq = super::Svlq::from(i);
        //     let value = i32::try_from(svlq).unwrap();
        //     assert_eq!(value, i as i32);
        // }

        let mut rng = rand::thread_rng();
        for _ in 0..100000 {
            let i = rng.gen_range(i32::MIN..=i32::MAX);
            let svlq = super::Svlq::from(i);
            let value = i32::try_from(svlq).unwrap();
            assert_eq!(value, i as i32);
        }
    }

    #[test]
    fn can_convert_all_i64() {
        for i in i8::MIN..=i8::MAX {
            let svlq = super::Svlq::from(i);
            let value = i64::try_from(svlq).unwrap();
            assert_eq!(value, i as i64);
        }

        for i in i16::MIN..=i16::MAX {
            let svlq = super::Svlq::from(i);
            let value = i64::try_from(svlq).unwrap();
            assert_eq!(value, i as i64);
        }

        let mut rng = rand::thread_rng();

        for _ in 0..100000 {
            let i = rng.gen_range(i32::MIN..=i32::MAX);
            let svlq = super::Svlq::from(i);
            let value = i64::try_from(svlq).unwrap();
            assert_eq!(value, i as i64);
        }

        for _ in 0..100000 {
            let i = rng.gen_range(i64::MIN..=i64::MAX);
            let svlq = super::Svlq::from(i);
            let value = i64::try_from(svlq).unwrap();
            assert_eq!(value, i as i64);
        }
    }
}
