use crate::prelude::*;
use alloc::vec::Vec;

pub fn decode_i8<'a>(
    bits: &'a BitBufferSlice,
    idx: usize,
    output: &mut Vec<i8>,
) -> Result<Option<usize>, CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }

    let mut idx = idx;

    // TODO: Optimize with bit operations
    if !(bits[idx] && !bits[idx + 1] && !bits[idx + 2] && bits[idx + 3]) {
        return Err(CodingError::InvalidBits);
    }

    // Skip 1001
    idx += 4;

    if idx >= bits.len() {
        return Err(CodingError::NotEnoughBits);
    }

    // Loops until we get next header 1001 or end of columns 1011
    // TODO: Optimize with bit operations
    while (idx < bits.len())
        && !(bits[idx] && !bits[idx + 1] && !bits[idx + 2] && bits[idx + 3])
        && !((bits[idx] && !bits[idx + 1] && bits[idx + 2] && bits[idx + 3])
            && idx + 4 >= bits.len())
    {
        // Delta Decoding
        if bits[idx] {
            // Todo: Optimize addition operations in bits
            // Skipping 1
            idx += 1;
            let mut value: i8 = 0;
            // Case 1: 00
            if !bits[idx] && !bits[idx + 1] {
                // Skipping 00 and pad 0
                idx += 3;
                let mut value = value as i16;
                for i in (idx..idx + 32).step_by(16) {
                    for j in 0..16 {
                        value |= (bits[i + j] as i16) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value as i8);
                    value = 0;
                }
                idx += 32;
            }
            // Case 2: 010
            else if !bits[idx] && bits[idx + 1] && !bits[idx + 2] {
                // Skipping 010 and pad 00
                idx += 5;
                let mut value = value as i16;
                for i in (idx..idx + 30).step_by(10) {
                    for j in 0..10 {
                        value |= (bits[i + j] as i16) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value as i8);
                    value = 0;
                }
                idx += 30;
            }
            // Case 3: 10
            else if bits[idx] && !bits[idx + 1] {
                // Skipping 10 and pad 0
                idx += 3;
                let mut value = value as i16;
                for i in (idx..idx + 32).step_by(8) {
                    for j in 0..8 {
                        value |= (bits[i + j] as i16) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value as i8);
                    value = 0;
                }
                idx += 32;
            }
            // Case 4: 110
            else if bits[idx] && bits[idx + 1] && !bits[idx + 2] {
                // Skipping 110
                idx += 3;
                for i in (idx..idx + 32).step_by(4) {
                    for j in 0..4 {
                        value |= (bits[i + j] as i8) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 32;
            }
            // Case 5: 111
            else if bits[idx] && bits[idx + 1] && bits[idx + 2] {
                // Skipping 111 and pad 00
                idx += 5;

                for i in (idx..idx + 30).step_by(3) {
                    for j in 0..3 {
                        value |= (bits[i + j] as i8) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 30;
            } else {
                return Err(CodingError::InvalidBits);
            }
        } else {
            // Decode Delta-delta

            // Skipping 0
            idx += 1;

            let mut value = 0;
            if !bits[idx] && !bits[idx + 1] {
                // Skip 00
                idx += 2;

                value = bits[idx] as i8;
                let value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 1;
            } else if !bits[idx] && bits[idx + 1] {
                // Skip 01
                idx += 2;

                for i in 0..5 {
                    value |= (bits[idx + i] as i8) << i;
                }
                let value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 5;
            } else if bits[idx] && !bits[idx + 1] {
                // Skip 10
                idx += 2;

                let mut value: i16 = 0;
                for i in 0..9 {
                    value |= (bits[idx + i] as i16) << i;
                }
                // ZigZag decoding
                let value = (value >> 1) ^ -(value & 1);

                output.push(value as i8);
                idx += 9;
            } else {
                return Err(CodingError::InvalidBits);
            }
        }
    }
    return Ok(Some(idx));
}

pub fn decode_i16(
    bits: &'_ BitBufferSlice,
    idx: usize,
    output: &mut Vec<i16>,
) -> Result<Option<usize>, CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }

    let mut idx = idx;

    // TODO: Optimize with bit operations
    if !(bits[idx] && !bits[idx + 1] && !bits[idx + 2] && bits[idx + 3]) {
        return Err(CodingError::InvalidBits);
    }

    // Skip 1001
    idx += 4;

    if idx >= bits.len() {
        return Err(CodingError::NotEnoughBits);
    }

    // Loops until we get next header 1001
    // TODO: Optimize with bit operations

    while (idx < bits.len())
        && !(bits[idx] && !bits[idx + 1] && !bits[idx + 2] && bits[idx + 3])
        && !((bits[idx] && !bits[idx + 1] && bits[idx + 2] && bits[idx + 3])
            && idx + 4 >= bits.len())
    {
        // Decode delta
        if bits[idx] {
            // Todo: Optimize addition operations in bits
            // Skipping 1
            idx += 1;

            let mut value: i16 = 0;

            // Case 1: 00
            if !bits[idx] && !bits[idx + 1] {
                // Skipping 00 and pad 0
                idx += 3;

                let mut value: i32 = value as i32;

                for i in (idx..idx + 32).step_by(16) {
                    for j in 0..16 {
                        value |= (bits[i + j] as i32) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value as i16);
                    value = 0;
                }
                idx += 32;
            }
            // Case 2: 01
            else if !bits[idx] && bits[idx + 1] {
                // Skipping 01 and pad 000
                idx += 5;

                for i in (idx..idx + 30).step_by(10) {
                    for j in 0..10 {
                        value |= (bits[i + j] as i16) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 30;
            }
            // Case 3: 10
            else if bits[idx] && !bits[idx + 1] {
                // Skipping 10 and pad 0
                idx += 3;

                for i in (idx..idx + 32).step_by(8) {
                    for j in 0..8 {
                        value |= (bits[i + j] as i16) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 32;
            }
            // Case 4: 110
            else if bits[idx] && bits[idx + 1] && !bits[idx + 2] {
                // Skipping 110
                idx += 3;
                for i in (idx..idx + 32).step_by(4) {
                    for j in 0..4 {
                        value |= (bits[i + j] as i16) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 32;
            }
            // Case 5: 111
            else if bits[idx] && bits[idx + 1] && bits[idx + 2] {
                // Skipping 111 and pad 00
                idx += 5;

                for i in (idx..idx + 30).step_by(3) {
                    for j in 0..3 {
                        value |= (bits[i + j] as i16) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 30;
            } else {
                return Err(CodingError::InvalidBits);
            }
        } else {
            // Decode delta-delta

            // Skipping 0
            idx += 1;

            let mut value = 0;

            if !bits[idx] && !bits[idx + 1] {
                // Skip 00
                idx += 2;

                value = bits[idx] as i16;
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 1;
            } else if !bits[idx] && bits[idx + 1] {
                // Skip 01
                idx += 2;

                for i in 0..5 {
                    value |= (bits[idx + i] as i16) << i;
                }
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 5;
            } else if bits[idx] && !bits[idx + 1] {
                // Skip 10
                idx += 2;

                for i in 0..9 {
                    value |= (bits[idx + i] as i16) << i;
                }
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 9;
            } else if bits[idx] && bits[idx + 1] && !bits[idx + 2] {
                // Skip 110
                idx += 3;

                for i in 0..16 {
                    value |= (bits[idx + i] as i16) << i;
                }
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 16;
            } else if bits[idx] && bits[idx + 1] && bits[idx + 2] {
                // Skip 111
                idx += 3;

                let mut value: i64 = 0;
                for i in 0..64 {
                    value |= (bits[idx + i] as i64) << i;
                }
                value = (value >> 1) ^ -(value & 1);
                output.push(value as i16);
                idx += 64;
            } else {
                return Err(CodingError::InvalidBits);
            }
        }
    }
    return Ok(Some(idx));
}

pub fn decode_i32(
    bits: &'_ BitBufferSlice,
    idx: usize,
    output: &mut Vec<i32>,
) -> Result<Option<usize>, CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }

    let mut idx = idx;

    // TODO: Optimize with bit operations
    if !(bits[idx] && !bits[idx + 1] && !bits[idx + 2] && bits[idx + 3]) {
        return Err(CodingError::InvalidBits);
    }

    // Skip 1001
    idx += 4;

    if idx >= bits.len() {
        return Err(CodingError::NotEnoughBits);
    }

    // Loops until we get next header 1001
    // TODO: Optimize with bit operations

    while (idx < bits.len())
        && !(bits[idx] && !bits[idx + 1] && !bits[idx + 2] && bits[idx + 3])
        && !((bits[idx] && !bits[idx + 1] && bits[idx + 2] && bits[idx + 3])
            && idx + 4 >= bits.len())
    {
        // Decode Delta
        if bits[idx] {
            // Todo: Optimize addition operations in bits
            // Skipping 1
            idx += 1;

            let mut value: i32 = 0;

            // Case 1: 00
            if !bits[idx] && !bits[idx + 1] {
                // Skipping 00 and pad 0
                idx += 3;

                for i in (idx..idx + 32).step_by(16) {
                    for j in 0..16 {
                        value |= (bits[i + j] as i32) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 32;
            }
            // Case 2: 01
            else if !bits[idx] && bits[idx + 1] {
                // Skipping 01 and pad 000
                idx += 5;

                for i in (idx..idx + 30).step_by(10) {
                    for j in 0..10 {
                        value |= (bits[i + j] as i32) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 30;
            }
            // Case 3: 10
            else if bits[idx] && !bits[idx + 1] {
                // Skipping 10 and pad 0
                idx += 3;

                for i in (idx..idx + 32).step_by(8) {
                    for j in 0..8 {
                        value |= (bits[i + j] as i32) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 32;
            }
            // Case 4: 110
            else if bits[idx] && bits[idx + 1] && !bits[idx + 2] {
                // Skipping 110
                idx += 3;

                for i in (idx..idx + 32).step_by(4) {
                    for j in 0..4 {
                        value |= (bits[i + j] as i32) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 32;
            }
            // Case 5: 111
            else if bits[idx] && bits[idx + 1] && bits[idx + 2] {
                // Skipping 111 and pad 00
                idx += 5;

                for i in (idx..idx + 30).step_by(3) {
                    for j in 0..3 {
                        value |= (bits[i + j] as i32) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 30;
            } else {
                return Err(CodingError::InvalidBits);
            }
        } else {
            // Decode delta-delta

            // Skipping 0
            idx += 1;

            let mut value = 0;

            if !bits[idx] && !bits[idx + 1] {
                // Skip 00
                idx += 2;

                value = bits[idx] as i32;
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 1;
            } else if !bits[idx] && bits[idx + 1] {
                // Skip 01
                idx += 2;
                for i in 0..5 {
                    value |= (bits[idx + i] as i32) << i;
                }
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 5;
            } else if bits[idx] && !bits[idx + 1] {
                // Skip 10
                idx += 2;
                for i in 0..9 {
                    value |= (bits[idx + i] as i32) << i;
                }
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 9;
            } else if bits[idx] && bits[idx + 1] && !bits[idx + 2] {
                // Skip 110
                idx += 3;
                for i in 0..16 {
                    value |= (bits[idx + i] as i32) << i;
                }
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 16;
            } else if bits[idx] && bits[idx + 1] && bits[idx + 2] {
                // Skip 111
                idx += 3;

                let mut value: i64 = 0;
                for i in 0..64 {
                    value |= (bits[idx + i] as i64) << i;
                }
                value = (value >> 1) ^ -(value & 1);
                output.push(value as i32);
                idx += 64;
            } else {
                return Err(CodingError::InvalidBits);
            }
        }
    }
    return Ok(Some(idx));
}

pub fn decode_i64(
    bits: &'_ BitBufferSlice,
    idx: usize,
    output: &mut Vec<i64>,
) -> Result<Option<usize>, CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }

    let mut idx = idx;

    // TODO: Optimize with bit operations
    if !(bits[idx] && !bits[idx + 1] && !bits[idx + 2] && bits[idx + 3]) {
        return Err(CodingError::InvalidBits);
    }

    // Skip 1001
    idx += 4;

    if idx >= bits.len() {
        return Err(CodingError::NotEnoughBits);
    }

    // Loops until we get next header 1001
    // TODO: Optimize with bit operations

    while (idx < bits.len())
        && !(bits[idx] && !bits[idx + 1] && !bits[idx + 2] && bits[idx + 3])
        && !((bits[idx] && !bits[idx + 1] && bits[idx + 2] && bits[idx + 3])
            && idx + 4 >= bits.len())
    {
        // Decode Delta
        if bits[idx] {
            // Todo: Optimize addition operations in bits
            // Skipping 1
            idx += 1;

            let mut value: i64 = 0;

            // Case 1: 00
            if !bits[idx] && !bits[idx + 1] {
                // Skipping 00 and pad 0
                idx += 3;

                for i in (idx..idx + 32).step_by(16) {
                    for j in 0..16 {
                        value |= (bits[i + j] as i64) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 32;
            }
            // Case 2: 01
            else if !bits[idx] && bits[idx + 1] {
                // Skipping 01 and pad 000
                idx += 5;

                for i in (idx..idx + 30).step_by(10) {
                    for j in 0..10 {
                        value |= (bits[i + j] as i64) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 30;
            }
            // Case 3: 10
            else if bits[idx] && !bits[idx + 1] {
                // Skipping 10 and pad 0
                idx += 3;

                for i in (idx..idx + 32).step_by(8) {
                    for j in 0..8 {
                        value |= (bits[i + j] as i64) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 32;
            } else if bits[idx] && bits[idx + 1] && !bits[idx + 2] {
                // Skipping 110
                idx += 3;

                for i in (idx..idx + 32).step_by(4) {
                    for j in 0..4 {
                        value |= (bits[i + j] as i64) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 32;
            }
            // Case 5: 111
            else if bits[idx] && bits[idx + 1] && bits[idx + 2] {
                // Skipping 111 and pad 00
                idx += 5;

                for i in (idx..idx + 30).step_by(3) {
                    for j in 0..3 {
                        value |= (bits[i + j] as i64) << j;
                    }
                    value = (value >> 1) ^ -(value & 1); // ZigZag decoding
                    output.push(value);
                    value = 0;
                }
                idx += 30;
            } else {
                return Err(CodingError::InvalidBits);
            }
        } else {
            // Decode delta-delta

            // Skipping 0
            idx += 1;

            let mut value = 0;

            if !bits[idx] && !bits[idx + 1] {
                // Skip 00
                idx += 2;
                value = bits[idx] as i64;
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 1;
            } else if !bits[idx] && bits[idx + 1] {
                // Skip 01
                idx += 2;
                for i in 0..5 {
                    value |= (bits[idx + i] as i64) << i;
                }
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 5;
            } else if bits[idx] && !bits[idx + 1] {
                // Skip 10
                idx += 2;
                for i in 0..9 {
                    value |= (bits[idx + i] as i64) << i;
                }
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 9;
            } else if bits[idx] && bits[idx + 1] && !bits[idx + 2] {
                // Skip 110
                idx += 3;
                for i in 0..16 {
                    value |= (bits[idx + i] as i64) << i;
                }
                value = (value >> 1) ^ -(value & 1);
                output.push(value);
                idx += 16;
            } else if bits[idx] && bits[idx + 1] && bits[idx + 2] {
                // Skip 111
                idx += 3;
                let mut value: i128 = 0;
                for i in 0..64 {
                    value |= (bits[idx + i] as i128) << i;
                }
                let value = (value >> 1) ^ -(value & 1);
                output.push(value as i64);
                idx += 64;
            } else {
                return Err(CodingError::InvalidBits);
            }
        }
    }

    return Ok(Some(idx));
}
