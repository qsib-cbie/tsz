use tsz_compress::prelude::*;

#[cfg(test)]
mod tests {
    use bitvec::prelude::BitVec;

    use super::*;

    #[test]
    fn test_macro_compress() {
        #[derive(Copy, Clone, Debug, DeltaEncodable, Compressible, Decompressible)]
        pub struct AnotherRow {
            pub ts: i64,
            pub val0: i8,
            pub val1: i16,
            pub val2: i32,
            pub val3: i64,
        }

        let mut c = Compressor::new();

        let lower = -100000;
        let upper = 100000;
        for i in lower..upper {
            let row = AnotherRow {
                ts: i,
                val0: i as i8,
                val1: i as i16,
                val2: i as i32,
                val3: i as i64,
            };
            c.compress(row);
        }

        let bits = c.finish();
        println!(
            "bits: {} ({:0.1}x)",
            bits.len(),
            (((upper - lower) * std::mem::size_of::<AnotherRow>() as i64) * 8) as f64
                / bits.len() as f64
        );

        let mut d = Decompressor::new(&bits);
        for (i, row) in d.decompress::<AnotherRow>().unwrap().enumerate() {
            let i = i - lower as usize;
            assert_eq!(row.ts, i as i64);
            assert_eq!(row.val0, i as i8);
            assert_eq!(row.val1, i as i16);
            assert_eq!(row.val2, i as i32);
            assert_eq!(row.val3, i as i64);
        }
    }

    #[test]
    fn test_macro_hygiene() {
        #[derive(Copy, Clone, Debug, DeltaEncodable, Compressible, Decompressible)]
        pub struct ARow {
            pub ts: i64,
            pub val0: i8,
            pub val1: i16,
            pub val2: i32,
            pub val3: i64,
        }

        let _ = Compressor::<ARow>::new();
        let bits = BitVec::new();
        let mut d = Decompressor::new(&bits);
        let _ = d.decompress::<ARow>();

        #[derive(Copy, Clone, Debug, DeltaEncodable, Compressible, Decompressible)]
        pub struct BRow {
            pub ts: i64,
            pub val0: i8,
            pub val1: i16,
            pub val2: i32,
            pub val3: i64,
            pub val4: i128,
        }

        let _ = Compressor::<BRow>::new();
        let bits = BitVec::new();
        let mut d = Decompressor::new(&bits);
        let _ = d.decompress::<BRow>();
    }
}
