#![allow(unused)]
use tsz_compress::prelude::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_macro_compress_test_macro_compress_sanity1() {
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
    fn test_macro_compress_sanity3() {
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
    fn test_macro_compress_sanity4() {
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
    fn test_macro_compress_sanity5() {
        #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
        struct TestRow {
            a: i8,
            b: i16,
            c: i32,
            d: i64,
        }
        let row = TestRow {
            a: 99 as i8,
            b: 999 as i16,
            c: 999 as i32,
            d: 999 as i64,
        };

        // Initialize the compressor
        let mut compressor = TestRowCompressorImpl::new();

        // Compress
        for _ in 0..100 {
            compressor.compress(row);
        }

        // Finalize the compression
        let bit_buffer = compressor.finish();

        // Initialize the decompressor
        let mut decompressor = TestRowDecompressorImpl::new();

        // Decompress the bit buffer
        decompressor.decompress(&bit_buffer);

        // Assert that the decompressed data matches the original

        assert_eq!(decompressor.a_col_vec, vec![row.a; 100]);
        assert_eq!(decompressor.b_col_vec, vec![row.b; 100]);
    }

    #[test]
    fn fuzz() {
        use rand::Rng;
        #[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
        struct TestRow {
            a: i8,
            b: i16,
            c: i32,
            d: i64,
        }

        let mut rng = rand::thread_rng();

        for i in 0..100 {
            // Initialize the compressor
            let mut compressor = TestRowCompressorImpl::new();

            let mut vec_a: Vec<i8> = Vec::new();
            let mut vec_b: Vec<i16> = Vec::new();
            let mut vec_c: Vec<i32> = Vec::new();
            let mut vec_d: Vec<i64> = Vec::new();

            // Compress
            for _ in 0..rng.gen_range(0..1000) {
                let row = TestRow {
                    a: rng.gen_range(i8::MIN..=i8::MAX) as i8,
                    b: rng.gen_range(i16::MIN..=i16::MAX) as i16,
                    c: rng.gen_range(i32::MIN..=i32::MAX) as i32,
                    d: rng.gen_range(i64::MIN..=i64::MAX) as i64,
                };

                vec_a.push(row.a);
                vec_b.push(row.b);
                vec_c.push(row.c);
                vec_d.push(row.d);
                compressor.compress(row);
            }

            // Finalize the compression
            let bit_buffer = compressor.finish();

            // Initialize the decompressor
            let mut decompressor = TestRowDecompressorImpl::new();

            // Decompress the bit buffer
            decompressor.decompress(&bit_buffer);

            // Assert that the decompressed data matches the original
            assert_eq!(decompressor.a_col_vec, vec_a);
            assert_eq!(decompressor.b_col_vec, vec_b);
            assert_eq!(decompressor.c_col_vec, vec_c);
            assert_eq!(decompressor.d_col_vec, vec_d);
        }
    }
}
