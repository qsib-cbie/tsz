use tsz_macro::*;

#[derive(Compressible, Decompressible)]
pub struct Row {
    pub ts: i64,
    pub val: i32,
}

fn main() {}
