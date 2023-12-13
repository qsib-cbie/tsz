# TSZ (V2) Benchmarks

This crate contains benchmarks for TSZ along with dependencies to run benchmarks that might pollute the main crates.

## Results

In the real-world benchmark consisting of (de)compressing 5 minutes of wearable accelerometer data sampled at 3.3kHz with 10Hz lowpass filters applied to x and y axis and 1500Hz lowpass filter applied to z axis, TSZ V2 compresses around 80 million rows per second. 

At the time of writing, the benchmarks are single-threaded. On a Mac Studio, compressing the 5-minutes of data takes about 12.5ms and decompressing takes about 15.5ms. A second-phase compression algorithm like LZ4 added 1.5ms to the compression and decompression. That ~10% runtime cost comes with big space savings on TSZ V2's nibble aligned encoding. LZ4 directly on the raw data when flattened in column-major bytes is poor.

This benchmark demonstrates how TSZ can quickly encode sampled data into a format that is easier for LZ4 to compress efficiently. End-to-end, TSZ V2 + LZ4 is 1.5x faster to compress and 6.1x smaller.


| Benchmark                   | Time (ms)           |
|-----------------------------|---------------------|
| compress txyz 10k           | 123.16 - 123.56 µs  |
| compress txyz 100k          | 1.2496 - 1.2599 ms  |
| compress txyz 941009 rows   | 12.740 - 12.841 ms  |
| decompress txyz 941009 rows | 15.103 - 15.250 ms  |
| two-phase compress txyz     | 13.858 - 13.981 ms  |
| two-phase decompress txyz   | 16.130 - 16.239 ms  |



| Compression       | Size (Bytes) | Compression Ratio | Compression Speed (ms) | Compression Rate (MBps) |
|-------------------|--------------|-------------------|------------------------|-------------------------|
| Original          | 18,820,180   | 1.00x             | N/A                    | N/A                     |
| TSZ Phase 1       | 3,138,798    | 6.00x             | 12.791                 | 1471.44                 |
| LZ4 (TSZ Phase 2) | 1,129,653    | 16.67x            | 13.920                 | 1352.10                 |
| Only LZ4          | 6,851,887    | 2.75x             | 21.432                 | 878.14                  |


## Caveats

The row size could use i16's for x, y, and z in this demo. This is chosen to fit larger bit-widths. For example, a 19-bit ADC will end up stuffed into an i32 quite commonly. The despite the oversizing, LZ4 could not capitalize on the under-utilized bit0widths.

This is 5-minutes of accelerometer data from a person wearing it mostly in the upright positions. Data streams from different front-ends will elicit greater chaos in the hot code paths. Performance varies across hosts, especially considering the potential inability for the embedded SoC to do branch prediction.

There _is_ allocation in the compression scheme. Performance tests on embedded indicate two-level segregated fit is more appropriate than linked-list first fit allocation scheme. Manual performance validation happened on an nRF52840 SoC with between 128 and 192 KiB available on the heap.
