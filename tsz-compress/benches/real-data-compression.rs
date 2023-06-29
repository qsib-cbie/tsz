use criterion::{criterion_group, criterion_main, Criterion};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::fs::File;
use async_compression::tokio::bufread::ZstdDecoder;

// TODO: Decompress keys and values separately.

const BATCH_SIZE: usize = 100_000;

async fn decompress_data() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "tests/data/0001-1686168000000000-9223372037005771051-index.bin";
    let file = File::open(file_path).await?;
    let mut buf_reader = BufReader::new(file);

    let mut compressed = Vec::new();
    buf_reader.read_to_end(&mut compressed).await?;

    let mut decoder = ZstdDecoder::new(compressed.as_slice());

    let mut batch = vec![0; BATCH_SIZE];
    while decoder.read_exact(&mut batch).await.is_ok() {}

    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("decompression_benchmark", |b| {
        b.iter(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                decompress_data().await.unwrap();
            });
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
