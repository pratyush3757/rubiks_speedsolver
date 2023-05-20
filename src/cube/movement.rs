use crate::cube::traits::{MoveCube, FaceBitMask};
use crate::cube::Cube;

impl MoveCube for Cube {
    fn u_clock(&mut self) {
        self.yellow = self.yellow.rotate_right(16);
        let temp = Self::upper_row(self.orange);
        self.orange = Self::replace_side(self.orange, Self::upper_row(self.blue), Self::UPPER_ROW_MASK); 
        self.blue = Self::replace_side(self.blue, Self::upper_row(self.red), Self::UPPER_ROW_MASK); 
        self.red = Self::replace_side(self.red, Self::upper_row(self.green), Self::UPPER_ROW_MASK); 
        self.green = Self::replace_side(self.green, temp, Self::UPPER_ROW_MASK); 
    }

    fn u_counter_clock(&mut self) {
        self.yellow = self.yellow.rotate_left(16);
        let temp = Self::upper_row(self.orange);
        self.orange = Self::replace_side(self.orange, Self::upper_row(self.green), Self::UPPER_ROW_MASK); 
        self.green = Self::replace_side(self.green, Self::upper_row(self.red), Self::UPPER_ROW_MASK); 
        self.red = Self::replace_side(self.red, Self::upper_row(self.blue), Self::UPPER_ROW_MASK); 
        self.blue = Self::replace_side(self.blue, temp, Self::UPPER_ROW_MASK); 
    }

    fn u_double(&mut self) {
        self.yellow = self.yellow.rotate_left(32);
        let temp = Self::upper_row(self.orange);
        self.orange = Self::replace_side(self.orange, Self::upper_row(self.red), Self::UPPER_ROW_MASK); 
        self.red = Self::replace_side(self.red, temp, Self::UPPER_ROW_MASK);
        let temp = Self::upper_row(self.blue);
        self.blue = Self::replace_side(self.blue, Self::upper_row(self.green), Self::UPPER_ROW_MASK); 
        self.green = Self::replace_side(self.green, temp, Self::UPPER_ROW_MASK); 

    }

    fn l_clock(&mut self) {
        self.blue = self.blue.rotate_right(16);
        let temp = Self::left_col(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::right_col(self.orange).rotate_right(32), Self::LEFT_COL_MASK);
        self.orange = Self::replace_side(self.orange, Self::left_col(self.white).rotate_right(32), Self::RIGHT_COL_MASK);
        self.white = Self::replace_side(self.white, Self::left_col(self.red), Self::LEFT_COL_MASK);
        self.red = Self::replace_side(self.red, temp, Self::LEFT_COL_MASK);
    }

    fn l_counter_clock(&mut self) {
        self.blue = self.blue.rotate_left(16);
        let temp = Self::left_col(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::left_col(self.red), Self::LEFT_COL_MASK);
        self.red = Self::replace_side(self.red, Self::left_col(self.white), Self::LEFT_COL_MASK);
        self.white = Self::replace_side(self.white, Self::right_col(self.orange).rotate_right(32), Self::LEFT_COL_MASK);
        self.orange = Self::replace_side(self.orange, temp.rotate_right(32), Self::RIGHT_COL_MASK);
    }

    fn l_double(&mut self) {
        self.blue = self.blue.rotate_right(32);
        let temp = Self::left_col(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::left_col(self.white), Self::LEFT_COL_MASK);
        self.white = Self::replace_side(self.white, temp, Self::LEFT_COL_MASK);
        let temp = Self::left_col(self.red);
        self.red = Self::replace_side(self.red, Self::right_col(self.orange).rotate_right(32), Self::LEFT_COL_MASK);
        self.orange = Self::replace_side(self.orange, temp.rotate_right(32), Self::RIGHT_COL_MASK);
    }

    fn f_clock(&mut self) {
        self.red = self.red.rotate_right(16);
        let temp = Self::down_row(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::right_col(self.blue).rotate_right(16), Self::DOWN_ROW_MASK);
        self.blue = Self::replace_side(self.blue, Self::upper_row(self.white).rotate_right(16), Self::RIGHT_COL_MASK);
        self.white = Self::replace_side(self.white, Self::left_col(self.green).rotate_right(16), Self::UPPER_ROW_MASK);
        self.green = Self::replace_side(self.green, temp.rotate_right(16), Self::LEFT_COL_MASK);
    }

    fn f_counter_clock(&mut self) {
        self.red = self.red.rotate_left(16);
        let temp = Self::down_row(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::left_col(self.green).rotate_left(16), Self::DOWN_ROW_MASK);
        self.green = Self::replace_side(self.green, Self::upper_row(self.white).rotate_left(16), Self::LEFT_COL_MASK);
        self.white = Self::replace_side(self.white, Self::right_col(self.blue).rotate_left(16), Self::UPPER_ROW_MASK);
        self.blue = Self::replace_side(self.blue, temp.rotate_left(16), Self::RIGHT_COL_MASK);
    }

    fn f_double(&mut self) {
        self.red = self.red.rotate_right(32);
        let temp = Self::down_row(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::upper_row(self.white).rotate_right(32), Self::DOWN_ROW_MASK);
        self.white = Self::replace_side(self.white, temp.rotate_right(32), Self::UPPER_ROW_MASK);
        let temp = Self::left_col(self.green);
        self.green = Self::replace_side(self.green, Self::right_col(self.blue).rotate_right(32), Self::LEFT_COL_MASK);
        self.blue = Self::replace_side(self.blue, temp.rotate_right(32), Self::RIGHT_COL_MASK);
    }

