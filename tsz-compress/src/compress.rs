use bitvec::prelude::*;
pub use tsz_macro::*;

///
/// Use `u8` as the storage type for a `BitVec` to represent
/// bits for easier conversion to and from bytes.
///
pub type BitBuffer = BitVec<u32, Lsb0>;
pub type BitBufferSlice = BitSlice<u32, Lsb0>;

///
/// A `Compressor` instance holds the state of the compression process.
///
/// Implement the `Compress` trait for your data type.
/// Create a `Compressor` instance.
/// Call `compress` for each row of data, handing off your `Compressor` instance.
/// Call `finish` to get the compressed data.
///
#[derive(Default)]
pub struct Compressor<T: Compress> {
    pub output: BitBuffer,
    pub row_n: Option<T>,
    pub row_n1: Option<T>,
}

///
/// A `Decompressor` instance holds the state of the decompression process.
///
/// Implement the `Decompress` trait for your data type.
/// Create a `Decompressor` instance.
/// Call `decompress` to iterate over the decompressed rows, handing off your `Decompressor` instance.
///
pub struct Decompressor<'de> {
    pub input: &'de BitBufferSlice,
}

///
/// Implement this trait for your data type to be able to compress it.
///
/// Derive the `Compressible` trait to get a default implementation.
/// Derive the `DeltaEncodable` trait to get a default implementation for a `RowDelta`, which will be used as the `Delta` type.
///
pub trait Compress: Copy + Sized {
    /// Full and Delta may differ in signedness or storage
    /// Full is the representation of the value as a whole
    /// Delta is the representation of the difference between the value and the previous value or the difference between differences
    type Full: IntoCompressBits;
    type Delta: IntoCompressBits;

    fn into_full(self) -> Self::Full;
    fn into_delta(self, prev_row: &Self) -> Self::Delta;
    fn into_deltadelta(self, prev_prev_row: &Self, prev_row: &Self) -> Self::Delta;
}

///
/// Implement this trait for your data type to be able to decompress it.
///
/// Derive the `Decompressible` trait to get a default implementation.
/// Derive the `DeltaEncodable` trait to get a default implementation for a `RowDelta`, which will be used as the `Delta` type.
///
pub trait Decompress: Copy + Sized {
    type Full: FromCompressBits;
    type Delta: FromCompressBits;

    fn from_full(bits: &BitBufferSlice) -> Result<(Self, &BitBufferSlice), &'static str>;
    fn from_delta<'a>(
        bits: &'a BitBufferSlice,
        prev_row: &Self,
    ) -> Result<(Self, &'a BitBufferSlice), &'static str>;
    fn from_deltadelta<'a>(
        bits: &'a BitBufferSlice,
        prev_row: &Self,
        prev_prev_row: &Self,
    ) -> Result<(Self, &'a BitBufferSlice), &'static str>;
}

pub trait IntoCompressBits: Sized {
    fn into_bits(self, out: &mut BitBuffer);
}

pub trait FromCompressBits: Sized {
    fn from_bits(input: &BitBufferSlice) -> Result<(Self, &BitBufferSlice), &'static str>;
}

impl<T: Compress> Compressor<T> {
    ///
    /// Create a new `Compressor` instance.
    ///
    pub fn new() -> Self {
        Self {
            output: BitBuffer::new(),
            row_n: None,
            row_n1: None,
        }
    }

    ///
    /// Compress a row of data.
    ///
    /// Handles the first two rows differently to subsequent rows.
    ///
    pub fn compress(&mut self, row: T) {
        let Some(row_n) = self.row_n.take() else {
            self.row_n = Some(row);

            // The first row is represented as the each value
            // Encoded to unsigned VLQ
            let representation = row.into_full();
            representation.into_bits(&mut self.output);

            return;
        };

        let Some(row_n1) = self.row_n1.take() else {
            self.row_n = Some(row_n);
            self.row_n1 = Some(row);

            // The second row is represented as the difference between the first row and the second row
            // Encoded to Gorilla Delta-Delta Encoding
            let representation = row.into_delta(&row_n);
            representation.into_bits(&mut self.output);

            return;
        };

        // Each subsequent row is represented as the deltadelta = (row - row_n1) - (row_n1 - row_n)
        // Encoded to Gorilla Delta-Delta Encoding
        let representation = row.into_deltadelta(&row_n, &row_n1);
        representation.into_bits(&mut self.output);

        // Move the rows along

        self.row_n = Some(row_n1);
        self.row_n1 = Some(row);
    }

    ///
    /// Check the number of bytes in the compressed data.
    ///
    pub fn len(&self) -> usize {
        // Round up to the nearest byte
        let num_bits = self.output.len();
        (num_bits + 7) / 8
    }

