use tsz_compress::prelude::*;

#[cfg(test)]
mod tests {
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
        for (i, row) in d.decompress::<AnotherRow>().enumerate() {
            let row = row.unwrap();
            let i = i as isize + lower as isize;
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
        let bits = BitBuffer::new();
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
        let bits = BitBuffer::new();
        let mut d = Decompressor::new(&bits);
        let _ = d.decompress::<BRow>();
    }

    #[test]
    fn test_one_row() {
        #[derive(Clone, Copy, DeltaEncodable, Compressible, Decompressible)]
        struct Row {
            a: i64,
        }

        let mut c = Compressor::new();
        let row = Row { a: 1 };
        c.compress(row);

        let bits = c.finish();
        assert!(!bits.is_empty());
        let mut d = Decompressor::new(&bits);
        let row = d.decompress::<Row>().next().unwrap();
        let row = row.unwrap();
        assert_eq!(row.a, 1);
    }

    #[test]
    fn test_two_rows() {
        #[derive(Clone, Copy, DeltaEncodable, Compressible, Decompressible)]
        struct Row {
            a: i64,
        }

        let mut c = Compressor::new();
        let row = Row { a: 1 };
        c.compress(row);
        let row = Row { a: 2 };
        c.compress(row);

        let bits = c.finish();
        assert!(!bits.is_empty());
        let mut d = Decompressor::new(&bits);
        let mut itr = d.decompress::<Row>();
        let row = itr.next().unwrap().unwrap();
        assert_eq!(row.a, 1);
        let row = itr.next().unwrap().unwrap();
        assert_eq!(row.a, 2);
    }

    #[test]
    fn test_five_rows() {
        #[derive(Clone, Copy, DeltaEncodable, Compressible, Decompressible)]
        struct Row {
            a: i64,
        }

        let mut c = Compressor::new();
        let row = Row { a: 1 };
        c.compress(row);
        let row = Row { a: 2 };
        c.compress(row);
        let row = Row { a: 3 };
        c.compress(row);
        let row = Row { a: 4 };
        c.compress(row);
        let row = Row { a: 5 };
        c.compress(row);

        let bits = c.finish();
        assert!(!bits.is_empty());
        let mut d = Decompressor::new(&bits);
        let mut itr = d.decompress::<Row>();
        let row = itr.next().unwrap().unwrap();
        assert_eq!(row.a, 1);
        let row = itr.next().unwrap().unwrap();
        assert_eq!(row.a, 2);
        let row = itr.next().unwrap().unwrap();
        assert_eq!(row.a, 3);
        let row = itr.next().unwrap().unwrap();
        assert_eq!(row.a, 4);
        let row = itr.next().unwrap().unwrap();
        assert_eq!(row.a, 5);
    }

    #[test]
    fn test_size() {
        #[derive(Clone, Copy, DeltaEncodable, Compressible, Decompressible)]
        struct Row {
            a: i64,
        }

        let mut c = Compressor::new();
        for i in 0..10000 {
            let row = Row { a: i };
            c.compress(row);
        }

        // Expect a byte size for 1 bit for every 64 bits per value plus the header
        assert!(!c.is_empty());
        assert_eq!(c.len(), 2 + (10000 / (64 / 8)));
    }
}
