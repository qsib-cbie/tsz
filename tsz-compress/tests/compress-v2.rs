#![allow(unused)]
use tsz_compress::prelude::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_macro_compress_test_macro_compress_sanity1() {
        /// Test 10 samples (size of queue)
        #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
        struct TestRow {
            a: i8,
        }
        let row = TestRow { a: 1 as i8 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new();

        // Compress row
        for _ in 0..10 {
            compressor.compress(row);
        }

        // Finalize the compression
        let bit_buffer = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bit_buffer);

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.a_col_vec, vec![row.a; 10]);
    }

    #[test]
    fn test_macro_compress_sanity2() {
        /// Test 5 samples i.e. multiple delta/delta-delta cases
        #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
        struct TestRow {
            a: i64,
        }
        let row = TestRow { a: i64::MAX as i64 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new();

        // Compress
        for _ in 0..5 {
            compressor.compress(row);
        }

        // Finalize the compression
        let bit_buffer = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bit_buffer);

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.a_col_vec, vec![row.a; 5]);
    }

    #[test]
    fn test_macro_compress_sanity3() {
        /// Test 6 samples i.e. multiple delta/delta-delta cases
        #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
        struct TestRow {
            a: i8,
        }
        let row = TestRow { a: 1 as i8 };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new();

        // Compress
        for _ in 0..6 {
            compressor.compress(row);
        }

        // Finalize the compression
        let bit_buffer = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bit_buffer);

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.a_col_vec, vec![row.a; 6]);
    }

    #[test]
    fn test_macro_compress_sanity4() {
        /// Test 14 samples i.e. multiple delta/delta-delta cases
        #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
        struct TestRow {
            a: i8,
        }

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new();

        // Compress
        for i in 0..14 {
            let row1 = TestRow { a: 1 as i8 };
            compressor.compress(row1);
        }

        // Finalize the compression
        let bit_buffer = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bit_buffer);

        // Assert that the decompressed data matches the original
        assert_eq!(decompressor.a_col_vec, vec![1; 14]);
    }

    #[test]
    fn test_macro_compress_sanity_i8() {
        /// Test i8 samples
        #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
        struct TestRow {
            a: i8,
        }

        let values: Vec<i8> = vec![17, 26, -15, -118, 119, -104, 67];

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new();

        // Compress
        for value in &values {
            compressor.compress(TestRow { a: *value });
        }

        // Finalize the compression
        let bit_buffer = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bit_buffer);

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.a_col_vec);
    }

    #[test]
    fn test_macro_compress_sanity_i16() {
        /// Test i16 samples
        #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
        struct TestRow {
            a: i16,
        }

        let values: Vec<i16> = vec![
            15290, -4688, 5220, -22839, -2379, -12834, -12665, 25835, 8834, 30921, 5820, -6408,
            6787, 15854, 9690, 4532,
        ];

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new();

        // Compress
        for value in &values {
            compressor.compress(TestRow { a: *value });
        }

        // Finalize the compression
        let bit_buffer = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bit_buffer);

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.a_col_vec);
    }

    #[test]
    fn test_macro_compress_sanity_i32() {
        /// Test i32 samples
        #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
        struct TestRow {
            a: i32,
        }

        let values: Vec<i32> = vec![
            1288174998,
            423886879,
            -339944271,
            575826316,
            570087820,
            550279880,
            1415943600,
            -328716683,
            1587107296,
            1195809421,
            -1177744317,
            -314426320,
            -869471864,
            903835789,
            1161983258,
            168819337,
            -1316403322,
            1600636560,
            1772467692,
            -1875642966,
            -1840341864,
            -1990432744,
            425148140,
            -373026362,
            335841843,
            -1857450600,
            592426279,
            2009184246,
            -94333397,
            1827733844,
            1684685676,
            -98864445,
        ];

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new();

        // Compress
        for value in &values {
            compressor.compress(TestRow { a: *value });
        }

        // Finalize the compression
        let bit_buffer = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bit_buffer);

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.a_col_vec);
    }

    #[test]
    fn test_macro_compress_sanity_i64() {
        /// Test i64 samples
        #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
        struct TestRow {
            a: i64,
        }

        let values: Vec<i64> = vec![
            -2334705132948491184,
            -1071135583981073678,
            -7292727408068708805,
            -8841393818463248252,
            -3787505109737062054,
            -5969166282397706208,
            8047182092084945423,
            8452246387379415812,
            7205984133897855049,
            -6469434605748900099,
            -3590884490458792149,
            317386071117022462,
            -6254539305955299603,
            5223403990112017603,
            5841061521166102242,
            9096340501540049846,
            -3180055806606307044,
            5582275661976568115,
            -5193370994696615992,
        ];

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new();

        // Compress
        for value in &values {
            compressor.compress(TestRow { a: *value });
        }

        // Finalize the compression
        let bit_buffer = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bit_buffer);

        // Assert that the decompressed data matches the original
        assert_eq!(values, decompressor.a_col_vec);
    }

    #[test]
    fn test_macro_compress_sanity_upgrade_types() {
        /// Test: delta and delta-delta in upgraded integer types (i8 -> i16 and so on)
        #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
        struct TestRow {
            a: i8,
            b: i16,
            c: i32,
            d: i64,
        }

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new();

        let a_col_vec = vec![-128, 127, -127, 126, 0, -1];
        let b_col_vec = vec![-32768, 32767, -32767, 32766, 0, -1];
        let c_col_vec = vec![-2147483648, 2147483647, -2147483647, 2147483646, 0, -1];
        let d_col_vec = vec![
            -9223372036854775808,
            9223372036854775807,
            -9223372036854775807,
            9223372036854775806,
            0,
            -1,
        ];

        for i in 0..a_col_vec.len() {
            // Compress
            compressor.compress(TestRow {
                a: a_col_vec[i],
                b: b_col_vec[i],
                c: c_col_vec[i],
                d: d_col_vec[i],
            });
        }

        // Finalize the compression
        let bit_buffer = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bit_buffer);

        // Assert that the decompressed data matches the original
        assert_eq!(a_col_vec, decompressor.a_col_vec);
        assert_eq!(b_col_vec, decompressor.b_col_vec);
        assert_eq!(c_col_vec, decompressor.c_col_vec);
        assert_eq!(c_col_vec, decompressor.c_col_vec);
    }
}
