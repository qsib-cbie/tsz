use tsz_compress::prelude::*;

extern crate alloc;

#[derive(Debug, Copy, Clone, CompressV2, DecompressV2)]
pub struct TestRow {
    t: i64,
    a: i8,
    b: i16,
    c: i32,
}

fn main() {
    let row = TestRow {
        t: 0,
        a: 1,
        b: 2,
        c: 3,
    };

    // Initialize the compressor
    let mut compressor = compress::TestRowCompressorImpl::new(128);

    // Compress row
    for _ in 0..10 {
        compressor.compress(row);
    }

    // Finalize the compression
    let bytes = compressor.finish();

    // Initialize the decompressor
    let mut decompressor = decompress::TestRowDecompressorImpl::new();

    // Decompress the bytes into columnar buffers in the decompressor
    decompressor.decompress(&bytes).unwrap();

    // Assert that the decompressed data matches the original
    assert_eq!(decompressor.col_a(), vec![row.a; 10]);

    // Rotate the decompressed data into rows
    decompressor.rows().iter().for_each(|x| {
        assert_eq!(row.t, x.t);
        assert_eq!(row.a, x.a);
        assert_eq!(row.b, x.b);
        assert_eq!(row.c, x.c);
    });
}
