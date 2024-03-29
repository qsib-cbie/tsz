pub mod headers {
    pub const START_OF_COLUMN: u8 = 0b1001;
    pub const FIRST_ROW: u8 = 0b0110;
    pub const SECOND_ROW: u8 = FIRST_ROW;

    // DELTA ENCODING
    pub const THREE_BITS_TEN_SAMPLES: u8 = 0b1111;
    pub const SIX_BITS_FIVE_SAMPLES: u8 = 0b1110;
    pub const EIGHT_BITS_FOUR_SAMPLES: u8 = 0b1100;
    pub const TEN_BITS_THREE_SAMPLES: u8 = 0b1010;
    pub const SIXTEEN_BITS_TWO_SAMPLES: u8 = 0b1000;
    pub const THIRTY_TWO_BITS_ONE_SAMPLE: u8 = 0b1011;
    pub const SIXTY_FOUR_BITS_ONE_SAMPLE: u8 = 0b1101;
}
