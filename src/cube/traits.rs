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

    fn upper_left(&self) -> u8;
    fn upper(&self) -> u8;
    fn upper_right(&self) -> u8;
    fn right(&self) -> u8;
    fn down_right(&self) -> u8;
    fn down(&self) -> u8;
    fn down_left(&self) -> u8;
    fn left(&self) -> u8;

    fn upper_row(&self) -> u64;
    fn down_row(&self) -> u64;
    fn left_col(&self) -> u64;
    fn right_col(&self) -> u64;

    fn replace_side(&mut self, with: u64, mask: u64);
}

impl FaceBitMask for u64 {
    fn upper_left(&self) -> u8 {
        ((self >> 56) & 0xff_u64) as u8
    }
    fn upper(&self) -> u8 {
        ((self >> 48) & 0xff_u64) as u8
    }
    fn upper_right(&self) -> u8 {
        ((self >> 40) & 0xff_u64) as u8
    }
    fn right(&self) -> u8 {
        ((self >> 32) & 0xff_u64) as u8
    }
    fn down_right(&self) -> u8 {
        ((self >> 24) & 0xff_u64) as u8
    }
    fn down(&self) -> u8 {
        ((self >> 16) & 0xff_u64) as u8
    }
    fn down_left(&self) -> u8 {
        ((self >> 8) & 0xff_u64) as u8
    }
    fn left(&self) -> u8 {
        (self & 0xff_u64) as u8
    }

    fn upper_row(&self) -> u64 {
        self & Self::UPPER_ROW_MASK
    }
    fn down_row(&self) -> u64 {
        self & Self::DOWN_ROW_MASK
    }
    fn left_col(&self) -> u64 {
        self & Self::LEFT_COL_MASK
    }
    fn right_col(&self) -> u64 {
        self & Self::RIGHT_COL_MASK
    }

    fn replace_side(&mut self, with: u64, mask: u64) {
        *self &= !mask; // Unset bits
        *self |= with;
    }
}

pub trait MoveCubeFace {
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

pub trait RotateCube {
    fn x_clock(&mut self);
    fn x_counter_clock(&mut self);
    fn x_double(&mut self);
    fn y_clock(&mut self);
    fn y_counter_clock(&mut self);
    fn y_double(&mut self);
    fn z_clock(&mut self);
    fn z_counter_clock(&mut self);
    fn z_double(&mut self);
    fn m_clock(&mut self);
    fn m_counter_clock(&mut self);
    fn m_double(&mut self);
    fn e_clock(&mut self);
    fn e_counter_clock(&mut self);
    fn e_double(&mut self);
    fn s_clock(&mut self);
    fn s_counter_clock(&mut self);
    fn s_double(&mut self);
}
