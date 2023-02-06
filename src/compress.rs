use bitvec::prelude::*;

///
/// A `Compressor` instance holds the state of the compression process.
/// 
/// Implement the `Compress` trait for your data type. 
/// Create a `Compressor` instance.
/// Call `compress` for each row of data, handing off your `Compressor` instance.
/// Call `finish` to get the compressed data.
/// 
pub struct Compressor<T: Compress> {
    pub output: BitVec,
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
pub struct Decompressor<'de, T: Decompress> {
    pub input: &'de BitSlice,
    pub row_n: Option<T>,
    pub row_n1: Option<T>,
}

pub trait Compress: Copy + Sized {
    /// Full and Delta may differ in signedness or storage
    /// Full is the representation of the value as a whole
    /// Delta is the representation of the difference between the value and the previous value or the difference between differences
    type Full: IntoCompressBits;
    type Delta: IntoCompressBits;

    fn into_full(&self) -> Self::Full;
    fn into_delta(&self, prev_row: &Self) -> Self::Delta;
    fn into_deltadelta(&self, prev_prev_row: &Self, prev_row: &Self) -> Self::Delta;
}

pub trait Decompress: Copy + Sized {
    type Full: FromCompressBits;
    type Delta: FromCompressBits;

    fn from_full(bits: &BitSlice) -> Result<(Self, usize), &'static str>;
    fn from_delta(bits: &BitSlice, prev_row: &Self) -> Result<(Self, usize), &'static str>;
    fn from_deltadelta(bits: &BitSlice, prev_row: &Self, delta: Self::Delta) -> Result<(Self, usize), &'static str>;
}

pub trait IntoCompressBits: Sized {
    fn into_bits(&self, out: &mut BitVec);
}

pub trait FromCompressBits: Sized {
    fn from_bits(input: &BitSlice) -> Self;
}

impl<T: Compress> Compressor<T> {
    pub fn new() -> Self {
        Self {
            output: BitVec::new(),
            row_n: None,
            row_n1: None,
        }
    }

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

    pub fn finish(self) -> BitVec {
        self.output
    }
}

impl <'de, T: Decompress> Decompressor<'de, T> {
    pub fn new(input: &'de BitSlice) -> Self {
        Self {
            input,
            row_n: None,
            row_n1: None,
        }
    }

    pub fn decompress(&mut self) -> Result<impl Iterator<Item = T>, &'static str> {
        // The first row is represented as the each value
        // Encoded to unsigned VLQ

        let (row, read) = match T::from_full(self.input) {
            Ok(row) => row,
            Err(e) => return Err(e),
        };
        self.input = &self.input[read..];


        //     let representation = T::Full::from_bits(&self.input);
        //     let row = T::from_full(representation);

        //     self.row_n = Some(row);

        //     // todo emit this row
            

        //     return;
        // };

        // let Some(row_n1) = self.row_n1.take() else {
        //     self.row_n1 = Some(row);
            
        //     // The second row is represented as the difference between the first row and the second row
        //     // Encoded to Gorilla Delta-Delta Encoding
        //     let representation = row.into_delta(&row_n);
        //     representation.into_bits(&mut self.output);

        //     return;
        // };

        // // Each subsequent row is represented as the deltadelta = (row - row_n1) - (row_n1 - row_n)
        // // Encoded to Gorilla Delta-Delta Encoding
        // let representation = row.into_deltadelta(&row_n, &row_n1);
        // representation.into_bits(&mut self.output);

        // // Move the rows along
        
        // self.row_n = Some(row_n1);
        // self.row_n1 = Some(row);

