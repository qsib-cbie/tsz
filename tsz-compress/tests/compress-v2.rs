#![allow(unused)]
use tsz_compress::prelude::*;

extern crate alloc;

use rand::Rng;

#[cfg(test)]
mod tests {

    use super::*;
    use rand::Rng;

    // Tests for 0 rows compressed and decompressed
    #[test]
    fn test_macro_compress_i8_zero_rows() {
        // Test with delta within i8 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i8,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }

        use row::*;
        const N: usize = 0;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        let result = decompressor.decompress(&bytes);

        // Assert that the decompressed data matches the original
        assert_eq!(result.unwrap(), ());
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_i16_zero_rows() {
        // Test with delta within i16 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }

        use row::*;
        const N: usize = 0;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        let result = decompressor.decompress(&bytes);

        // Assert that the decompressed data matches the original
        assert_eq!(result.unwrap(), ());
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_i32_zero_rows() {
        // Test with delta within i32 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }

        use row::*;
        const N: usize = 0;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        let result = decompressor.decompress(&bytes);

        // Assert that the decompressed data matches the original
        assert_eq!(result.unwrap(), ());
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_i64_zero_rows() {
        // Test with delta within i64 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }

        use row::*;
        const N: usize = 0;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        let result = decompressor.decompress(&bytes);

        // Assert that the decompressed data matches the original
    }

    // Tests for 1 row compressed and decompressed
    #[test]
    fn test_macro_compress_i8_one_rows() {
        // Test with delta within i8 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i8,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }

        use row::*;
        const N: usize = 1;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        let result = decompressor.decompress(&bytes);

        // Assert that the decompressed data matches the original
        assert_eq!(result.unwrap(), ());
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_i16_one_row() {
        // Test with delta within i16 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }

        use row::*;
        const N: usize = 1;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        let result = decompressor.decompress(&bytes);

        // Assert that the decompressed data matches the original
        assert_eq!(result.unwrap(), ());
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_i32_one_row() {
        // Test with delta within i32 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }

        use row::*;
        const N: usize = 1;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        let result = decompressor.decompress(&bytes);

        // Assert that the decompressed data matches the original
        assert_eq!(result.unwrap(), ());
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_i64_one_row() {
        // Test with delta within i64 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }

        use row::*;
        const N: usize = 1;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        let result = decompressor.decompress(&bytes);

        // Assert that the decompressed data matches the original
        assert_eq!(result.unwrap(), ());
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_sanity_i8_bit_width_values1() {
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i8,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;
        const N: usize = 20;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_sanity_i8_bit_width_values2() {
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i8,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        let values = vec![
            -100,
            -9,
            i8::MIN,
            i8::MAX,
            120,
            2,
            -85,
            -10,
            1,
            72,
            49,
            95,
            -97,
            -94,
        ];

        for value in &values {
            let row = TestRow { a: *value };
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i8_bit_width_values_random() {
        // Test with random values within i8 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i8,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            // Initialize the compressor
            let mut compressor = TestRowCompressorImpl::new(128);

            // Number of samples in the input vector
            let end_range = rng.gen_range(100..10000);

            // Create a vector with the specified number of elements
            let mut values = vec![0i8; end_range];

            // Fill the vector with random i8 values
            rng.fill(values.as_mut_slice());

            // Compression
            for value in &values {
                let row = TestRow { a: *value };
                compressor.compress(row);
            }

            // Finalize the compression
            let bytes = compressor.finish();

            // Initialize the decompressor
            let mut decompressor = TestRowDecompressorImpl::new();

            // Decompress the bit buffer
            decompressor.decompress(&bytes).unwrap();

            // Assert that the decompressed data matches the original
            assert_eq!(values, decompressor.col_a());
        }
    }

    #[test]
    fn test_macro_compress_i8_bit_width_values_all_deltas() {
        // Test with all deltas possible by i8 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i8,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i8> = Vec::with_capacity((i8::MAX as usize + 1) * 8 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        // Compress steady state values such that delta ranges from {-128 - 127 = -255} and {127 - -128 = 255}
        for i in i8::MIN..=i8::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i8::MIN });
            values.push(i);
            values.push(i8::MIN);
        }

        for i in i8::MIN..=i8::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i8::MAX });
            values.push(i);
            values.push(i8::MAX);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_sanity_i16_bit_width_values1() {
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;
        const N: usize = 20;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_sanity_i16_bit_width_values2() {
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        let values = vec![-100, -9, -31, -2, 1, 72, 49, 95, -97, -94];

        for value in &values {
            let row = TestRow { a: *value };
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_sanity_i16_bit_width_values3() {
        // Test with deltas out of i16 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        let values = vec![1, -26679, 28996, 1, 1, 1, 1, 1, 1, 1]; // 28895 - -26679 = 55574 > i16::MAX

        for value in &values {
            let row = TestRow { a: *value };
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i16_bit_width_values_random() {
        // Test with random values within i16 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            // Initialize the compressor
            let mut compressor = TestRowCompressorImpl::new(128);

            // Number of samples in the input vector
            let end_range = rng.gen_range(100..10000);

            // Create a vector with the specified number of elements
            let mut values = vec![0i16; end_range];

            // Fill the vector with random i16 values
            rng.fill(values.as_mut_slice());

            // Compression
            for value in &values {
                let row = TestRow { a: *value };
                compressor.compress(row);
            }

            // Finalize the compression
            let bytes = compressor.finish();

            // Initialize the decompressor
            let mut decompressor = TestRowDecompressorImpl::new();

            // Decompress the bit buffer
            decompressor.decompress(&bytes).unwrap();

            // Assert that the decompressed data matches the original
            assert_eq!(values, decompressor.col_a());
        }
    }

    #[test]
    fn test_macro_compress_i16_bit_width_values_all_deltas() {
        // Test with all deltas possible by i16 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i16> = Vec::with_capacity((i16::MAX as usize + 1) * 8 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        // Compress steady state values such that delta ranges from {-32768 - 32767 = -65535} and {32767 - -32768 = 65535}
        for i in i16::MIN..=i16::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i16::MIN });
            values.push(i);
            values.push(i16::MIN);
        }

        for i in i16::MIN..=i16::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i16::MAX });
            values.push(i);
            values.push(i16::MAX);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_sanity_i32_bit_width_values1() {
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;
        const N: usize = 13;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_sanity_i32_bit_width_values2() {
        // Test with delta within i32 range
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        let values = vec![1, i32::MIN / 2, (i32::MAX - 1) / 2, 1, 1, 1, 1, 1, 1, 1];

        for value in &values {
            let row = TestRow { a: *value };
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i32_value_edge_cases() {
        // Test with edge cases of deltas possible by i32 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        let values = vec![
            1,
            1,
            i32::MIN,
            i32::MAX,
            i32::MIN,
            i32::MAX,
            i32::MIN,
            i32::MAX,
            i32::MIN,
            i32::MAX,
            1,
        ];

        for value in &values {
            let row = TestRow { a: *value };
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i32_bit_width_values_random() {
        // Test with random i32 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            // Initialize the compressor
            let mut compressor = TestRowCompressorImpl::new(128);

            // Number of samples in the input vector
            let end_range = rng.gen_range(100..10000);

            // Generate input vector randomly
            let values: Vec<i32> = (0..end_range)
                .map(|_| rng.gen_range(i32::MIN..i32::MAX))
                .collect();

            // Compression
            for value in &values {
                let row = TestRow { a: *value };
                compressor.compress(row);
            }

            // Finalize the compression
            let bytes = compressor.finish();

            // Initialize the decompressor
            let mut decompressor = TestRowDecompressorImpl::new();

            // Decompress the bit buffer
            decompressor.decompress(&bytes).unwrap();

            // Assert that the decompressed data matches the original
            assert_eq!(values, decompressor.col_a());
        }
    }

    #[test]
    fn test_macro_compress_sanity_i64_bit_width_values1() {
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;
        const N: usize = 13;

        // Test 10 samples (size of queue)
        let row = TestRow { a: 1 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_i64_value_edge_cases() {
        // Test with edge cases of i64 deltas possible by i64 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        let values = vec![
            1,
            1,
            (i64::MIN) / 2,
            (i64::MAX - 1) / 2,
            (i64::MIN) / 2,
            (i64::MAX - 1) / 2,
            (i64::MIN) / 2,
            (i64::MAX - 1) / 2,
            (i64::MIN) / 2,
            (i64::MAX - 1) / 2,
            1,
        ];

        for value in &values {
            let row = TestRow { a: *value };
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i64_bit_width_values_random() {
        // Test with random i64 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            // Initialize the compressor
            let mut compressor = TestRowCompressorImpl::new(128);

            // Number of samples in the input vector
            let end_range = rng.gen_range(100..1000);

            // Generate input vector randomly
            let values: Vec<i64> = (0..end_range)
                .map(|_| rng.gen_range((i64::MIN / 2)..(i64::MAX - 1) / 2))
                .collect();

            // Compression
            for value in &values {
                let row = TestRow { a: *value };
                compressor.compress(row);
            }

            // Finalize the compression
            let bytes = compressor.finish();

            // Initialize the decompressor
            let mut decompressor = TestRowDecompressorImpl::new();

            // Decompress the bit buffer
            decompressor.decompress(&bytes).unwrap();

            // Assert that the decompressed data matches the original
            assert_eq!(values, decompressor.col_a());
        }
    }
}

#[cfg(test)]
mod test_field_attributes {

    use super::*;
    use rand::Rng;

    #[test]
    fn test_macro_compress_sanity_i8_bit_width_values() {
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i8")]
                pub a: i8,
                #[tsz(delta = "i16")]
                pub b: i8,
                #[tsz(delta = "i32")]
                pub c: i8,
                #[tsz(delta = "i64")]
                pub d: i8,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;
        const N: usize = 20;

        // Test 10 samples (size of queue)
        let row = TestRow {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
        };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
        assert_eq!(decompressor.col_b(), vec![row.b; N]);
        assert_eq!(decompressor.col_c(), vec![row.c; N]);
        assert_eq!(decompressor.col_d(), vec![row.d; N]);
    }

    #[test]
    fn test_macro_compress_sanity_i16_bit_width_values() {
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i8")]
                pub a: i16,
                #[tsz(delta = "i16")]
                pub b: i16,
                #[tsz(delta = "i32")]
                pub c: i16,
                #[tsz(delta = "i64")]
                pub d: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;
        const N: usize = 20;

        // Test 10 samples (size of queue)
        let row = TestRow {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
        };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
        assert_eq!(decompressor.col_b(), vec![row.b; N]);
        assert_eq!(decompressor.col_c(), vec![row.c; N]);
        assert_eq!(decompressor.col_d(), vec![row.d; N]);
    }

    #[test]
    fn test_macro_compress_sanity_i32_bit_width_values() {
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i8")]
                pub a: i32,
                #[tsz(delta = "i16")]
                pub b: i32,
                #[tsz(delta = "i32")]
                pub c: i32,
                #[tsz(delta = "i64")]
                pub d: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;
        const N: usize = 20;

        // Test 10 samples (size of queue)
        let row = TestRow {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
        };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Compress row
        for _ in 0..N {
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
        assert_eq!(decompressor.col_b(), vec![row.b; N]);
        assert_eq!(decompressor.col_c(), vec![row.c; N]);
        assert_eq!(decompressor.col_d(), vec![row.d; N]);
    }

    #[test]
    fn test_macro_compress_i8_value_i8_delta_all() {
        // Test with all i8 deltas possible by i8 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i8")]
                pub a: i8,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i8> = Vec::with_capacity((i8::MAX as usize + 1) * 8 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in i8::MIN / 2..=(i8::MAX - 1) / 2 {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i8::MIN / 2 });
            values.push(i);
            values.push(i8::MIN / 2);
        }

        for i in i8::MIN / 2..=(i8::MAX - 1) / 2 {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: (i8::MAX - 1) / 2,
            });
            values.push(i);
            values.push((i8::MAX - 1) / 2);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i8_value_i16_delta_all() {
        // Test with all i16 deltas possible by i8 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i16")]
                pub a: i8,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i8> = Vec::with_capacity((i8::MAX as usize + 1) * 8 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in i8::MIN..=i8::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i8::MIN });
            values.push(i);
            values.push(i8::MIN);
        }

        for i in i8::MIN..=i8::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i8::MAX });
            values.push(i);
            values.push(i8::MAX);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i8_value_i32_delta_all() {
        // Test with all i32 deltas possible by i8 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i32")]
                pub a: i8,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i8> = Vec::with_capacity((i8::MAX as usize + 1) * 8 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in i8::MIN..=i8::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i8::MIN });
            values.push(i);
            values.push(i8::MIN);
        }

        for i in i8::MIN..=i8::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i8::MAX });
            values.push(i);
            values.push(i8::MAX);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i8_value_i64_delta_all() {
        // Test with all i64 deltas possible by i8 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i64")]
                pub a: i8,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i8> = Vec::with_capacity((i8::MAX as usize + 1) * 8 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in i8::MIN..=i8::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i8::MIN });
            values.push(i);
            values.push(i8::MIN);
        }

        for i in i8::MIN..=i8::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i8::MAX });
            values.push(i);
            values.push(i8::MAX);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i16_value_i8_delta_all() {
        // Test with all i8 deltas possible by i16 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i8")]
                pub a: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i16> = Vec::with_capacity((i16::MAX as usize + 1) * 4 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in ((i8::MIN / 2) as i16)..=(((i8::MAX - 1) / 2) as i16) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: (i8::MIN / 2) as i16,
            });
            values.push(i);
            values.push((i8::MIN / 2) as i16);
        }

