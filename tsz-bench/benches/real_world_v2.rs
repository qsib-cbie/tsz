extern crate alloc;

use std::mem::size_of;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use polars::prelude::*;
use tsz_compress::prelude::*;

#[derive(Copy, Clone, CompressV2, DecompressV2)]
#[repr(packed)]
pub struct TxyzValue {
    t: i64,
    x: i32,
    y: i32,
    z: i32,
}

fn criterion_benchmark(c: &mut Criterion) {
    const FILE_NAME: &str = "data/TsXyzRows.parquet";
    let file_reader = std::fs::File::open(FILE_NAME).unwrap();
    let mut df = ParquetReader::new(file_reader).finish().unwrap();
    df.as_single_chunk_par();
    let ts = df
        .column("t")
        .unwrap()
        .datetime()
        .unwrap()
        .cont_slice()
        .unwrap();
    let x = df.column("x").unwrap().i32().unwrap().cont_slice().unwrap();
    let y = df.column("y").unwrap().i32().unwrap().cont_slice().unwrap();
    let z = df.column("z").unwrap().i32().unwrap().cont_slice().unwrap();

    let mut compressor = compress::TxyzValueCompressorImpl::new(1024);
    for i in 0..ts.len() {
        compressor.compress(TxyzValue {
            t: ts[i],
            x: x[i],
            y: y[i],
            z: z[i],
        });
    }
    let bytes = compressor.finish();

    let mut idxs = std::iter::repeat((0..ts.len()).clone()).flatten();

    c.bench_function("compress txyz 10k", |b| {
        b.iter(|| {
            for _ in 0..10_000 {
                let i = unsafe { idxs.next().unwrap_unchecked() };
                compressor.compress(TxyzValue {
                    t: ts[i],
                    x: x[i],
                    y: y[i],
                    z: z[i],
                });
            }
            let bytes = compressor.finish();
            black_box(bytes);
        });
    });

    c.bench_function("compress txyz 100k", |b| {
        b.iter(|| {
            for _ in 0..100_000 {
                let i = unsafe { idxs.next().unwrap_unchecked() };
                compressor.compress(TxyzValue {
                    t: ts[i],
                    x: x[i],
                    y: y[i],
                    z: z[i],
                });
            }
            let bytes = compressor.finish();
            black_box(bytes);
        });
    });

    println!("compressing {} rows", ts.len());
    c.bench_function("compress txyz", |b| {
        b.iter(|| {
            let mut compressor = compress::TxyzValueCompressorImpl::new(ts.len());
            for i in 0..ts.len() {
                compressor.compress(TxyzValue {
                    t: ts[i],
                    x: x[i],
                    y: y[i],
                    z: z[i],
                });
            }
            let first_phase_bytes = compressor.finish();
            black_box(first_phase_bytes);
        });
    });

    println!("decompressing {} rows", ts.len());
    let mut decompressor = decompress::TxyzValueDecompressorImpl::new();
    c.bench_function("decompress txyz", |b| {
        b.iter(|| {
            decompressor.decompress(&bytes).unwrap();
            assert!(decompressor.col_t().len() == decompressor.col_z().len());
        });
    });

    c.bench_function("two-phase compress txyz", |b| {
        b.iter(|| {
            let mut compressor = compress::TxyzValueCompressorImpl::new(ts.len());
            for i in 0..ts.len() {
                compressor.compress(TxyzValue {
                    t: ts[i],
                    x: x[i],
                    y: y[i],
                    z: z[i],
                });
            }
            let first_phase_bytes = compressor.finish();
            let second_phase_bytes = compress_prepend_size(&first_phase_bytes);
            black_box(second_phase_bytes);
        });
    });

    let mut compressor = compress::TxyzValueCompressorImpl::new(ts.len());
    for i in 0..ts.len() {
        compressor.compress(TxyzValue {
            t: ts[i],
            x: x[i],
            y: y[i],
            z: z[i],
        });
    }
    let first_phase_bytes = compressor.finish();
    let second_phase_bytes = compress_prepend_size(&first_phase_bytes);
    let original_size = size_of::<TxyzValue>() * ts.len();
    println!("Original size: {}", original_size);
    println!("TSZ Phase 1 size: {}", first_phase_bytes.len());
    println!("LZ4 Phase 2 size: {}", second_phase_bytes.len());
    c.bench_function("two-phase decompress txyz", |b| {
        b.iter(|| {
            let first_phase_bytes = decompress_size_prepended(&second_phase_bytes).unwrap();
            let mut decompressor = decompress::TxyzValueDecompressorImpl::new();
            decompressor.decompress(&first_phase_bytes).unwrap();
            assert!(decompressor.col_t().len() == decompressor.col_z().len());
            // rotate costs CPU, columnar is faster
            let rows = decompressor.rows();
            black_box(rows);
        });
    });

    // Concatenate all columns into a single column, the LZ4 compress for comparison
    let ts = unsafe {
        core::slice::from_raw_parts(ts.as_ptr() as *const u8, ts.len() * size_of::<i64>())
    };
    let x =
        unsafe { core::slice::from_raw_parts(x.as_ptr() as *const u8, x.len() * size_of::<i32>()) };
    let y =
        unsafe { core::slice::from_raw_parts(y.as_ptr() as *const u8, y.len() * size_of::<i32>()) };
    let z =
        unsafe { core::slice::from_raw_parts(z.as_ptr() as *const u8, z.len() * size_of::<i32>()) };
    let mut col_major_bytes = Vec::with_capacity(ts.len() + x.len() + y.len() + z.len());
    col_major_bytes.extend_from_slice(ts);
    col_major_bytes.extend_from_slice(x);
    col_major_bytes.extend_from_slice(y);
    col_major_bytes.extend_from_slice(z);
    let lz4_bytes = compress_prepend_size(&col_major_bytes);
    println!("Only LZ4 on column-major bytes size: {}", lz4_bytes.len());
    c.bench_function("only LZ4-only compress txyz", |b| {
        b.iter(|| {
            let lz4_bytes = compress_prepend_size(&col_major_bytes);
            black_box(lz4_bytes);
        });
    });
}

criterion_group!(name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(10));
    targets = criterion_benchmark);
criterion_main!(benches);
