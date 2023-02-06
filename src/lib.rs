//! # Tight Time-series Compression
//!
//! Time-series data points consist of a timestamp and typically one or more values.
//! This crate implements a specialized compression algorithm for time-series data that meets the following requirements:
//!
//! 0. Timestamps are integral values (signed or unsigned).
//!   a. Data points may have one or more timestamps (SoC time, UTC time, etc.)
//! 1. Data point values are integral values (signed or unsigned).
//!   a. Separate value columns may have variable bit-widths, but all values in a column must have the same bit-width.
//! 2. Data points are sorted by timestamp.
//! 3. Data points are sampled at a regular interval, but not required to have exactly the same interval.
//! 4. Data points are compressed in a lossless manner.
//! 5. Data points are compressed as a stream, without requiring the entire time-series to be available at once.
//! 6. Compressed payloads are small and can be transmitted over a BLE network.
//!   a. Typical payloads are 100-251 byte packets.
//!   b. Payloads will be precisely at most MTU-sized packets that not require fragmentation.
//!
//! ## Targetted Use Case
//!
//! The primary use case for this crate is to compress time-series data for transmission over a BLE network.
//! Data generated by ultra-low power BLE sensors often comes with a uint32_t or int64_t timestamp and a several data channels, typically int16_t or smaller in bit-width.
//! Sensor data often has a temporal relationship that encodes frequency and amplitude information changing over time.
//! Sensors may not have more than a small number of KiB of RAM available. Both the compression algorithm and the buffered encoding must be able to operate on a small amount of memory.
//!
//! An nRF52840 SoC has 256 KiB of RAM, and the BLE stack requires 64 KiB of RAM. If a sensor records 4 channels of int16_t data at 1 Hz with an int64_t millisecond timestamp, the uncompressed data will be 16B per second, or 57KiB per hour.
//! Some sensors may lose connection for hours, so this uncompressed data would fill RAM in 4 hours of RAM. A typical BLE TX MTU of 251 bytes can notify packets efficiently without fragmentation.
//!
//! A compression algorithm that can reduce the bytes per row from 16B to 1B would reduce the RAM requirement to 3.5 KiB per hour. This reduction would allow the sensor to record for more than 2 days between connections.
//! For the nRF52840 SoC, this would allow the sensor to optimize for power consumption by using the radio less frequently, potentially adding months or years of battery life. Therefore, we will target streaming compression to packets of 251 bytes.
//!
//! ## Compression Algorithm
//!
//! Based off of Gorilla paper: <https://www.vldb.org/pvldb/vol8/p1816-teller.pdf>, we will specialize the algorithm for our use case. From Gorilla's experience in practice, we can hope to achieve the operational target from the use-case above.
//!
//! > "we found that about 96% of all time stamps can be compressed to a single bit".
//!
//! Terminology from Gorilla and here will be used interchangeably. block = packet = payload, point = data point, value = data value, timestamp = timestamp.
//!
//! The primary difference between Gorilla and this specialization is that all values are integral values that can be treated similarly to timestamps except with a different bit-widths.
//! Floating point values are not encoded and must be converted to integral values before encoding.
//!
//! ### Timestamps
//!
//! Timestamps are integral values, and are encoded as a delta from the previous timestamp. The timestamp ticks are assumed to be
//! in ticks of milliseconds, in ticks of microseconds, or in 32768 Hz clock ticks.
//!
//! The first timestamp in the block is an absolute timestamp. The first timestamp in the block is a 64-bit unsigned integer encoded as a variable length quantity (VLQ).
//! Bytes of preceding zeros are omitted, and the first non-zero byte is encoded with the high bit set to 1. The remaining bytes are most-significant bits not-encoded are set to 0.
//!
//! ### VLQ Encoding
//! See <https://en.wikipedia.org/wiki/Variable-length_quantity> for more information.
//!
//! As an example, 167544397000000 in epoch microseconds is Friday, February 3, 2023 5:09:27 PM GMT. In binary, this is 0b0000000000000101111100111100111010111110011010100011111111000000
//! This requires 8 bytes to VLQ encode.
//!
//! Another example, 3600000 in milliseconds is 1 hour. This requires 4 bytes to VLQ encode.
//!
//! ### Subsequent Timestamp and Value Encoding
//!
//! Subsequent timestamps and values are treated the same way. Per packet each timestamp and value column is encoded as a delta from the delta of the first and the second.
//! As above, the first timestamp is fully represented. The second timestamp is encoded as the a delta, which is the difference between the first and second timestamp.
//! The second timestamp may be reconstructed by adding the first timestamp to the second timestamp delta.
//!
//! For all following timestamps/columnar values, the delta-delta for each data point is encoded depending on the magnitude of change (due to interval duration, timestamp resolution, or inherent time-series data properties).
//!
//! 1. The delta delta is calculated as `D = (t - t_prev) - (t_prev - t_prev_prev)`.
//! 2. If `D` is 0, then the timestamp is encoded as a single bit with value 0.
//! 3. If `D` is between [-7, 8], store '10' followed by the value (4 bits).
//! 3. If `D` is between [-63, 64], store '110' followed by the value (7 bits).
//! 4. If `D` is between [-255, 256], store '1110' followed by the value (9 bits).
//! 5. If `D` is between [-2047, 2048], store '11110' followed by the value (12 bits).
//! 6. If `D` is between [-16383, 16384], store '111110' followed by the value (15 bits).
//! 7. If `D` is between [-131071, 131072], store '1111110' followed by the value (18 bits).
//! 8. If `D` is between [- ((1 << 31) - 1), (1 << 31)], store '11111110' followed by the value (32 bits).
//! 9. If `D` is between [- ((1 << 63) - 1), (1 << 63)], store '11111111' followed by the value (64 bits).
//!
//! ### Example
//!
//! The following example encodes 2 timestamps and 4 values. The first timestamp is an SoC uptime ms. The second timestamp is UTC us. The values are 4 channels of int16_t data incrementing slowly and sometimes resetting. Data in this example is collected at 1 Hz.
//!
//! | soc | utc | channel0 | channel1 | channel2 | channel3 |
//! | --- | --- | -------- | -------- | -------- | -------- |
//! | 250 | 1675465460000000 | 0 | 100 | 200 | 300 |
//! | 1250 | 1675465461000153 | 2 | 101 | 200 | 299 |
//! | 2250 | 1675465462000512 | 4 | 103 | 201 | 301 |
//! | 3251 | 1675465463000913 | 7 | 104 | 202 | 302 |
//! | 4251 | 1675465464001300 | 9 | 105 | 203 | 303 |
//!
//! This example includes 5 data points. The first data point is encoded as a full timestamp. The second data point is encoded as a delta from the first data point. The third data point is encoded as a delta from the second data point. The fourth data point is encoded as a delta from the third data point. The fifth data point is encoded as a delta from the fourth data point.
//!
//! Be warned, the 0bxxxxxxx binary expansion was autogenerated w/ Copilot and probably isn't quite right.
//!
//! For the first row, the full value is VLQ encoded for each column:
//!
//! | soc | utc | channel0 | channel1 | channel2 | channel3 |
//! | --- | --- | -------- | -------- | -------- | -------- |
//! | 251 | 1675465460000000 | 0 | 100 | 200 | 300 |
//!
//!
//! For the second row, the delta between the first and second row is encoded with the delta-delta encoding for each column:
//!
//! | soc_d | utc_d | channel0_d | channel1_d | channel2_d | channel3_d |
//! | --- | --- | -------- | -------- | -------- | -------- |
//! | 1000 | 1000153 | 2 | 1 | 0 | -1 |
//! | 0b11110 0b001111101000 | 0b11111110 0b00000000000011110100001011011001 | 0b10 0b0010 | 0b10 0b0001 | 0b0 | 0b10 0b1111 |
//!
//! For the third row, the delta-delta is encoded for each column:
//!
//! | soc_dd | utc_dd | channel0_dd | channel1_dd | channel2_dd | channel3_dd |
//! | --- | --- | -------- | -------- | -------- | -------- |
//! | (2250 - 1250) - (1250 - 250) = 0 | (1675465462000512 - 1675465461000153) - (1675465461000153 - 1675465460000000) = 206 | (4 - 2) - (2 - 0) = 0 | (103 - 101) - (101 - 100) = 1 | (201 - 200) - (200 - 200) = 1 | (301 - 299) - (299 - 300) = 3 |
//! | 0b0 | 0b110 0b011001110 | 0b0 | 0b10 0b0001 | 0b10 0b0001 | 0b10 0b0011 |
//!
//! For the fourth row, the delta-delta is encoded for each column:
//!
//! | soc_dd | utc_dd | channel0_dd | channel1_dd | channel2_dd | channel3_dd |
//! | --- | --- | -------- | -------- | -------- | -------- |
//! | (3251 - 2250) - (2250 - 1250) = 1 | (1675465463000913 - 1675465462000512) - (1675465462000512 - 1675465461000153) = 42 | (7 - 4) - (4 - 2) = 1 | (104 - 103) - (103 - 101) = -1 | (202 - 201) - (201 - 200) = 0 | (302 - 301) - (301 - 299) = -1 |
//! | 0b10 0b0001 | 0b110 0b0101010 | 0b10 0b0001 | 0b10 0b1111 | 0b0 | 0b10 0b1111 |
//!
//! For the fifth row, the delta-delta is encoded for each column:
//!
//! | soc_dd | utc_dd | channel0_dd | channel1_dd | channel2_dd | channel3_dd |
//! | --- | --- | -------- | -------- | -------- | -------- |
//! | (4251 - 3251) - (3251 - 2250) = -1 | (1675465464001300 - 1675465463000913) - (1675465463000913 - 1675465462000512) = -14 | (9 - 7) - (7 - 4) = -1 | (105 - 104) - (104 - 103) = 0 | (203 - 202) - (202 - 201) = 0 | (303 - 302) - (302 - 301) = 0 |
//! | 0b10 0b1111 | 0b110 0b1110010 | 0b10 0b1111 | 0b0 | 0b0 | 0b0 |
//!
//!
//! | soc_bits | utc_bits | channel0_bits | channel1_bits | channel2_bits | channel3_bits |
//! | --- | --- | -------- | -------- | -------- | -------- |
//! | 16  | 64 | 8 | 8 | 16 | 16 |
//! | 17 | 40 | 6 | 6 | 6 | 1 | 6 |
//! | 1 | 10 | 1 | 6 | 6 | 6 |
//! | 6 | 10  | 6 | 6 | 1 | 6 |
//! | 6 | 10 | 6 | 1 | 1 | 1 |
//!
//! The total number of bits is 16 + 64 + 8 + 8 + 16 + 16 + 17 + 40 + 6 + 6 + 6 + 1 + 6 + 1 + 10 + 1 + 6 + 6 + 6 + 6 + 10 + 6 + 6 + 1 + 6 + 6 + 10 + 6 + 1 + 1 + 1 = 300 bits.
//! With an uncompressed row size of 64 + 64 + 16 + 16 + 16 + 16 = 192 bits, this is a ratio of 960 / 300 = 3.2.
//!
//! The average delta-delta row size in the example was 30 bits and the target MTU size is 251 * 8 = 2008 bits. (2000 - 200) / 30 = 60 rows.
//! So for a max of about 60 rows with similar data, we could represent 62 * 192 = 11904 bits with 2008 bits for 5.9x compression.
//!
//! ### Framing and Nullability
//!
//! In order to parse the data, we need to know the number of columns to parse as each timestamp or row is read. This information is encoded in a framing header
//! at the beginning of each packet. Nullifying a column is done by setting the corresponding bit in the header to 0. All delta-delta values will skip nullified rows.
//! This can greatly reduce the number of bits required to encode a row for rows that may be interpolated or have missing data across packets.
//! The header prefix is a VLQ encoded integer that is the number of columns in the packet as well as a VLQ encoded bit mask of column nullability where 0 is null and 1 is not null.
//!
//! For the example above, the header would be 6 columns and none of them null: 0b00000110 0b111111.
//!
//! If the above example was modified to interpolate the UTC us, the header would be 6 columns and the UTC column would be null: 0b00000110 0b101111.
//! If the header was nullified, then the utc_d and utc_dd values would all be skipped. The resulting total number of bits would be 16 + 64 + 8 + 8 + 16 + 16 + 17 + 6 + 6 + 6 + 1 + 1 + 1 + 6 + 6 + 6 + 6 + 6 + 6 + 6 + 1 + 6 + 6 + 6 + 1 + 1 + 1 = 230 bits.
//! In both cases the header is an additional 14 bits. This is 314 / 244 = 1.28x compression over fully representing the UTC us.
//!
//!
//!
//!
//!

#![cfg_attr(not(test), no_std, no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;

pub mod compress;
pub mod vlq;

// A static variable to flag initialization once.
static mut INITED: AtomicBool = AtomicBool::new(false);

///
/// Initialize (the Cortex-M heap) to prepare encoding.
///
/// This should only be called once.
///
#[no_mangle]
pub unsafe extern "C" fn tsz_init(_heap_start: *mut u8, _heap_size: u32) {
    let Ok(false  ) = INITED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) else {
        return;
    };
}

#[cfg(test)]
mod tests {
    use std::thread::spawn;

    use super::*;

    #[test]
    fn thread_safe_init() {
        (0..16)
            .map(|_| {
                spawn(move || unsafe {
                    tsz_init(core::ptr::null_mut(), 0);
                })
            })
            .for_each(|t| t.join().unwrap());
    }
}
