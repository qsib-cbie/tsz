use core::ops::Add;
use core::ops::Sub;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use rand::Rng;
use tsz_compress::compress::*;
use tsz_compress::delta::*;
use tsz_compress::svlq::*;
use tsz_compress::uvlq::*;

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
        if self.ts >= i64::MIN as i128 && self.ts <= i64::MAX as i128 {
            encode_delta_i64(self.ts as i64, out);
        } else {
            unimplemented!()
        }

        encode_delta_i16(self.v8, out);
        encode_delta_i32(self.v16, out);
        encode_delta_i64(self.v32, out);

        if self.v64 >= i128::MIN as i128 && self.v64 <= i128::MAX as i128 {
            encode_delta_i64(self.v64 as i64, out);
        } else {
            unimplemented!()
        }

        encode_delta_i16(self.vi8, out);
        encode_delta_i32(self.vi16, out);
        encode_delta_i64(self.vi32, out);

        if self.vi64 >= i64::MIN as i128 && self.vi64 <= i64::MAX as i128 {
            encode_delta_i64(self.vi64 as i64, out);
        } else {
            unimplemented!()
        }
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

    fn from_full<'a>(bits: &'a BitBufferSlice) -> Result<(Self, &'a BitBufferSlice), &'static str> {
        TestRow::from_bits(bits).map_err(|_| "failed to unmarshal full row")
    }

    fn from_delta<'a>(
        bits: &'a BitBufferSlice,
        prev_row: &Self,
    ) -> Result<(Self, &'a BitBufferSlice), &'static str> {
        let delta = TestRowDelta::from_bits(bits).map_err(|_| "failed to unmarshal delta row")?;
        Ok((*prev_row + delta.0, delta.1))
    }

    fn from_deltadelta<'a>(
        bits: &'a BitBufferSlice,
        prev_row: &Self,
        prev_prev_row: &Self,
    ) -> Result<(Self, &'a BitBufferSlice), &'static str> {
        // t = D + (t_prev - t_prev_prev) + t_prev
        let deltadelta =
            TestRowDelta::from_bits(bits).map_err(|_| "failed to unmarshal deltadelta row")?;
        Ok((
            *prev_row + (*prev_row - *prev_prev_row) + deltadelta.0,
            deltadelta.1,
        ))
    }
}

fn compress(values: Vec<TestRow>) -> BitBuffer {
    let mut compressor = Compressor::new();
    values.into_iter().for_each(|row| compressor.compress(row));
    let compressed = compressor.finish();
    compressed
}

fn criterion_benchmark(c: &mut Criterion) {
    // let mut rng = rand::thread_rng();

    let mut values = vec![];
    let mut j = 0;
    for i in 0..500usize {
        let row = TestRow {
            ts: (j + i).try_into().unwrap(),
            v8: ((j + i) % u8::MAX as usize).try_into().unwrap(),
            v16: ((j + i) % u16::MAX as usize).try_into().unwrap(),
            v32: ((j + i) % u32::MAX as usize).try_into().unwrap(),
            v64: ((j + i) % u64::MAX as usize).try_into().unwrap(),
            vi8: (((j + i) % u8::MAX as usize) as i16 - (u8::MAX / 2) as i16)
                .try_into()
                .unwrap(),
            vi16: (((j + i) % u16::MAX as usize) as i32 - (u16::MAX / 2) as i32)
                .try_into()
                .unwrap(),
            vi32: (j + i).try_into().unwrap(),
            vi64: (j + i).try_into().unwrap(),
        };
        j += i;
        values.push(row);
    }

    c.bench_function("compress monotontic 500", |b| {
        b.iter(|| compress(black_box(values.clone())))
    });

    let mut values = vec![];
    let j = 0;
    for i in 0..500usize {
        let row = TestRow {
            ts: (j + i).try_into().unwrap(),
            v8: ((j + i) % u8::MAX as usize).try_into().unwrap(),
            v16: ((j + i) % u16::MAX as usize).try_into().unwrap(),
            v32: ((j + i) % u32::MAX as usize).try_into().unwrap(),
            v64: ((j + i) % u64::MAX as usize).try_into().unwrap(),
            vi8: (((j + i) % u8::MAX as usize) as i16 - (u8::MAX / 2) as i16)
                .try_into()
                .unwrap(),
            vi16: (((j + i) % u16::MAX as usize) as i32 - (u16::MAX / 2) as i32)
                .try_into()
                .unwrap(),
            vi32: (j + i).try_into().unwrap(),
            vi64: (j + i).try_into().unwrap(),
        };
        values.push(row);
    }

    c.bench_function("compress linear 500", |b| {
        b.iter(|| compress(black_box(values.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