    fn r_clock(&mut self) {
        self.green = self.green.rotate_right(16);
        let temp = Self::right_col(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::right_col(self.red), Self::RIGHT_COL_MASK);
        self.red = Self::replace_side(self.red, Self::right_col(self.white), Self::RIGHT_COL_MASK);
        self.white = Self::replace_side(self.white, Self::left_col(self.orange).rotate_right(32), Self::RIGHT_COL_MASK);
        self.orange = Self::replace_side(self.orange, temp.rotate_right(32), Self::LEFT_COL_MASK);
    }

    fn r_counter_clock(&mut self) {
        self.green = self.green.rotate_left(16);
        let temp = Self::right_col(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::left_col(self.orange).rotate_right(32), Self::RIGHT_COL_MASK);
        self.orange = Self::replace_side(self.orange, Self::right_col(self.white).rotate_right(32), Self::LEFT_COL_MASK);
        self.white = Self::replace_side(self.white, Self::right_col(self.red), Self::RIGHT_COL_MASK);
        self.red = Self::replace_side(self.red, temp, Self::RIGHT_COL_MASK);
    }

    fn r_double(&mut self) {
        self.green = self.green.rotate_right(32);
        let temp = Self::right_col(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::right_col(self.white), Self::RIGHT_COL_MASK);
        self.white = Self::replace_side(self.white, temp, Self::RIGHT_COL_MASK);
        let temp = Self::left_col(self.orange);
        self.orange = Self::replace_side(self.orange, Self::right_col(self.red).rotate_right(32), Self::LEFT_COL_MASK);
        self.red = Self::replace_side(self.red, temp.rotate_right(32), Self::RIGHT_COL_MASK);
    }

    fn b_clock(&mut self) {
        self.orange = self.orange.rotate_right(16);
        let temp = Self::upper_row(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::right_col(self.green).rotate_left(16), Self::UPPER_ROW_MASK);
        self.green = Self::replace_side(self.green, Self::down_row(self.white).rotate_left(16), Self::RIGHT_COL_MASK);
        self.white = Self::replace_side(self.white, Self::left_col(self.blue).rotate_left(16), Self::DOWN_ROW_MASK);
        self.blue = Self::replace_side(self.blue, temp.rotate_left(16), Self::LEFT_COL_MASK);
    }

    fn b_counter_clock(&mut self) {
        self.orange = self.orange.rotate_right(16);
        let temp = Self::upper_row(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::left_col(self.blue).rotate_right(16), Self::UPPER_ROW_MASK);
        self.blue = Self::replace_side(self.blue, Self::down_row(self.white).rotate_right(16), Self::LEFT_COL_MASK);
        self.white = Self::replace_side(self.white, Self::right_col(self.green).rotate_right(16), Self::DOWN_ROW_MASK);
        self.green = Self::replace_side(self.green, temp.rotate_right(16), Self::RIGHT_COL_MASK);
    }

    fn b_double(&mut self) {
        self.orange = self.orange.rotate_right(32);
        let temp = Self::upper_row(self.yellow);
        self.yellow = Self::replace_side(self.yellow, Self::down_row(self.white).rotate_right(32), Self::UPPER_ROW_MASK);
        self.white = Self::replace_side(self.white, temp.rotate_right(32), Self::DOWN_ROW_MASK);
        let temp = Self::left_col(self.blue);
        self.blue = Self::replace_side(self.blue, Self::right_col(self.green).rotate_right(32), Self::LEFT_COL_MASK);
        self.green = Self::replace_side(self.green, temp.rotate_right(32), Self::RIGHT_COL_MASK);
    }

    fn d_clock(&mut self) {
        self.white = self.white.rotate_right(16);
        let temp = Self::down_row(self.orange);
        self.orange = Self::replace_side(self.orange, Self::down_row(self.green), Self::DOWN_ROW_MASK); 
        self.green = Self::replace_side(self.green, Self::down_row(self.red), Self::DOWN_ROW_MASK); 
        self.red = Self::replace_side(self.red, Self::down_row(self.blue), Self::DOWN_ROW_MASK); 
        self.blue = Self::replace_side(self.blue, temp, Self::DOWN_ROW_MASK); 
    }

    fn d_counter_clock(&mut self) {
        self.white = self.white.rotate_left(16);
        let temp = Self::down_row(self.orange);
        self.orange = Self::replace_side(self.orange, Self::down_row(self.blue), Self::DOWN_ROW_MASK); 
        self.blue = Self::replace_side(self.blue, Self::down_row(self.red), Self::DOWN_ROW_MASK); 
        self.red = Self::replace_side(self.red, Self::down_row(self.green), Self::DOWN_ROW_MASK); 
        self.green = Self::replace_side(self.green, temp, Self::DOWN_ROW_MASK); 
    }

    fn d_double(&mut self) {
        self.white = self.white.rotate_left(32);
        let temp = Self::down_row(self.orange);
        self.orange = Self::replace_side(self.orange, Self::down_row(self.red), Self::DOWN_ROW_MASK); 
        self.red = Self::replace_side(self.red, temp, Self::DOWN_ROW_MASK);
        let temp = Self::down_row(self.blue);
        self.blue = Self::replace_side(self.blue, Self::down_row(self.green), Self::DOWN_ROW_MASK); 
        self.green = Self::replace_side(self.green, temp, Self::DOWN_ROW_MASK); 
    }
}
