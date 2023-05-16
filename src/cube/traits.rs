pub trait FaceBitMask {
    // const upper_left: u64 = 0xff_00_00_00_00_00_00_00;
    // const U: u64 = 0x00_ff_00_00_00_00_00_00;
    // const upper_right: u64 = 0x00_00_ff_00_00_00_00_00;
    // const R: u64 = 0x00_00_00_ff_00_00_00_00;
    // const down_right: u64 = 0x00_00_00_00_ff_00_00_00;
    // const D: u64 = 0x00_00_00_00_00_ff_00_00;
    // const down_left: u64 = 0x00_00_00_00_00_00_ff_00;
    // const L: u64 = 0x00_00_00_00_00_00_00_ff;

    // Facelets in a face:
    //  |   1   |   2   |   3   |
    //  |   4   |       |   5   |
    //  |   6   |   7   |   8   |
    //
    //  Facelet as a u64 (8 * u8):
    //  1_2_3_5_8_7_6_4
    //  This config helps in turning a face by just rolling the bytes
    //  to the left or right
    fn upper_left(x: u64) -> u8 {
        ((x >> 56) & 0xff_u64) as u8
    }
    fn upper(x: u64) -> u8 {
        ((x >> 48) & 0xff_u64) as u8
    }
    fn upper_right(x: u64) -> u8 {
        ((x >> 40) & 0xff_u64) as u8
    }
    fn right(x: u64) -> u8 {
        ((x >> 32) & 0xff_u64) as u8
    }
    fn down_right(x: u64) -> u8 {
        ((x >> 24) & 0xff_u64) as u8
    }
    fn down(x: u64) -> u8 {
        ((x >> 16) & 0xff_u64) as u8
    }
    fn down_left(x: u64) -> u8 {
        ((x >> 8) & 0xff_u64) as u8
    }
    fn left(x: u64) -> u8 {
        (x & 0xff_u64) as u8
    }
}
