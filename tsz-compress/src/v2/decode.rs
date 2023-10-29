use crate::prelude::*;

// Delta Decoding

pub fn decode_delta_i64(bits: &'_ BitBufferSlice) -> Result<([i64; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i64; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut value: i64 = 0;

    if !bits[0] {
        return Err(CodingError::InvalidBits);
    }

    // Case 1: 00
    if !bits[1] && !bits[2] {
        // Skipping pad 0
        for i in (4..36).step_by(16) {
            for j in 0..16 {
                value |= (bits[i + j] as i64) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 2: 01
    else if !bits[1] && bits[2] {
        // Skipping pad 000
        for i in (6..36).step_by(10) {
            for j in 0..10 {
                value |= (bits[i + j] as i64) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 3: 10
    else if bits[1] && !bits[2] {
        // Skipping pad 0
        for i in (4..36).step_by(8) {
            for j in 0..8 {
                value |= (bits[i + j] as i64) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 4: 110
    else if bits[1] && bits[2] && !bits[3] {
        for i in (4..36).step_by(4) {
            for j in 0..4 {
                value |= (bits[i + j] as i64) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 5: 111
    else if bits[1] && bits[2] && bits[3] {
        // Skipping pad 00
        for i in (6..36).step_by(3) {
            for j in 0..3 {
                value |= (bits[i + j] as i64) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    } else {
        return Err(CodingError::InvalidBits);
    }
    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_i32(bits: &'_ BitBufferSlice) -> Result<([i32; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i32; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut value: i32 = 0;

    if !bits[0] {
        return Err(CodingError::InvalidBits);
    }

    // Case 1: 00
    if !bits[1] && !bits[2] {
        // Skipping pad 0
        for i in (4..36).step_by(16) {
            for j in 0..16 {
                value |= (bits[i + j] as i32) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 2: 01
    else if !bits[1] && bits[2] {
        // Skipping pad 000
        for i in (6..36).step_by(10) {
            for j in 0..10 {
                value |= (bits[i + j] as i32) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 3: 10
    else if bits[1] && !bits[2] {
        // Skipping pad 0
        for i in (4..36).step_by(8) {
            for j in 0..8 {
                value |= (bits[i + j] as i32) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 4: 110
    else if bits[1] && bits[2] && !bits[3] {
        for i in (4..36).step_by(4) {
            for j in 0..4 {
                value |= (bits[i + j] as i32) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 5: 111
    else if bits[1] && bits[2] && bits[3] {
        // Skipping pad 00
        for i in (6..36).step_by(3) {
            for j in 0..3 {
                value |= (bits[i + j] as i32) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    } else {
        return Err(CodingError::InvalidBits);
    }
    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_i16(bits: &'_ BitBufferSlice) -> Result<([i16; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i16; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut value: i16 = 0;

    if !bits[0] {
        return Err(CodingError::InvalidBits);
    }

    // Case 1: 00
    if !bits[1] && !bits[2] {
        // Skipping pad
        let mut value: i32 = value as i32;
        for i in (4..36).step_by(16) {
            for j in 0..16 {
                value |= (bits[i + j] as i32) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value as i16;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 2: 01
    else if !bits[1] && bits[2] {
        // Skipping pad 000
        for i in (6..36).step_by(10) {
            for j in 0..10 {
                value |= (bits[i + j] as i16) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 3: 10
    else if bits[1] && !bits[2] {
        // Skipping pad 0
        for i in (4..36).step_by(8) {
            for j in 0..8 {
                value |= (bits[i + j] as i16) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 4: 110
    else if bits[1] && bits[2] && !bits[3] {
        for i in (4..36).step_by(4) {
            for j in 0..4 {
                value |= (bits[i + j] as i16) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 5: 111
    else if bits[1] && bits[2] && bits[3] {
        // Skipping pad 00
        for i in (6..36).step_by(3) {
            for j in 0..3 {
                value |= (bits[i + j] as i16) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    } else {
        return Err(CodingError::InvalidBits);
    }
    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_i8(bits: &'_ BitBufferSlice) -> Result<([i8; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i8; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut value: i8 = 0;

    if !bits[0] {
        return Err(CodingError::InvalidBits);
    }

    // Case 1: 00
    if !bits[1] && !bits[2] {
        // Skipping pad 0
        let mut value = value as i16;
        for i in (4..36).step_by(16) {
            for j in 0..16 {
                value |= (bits[i + j] as i16) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value as i8;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 2: 01
    else if !bits[1] && bits[2] {
        // Skipping pad 000
        let mut value = value as i16;
        for i in (6..36).step_by(10) {
            for j in 0..10 {
                value |= (bits[i + j] as i16) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value as i8;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 3: 10
    else if bits[1] && !bits[2] {
        // Skipping pad 0
        let mut value = value as i16;
        for i in (4..36).step_by(8) {
            for j in 0..8 {
                value |= (bits[i + j] as i16) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value as i8;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 4: 110
    else if bits[1] && bits[2] && !bits[3] {
        for i in (4..36).step_by(4) {
            for j in 0..4 {
                value |= (bits[i + j] as i8) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    }
    // Case 5: 111
    else if bits[1] && bits[2] && bits[3] {
        // Skipping pad 00
        for i in (6..36).step_by(3) {
            for j in 0..3 {
                value |= (bits[i + j] as i8) << j;
            }
            value = (value >> 1) ^ -(value & 1); // ZigZag decoding
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            value = 0;
        }
    } else {
        return Err(CodingError::InvalidBits);
    }
    return Ok((decoded_buffer, decoded_buffer_index));
}

// Delta Delta Decoding
pub fn decode_delta_delta_i8(bits: &'_ BitBufferSlice) -> Result<([i8; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i8; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut idx = 0;
    while idx < bits.len() {
        if bits[idx] {
            return Err(CodingError::InvalidBits);
        }
        let mut value = 0;
        if !bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            value = bits[idx] as i8;
            let value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 1;
        } else if !bits[idx + 1] && bits[idx + 2] {
            idx += 3;
            for i in 0..5 {
                value |= (bits[idx + i] as i8) << i;
            }
            let value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 5;
        } else if bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            let mut value: i16 = 0;
            for i in 0..9 {
                value |= (bits[idx + i] as i16) << i;
            }
            // ZigZag decoding
            let value = (value >> 1) ^ -(value & 1);

            decoded_buffer[decoded_buffer_index] = value as i8;
            decoded_buffer_index += 1;
            idx += 9;
        } else {
            return Err(CodingError::InvalidBits);
        }
    }
    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_delta_i16(bits: &'_ BitBufferSlice) -> Result<([i16; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i16; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut idx = 0;
    while idx < bits.len() {
        if bits[idx] {
            return Err(CodingError::InvalidBits);
        }
        let mut value = 0;
        if !bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            value = bits[idx] as i16;
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 1;
        } else if !bits[idx + 1] && bits[idx + 2] {
            idx += 3;
            for i in 0..5 {
                value |= (bits[idx + i] as i16) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 5;
        } else if bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            for i in 0..9 {
                value |= (bits[idx + i] as i16) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 9;
        } else if bits[idx + 1] && bits[idx + 2] && !bits[idx + 3] {
            idx += 4;
            for i in 0..16 {
                value |= (bits[idx + i] as i16) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 16;
        } else if bits[idx + 1] && bits[idx + 2] && bits[idx + 3] {
            idx += 4;
            let mut value: i64 = 0;
            for i in 0..64 {
                value |= (bits[idx + i] as i64) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value as i16;
            decoded_buffer_index += 1;
            idx += 64;
        } else {
            return Err(CodingError::InvalidBits);
        }
    }

    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_delta_i32(bits: &'_ BitBufferSlice) -> Result<([i32; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i32; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut idx = 0;
    while idx < bits.len() {
        if bits[idx] {
            return Err(CodingError::InvalidBits);
        }
        let mut value = 0;
        if !bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            value = bits[idx] as i32;
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 1;
        } else if !bits[idx + 1] && bits[idx + 2] {
            idx += 3;
            for i in 0..5 {
                value |= (bits[idx + i] as i32) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 5;
        } else if bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            for i in 0..9 {
                value |= (bits[idx + i] as i32) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 9;
        } else if bits[idx + 1] && bits[idx + 2] && !bits[idx + 3] {
            idx += 4;
            for i in 0..16 {
                value |= (bits[idx + i] as i32) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 16;
        } else if bits[idx + 1] && bits[idx + 2] && bits[idx + 3] {
            idx += 4;
            let mut value: i64 = 0;
            for i in 0..64 {
                value |= (bits[idx + i] as i64) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value as i32;
            decoded_buffer_index += 1;
            idx += 64;
        } else {
            return Err(CodingError::InvalidBits);
        }
    }

    return Ok((decoded_buffer, decoded_buffer_index));
}

pub fn decode_delta_delta_i64(bits: &'_ BitBufferSlice) -> Result<([i64; 10], usize), CodingError> {
    if bits.is_empty() {
        return Err(CodingError::NotEnoughBits);
    }
    let mut decoded_buffer: [i64; 10] = [0; 10];
    let mut decoded_buffer_index = 0;
    let mut idx = 0;
    while idx < bits.len() {
        if bits[idx] {
            return Err(CodingError::InvalidBits);
        }
        let mut value = 0;
        if !bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            value = bits[idx] as i64;
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 1;
        } else if !bits[idx + 1] && bits[idx + 2] {
            idx += 3;
            for i in 0..5 {
                value |= (bits[idx + i] as i64) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 5;
        } else if bits[idx + 1] && !bits[idx + 2] {
            idx += 3;
            for i in 0..9 {
                value |= (bits[idx + i] as i64) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 9;
        } else if bits[idx + 1] && bits[idx + 2] && !bits[idx + 3] {
            idx += 4;
            for i in 0..16 {
                value |= (bits[idx + i] as i64) << i;
            }
            value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value;
            decoded_buffer_index += 1;
            idx += 16;
        } else if bits[idx + 1] && bits[idx + 2] && bits[idx + 3] {
            idx += 4;
            let mut value: i128 = 0;
            for i in 0..64 {
                value |= (bits[idx + i] as i128) << i;
            }
            let value = (value >> 1) ^ -(value & 1);
            decoded_buffer[decoded_buffer_index] = value as i64;
            decoded_buffer_index += 1;
            idx += 64;
        } else {
            return Err(CodingError::InvalidBits);
        }
    }
    return Ok((decoded_buffer, decoded_buffer_index));
}