    ///
    /// Check if the compressed data is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.output.is_empty()
    }

    ///
    /// Take the compressed data, suitable for constructing a `Decompressor` instance.
    ///
    pub fn finish(self) -> BitBuffer {
        self.output
    }
}

impl<'de> Decompressor<'de> {
    ///
    /// Create a new `Decompressor` instance that will decompress bits from the given data.
    ///
    pub fn new(input: &'de BitBufferSlice) -> Self {
        Self { input }
    }

    ///
    /// Decompress the data into an iterator over the rows.
    ///
    pub fn decompress<T: Decompress>(&mut self) -> DecompressIter<'_, T> {
        DecompressIter {
            input: self.input,
            finished: false,
            first_row: None,
            second_row: None,
            t_prev_prev: None,
            t_prev: None,
        }
    }
}

///
/// An iterator over the decompressed data.
///
#[derive(Clone)]
pub struct DecompressIter<'a, T> {
    input: &'a BitBufferSlice,
    finished: bool,
    first_row: Option<T>,
    second_row: Option<T>,
    t_prev_prev: Option<T>,
    t_prev: Option<T>,
}

///
/// This iterator is returned on a decompress call to a Decompressor instance.
///
impl<'a, T> Iterator for DecompressIter<'a, T>
where
    T: Decompress,
{
    type Item = Result<T, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let Some(first_row) = self.first_row.as_ref() else {
            // Base case, no data
            if self.input.is_empty() {
                self.finished = true;
                return None;
            }

            // The first row is represented as the each value
            // Encoded to unsigned VLQ
            let (first_row, trailing) = match T::from_full(self.input) {
                Ok(x) => x,
                Err(e) => {
                    self.finished = true;
                    return Some(Err(e));
                }
            };
            self.input = trailing;
            self.first_row = Some(first_row);
            return Some(Ok(first_row));
        };

        let Some(_second_row) = self.second_row.as_ref() else {
            // Base case, one row
            if self.input.is_empty() {
                self.finished = true;
                return None;
            }

            // The second row is represented as the difference between the first row and the second row
            // Encoded to Gorilla Delta-Delta Encoding
            let (second_row, trailing) = match T::from_delta(self.input, first_row) {
                Ok(x) => x,
                Err(e) => {
                    self.finished = true;
                    return Some(Err(e));
                }
            };
            self.input = trailing;
            self.second_row = Some(second_row);
            self.t_prev_prev = Some(*first_row);
            self.t_prev = Some(second_row);
            return Some(Ok(second_row));
        };

        // Each subsequent row is represented as the deltadelta = (row - row_n1) - (row_n1 - row_n)
        // Encoded to Gorilla Delta-Delta Encoding

        if self.input.is_empty() {
            self.finished = true;
            return None;
        }

        let t_prev = self.t_prev.take().unwrap();
        let t_prev_prev = self.t_prev_prev.take().unwrap();

        // Read the deltadelta, D, and reconstruct the row, t
        let (row, trailing) = match T::from_deltadelta(self.input, &t_prev, &t_prev_prev) {
            Ok(x) => x,
            Err(e) => {
                self.finished = true;
                return Some(Err(e));
            }
        };
        self.input = trailing;
        self.t_prev_prev = Some(t_prev);
        self.t_prev = Some(row);

        Some(Ok(row))
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Add;
    use core::ops::Sub;
    use rand::Rng;

    use crate::delta::*;
    use crate::svlq::*;
    use crate::uvlq::*;

    use super::*;

    #[test]
    fn test_compress() {
        // A row of data with a timestamp
        #[derive(Debug, Copy, Clone)]
        struct TestRow {
            ts: u64,
            v8: u8,
            v16: u16,
            v32: u32,
            v64: u64,
            vi8: i8,
            vi16: i16,
            vi32: i32,
            vi64: i64,
        }

        // A row to capture the difference between two rows
        #[derive(Debug, Copy, Clone)]
        struct TestRowDelta {
            ts: i128,
            v8: i16,
            v16: i32,
            v32: i64,
            v64: i128,
            vi8: i16,
            vi16: i32,
            vi32: i64,
            vi64: i128,
        }

        // How to take the difference between two rows
        impl Sub for TestRow {
            type Output = TestRowDelta;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output {
                    ts: self.ts as i128 - rhs.ts as i128,
                    v8: self.v8 as i16 - rhs.v8 as i16,
                    v16: self.v16 as i32 - rhs.v16 as i32,
                    v32: self.v32 as i64 - rhs.v32 as i64,
                    v64: self.v64 as i128 - rhs.v64 as i128,
                    vi8: self.vi8 as i16 - rhs.vi8 as i16,
                    vi16: self.vi16 as i32 - rhs.vi16 as i32,
                    vi32: self.vi32 as i64 - rhs.vi32 as i64,
                    vi64: self.vi64 as i128 - rhs.vi64 as i128,
                }
            }
        }

        // How to add a delta to a row to get another row
        impl Add<TestRowDelta> for TestRow {
            type Output = TestRow;

            fn add(self, rhs: TestRowDelta) -> Self::Output {
                Self::Output {
                    ts: (self.ts as i128 + rhs.ts) as u64,
                    v8: (self.v8 as i16 + rhs.v8) as u8,
                    v16: (self.v16 as i32 + rhs.v16) as u16,
                    v32: (self.v32 as i64 + rhs.v32) as u32,
                    v64: (self.v64 as i128 + rhs.v64) as u64,
                    vi8: (self.vi8 as i16 + rhs.vi8) as i8,
                    vi16: (self.vi16 as i32 + rhs.vi16) as i16,
                    vi32: (self.vi32 as i64 + rhs.vi32) as i32,
                    vi64: (self.vi64 as i128 + rhs.vi64) as i64,
                }
            }
        }

        // How to take the difference between two deltas
        impl Sub for TestRowDelta {
            type Output = TestRowDelta;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output {
                    ts: self.ts - rhs.ts,
                    v8: self.v8 - rhs.v8,
                    v16: self.v16 - rhs.v16,
                    v32: self.v32 - rhs.v32,
                    v64: self.v64 - rhs.v64,
                    vi8: self.vi8 - rhs.vi8,
                    vi16: self.vi16 - rhs.vi16,
                    vi32: self.vi32 - rhs.vi32,
                    vi64: self.vi64 - rhs.vi64,
                }
            }
        }

        // How to bit pack a row
        impl IntoCompressBits for TestRow {
            fn into_bits(self, out: &mut BitBuffer) {
                out.extend(Uvlq::from(self.ts).bits);
                out.extend(Uvlq::from(self.v8).bits);
                out.extend(Uvlq::from(self.v16).bits);
                out.extend(Uvlq::from(self.v32).bits);
                out.extend(Uvlq::from(self.v64).bits);
                out.extend(Svlq::from(self.vi8).bits);
                out.extend(Svlq::from(self.vi16).bits);
                out.extend(Svlq::from(self.vi32).bits);
                out.extend(Svlq::from(self.vi64).bits);
            }
        }

        // How to bit pack a delta
        impl IntoCompressBits for TestRowDelta {
            fn into_bits(self, out: &mut BitBuffer) {
                if self.ts < i64::MIN as i128 && self.ts > i64::MAX as i128 {
                    unimplemented!()
                }
                encode_delta_i64(self.ts as i64, out);

                encode_delta_i16(self.v8, out);
                encode_delta_i32(self.v16, out);
                encode_delta_i64(self.v32, out);

                if self.v64 < i128::MIN as i128 && self.v64 > i128::MAX as i128 {
                    unimplemented!()
                }
                encode_delta_i64(self.v64 as i64, out);

                encode_delta_i16(self.vi8, out);
                encode_delta_i32(self.vi16, out);
                encode_delta_i64(self.vi32, out);

                if self.vi64 < i64::MIN as i128 && self.vi64 > i64::MAX as i128 {
                    unimplemented!()
                }
                encode_delta_i64(self.vi64 as i64, out);
            }
        }

        // How to unmarshal a row from a bit slice
        impl FromCompressBits for TestRow {
            fn from_bits(input: &BitBufferSlice) -> Result<(Self, &BitBufferSlice), &'static str> {
                let (ts, ts_bits) = <(u64, usize)>::try_from(UvlqRef(input))?;
                let input = &input[ts_bits..];
                let (v8, v8_bits) = <(u8, usize)>::try_from(UvlqRef(input))?;
                let input = &input[v8_bits..];
                let (v16, v16_bits) = <(u16, usize)>::try_from(UvlqRef(input))?;
                let input = &input[v16_bits..];
                let (v32, v32_bits) = <(u32, usize)>::try_from(UvlqRef(input))?;
                let input = &input[v32_bits..];
                let (v64, v64_bits) = <(u64, usize)>::try_from(UvlqRef(input))?;
                let input = &input[v64_bits..];

                let (vi8, vi8_bits) = <(i8, usize)>::try_from(SvlqRef(input))?;
                let input = &input[vi8_bits..];
                let (vi16, vi16_bits) = <(i16, usize)>::try_from(SvlqRef(input))?;
                let input = &input[vi16_bits..];
                let (vi32, vi32_bits) = <(i32, usize)>::try_from(SvlqRef(input))?;
                let input = &input[vi32_bits..];
                let (vi64, vi64_bits) = <(i64, usize)>::try_from(SvlqRef(input))?;
                let input = &input[vi64_bits..];

                Ok((
                    Self {
                        ts,
                        v8,
                        v16,
                        v32,
                        v64,
                        vi8,
                        vi16,
                        vi32,
                        vi64,
                    },
                    input,
                ))
            }
        }

        // How to unmarshal a delta from a bit slice
        impl FromCompressBits for TestRowDelta {
            fn from_bits(input: &BitBufferSlice) -> Result<(Self, &BitBufferSlice), &'static str> {
                let (ts, input) = decode_delta_i64(input)?;
                let Some(input) = input else {
                    return Err("Early EOF");
                };
                let (v8, input) = decode_delta_i16(input)?;
                let Some(input) = input else {
                    return Err("Early EOF");
                };
                let (v16, input) = decode_delta_i32(input)?;
                let Some(input) = input else {
                    return Err("Early EOF");
                };
                let (v32, input) = decode_delta_i64(input)?;
                let Some(input) = input else {
                    return Err("Early EOF");
                };
                let (v64, input) = decode_delta_i64(input)?;
                let Some(input) = input else {
                    return Err("Early EOF");
                };
                let (vi8, input) = decode_delta_i16(input)?;
                let Some(input) = input else {
                    return Err("Early EOF");
                };
                let (vi16, input) = decode_delta_i32(input)?;
                let Some(input) = input else {
                    return Err("Early EOF");
                };
                let (vi32, input) = decode_delta_i64(input)?;
                let Some(input) = input else {
                    return Err("Early EOF");
                };
                let (vi64, input) = decode_delta_i64(input)?;
                let input = input.unwrap_or_default();

                Ok((
                    Self {
                        ts: ts as i128,
                        v8,
                        v16,
                        v32,
                        v64: v64 as i128,
                        vi8,
                        vi16,
                        vi32,
                        vi64: vi64 as i128,
                    },
                    input,
                ))
            }
        }

        // How to compute the representations for a series of rows
        impl Compress for TestRow {
            type Full = TestRow;

            type Delta = TestRowDelta;

            fn into_full(self) -> Self::Full {
                // println!("into_full({:?})", self);
                self
            }

            fn into_delta(self, prev_row: &Self) -> Self::Delta {
                let r = self - *prev_row;
                // println!("into_delta: {:?} - {:?} = {:?}", prev_row, self, r);
                r
            }

            fn into_deltadelta(self, prev_prev_row: &Self, prev_row: &Self) -> Self::Delta {
                // println!("into_deltadelta: {:?} - {:?} = {:?}",  (*self - *prev_row), (*prev_row - *prev_prev_row), (*self - *prev_row) - (*prev_row - *prev_prev_row));
                (self - *prev_row) - (*prev_row - *prev_prev_row)
            }
        }

        impl Decompress for TestRow {
            type Full = TestRow;
            type Delta = TestRowDelta;

            fn from_full<'a>(
                bits: &'a BitBufferSlice,
            ) -> Result<(Self, &'a BitBufferSlice), &'static str> {
                TestRow::from_bits(bits).map_err(|_| "failed to unmarshal full row")
            }

            fn from_delta<'a>(
                bits: &'a BitBufferSlice,
                prev_row: &Self,
            ) -> Result<(Self, &'a BitBufferSlice), &'static str> {
                let delta =
                    TestRowDelta::from_bits(bits).map_err(|_| "failed to unmarshal delta row")?;
                Ok((*prev_row + delta.0, delta.1))
            }

            fn from_deltadelta<'a>(
                bits: &'a BitBufferSlice,
                prev_row: &Self,
                prev_prev_row: &Self,
            ) -> Result<(Self, &'a BitBufferSlice), &'static str> {
                // t = D + (t_prev - t_prev_prev) + t_prev
                let deltadelta = TestRowDelta::from_bits(bits)
                    .map_err(|_| "failed to unmarshal deltadelta row")?;
                Ok((
                    *prev_row + (*prev_row - *prev_prev_row) + deltadelta.0,
                    deltadelta.1,
                ))
            }
        }

        let mut compressor = Compressor::new();

        let lower = -32;
        let j = 0;
        for i in lower..10isize {
            let row = TestRow {
                ts: (j + i - lower) as u64,
                v8: (j + i - lower) as u8,
                v16: (j + i - lower) as u16,
                v32: (j + i - lower) as u32,
                v64: (j + i - lower) as u64,
                vi8: (j + i) as i8,
                vi16: (j + i) as i16,
                vi32: (j + i) as i32,
                vi64: (j + i) as i64,
            };
            // j += i;
            println!("compressing row {:?}", row);
            compressor.compress(row);
        }

        let encoded = compressor.finish();
        println!("{:?}", encoded);

        let mut decompressor = Decompressor::new(&encoded);
        for (idx, row) in decompressor.decompress::<TestRow>().enumerate() {
            println!("{:?}: {:?}", idx, row);
        }
    }
}
