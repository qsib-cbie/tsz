use criterion::{criterion_group, criterion_main, Criterion};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::fs::File;
use tokio::runtime::Runtime;
use async_compression::tokio::bufread::ZstdDecoder;
use core::convert::TryInto;
use log::{error, info};
use tokio::task::spawn_blocking;
use tsz_compress::prelude::*;

const BATCH_SIZE: usize = 100_000;

#[derive(Copy, Clone, DeltaEncodable, Compressible, Decompressible)]
struct MyKeyStruct {
    timestamp_us: i64,
    partition: i64,
}

#[derive(Copy, Clone, DeltaEncodable, Compressible, Decompressible)]
struct MyValueStruct {
    value: i64,
}

async fn decompress_data(batch_size: usize) -> Result<(), String> {
    let file_path: &str = "tests/data/0001-1686168000000000-9223372037005771051-index.bin";
    let file = File::open(file_path).await.map_err(|err| err.to_string())?;
    let bytes = file.metadata().await.map_err(|err| err.to_string())?.len();
    let buf_reader = BufReader::new(file);
    let mut zfile = ZstdDecoder::new(buf_reader);
    let mut compressed = Vec::new();
    zfile.read_to_end(&mut compressed).await.map_err(|err| err.to_string())?;
    drop(zfile);

    let result = spawn_blocking(move || {
        // Expect the header to be "TKSS"
        let header = "_TKSS_0001_".as_bytes();
        if &compressed[0..header.len()] != header {
            return Err(format!("Invalid Header: {}", <&str as Into<String>>::into(file_path.clone())));
        }
        let compressed = &compressed[header.len()..];

        // The next 8 bytes are the u64 byte length of the compressed data
        let keys_byte_size = u64::from_be_bytes(compressed[..8].try_into().map_err(|_| "Invalid length".to_string())?);
        if keys_byte_size + 16 > compressed.len() as u64 {
            error!(
                "Not enough bytes left: {} < {}",
                compressed.len(),
                keys_byte_size + 16
            );
            return Err(format!("Invalid Length: {}", <&str as Into<String>>::into(file_path.clone())));
        }
        let compressed = &compressed[8..];

        // The next 8 bytes are the u64 bit length of the compressed data
        let keys_bit_size = u64::from_be_bytes(compressed[..8].try_into().map_err(|_| "Invalid length".to_string())?);
        if keys_bit_size as usize > compressed.len() * 8 {
            error!(
                "More bits than bytes: {} > {}",
                keys_bit_size,
                compressed.len() * 8
            );
            return Err(format!("Invalid Length: {}", <&str as Into<String>>::into(file_path.clone())));
        }
        // Confirm there are keys_byte_size + 16 bytes left at minimum
        if keys_byte_size + 16 > compressed.len() as u64 {
            error!(
                "Not enough bytes left: {} < {}",
                compressed.len(),
                keys_byte_size + 16
            );
            return Err(format!("Invalid Length: {}", <&str as Into<String>>::into(file_path.clone())));
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
            return Err(format!("Invalid Header: {}", <&str as Into<String>>::into(file_path.clone())));
        }
        let compressed = &compressed[header.len()..];

        // The next 8 bytes are the u64 byte length of the compressed data
        let values_byte_size = u64::from_be_bytes(compressed[..8].try_into().map_err(|_| "Invalid length".to_string())?);
        if values_byte_size + 8 > compressed.len() as u64 {
            error!(
                "Not enough bytes left: {} < {}",
                compressed.len(),
                values_byte_size + 8
            );
            return Err(format!("Invalid Length: {}", <&str as Into<String>>::into(file_path.clone())));
        }
        let compressed = &compressed[8..];

        let values_bit_size = u64::from_be_bytes(compressed[..8].try_into().map_err(|_| "Invalid length".to_string())?);
        let compressed = &compressed[8..];

        if values_byte_size != compressed.len() as u64 {
            error!(
                "Values byte size doesn't match: {} != {}",
                values_byte_size,
                compressed.len()
            );
            return Err(format!("Invalid Length: {}", <&str as Into<String>>::into(file_path.clone())));
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

        let chunk_size = batch_size;
        let mut decompressed_keys = Vec::new();
        let mut decompressed_values = Vec::new();
        let start = std::time::Instant::now();

        loop {
            let chunk_keys = keys_decompressor.decompress::<MyKeyStruct>();
            let chunk_values = values_decompressor.decompress::<MyValueStruct>();
            decompressed_keys.extend(chunk_keys);
            decompressed_values.extend(chunk_values);
            if decompressed_keys.len() >= chunk_size && decompressed_values.len() >= chunk_size {
                break;
            }
        }

        info!(
            "Decompressed {} keys and {} values in {:?}",
            decompressed_keys.len(),
            decompressed_values.len(),
            start.elapsed()
        );

        Ok(())
    })
    .await;

    result.unwrap_or_else(|err| {
        error!("Decompression error: {}", err);
        Err(err.to_string())
    })
}

fn benchmark_decompress_data(c: &mut Criterion) {
    c.bench_function("decompress_data_batch_100000", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(decompress_data(BATCH_SIZE))
        })
    });
}

criterion_group!(benches, benchmark_decompress_data);
criterion_main!(benches);
