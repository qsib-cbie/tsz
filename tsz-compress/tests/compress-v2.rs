#![allow(unused)]
use tsz_compress::prelude::*;

extern crate alloc;

use rand::Rng;

#[cfg(test)]
mod tests {

    use super::*;
    use rand::Rng;

    #[test]
    fn test_macro_compress_sanity1_i16() {
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

        /// Test 10 samples (size of queue)
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
    fn test_macro_compress_sanity2_i16() {
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
    fn test_macro_compress_sanity3_i16() {
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
    fn test_macro_compress_random_i16() {
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
    fn test_macro_compress_all_i16_deltas() {
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
    fn test_macro_compress_sanity1_i32() {
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

        /// Test 10 samples (size of queue)
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
    fn test_macro_compress_sanity2_i32() {
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
    fn test_macro_compress_random_i32() {
        // Test with random values with delta within i32 range
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

            // Generate input vector randomly such that delta is in i32 range
            let values: Vec<i32> = (0..end_range)
                .map(|_| rng.gen_range((i32::MIN / 2)..(i32::MAX - 1) / 2))
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
