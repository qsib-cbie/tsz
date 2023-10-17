use crate::prelude::*;

pub mod queue;
pub use queue::*;
pub trait EmitBits<T> {
    /// Returns the number of elements popped from the queue.

    fn emit_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize;
}

impl EmitBits<i64> for CompressionQueue<i64, 10> {
    #[allow(unused)]
    fn emit_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut iter = self.iter();
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;
        while let Some(value) = iter.next() {
            let remaining = iter.size_hint().0;
            let index = 10 - remaining;

            if ((0..=2).contains(&index) && !(-32768..=32767).contains(&value)) {
                sixteen = false;
            }
            if ((0..=3).contains(&index) && !(-512..=511).contains(&value)) {
                ten = false;
            }
            if ((0..=4).contains(&index) && !(-128..=127).contains(&value)) {
                eight = false;
            }
            if ((0..=8).contains(&index) && !(-8..=7).contains(&value)) {
                four = false;
            }
            if ((0..=10).contains(&index) && !(-4..=3).contains(&value)) {
                three = false;
            }
        }

        if three {
            out.push(true);
            out.push(true);
            out.push(true);
            out.push(true);
            out.push(false);
            out.push(false);
            for _ in 0..10 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i64) ^ (value >> 63i64);
                    return 10;
                }
            }
        } else if four {
            for _ in 0..8 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i64) ^ (value >> 63i64);
                    return 8;
                }
            }
        } else if eight {
            for _ in 0..4 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i64) ^ (value >> 63i64);
                    return 4;
                }
            }
        } else if ten {
            for _ in 0..3 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i64) ^ (value >> 63i64);
                    return 3;
                }
            }
        } else if sixteen {
            for _ in 0..2 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i64) ^ (value >> 63i64);
                    return 2;
                }
            }
        }
        0
    }
}

impl EmitBits<i32> for CompressionQueue<i32, 10> {
    #[allow(unused)]
    fn emit_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut iter = self.iter();
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;
        while let Some(value) = iter.next() {
            let remaining = iter.size_hint().0;
            let index = 10 - remaining;

            if ((0..=2).contains(&index) && !(-32768..=32767).contains(&value)) {
                sixteen = false;
            }
            if ((0..=3).contains(&index) && !(-512..=511).contains(&value)) {
                ten = false;
            }
            if ((0..=4).contains(&index) && !(-128..=127).contains(&value)) {
                eight = false;
            }
            if ((0..=8).contains(&index) && !(-8..=7).contains(&value)) {
                four = false;
            }
            if ((0..=10).contains(&index) && !(-4..=3).contains(&value)) {
                three = false;
            }
        }

        if three {
            for _ in 0..10 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32);
                    return 10;
                }
            }
        } else if four {
            for _ in 0..8 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32);
                    return 8;
                }
            }
        } else if eight {
            for _ in 0..4 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32);
                    return 4;
                }
            }
        } else if ten {
            for _ in 0..3 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32);
                    return 3;
                }
            }
        } else if sixteen {
            for _ in 0..2 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i32) ^ (value >> 31i32);
                    return 2;
                }
            }
        }
        0
    }
}

impl EmitBits<i16> for CompressionQueue<i16, 10> {
    #[allow(unused)]
    fn emit_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut iter = self.iter();
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;
        while let Some(value) = iter.next() {
            let remaining = iter.size_hint().0;
            let index = 10 - remaining;

            if ((0..=3).contains(&index) && !(-512..=511).contains(&value)) {
                ten = false;
            }
            if ((0..=4).contains(&index) && !(-128..=127).contains(&value)) {
                eight = false;
            }
            if ((0..=8).contains(&index) && !(-8..=7).contains(&value)) {
                four = false;
            }
            if ((0..=10).contains(&index) && !(-4..=3).contains(&value)) {
                three = false;
            }
        }

        if three {
            out.push(true);
            out.push(true);
            out.push(true);
            out.push(true);
            out.push(false);
            out.push(false);
            for _ in 0..10 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16);
                    return 10;
                }
            }
        } else if four {
            for _ in 0..8 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16);
                    return 8;
                }
            }
        } else if eight {
            for _ in 0..4 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16);
                    return 4;
                }
            }
        } else if ten {
            for _ in 0..3 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16);
                    return 3;
                }
            }
        } else if sixteen {
            for _ in 0..2 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i16) ^ (value >> 15i16);
                    return 2;
                }
            }
        }
        0
    }
}

impl EmitBits<i8> for CompressionQueue<i8, 10> {
    #[allow(unused)]
    fn emit_bits(&mut self, out: &mut BitBuffer, flush: bool) -> usize {
        let mut iter = self.iter();
        let mut three = true;
        let mut four = true;
        let mut eight = true;
        let mut ten = true;
        let mut sixteen = true;
        while let Some(value) = iter.next() {
            let remaining = iter.size_hint().0;
            let index = 10 - remaining;

            if ((0..=8).contains(&index) && !(-8..=7).contains(&value)) {
                four = false;
            }
            if ((0..=10).contains(&index) && !(-4..=3).contains(&value)) {
                three = false;
            }
        }

        if three {
            out.push(true);
            out.push(true);
            out.push(true);
            out.push(true);
            out.push(false);
            out.push(false);
            for _ in 0..10 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i8) ^ (value >> 7i8);
                    return 10;
                }
            }
        } else if four {
            for _ in 0..8 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i8) ^ (value >> 7i8);
                    return 8;
                }
            }
        } else if eight {
            for _ in 0..4 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i8) ^ (value >> 7i8);
                    return 4;
                }
            }
        } else if ten {
            for _ in 0..3 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i8) ^ (value >> 7i8);
                    return 3;
                }
            }
        } else if sixteen {
            for _ in 0..2 {
                if let Some(value) = self.pop() {
                    let value = (value << 1i8) ^ (value >> 7i8);
                    return 2;
                }
            }
        }
        0
    }
}

///
/// High-level interface for compression.
///
pub trait TszCompressV2 {
    /// The type of the row to compress.
    type T: Copy;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_impl_compress() {
        #[derive(Copy, Clone)]
        struct TestRow {
            a: i32,
            b: i64,
        }
        struct CompressorImpl {
            a_queue: CompressionQueue<i32, 10>,
            b_queue: CompressionQueue<i64, 10>,
        }

        impl TszCompressV2 for CompressorImpl {
            type T = TestRow;
            fn compress(&mut self, row: TestRow) {
                self.a_queue.push(row.a);
                self.b_queue.push(row.b);
            }

            fn len(&self) -> usize {
                self.a_queue.len() + self.b_queue.len()
            }

            fn bit_rate(&self) -> usize {
                0
            }

            fn finish(self) -> BitBuffer {
                BitBuffer::new()
            }
        }
    }
}
