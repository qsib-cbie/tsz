pub mod consts;
pub mod decode;
pub mod encode;
pub mod halfvec;
pub mod queue;
pub use decode::*;
pub use encode::*;
pub use queue::*;

#[derive(Debug)]
pub enum CodingError {
    /// There is not enough data to decode a single value.
    Empty,
    /// There were not enough bits to finish decoding an expected value.
    NotEnoughBits,
    /// There were bits that indicated an invalid value.
    InvalidBits,
    /// The first column tag was invalid.
    InvalidInitialColumnTag,
    /// A non-first column tag was invalid.
    InvalidColumnTag,
    /// The number of rows decoded did not match the expected number of rows.
    ColumnLengthMismatch(ColumnLengths),
    /// The number of rows to decode cannot be valid
    InvalidRowCount(usize),
}

#[derive(Debug)]
pub struct ColumnLengths {
    pub expected_rows: usize,
    pub column_lengths: ::alloc::vec::Vec<usize>,
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
    /// Returns true if no bits have been compressed.
    ///
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    ///
    /// Return an estimate of bits per column value as the number of
    /// compressed bits / count of column values compressed / columns per row.
    ///
    fn bit_rate(&self) -> usize;

    ///
    /// The number of rows that have been compressed.
    /// This is an exact answer for rows consumed including rows that may not have been emitted.
    ///
    fn row_count(&self) -> usize;

    ///
    /// Finish compression, consuming the compressor and returning the compressed bytes.
    /// This may leave intermediate buffers in a cleared state.
    ///
    fn finish(&mut self) -> ::alloc::vec::Vec<u8>;
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
    /// rust
    /// fn col_a(&self) -> &[i8];
    /// fn col_b(&self) -> &[i32];
    ///
    ///
    /// # Returns
    /// A vector of rows, where each row is a struct with the same fields as the original.
    ///
    fn rows(&self) -> ::alloc::vec::Vec<Self::T>;
}
