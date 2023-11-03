use crate::prelude::*;

pub mod decode;
pub mod encode;
pub mod queue;
pub mod tests;
pub use decode::*;
pub use encode::*;
pub use queue::*;

#[derive(Debug)]
pub enum CodingError {
    NotEnoughBits,
    InvalidBits,
}

///
/// High-level interface for compression.
///
pub trait TszCompressV2 {
    /// The type of the row to compress.
    type T: Copy;

    ///
    /// Initializes a new instance of the Compressor.
    ///
    fn new() -> Self;

    ///
    /// Lazily compress a row.
    ///
    fn compress(&mut self, row: Self::T);

    ///
    /// The number of bits that have been compressed.
    /// This is an estimate, as the last few samples may have been emitted are estimated.
    ///
    fn len(&self) -> usize;

    ///
    /// Return an estimate of bits per column value as the number of
    /// compressed bits / count of column values compressed / columns per row.
    ///
    fn bit_rate(&self) -> usize;

    ///
    /// Finish compression and return the compressed data.
    ///
    fn finish(self) -> BitBuffer;
}

///
/// High-level interface for decompression.
///
pub trait TszDecompressV2 {
    ///
    /// Initializes a new instance of the Decompressor.
    ///
    fn new() -> Self;

    ///
    /// Decompress a row.
    ///
    fn decompress(&mut self, bits: &BitBufferSlice);
}
