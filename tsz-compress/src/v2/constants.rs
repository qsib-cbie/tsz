pub struct Headers;

impl Headers {
    pub const START_OF_COLUMN: u8 = 0b1001;

    // DELTA ENCODING
    pub const THREE_BITS_TEN_SAMPLES: u8 = 0b1111;
    pub const SIX_BITS_FIVE_SAMPLES: u8 = 0b1110;
    pub const EIGHT_BITS_FOUR_SAMPLES: u8 = 0b1100;
    pub const TEN_BITS_THREE_SAMPLES: u8 = 0b1010;
    pub const SIXTEEN_BITS_TWO_SAMPLES: u8 = 0b1000;
    pub const THIRTY_TWO_BITS_ONE_SAMPLE: u8 = 0b1011;
}
