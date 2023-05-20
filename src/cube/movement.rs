use crate::cube::traits::{MoveCube, FaceBitMask};
use crate::cube::Cube;

impl MoveCube for Cube {
    fn u_clock(&mut self) {
        self.yellow = self.yellow.rotate_right(16);
        let temp = self.orange.upper_row();
        self.orange.replace_side(self.blue.upper_row(), u64::UPPER_ROW_MASK); 
        self.blue.replace_side(self.red.upper_row(), u64::UPPER_ROW_MASK); 
        self.red.replace_side(self.green.upper_row(), u64::UPPER_ROW_MASK); 
        self.green.replace_side(temp, u64::UPPER_ROW_MASK); 
    }

    fn u_counter_clock(&mut self) {
        self.yellow = self.yellow.rotate_left(16);
        let temp = self.orange.upper_row();
        self.orange.replace_side(self.green.upper_row(), u64::UPPER_ROW_MASK); 
        self.green.replace_side(self.red.upper_row(), u64::UPPER_ROW_MASK); 
        self.red.replace_side(self.blue.upper_row(), u64::UPPER_ROW_MASK); 
        self.blue.replace_side(temp, u64::UPPER_ROW_MASK); 
    }

    fn u_double(&mut self) {
        self.yellow = self.yellow.rotate_left(32);
        let temp = self.orange.upper_row();
        self.orange.replace_side(self.red.upper_row(), u64::UPPER_ROW_MASK); 
        self.red.replace_side(temp, u64::UPPER_ROW_MASK);
        let temp = self.blue.upper_row();
        self.blue.replace_side(self.green.upper_row(), u64::UPPER_ROW_MASK); 
        self.green.replace_side(temp, u64::UPPER_ROW_MASK); 

    }

