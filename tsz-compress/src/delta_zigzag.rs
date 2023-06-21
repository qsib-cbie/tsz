use crate::prelude::{BitBuffer, BitBufferSlice};

enum DeltaI8 {
    Four([i8; 4]),
    Eight([i8; 8]),
    Ten([i8; 10]),
}

enum DeltaI16 {
    Two([i16; 2]),
    Three([i16; 3]),
    Four([i16; 4]),
    Eight([i16; 8]),
    Ten([i16; 10]),
}

enum DeltaI32 {
    Two([i32; 2]),
    Three([i32; 3]),
    Four([i32; 4]),
    Eight([i32; 8]),
    Ten([i32; 10]),
}

enum DeltaI64 {
    Two([i64; 2]),
    Three([i64; 3]),
    Four([i64; 4]),
    Eight([i64; 8]),
    Ten([i64; 10]),
}

// IntoIterator on DeltaI8
impl IntoIterator for DeltaI8 {
    type Item = i8;
    match self {
        DeltaI8::Four(array) => {
            type IntoIter = std::array::IntoIter<i8, 4>;
        }
        DeltaI8::Eight(array) => {
            type IntoIter = std::array::IntoIter<i8, 8>;
        }
        DeltaI8::Ten(array) => {
            type IntoIter = std::array::IntoIter<i8, 10>;
        }
    }
    fn into_iter(self) -> Self::IntoIter {
        match self {
            DeltaI8::Four(array) => array.into_iter(),
            DeltaI8::Eight(array) => array.into_iter(),
            DeltaI8::Ten(array) => array.into_iter(),
        }
    }
}