        Ok(None.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Sub;
    use crate::uvlq::*;
    use crate::svlq::*;

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
            fn into_bits(&self, out: &mut BitVec) {
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
            fn into_bits(&self, out: &mut BitVec) {
                out.extend(Svlq::from(self.ts).bits);
                out.extend(Svlq::from(self.v8).bits);
                out.extend(Svlq::from(self.v16).bits);
                out.extend(Svlq::from(self.v32).bits);
                out.extend(Svlq::from(self.v64).bits);
                out.extend(Svlq::from(self.vi8).bits);
                out.extend(Svlq::from(self.vi16).bits);
                out.extend(Svlq::from(self.vi32).bits);
                out.extend(Svlq::from(self.vi64).bits);
            }
        }

        // How to unmarshal a row from a bit slice
        impl FromCompressBits for TestRow {
            fn from_bits(input: &BitSlice) -> Self {
                let (ts, ts_bits) = Uvlq::from_bits(input);
                let (v8, v8_bits) = Uvlq::from_bits(&input[ts_bits..]);
                let (v16, v16_bits) = Uvlq::from_bits(&input[ts_bits + v8_bits..]);
                let (v32, v32_bits) = Uvlq::from_bits(&input[ts_bits + v8_bits + v16_bits..]);
                let (v64, v64_bits) = Uvlq::from_bits(&input[ts_bits + v8_bits + v16_bits + v32_bits..]);
                let (vi8, vi8_bits) = Svlq::from_bits(&input[ts_bits + v8_bits + v16_bits + v32_bits + v64_bits..]);
                let (vi16, vi16_bits) = Svlq::from_bits(&input[ts_bits + v8_bits + v16_bits + v32_bits + v64_bits + vi8_bits..]);
                let (vi32, vi32_bits) = Svlq::from_bits(&input[ts_bits + v8_bits + v16_bits + v32_bits + v64_bits + vi8_bits + vi16_bits..]);
                let (vi64, vi64_bits) = Svlq::from_bits(&input[ts_bits + v8_bits + v16_bits + v32_bits + v64_bits + vi8_bits + vi16_bits + vi32_bits..]);

                Self {
                    ts: ts.into(),
                    v8: v8.into(),
                    v16: v16.into(),
                    v32: v32.into(),
                    v64: v64.into(),
                    vi8: vi8.into(),
                    vi16: vi16.into(),
                    vi32: vi32.into(),
                    vi64: vi64.into(),
                }
            }
        }

        // How to unmarshal a delta from a bit slice
        impl FromCompressBits for TestRowDelta {
            fn from_bits(input: &BitSlice) -> Self {
                let (ts, ts_bits) = Svlq::from_bits(input);
                let (v8, v8_bits) = Svlq::from_bits(&input[ts_bits..]);
                let (v16, v16_bits) = Svlq::from_bits(&input[ts_bits + v8_bits..]);
                let (v32, v32_bits) = Svlq::from_bits(&input[ts_bits + v8_bits + v16_bits..]);
                let (v64, v64_bits) = Svlq::from_bits(&input[ts_bits + v8_bits + v16_bits + v32_bits..]);
                let (vi8, vi8_bits) = Svlq::from_bits(&input[ts_bits + v8_bits + v16_bits + v32_bits + v64_bits..]);
                let (vi16, vi16_bits) = Svlq::from_bits(&input[ts_bits + v8_bits + v16_bits + v32_bits + v64_bits + vi8_bits..]);
                let (vi32, vi32_bits) = Svlq::from_bits(&input[ts_bits + v8_bits + v16_bits + v32_bits + v64_bits + vi8_bits + vi16_bits..]);
                let (vi64, vi64_bits) = Svlq::from_bits(&input[ts_bits + v8_bits + v16_bits + v32_bits + v64_bits + vi8_bits + vi16_bits + vi32_bits..]);

                Self {
                    ts: ts.into(),
                    v8: v8.into(),
                    v16: v16.into(),
                    v32: v32.into(),
                    v64: v64.into(),
                    vi8: vi8.into(),
                    vi16: vi16.into(),
                    vi32: vi32.into(),
                    vi64: vi64.into(),
                }
            }
        }

        // How to compute the representations for a series of rows
        impl Compress for TestRow {
            type Full = TestRow;

            type Delta = TestRowDelta;

            fn into_full(&self) -> Self::Full {
                *self
            }

            fn into_delta(&self, prev_row: &Self) -> Self::Delta {
                *prev_row - *self
            }

            fn into_deltadelta(&self, prev_prev_row: &Self, prev_row: &Self) -> Self::Delta {
                (*self - *prev_row) - (*prev_row - *prev_prev_row)
            }
        }

        impl Decompress for TestRow {
            type Full = TestRow;
            type Delta = TestRowDelta;

            fn from_full(bits: &BitSlice) -> Result<(Self, usize), &'static str> {
                todo!()
            }

            fn from_delta(bits: &BitSlice, prev_row: &Self) -> Result<(Self, usize), &'static str> {
                todo!()
            }

            fn from_deltadelta(bits: &BitSlice, prev_row: &Self, delta: Self::Delta) -> Result<(Self, usize), &'static str> {
                todo!()
            }

        }

        let mut compressor = Compressor::new();

        for i in 0..2usize {
            let row = TestRow {
                ts: i.try_into().unwrap(),
                v8: i.try_into().unwrap(),
                v16: i.try_into().unwrap(),
                v32: i.try_into().unwrap(),
                v64: i.try_into().unwrap(),
                vi8: i.try_into().unwrap(),
                vi16: i.try_into().unwrap(),
                vi32: i.try_into().unwrap(),
                vi64: i.try_into().unwrap(),
            };
            compressor.compress(row);
        }


        let encoded = compressor.finish();
        println!("{:?}", encoded);

        let mut decompressor = Decompressor::<TestRow>::new(&encoded);
        for (idx, row) in decompressor.decompress().unwrap().enumerate() {
            println!("{:?}", row);
        }


    }
}