    fn l_clock(&mut self) {
        self.blue = self.blue.rotate_right(16);
        let temp = self.yellow.left_col();
        self.yellow.replace_side(self.orange.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.orange.replace_side(self.white.left_col().rotate_right(32), u64::RIGHT_COL_MASK);
        self.white.replace_side(self.red.left_col(), u64::LEFT_COL_MASK);
        self.red.replace_side(temp, u64::LEFT_COL_MASK);
    }

    fn l_counter_clock(&mut self) {
        self.blue = self.blue.rotate_left(16);
        let temp = self.yellow.left_col();
        self.yellow.replace_side(self.red.left_col(), u64::LEFT_COL_MASK);
        self.red.replace_side(self.white.left_col(), u64::LEFT_COL_MASK);
        self.white.replace_side(self.orange.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.orange.replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn l_double(&mut self) {
        self.blue = self.blue.rotate_right(32);
        let temp = self.yellow.left_col();
        self.yellow.replace_side(self.white.left_col(), u64::LEFT_COL_MASK);
        self.white.replace_side(temp, u64::LEFT_COL_MASK);
        let temp = self.red.left_col();
        self.red.replace_side(self.orange.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.orange.replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn f_clock(&mut self) {
        self.red = self.red.rotate_right(16);
        let temp = self.yellow.down_row();
        self.yellow.replace_side(self.blue.right_col().rotate_right(16), u64::DOWN_ROW_MASK);
        self.blue.replace_side(self.white.upper_row().rotate_right(16), u64::RIGHT_COL_MASK);
        self.white.replace_side(self.green.left_col().rotate_right(16), u64::UPPER_ROW_MASK);
        self.green.replace_side(temp.rotate_right(16), u64::LEFT_COL_MASK);
    }

    fn f_counter_clock(&mut self) {
        self.red = self.red.rotate_left(16);
        let temp = self.yellow.down_row();
        self.yellow.replace_side(self.green.left_col().rotate_left(16), u64::DOWN_ROW_MASK);
        self.green.replace_side(self.white.upper_row().rotate_left(16), u64::LEFT_COL_MASK);
        self.white.replace_side(self.blue.right_col().rotate_left(16), u64::UPPER_ROW_MASK);
        self.blue.replace_side(temp.rotate_left(16), u64::RIGHT_COL_MASK);
    }

    fn f_double(&mut self) {
        self.red = self.red.rotate_right(32);
        let temp = self.yellow.down_row();
        self.yellow.replace_side(self.white.upper_row().rotate_right(32), u64::DOWN_ROW_MASK);
        self.white.replace_side(temp.rotate_right(32), u64::UPPER_ROW_MASK);
        let temp = self.green.left_col();
        self.green.replace_side(self.blue.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.blue.replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn r_clock(&mut self) {
        self.green = self.green.rotate_right(16);
        let temp = self.yellow.right_col();
        self.yellow.replace_side(self.red.right_col(), u64::RIGHT_COL_MASK);
        self.red.replace_side(self.white.right_col(), u64::RIGHT_COL_MASK);
        self.white.replace_side(self.orange.left_col().rotate_right(32), u64::RIGHT_COL_MASK);
        self.orange.replace_side(temp.rotate_right(32), u64::LEFT_COL_MASK);
    }

    fn r_counter_clock(&mut self) {
        self.green = self.green.rotate_left(16);
        let temp = self.yellow.right_col();
        self.yellow.replace_side(self.orange.left_col().rotate_right(32), u64::RIGHT_COL_MASK);
        self.orange.replace_side(self.white.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.white.replace_side(self.red.right_col(), u64::RIGHT_COL_MASK);
        self.red.replace_side(temp, u64::RIGHT_COL_MASK);
    }

    fn r_double(&mut self) {
        self.green = self.green.rotate_right(32);
        let temp = self.yellow.right_col();
        self.yellow.replace_side(self.white.right_col(), u64::RIGHT_COL_MASK);
        self.white.replace_side(temp, u64::RIGHT_COL_MASK);
        let temp = self.orange.left_col();
        self.orange.replace_side(self.red.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.red.replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn b_clock(&mut self) {
        self.orange = self.orange.rotate_right(16);
        let temp = self.yellow.upper_row();
        self.yellow.replace_side(self.green.right_col().rotate_left(16), u64::UPPER_ROW_MASK);
        self.green.replace_side(self.white.down_row().rotate_left(16), u64::RIGHT_COL_MASK);
        self.white.replace_side(self.blue.left_col().rotate_left(16), u64::DOWN_ROW_MASK);
        self.blue.replace_side(temp.rotate_left(16), u64::LEFT_COL_MASK);
    }

    fn b_counter_clock(&mut self) {
        self.orange = self.orange.rotate_right(16);
        let temp = self.yellow.upper_row();
        self.yellow.replace_side(self.blue.left_col().rotate_right(16), u64::UPPER_ROW_MASK);
        self.blue.replace_side(self.white.down_row().rotate_right(16), u64::LEFT_COL_MASK);
        self.white.replace_side(self.green.right_col().rotate_right(16), u64::DOWN_ROW_MASK);
        self.green.replace_side(temp.rotate_right(16), u64::RIGHT_COL_MASK);
    }

    fn b_double(&mut self) {
        self.orange = self.orange.rotate_right(32);
        let temp = self.yellow.upper_row();
        self.yellow.replace_side(self.white.down_row().rotate_right(32), u64::UPPER_ROW_MASK);
        self.white.replace_side(temp.rotate_right(32), u64::DOWN_ROW_MASK);
        let temp = self.blue.left_col();
        self.blue.replace_side(self.green.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.green.replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn d_clock(&mut self) {
        self.white = self.white.rotate_right(16);
        let temp = self.orange.down_row();
        self.orange.replace_side(self.green.down_row(), u64::DOWN_ROW_MASK); 
        self.green.replace_side(self.red.down_row(), u64::DOWN_ROW_MASK); 
        self.red.replace_side(self.blue.down_row(), u64::DOWN_ROW_MASK); 
        self.blue.replace_side(temp, u64::DOWN_ROW_MASK); 
    }

    fn d_counter_clock(&mut self) {
        self.white = self.white.rotate_left(16);
        let temp = self.orange.down_row();
        self.orange.replace_side(self.blue.down_row(), u64::DOWN_ROW_MASK); 
        self.blue.replace_side(self.red.down_row(), u64::DOWN_ROW_MASK); 
        self.red.replace_side(self.green.down_row(), u64::DOWN_ROW_MASK); 
        self.green.replace_side(temp, u64::DOWN_ROW_MASK); 
    }

    fn d_double(&mut self) {
        self.white = self.white.rotate_left(32);
        let temp = self.orange.down_row();
        self.orange.replace_side(self.red.down_row(), u64::DOWN_ROW_MASK); 
        self.red.replace_side(temp, u64::DOWN_ROW_MASK);
        let temp = self.blue.down_row();
        self.blue.replace_side(self.green.down_row(), u64::DOWN_ROW_MASK); 
        self.green.replace_side(temp, u64::DOWN_ROW_MASK); 
    }
}
