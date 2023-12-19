use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tsz_compress::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    const FILE_NAME: &str = "data/0001-1686168000000000-9223372037005771051-index.tsz";
    #[derive(Default, DeltaEncodable, Decompressible, Compressible, Copy, Clone, Debug)]
    pub(crate) struct XyzValue {
        x: i32,
        y: i32,
        z: i32,
    }

    #[derive(
        Debug,
        Default,
        Clone,
        Copy,
        PartialEq,
        PartialOrd,
        Eq,
        Ord,
        Hash,
        DeltaEncodable,
        Compressible,
        Decompressible,
    )]
    pub struct PartitionedTimeKey {
        pub timestamp_us: i64,
    }

    let compressed = std::fs::read(FILE_NAME).unwrap();

    // Expect the header to be "TKSS"
    let header = "_TKSS_0001_".as_bytes();
    assert!(&compressed[0..header.len()] == header);
    let compressed = &compressed[header.len()..];

    // The next 8 bytes are the u64 byte length of the compressed data
    let keys_byte_size = u64::from_be_bytes(compressed[..8].try_into().unwrap()) as usize;
    let compressed = &compressed[8..];

    // The next 8 bytes are the u64 bit length of the compressed data
    let keys_bit_size = u64::from_be_bytes(compressed[..8].try_into().unwrap()) as usize;
    assert!((keys_bit_size as usize) < compressed.len() * 8);
    let compressed = &compressed[8..];

    println!(
        "Read Keys: {} bytes, {} bits",
        keys_byte_size, keys_bit_size
    );

    // The next `key_byte_size` bytes are the compressed data
    let compressed_keys = &compressed[..keys_byte_size as usize];
    let keys_bits = BitBufferSlice::from_slice(compressed_keys);
    let keys_bits = keys_bits.split_at(keys_bit_size as usize).0;
    let mut keys_decompressor = Decompressor::new(keys_bits);
    let compressed = &compressed[keys_byte_size as usize..];

    // Expect the header to be "TKSS"
    let header = "_TKSS_0001_".as_bytes();
    assert!(&compressed[0..header.len()] == header);
    let compressed = &compressed[header.len()..];

    // The next 8 bytes are the u64 byte length of the compressed data
    let values_byte_size = u64::from_be_bytes(compressed[..8].try_into().unwrap()) as usize;
    assert!((values_byte_size + 8) <= compressed.len());
    let compressed = &compressed[8..];

    let values_bit_size = u64::from_be_bytes(compressed[..8].try_into().unwrap()) as usize;
    let compressed = &compressed[8..];

    assert!(values_byte_size == compressed.len());
    println!(
        "Read Values: {} bytes, {} bits",
        values_byte_size, values_bit_size
    );

    // The next `value_byte_size` bytes are the compressed data
    let compressed_values = &compressed[..values_byte_size as usize];
    let values_bits = BitBufferSlice::from_slice(compressed_values);
    let values_bits = values_bits.split_at(values_bit_size as usize).0;
    let mut values_decompressor = Decompressor::new(values_bits);

    println!(
        "Decompressing {} bytes of keys and {} bytes of values",
        keys_byte_size, values_byte_size
    );

    let decompress_iter = keys_decompressor
        .decompress::<PartitionedTimeKey>()
        .zip(values_decompressor.decompress::<XyzValue>())
        .into_iter()
        .map(|(k, v)| (k.unwrap(), v.unwrap()));

    let rows = decompress_iter.clone().collect::<Vec<_>>();

    let mut infinite_iter = std::iter::repeat(decompress_iter).flatten();
    c.bench_function("decompress xyz 10k", |b| {
        b.iter(|| {
            black_box(for _ in 0..10_000 {
                let foo = infinite_iter.next().unwrap();
                black_box(foo);
            })
        })
    });
    c.bench_function("decompress xyz 100k", |b| {
        b.iter(|| {
            black_box(for _ in 0..100_000 {
                let foo = infinite_iter.next().unwrap();
                black_box(foo);
            })
        })
    });
    c.bench_function("decompress xyz 1M", |b| {
        b.iter(|| {
            black_box(for _ in 0..1_000_000 {
                let foo = infinite_iter.next().unwrap();
                black_box(foo);
            })
        })
    });

    let mut infinite_iter = std::iter::repeat(rows.iter()).flatten();
    let mut keys_compressor = Compressor::<PartitionedTimeKey>::new(1_000_000);
    let mut values_compressor = Compressor::<XyzValue>::new(1_000_000);
    c.bench_function("compress xyz 10k", |b| {
        b.iter(|| {
            black_box(for _ in 0..10_000 {
                let row = infinite_iter.next().unwrap();
                keys_compressor.compress(row.0);
                values_compressor.compress(row.1);
            })
        })
    });
    let mut keys_compressor = Compressor::<PartitionedTimeKey>::new(1_000_000);
    let mut values_compressor = Compressor::<XyzValue>::new(1_000_000);
    c.bench_function("compress xyz 100k", |b| {
        b.iter(|| {
            black_box(for _ in 0..100_000 {
                let row = infinite_iter.next().unwrap();
                keys_compressor.compress(row.0);
                values_compressor.compress(row.1);
            })
        })
    });
    let mut keys_compressor = Compressor::<PartitionedTimeKey>::new(1_000_000);
    let mut values_compressor = Compressor::<XyzValue>::new(1_000_000);
    c.bench_function("compress xyz 1M", |b| {
        b.iter(|| {
            black_box(for _ in 0..1_000_000 {
                let row = infinite_iter.next().unwrap();
                keys_compressor.compress(row.0);
                values_compressor.compress(row.1);
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