// IntoIterator on DeltaI16
impl IntoIterator for DeltaI16 {
    type Item = i16;
    match self {
        DeltaI16::Two(array) => {
            type IntoIter = std::array::IntoIter<i16, 2>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI16::Three(array) => {
            type IntoIter = std::array::IntoIter<i16, 3>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI16::Four(array) => {
            type IntoIter = std::array::IntoIter<i16, 4>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI16::Eight(array) => {
            type IntoIter = std::array::IntoIter<i16, 8>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI16::Ten(array) => {
            type IntoIter = std::array::IntoIter<i16, 10>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
    }
}

// IntoIterator on DeltaI32
impl IntoIterator for DeltaI32 {
    type Item = i32;
    match self {
        DeltaI32::Two(array) => {
            type IntoIter = std::array::IntoIter<i32, 2>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI32::Three(array) => {
            type IntoIter = std::array::IntoIter<i32, 3>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI32::Four(array) => {
            type IntoIter = std::array::IntoIter<i32, 4>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI32::Eight(array) => {
            type IntoIter = std::array::IntoIter<i32, 8>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI32::Ten(array) => {
            type IntoIter = std::array::IntoIter<i32, 10>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
    }
}

// IntoIterator on DeltaI64
impl IntoIterator for DeltaI64 {
    type Item = i64;
    match self {
        DeltaI64::Two(array) => {
            type IntoIter = std::array::IntoIter<i64, 2>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI64::Three(array) => {
            type IntoIter = std::array::IntoIter<i64, 3>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI64::Four(array) => {
            type IntoIter = std::array::IntoIter<i64, 4>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI64::Eight(array) => {
            type IntoIter = std::array::IntoIter<i64, 8>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
        DeltaI64::Ten(array) => {
            type IntoIter = std::array::IntoIter<i64, 10>;
            fn into_iter(self) -> Self::IntoIter {
                array.into_iter()
            }
        },
    }
}

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
) -> Result<(DeltaI8, Option<&'_ BitBufferSlice>), &'static str> {
    if !bits[0] {
        return Err("Not valid for delta decoding");
    }
    if !bits[1] {
        return Err("Not enough bits to decode");
    }
    let mut idx = 0;
    let output = DeltaI8::Four([0i8; 4]);
    if !bits[2] {
        // 4 samples of 8 bits
        let mut values = [0i8; 4];
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
        let output = DeltaI8::Four(values);
    } else if !bits[3] {
        // 8 samples of 4 bits
        let mut values = [0i8; 8];
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
        let output = DeltaI8::Eight(values);
    } else {
        // 10 samples of 3 bits
        let mut values = [0i8; 10];
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
        let output = DeltaI8::Ten(values);
    }
    if bits.len() > idx {
        Ok((output, Some(&bits[idx..])))
    } else {
        Ok((output, None))
    }
}

pub fn decode_delta_i16(
    bits: &'_ BitBufferSlice,
) -> Result<(DeltaI16, Option<&'_ BitBufferSlice>), &'static str> {
    if !bits[0]{
        return Err("Not valid for delta decoding");
    }
    let mut idx = 0;
    let output = DeltaI16::Four([0i16; 4]);
    if bits[1]{
        if !bits[2] {
            // 4 samples of 8 bits
            let mut values = [0i16; 4];
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
            let output = DeltaI16::Four(values);
        } else if !bits[3] {
            // 8 samples of 4 bits
            let mut values = [0i16; 8];
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
            let output = DeltaI16::Eight(values);
        } else {
            // 10 samples of 3 bits
            let mut values = [0i16; 10];
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
            let output = DeltaI16::Ten(values);
        }
    } else if bits[2] {
        // 3 samples of 10 bits
        let mut values = [0i16; 3];
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
        let output = DeltaI16::Three(values);
    } else {
        // 2 samples of 16 bits
        let mut values = [0i16; 2];
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
        let output = DeltaI16::Two(values);
    }
    if bits.len() > idx {
        Ok((output, Some(&bits[idx..])))
    } else {
        Ok((output, None))
    }
}

pub fn decode_delta_i32(
    bits: &'_ BitBufferSlice,
) -> Result<(DeltaI32, Option<&'_ BitBufferSlice>), &'static str> {
    if !bits[0]{
        return Err("Not valid for delta decoding");
    }
    let mut idx = 0;
    let output = DeltaI32::Four([0i32; 4]);
    if bits[1]{
        if !bits[2] {
            // 4 samples of 8 bits
            let mut values = [0i32; 4];
            // read 4 bits
            idx += 4;
            // for 4 samples
            for i in 0..4 {
                let mut value = 0;
                // read 8 bits
                for j in 0..8 {
                    value |= (bits[idx + j] as u32) << j;
                }
                let result = ((value >> 1) as i32) ^ (-((value & 1) as i32));
                values[i] = result;
                idx += 8;
            }
            let output = DeltaI32::Four(values);
        } else if !bits[3] {
            // 8 samples of 4 bits
            let mut values = [0i32; 8];
            // read 4 bits
            idx += 4;
            // for 8 samples
            for i in 0..8 {
                let mut value = 0;
                // read 4 bits
                for j in 0..4 {
                    value |= (bits[idx + j] as u32) << j;
                }
                let result = ((value >> 1) as i32) ^ (-((value & 1) as i32));
                values[i] = result;
                idx += 4;
            }
            let output = DeltaI32::Eight(values);
        } else {
            // 10 samples of 3 bits
            let mut values = [0i32; 10];
            // read 6 bits
            idx += 6;
            // for 10 samples
            for i in 0..10 {
                let mut value = 0;
                // read 3 bits
                for j in 0..3 {
                    value |= (bits[idx + j] as u32) << j;
                }
                let result = ((value >> 1) as i32) ^ (-((value & 1) as i32));
                values[i] = result;
                idx += 3;
            }
            let output = DeltaI32::Ten(values);
        }
    } else if bits[2] {
        // 3 samples of 10 bits
        let mut values = [0i32; 3];
        // read 6 bits
        idx += 6;
        // for 3 samples
        for i in 0..3 {
            let mut value = 0;
            // read 10 bits
            for j in 0..10 {
                value |= (bits[idx+j] as u32) << j; 
            }
            let result = ((value >> 1) as i32) ^ (-((value&1) as i32));
            values[i] = result;
            idx += 10;
        }
        let output = DeltaI32::Three(values);
    } else {
        // 2 samples of 16 bits
        let mut values = [0i32; 2];
        // read 4 bits
        idx += 4;
        // for 2 samples
        for i in 0..2 {
            let mut value = 0;
            // read 16 bits
            for j in 0..16 {
                value |= (bits[idx+j] as u32) << j; 
            }
            let result = ((value >> 1) as i32) ^ (-((value&1) as i32));
            values[i] = result;
            idx += 16;
        }
        let output = DeltaI32::Two(values);
    }
    if bits.len() > idx {
        Ok((output, Some(&bits[idx..])))
    } else {
        Ok((output, None))
    }
}

pub fn decode_delta_i64(
    bits: &'_ BitBufferSlice,
) -> Result<(DeltaI64, Option<&'_ BitBufferSlice>), &'static str> {
    if !bits[0]{
        return Err("Not valid for delta decoding");
    }
    let mut idx = 0;
    let output = DeltaI64::Four([0i64; 4]);
    if bits[1]{
        if !bits[2] {
            // 4 samples of 8 bits
            let mut values = [0i64; 4];
            // read 4 bits
            idx += 4;
            // for 4 samples
            for i in 0..4 {
                let mut value = 0;
                // read 8 bits
                for j in 0..8 {
                    value |= (bits[idx + j] as u64) << j;
                }
                let result = ((value >> 1) as i64) ^ (-((value & 1) as i64));
                values[i] = result;
                idx += 8;
            }
            let output = DeltaI64::Four(values);
        } else if !bits[3] {
            // 8 samples of 4 bits
            let mut values = [0i64; 8];
            // read 4 bits
            idx += 4;
            // for 8 samples
            for i in 0..8 {
                let mut value = 0;
                // read 4 bits
                for j in 0..4 {
                    value |= (bits[idx + j] as u64) << j;
                }
                let result = ((value >> 1) as i64) ^ (-((value & 1) as i64));
                values[i] = result;
                idx += 4;
            }
            let output = DeltaI64::Eight(values);
        } else {
            // 10 samples of 3 bits
            let mut values = [0i64; 10];
            // read 6 bits
            idx += 6;
            // for 10 samples
            for i in 0..10 {
                let mut value = 0;
                // read 3 bits
                for j in 0..3 {
                    value |= (bits[idx + j] as u64) << j;
                }
                let result = ((value >> 1) as i64) ^ (-((value & 1) as i64));
                values[i] = result;
                idx += 3;
            }
            let output = DeltaI64::Ten(values);
        }
    } else if bits[2] {
        // 3 samples of 10 bits
        let mut values = [0i64; 3];
        // read 6 bits
        idx += 6;
        // for 3 samples
        for i in 0..3 {
            let mut value = 0;
            // read 10 bits
            for j in 0..10 {
                value |= (bits[idx+j] as u64) << j; 
            }
            let result = ((value >> 1) as i64) ^ (-((value&1) as i64));
            values[i] = result;
            idx += 10;
        }
        let output = DeltaI64::Three(values);
    } else {
        // 2 samples of 16 bits
        let values = [0i64; 2];
        // read 4 bits
        idx += 4;
        // for 2 samples
        for i in 0..2 {
            let mut value = 0;
            // read 16 bits
            for j in 0..16 {
                value |= (bits[idx+j] as u64) << j; 
            }
            let result = ((value >> 1) as i64) ^ (-((value&1) as i64));
            values[i] = result;
            idx += 16;
        }
        let output = DeltaI64::Two(values);
    }
    if bits.len() > idx {
        Ok((output, Some(&bits[idx..])))
    } else {
        Ok((output, None))
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
        let (decoded, remaining) = decode_delta_i8(&bits).unwrap();
        let mut ind = 0;
        for de_value in decoded {
            assert_eq!(values[ind], de_value);
            ind += 1;
        }
        assert!(remaining.is_none());
    }

    fn encode_decode_i16(values: &mut[i16]) {
        let mut bits = BitBuffer::new();
        encode_delta_i16(values, &mut bits);
        // println!("Encoded: {}", bits);
        let (decoded, remaining) = decode_delta_i16(&bits).unwrap();
        let mut ind = 0;
        for de_value in decoded {
            assert_eq!(values[ind], de_value);
            ind += 1;
        }
        assert!(remaining.is_none());
    }

    fn encode_decode_i32(values: &mut[i32]) {
        let mut bits = BitBuffer::new();
        encode_delta_i32(values, &mut bits);
        let (decoded, remaining) = decode_delta_i32(&bits).unwrap();
        let mut ind = 0;
        for de_value in decoded {
            assert_eq!(values[ind], de_value);
            ind += 1;
        }
        assert!(remaining.is_none());
    }

    fn encode_decode_i64(values: &mut[i64]) {
        let mut bits = BitBuffer::new();
        encode_delta_i64(values, &mut bits);
        let (decoded, remaining) = decode_delta_i64(&bits).unwrap();
        let mut ind = 0;
        for de_value in decoded {
            assert_eq!(values[ind], de_value);
            ind += 1;
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