        for i in ((i8::MIN / 2) as i16)..=(((i8::MAX - 1) / 2) as i16) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: ((i8::MAX - 1) / 2) as i16,
            });
            values.push(i);
            values.push(((i8::MAX - 1) / 2) as i16);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i16_value_i16_delta_all() {
        // Test with all i16 deltas possible by i16 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i16")]
                pub a: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i16> = Vec::with_capacity((i16::MAX as usize + 1) * 4 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in ((i16::MIN) / 2)..=((i16::MAX - 1) / 2) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: (i16::MIN) / 2 });
            values.push(i);
            values.push((i16::MIN) / 2);
        }

        for i in ((i16::MIN) / 2)..=((i16::MAX - 1) / 2) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: (i16::MAX - 1) / 2,
            });
            values.push(i);
            values.push((i16::MAX - 1) / 2);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i16_value_i32_delta_all() {
        // Test with all deltas possible by i16 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i32")]
                pub a: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i16> = Vec::with_capacity((i16::MAX as usize + 1) * 8 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in i16::MIN..=i16::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i16::MIN });
            values.push(i);
            values.push(i16::MIN);
        }

        for i in i16::MIN..=i16::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i16::MAX });
            values.push(i);
            values.push(i16::MAX);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i16_value_i64_delta_all() {
        // Test with all deltas possible by i16 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i64")]
                pub a: i16,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i16> = Vec::with_capacity((i16::MAX as usize + 1) * 8 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in i16::MIN..=i16::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i16::MIN });
            values.push(i);
            values.push(i16::MIN);
        }

        for i in i16::MIN..=i16::MAX {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow { a: i16::MAX });
            values.push(i);
            values.push(i16::MAX);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i32_value_i8_delta_all() {
        // Test with all i8 deltas possible by i32 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i8")]
                pub a: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i32> = Vec::with_capacity((i8::MAX as usize + 1) * 4 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in ((i8::MIN / 2) as i32)..=(((i8::MAX - 1) / 2) as i32) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: (i8::MIN / 2) as i32,
            });
            values.push(i);
            values.push((i8::MIN / 2) as i32);
        }

        for i in ((i8::MIN / 2) as i32)..=(((i8::MAX - 1) / 2) as i32) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: ((i8::MAX - 1) / 2) as i32,
            });
            values.push(i);
            values.push(((i8::MAX - 1) / 2) as i32);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i32_value_i16_delta_all() {
        // Test with all i16 deltas possible by i32 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i16")]
                pub a: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i32> = Vec::with_capacity((i16::MAX as usize + 1) * 4 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in ((i16::MIN / 2) as i32)..=(((i16::MAX - 1) / 2) as i32) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: (i16::MIN / 2) as i32,
            });
            values.push(i);
            values.push((i16::MIN / 2) as i32);
        }

        for i in ((i16::MIN / 2) as i32)..=(((i16::MAX - 1) / 2) as i32) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: ((i16::MAX - 1) / 2) as i32,
            });
            values.push(i);
            values.push(((i16::MAX - 1) / 2) as i32);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i32_value_i32_delta_edge_cases() {
        // Test with edge cases of i32 deltas possible by i32 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i32")]
                pub a: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        let values = vec![
            1,
            1,
            i32::MIN / 2,
            (i32::MAX - 1) / 2,
            i32::MIN / 2,
            (i32::MAX - 1) / 2,
            i32::MIN / 2,
            (i32::MAX - 1) / 2,
            i32::MIN / 2,
            (i32::MAX - 1) / 2,
            1,
        ];

        for value in &values {
            let row = TestRow { a: *value };
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i32_value_i64_delta_edge_cases() {
        // Test with edge cases of i32 deltas possible by i32 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i64")]
                pub a: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        let values = vec![
            1,
            1,
            i32::MIN,
            i32::MAX,
            i32::MIN,
            i32::MAX,
            i32::MIN,
            i32::MAX,
            i32::MIN,
            i32::MAX,
            1,
        ];

        for value in &values {
            let row = TestRow { a: *value };
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i32_values_random() {
        // Test with random values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i64")]
                pub a: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            // Initialize the compressor
            let mut compressor = TestRowCompressorImpl::new(128);

            // Number of samples in the input vector
            let end_range = rng.gen_range(100..10000);

            // Generate input vector randomly such that delta is in i32 range
            let values: Vec<i32> = (0..end_range)
                .map(|_| rng.gen_range(i32::MIN..i32::MAX))
                .collect();

            // Compression
            for value in &values {
                let row = TestRow { a: *value };
                compressor.compress(row);
            }

            // Finalize the compression
            let bytes = compressor.finish();

            // Initialize the decompressor
            let mut decompressor = TestRowDecompressorImpl::new();

            // Decompress the bit buffer
            decompressor.decompress(&bytes).unwrap();

            // Assert that the decompressed data matches the original
            assert_eq!(values, decompressor.col_a());
        }
    }

    #[test]
    fn test_macro_compress_i64_value_i8_delta_all() {
        // Test with all i8 deltas possible by i64 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i8")]
                pub a: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i64> = Vec::with_capacity((i8::MAX as usize + 1) * 4 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in ((i8::MIN / 2) as i64)..=(((i8::MAX - 1) / 2) as i64) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: (i8::MIN / 2) as i64,
            });
            values.push(i);
            values.push((i8::MIN / 2) as i64);
        }

        for i in ((i8::MIN / 2) as i64)..=(((i8::MAX - 1) / 2) as i64) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: ((i8::MAX - 1) / 2) as i64,
            });
            values.push(i);
            values.push(((i8::MAX - 1) / 2) as i64);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i64_value_i16_delta_all() {
        // Test with all i16 deltas possible by i64 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i16")]
                pub a: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        // Initialize the values vector
        let mut values: Vec<i64> = Vec::with_capacity((i16::MAX as usize + 1) * 4 + 2);

        // Write the first and second value
        compressor.compress(TestRow { a: 0 });
        compressor.compress(TestRow { a: 0 });
        values.push(0);
        values.push(0);

        for i in ((i16::MIN / 2) as i64)..=(((i16::MAX - 1) / 2) as i64) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: (i16::MIN / 2) as i64,
            });
            values.push(i);
            values.push((i16::MIN / 2) as i64);
        }

        for i in ((i16::MIN / 2) as i64)..=(((i16::MAX - 1) / 2) as i64) {
            compressor.compress(TestRow { a: i });
            compressor.compress(TestRow {
                a: ((i16::MAX - 1) / 2) as i64,
            });
            values.push(i);
            values.push(((i16::MAX - 1) / 2) as i64);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i64_value_i32_delta_edge_cases() {
        // Test with edge cases of i32 deltas possible by i64 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i32")]
                pub a: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        let values: Vec<i64> = vec![
            1,
            1,
            (i32::MIN / 2) as i64,
            ((i32::MAX - 1) / 2) as i64,
            (i32::MIN / 2) as i64,
            ((i32::MAX - 1) / 2) as i64,
            (i32::MIN / 2) as i64,
            ((i32::MAX - 1) / 2) as i64,
            (i32::MIN / 2) as i64,
            ((i32::MAX - 1) / 2) as i64,
            1,
        ];

        for value in &values {
            let row = TestRow { a: *value };
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i64_value_i64_delta_edge_cases1() {
        // Test with edge cases of i32 deltas possible by i64 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i64")]
                pub a: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        let values = vec![
            1,
            1,
            (i32::MIN) as i64,
            (i32::MAX) as i64,
            (i32::MIN) as i64,
            (i32::MAX) as i64,
            (i32::MIN) as i64,
            (i32::MAX) as i64,
            (i32::MIN) as i64,
            (i32::MAX) as i64,
            1,
        ];

        for value in &values {
            let row = TestRow { a: *value };
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i64_value_i64_delta_edge_cases2() {
        // Test with edge cases of i64 deltas possible by i64 bit-widths values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i64")]
                pub a: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        let values = vec![
            1,
            1,
            i64::MIN / 2,
            (i64::MAX - 1) / 2,
            i64::MIN / 2,
            (i64::MAX - 1) / 2,
            i64::MIN / 2,
            (i64::MAX - 1) / 2,
            i64::MIN / 2,
            (i64::MAX - 1) / 2,
            1,
        ];

        for value in &values {
            let row = TestRow { a: *value };
            compressor.compress(row);
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.col_a());
    }

    #[test]
    fn test_macro_compress_i64_values_random() {
        // Test with random values
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                #[tsz(delta = "i64")]
                pub a: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            // Initialize the compressor
            let mut compressor = TestRowCompressorImpl::new(128);

            // Number of samples in the input vector
            let end_range = rng.gen_range(100..10000);

            // Generate input vector randomly such that delta is in i32 range
            let values: Vec<i64> = (0..end_range)
                .map(|_| rng.gen_range((i64::MIN / 2)..(i64::MAX - 1) / 2))
                .collect();

            // Compression
            for value in &values {
                let row = TestRow { a: *value };
                compressor.compress(row);
            }

            // Finalize the compression
            let bytes = compressor.finish();

            // Initialize the decompressor
            let mut decompressor = TestRowDecompressorImpl::new();

            // Decompress the bit buffer
            decompressor.decompress(&bytes).unwrap();

            // Assert that the decompressed data matches the original
            assert_eq!(values, decompressor.col_a());
        }
    }
}
