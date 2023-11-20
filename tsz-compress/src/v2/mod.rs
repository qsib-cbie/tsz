pub mod decode;
pub mod encode;
pub mod halfvec;
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
    /// `prealloc_rows` is a hint for initial capacity for internal buffers.
    /// It is not a hard limit, just a guess at the number of rows that will be compressed.
    ///
    fn new(prealloc_rows: usize) -> Self;

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
    fn finish(self) -> ::alloc::vec::Vec<u8>;
}

///
/// High-level interface for decompression.
///
pub trait TszDecompressV2 {
    type T: Copy;

    ///
    /// Initializes a new instance of the Decompressor.
    ///
    fn new() -> Self;

    ///
    /// Decompress all of the rows into columnar buffers.
    ///
    /// This operation will not overwrite existing data in the buffers.
    ///
    /// # Arguments
    /// * `bits` - The compressed data from a TszCmopressV2 instance.
    ///
    fn decompress(&mut self, bits: &[u8]) -> Result<(), CodingError>;

    ///
    /// Rotate the decompressed values into a vector of rows.
    ///
    /// If columnar data is desired, each implementation derived
    /// via macro will include an accessor for each column vector by name.
    ///
    /// For example, if derived for a struct with fields `a: i8` and `b: i32`
    /// then the following accessors will be generated:
    ///
    /// ```rust
    /// fn col_a(&self) -> &[i8];
    /// fn col_b(&self) -> &[i32];
    /// ```
    ///
    /// # Returns
    /// A vector of rows, where each row is a struct with the same fields as the original.
    ///
    fn rows(&self) -> ::alloc::vec::Vec<Self::T>;
}
