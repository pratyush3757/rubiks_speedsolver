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
    const UPPER_ROW_MASK: u64 = 0xff_ff_ff_00_00_00_00_00u64;
    const DOWN_ROW_MASK: u64 = 0x00_00_00_00_ff_ff_ff_00u64;
    const LEFT_COL_MASK: u64 = 0xff_00_00_00_00_00_ff_ffu64;
    const RIGHT_COL_MASK: u64 = 0x00_00_ff_ff_ff_00_00_00u64;

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

    fn upper_row(x: u64) -> u64 {
        x & Self::UPPER_ROW_MASK
    }
    fn down_row(x: u64) -> u64 {
        x & Self::DOWN_ROW_MASK
    }
    fn left_col(x: u64) -> u64 {
        x & Self::LEFT_COL_MASK
    }
    fn right_col(x: u64) -> u64 {
        x & Self::RIGHT_COL_MASK
    }

    fn replace_side(mut x: u64, new_x: u64, mask: u64) -> u64 {
        x &= !mask; // Unset bits
        x | new_x
    }
}

pub trait MoveCube {
    fn u_clock(&mut self);
    fn u_counter_clock(&mut self);
    fn u_double(&mut self);
    fn l_clock(&mut self);
    fn l_counter_clock(&mut self);
    fn l_double(&mut self);
    fn f_clock(&mut self);
    fn f_counter_clock(&mut self);
    fn f_double(&mut self);
    fn r_clock(&mut self);
    fn r_counter_clock(&mut self);
    fn r_double(&mut self);
    fn b_clock(&mut self);
    fn b_counter_clock(&mut self);
    fn b_double(&mut self);
    fn d_clock(&mut self);
    fn d_counter_clock(&mut self);
    fn d_double(&mut self);
}
