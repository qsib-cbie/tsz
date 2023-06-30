use criterion::{criterion_group, criterion_main, Criterion};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::fs::File;
use async_compression::tokio::bufread::ZstdDecoder;
use std::convert::TryInto;
use log::{error, info, trace};
use tokio::task::spawn_blocking;
use tsz_compress::prelude::{BitBuffer, BitBufferSlice, Decompressor};

const BATCH_SIZE: usize = 100_000;

//TODO: Debug and write structs for keys and values

async fn decompress_data(batch_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "tests/data/0001-1686168000000000-9223372037005771051-index.bin";
    let file = File::open(file_path).await?;
    let bytes = file.metadata().await?.len();
    let mut buf_reader = BufReader::new(file);
    let mut zfile = ZstdDecoder::new(buf_reader);
    let mut compressed = Vec::new();
    zfile.read_to_end(&mut compressed).await?;
    drop(zfile);

    let mut index = spawn_blocking(move || {
        // Expect the header to be "TKSS"
        let header = "_TKSS_0001_".as_bytes();
        if &compressed[0..header.len()] != header {
            return Err(format!("Invalid Header: {}", file_path.clone().into()));
        }
        let compressed = &compressed[header.len()..];

        // The next 8 bytes are the u64 byte length of the compressed data
        let keys_byte_size = u64::from_be_bytes(compressed[..8].try_into()?) as usize;
        if keys_byte_size + 16 > compressed.len() {
            error!(
                "Not enough bytes left: {} < {}",
                compressed.len(),
                (keys_byte_size + 16)
            );
            return Err(format!("Invalid Length: {}", file_path.clone().into()));
        }
        let compressed = &compressed[8..];

        // The next 8 bytes are the u64 bit length of the compressed data
        let keys_bit_size = u64::from_be_bytes(compressed[..8].try_into()?) as usize;
        if keys_bit_size as usize > compressed.len() * 8 {
            error!(
                "More bits than bytes: {} > {}",
                keys_bit_size,
                compressed.len() * 8
            );
            return Err(format!("Invalid Length: {}", file_path.clone().into()));
        }
        // Confirm there are keys_byte_size + 16 bytes left at minimum
        if keys_byte_size + 16 > compressed.len() {
            error!(
                "Not enough bytes left: {} < {}",
                compressed.len(),
                (keys_byte_size + 16)
            );
            return Err(format!("Invalid Length: {}", file_path.clone().into()));
        }
        let compressed = &compressed[8..];

        info!(
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
        if &compressed[0..header.len()] != header {
            return Err(format!("Invalid Header: {}", file_path.clone().into()));
        }
        let compressed = &compressed[header.len()..];

        // The next 8 bytes are the u64 byte length of the compressed data
        let values_byte_size = match u64::from_be_bytes(compressed[..8].try_into()) {
            Ok(size) => size as usize,
            Err(error) => return Err(format!("Error converting byte size: {}", error)),
        };
        if values_byte_size + 8 > compressed.len() {
            error!(
                "Not enough bytes left: {} < {}",
                compressed.len(),
                (values_byte_size + 8)
            );
            return Err(format!("Invalid Length: {}", file_path.clone().into()));
        }
        let compressed = &compressed[8..];

        let values_bit_size = u64::from_be_bytes(compressed[..8].try_into()?) as usize;
        let compressed = &compressed[8..];

        if values_byte_size != compressed.len() {
            error!(
                "Values byte size doesn't match: {} != {}",
                values_byte_size,
                compressed.len()
            );
            return Err(format!("Invalid Length: {}", file_path.clone().into()));
        }

        info!(
            "Read Values: {} bytes, {} bits",
            values_byte_size, values_bit_size
        );

        // The next `value_byte_size` bytes are the compressed data
        let compressed_values = &compressed[..values_byte_size as usize];
        let values_bits = BitBufferSlice::from_slice(compressed_values);
        let values_bits = values_bits.split_at(values_bit_size as usize).0;
        let mut values_decompressor = Decompressor::new(values_bits);
        
        info!(
            "Decompressing {} bytes of keys and {} bytes of values",
            keys_byte_size, values_byte_size
        );
        let start = std::time::Instant::now();
        let mut rows = keys_decompressor
            .decompress::<K>()
            .zip(values_decompressor.decompress::<V>())
            .into_iter()
            .map(|(k, v)| Ok((k?, v?)))
            .collect::<Result<Vec<_>, &'static str>>()?;
        info!(
            "Decompressed {} keys and values in {:?}",
            rows.len(),
            start.elapsed()
        );

        let CHUNK_SIZE: usize = batch_size;
        rows.chunks(CHUNK_SIZE)
            .into_iter()
            .enumerate()
            .for_each(|(i, chunk)| {
                let start = std::time::Instant::now();
                index.index.insert_sorted(chunk);
                trace!("Inserted {} in {:?}", i, start.elapsed());
            });

        Ok(index)
    })
    .await??;

    Ok(())
}

fn benchmark_decompress_data(c: &mut Criterion) {
    c.bench_function("decompress_data_batch_100000", |b| {
        b.iter(|| block_on(decompress_data_batch(BATCH_SIZE)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
