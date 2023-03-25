use tsz_compress::prelude::*;

#[derive(Copy, Clone, DeltaEncodable, Compressible, Decompressible)]
pub struct Row {
    pub ts: i64,
    pub val: i32,
}

fn main() {
    let mut c = Compressor::new();
    for i in 0..10 {
        let row = Row {
            ts: i,
            val: i as i32,
        };
        c.compress(row);
    }
}
