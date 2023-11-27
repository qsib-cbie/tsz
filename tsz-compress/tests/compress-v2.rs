#![allow(unused)]
use tsz_compress::prelude::*;

extern crate alloc;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_macro_compress_sanity1_i8() {
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
        const N: usize = 5;

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
        println!("bytes: {:?}", bytes);

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

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
    fn test_macro_compress_random_i16() {
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

        for _ in 0..100 {
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
        println!("bytes: {:?}", bytes);

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.col_a(), vec![row.a; N]);
    }

    #[test]
    fn test_macro_compress_random_i32() {
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

        for _ in 0..100 {
            // Initialize the compressor
            let mut compressor = TestRowCompressorImpl::new(128);

            // Number of samples in the input vector
            let end_range = rng.gen_range(100..10000);

            // Create a vector with the specified number of elements
            let mut values = vec![0i32; end_range];

            // Fill the vector with random i32 values
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
    fn test_macro_compress_sanity_all() {
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i8,
                pub b: i16,
                pub c: i32,
                pub d: i64,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;
        const N: usize = 13;

        /// Test 10 samples (size of queue)
        let row = TestRow {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
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
    fn test_macro_compress_sanity_delta_col_tys() {
        mod row {
            use tsz_compress::prelude::*;
            #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
            pub struct TestRow {
                pub a: i8,
                pub b: i16,
                pub c: i32,
            }

            pub use compress::TestRowCompressorImpl;
            pub use decompress::TestRowDecompressorImpl;
        }
        use row::*;

        let a_col_vec = vec![-128, 127, -127, 126, 0, -1];
        let b_col_vec = vec![-32768, 32767, -32767, 32766, 0, -1];
        let c_col_vec = vec![-2147483648, 2147483647, -2147483647, 2147483646, 0, -1];

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new(128);

        /// Test 10 samples (size of queue)
        for i in 0..a_col_vec.len() {
            // Compress row
            compressor.compress(TestRow {
                a: a_col_vec[i],
                b: b_col_vec[i],
                c: c_col_vec[i],
            });
        }

        // Finalize the compression
        let bytes = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bytes).unwrap();

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.col_a(), a_col_vec);
        assert_eq!(decompressor.col_b(), b_col_vec);
        assert_eq!(decompressor.col_c(), c_col_vec);
    }
}
