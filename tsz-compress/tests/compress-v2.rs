#![allow(unused)]
use tsz_compress::prelude::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_expand_v2() {
        #[derive(Copy, Clone, CompressV2)]
        struct TestRow {
            a: i8,
            b: i16,
            c: i32,
        }
    }
}
