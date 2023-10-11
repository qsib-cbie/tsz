use crate::prelude::BitBuffer;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::Debug;

use super::greedy_delta_v2::Queue;

///
/// Data comes in one row at a time, living for the duration of the function call
///
/// A compressor will make a copy of each column value into a column queue
///
/// A compressor will iterate over each column queue to decide to emit compressed bits
///
/// A column queue will pop N values at a time and a column bit buffer will be extended
///

pub struct Compressor<T: CompressibleRow> {
    columns: Vec<Box<dyn ColumnCompressor>>,
    _phantom: core::marker::PhantomData<T>,
}

pub trait CompressibleRow {
    const NUM_COLUMNS: usize;

    fn init_compressors() -> Vec<Box<dyn ColumnCompressor>>;
    fn as_ptr(&self, index: usize) -> *const u8;
}

pub trait ColumnCompressor {
    /// Consume a value and maybe emit bits
    fn push(&mut self, value: *const u8);

    /// Return the expected number of bits in the compressor
    fn len(&self) -> usize;
}

pub struct ColumnCompressorImpl<T> {
    queue: Queue<T, 10>,
    output: BitBuffer,
    avg_len: f32, // keep track of how large the samples are that haven't been popped yet
}

impl<T: Debug + Copy + ZigZagBits> ColumnCompressor for ColumnCompressorImpl<T> {
    fn push(&mut self, value: *const u8) {
        //
        // Safety: We know that the value is valid for the lifetime of the row
        //        and that it is a valid T.
        //       We also know that the queue is not full.
        //
        self.queue.push(unsafe { *(value as *const T) });

        //
        // If the queue is full, we need to decide how many samples to pop
        //
        if self.queue.is_full() {
            for t in self.queue.data.iter() {
                #[cfg(test)]
                {
                    println!(
                        "{:?} requires {} or {} bits",
                        t,
                        t.delta_bits(),
                        t.delta_delta_bits()
                    )
                }
            }
            self.queue.pop();
        }
    }

    fn len(&self) -> usize {
        self.output.len() + (self.avg_len * self.queue.data.len() as f32) as usize
    }
}

impl<T> ColumnCompressorImpl<T> {
    fn new() -> Self {
        ColumnCompressorImpl {
            queue: Queue::new(),
            output: BitBuffer::new(),
            avg_len: 1.2,
        }
    }
}

impl<T: CompressibleRow> Compressor<T> {
    pub fn new() -> Self {
        Compressor {
            columns: T::init_compressors(),
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn compress(&mut self, row: T) {
        for i in 0..T::NUM_COLUMNS {
            // Push a copy into the column queue
            self.columns[i].push(row.as_ptr(i));
        }
    }

    pub fn len(&self) -> usize {
        self.columns.iter().map(|c| c.len()).sum()
    }

    pub fn finish(self) -> BitBuffer {
        todo!();
    }
}

trait ZigZagBits {
    fn delta_bits(&self) -> usize;
    fn delta_delta_bits(&self) -> usize;
}

impl ZigZagBits for i8 {
    fn delta_bits(&self) -> usize {
        // todo check number of bits needed based on magnitude and cutoffs
        8
    }

    fn delta_delta_bits(&self) -> usize {
        // todo check number of bits needed based on magnitude and cutoffs
        90
    }
}

impl ZigZagBits for i16 {
    fn delta_bits(&self) -> usize {
        // todo check number of bits needed based on magnitude and cutoffs
        16
    }

    fn delta_delta_bits(&self) -> usize {
        // todo check number of bits needed based on magnitude and cutoffs
        91
    }
}

impl ZigZagBits for i32 {
    fn delta_bits(&self) -> usize {
        // todo check number of bits needed based on magnitude and cutoffs
        32
    }

    fn delta_delta_bits(&self) -> usize {
        // todo check number of bits needed based on magnitude and cutoffs
        92
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestRow {
        pub a: i8,
        pub b: i16,
        pub c: i32,
    }

    impl CompressibleRow for TestRow {
        const NUM_COLUMNS: usize = 3;

        fn init_compressors() -> Vec<Box<dyn ColumnCompressor>> {
            vec![
                Box::new(ColumnCompressorImpl::<i8>::new()),
                Box::new(ColumnCompressorImpl::<i16>::new()),
                Box::new(ColumnCompressorImpl::<i32>::new()),
            ]
        }

        fn as_ptr(&self, index: usize) -> *const u8 {
            match index {
                0 => &self.a as *const i8 as *const u8,
                1 => &self.b as *const i16 as *const u8,
                2 => &self.c as *const i32 as *const u8,
                _ => panic!("Invalid index"),
            }
        }
    }

    #[test]
    fn can_init() {
        let mut compressor = Compressor::<TestRow>::new();
    }

    #[test]
    fn can_push_row() {
        let mut compressor = Compressor::<TestRow>::new();
        let row = TestRow { a: 1, b: 2, c: 3 };
        compressor.compress(row);
        // todo check exact bits and rows match
    }

    #[test]
    fn can_finish() {
        let mut compressor = Compressor::<TestRow>::new();
        let row = TestRow { a: 1, b: 2, c: 3 };
        compressor.compress(row);
        let _bits = compressor.finish();
        // todo check exact bits and rows match
    }

    #[test]
    fn can_compress_11() {
        let mut compressor = Compressor::<TestRow>::new();
        for i in 0..11 {
            let row = TestRow {
                a: i,
                b: i as i16 * 10,
                c: i as i32 * 100,
            };
            compressor.compress(row);
        }
        let _bits = compressor.finish();
    }

    #[test]
    fn can_choose_zigzag_bits() {
        let num_bits = |i: i32| match i {
            -128..=127 => 8,
            -32768..=32767 => 16,
            _ => 32,
        };

        assert_eq!(num_bits(0), 8);
    }
}
