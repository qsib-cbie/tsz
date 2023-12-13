# TSZ Benchmarks

This crate contains benchmarks for TSZ along with dependencies to run benchmarks that might pollute the main crates.

## Results

In the real-world benchmark consisting of (de)compressing 5 minutes of wearable accelerometer data sampled at 3.3kHz with 10Hz lowpass filters applied to x and y axis and 1500Hz lowpass filter applied to z axis, TSZ V2 compresses around 80 million rows per second. 

At the time of writing, the benchmarks are single-threaded. On a Mac Studio, compressing the 5-minutes of data takes about 12.5ms and decompressing takes about 15.5ms. A second-phase compression algorithm like LZ4 added 1.5ms to the compression and decompression. That ~10% runtime cost saves about 50% space.


| Test                        | Time (ms)           |
|-----------------------------|---------------------|
| compress txyz 10k           | 123.16 - 123.56 µs  |
| compress txyz 100k          | 1.2496 - 1.2599 ms  |
| compress txyz 941009 rows   | 12.740 - 12.841 ms  |
| decompress txyz 941009 rows | 15.103 - 15.250 ms  |
| two-phase compress txyz     | 13.858 - 13.981 ms  |
| two-phase decompress txyz   | 16.130 - 16.239 ms  |





