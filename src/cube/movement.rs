use crate::cube::traits::{FaceBitMask, MoveCube};
use crate::cube::Cube;

impl MoveCube for Cube {
    fn u_clock(&mut self) {
        self.yellow = self.yellow.rotate_right(16);
        let temp = self.orange.upper_row();
        self.orange
            .replace_side(self.blue.upper_row(), u64::UPPER_ROW_MASK);
        self.blue
            .replace_side(self.red.upper_row(), u64::UPPER_ROW_MASK);
        self.red
            .replace_side(self.green.upper_row(), u64::UPPER_ROW_MASK);
        self.green.replace_side(temp, u64::UPPER_ROW_MASK);
    }

    fn u_counter_clock(&mut self) {
        self.yellow = self.yellow.rotate_left(16);
        let temp = self.orange.upper_row();
        self.orange
            .replace_side(self.green.upper_row(), u64::UPPER_ROW_MASK);
        self.green
            .replace_side(self.red.upper_row(), u64::UPPER_ROW_MASK);
        self.red
            .replace_side(self.blue.upper_row(), u64::UPPER_ROW_MASK);
        self.blue.replace_side(temp, u64::UPPER_ROW_MASK);
    }

    fn u_double(&mut self) {
        self.yellow = self.yellow.rotate_left(32);
        let temp = self.orange.upper_row();
        self.orange
            .replace_side(self.red.upper_row(), u64::UPPER_ROW_MASK);
        self.red.replace_side(temp, u64::UPPER_ROW_MASK);
        let temp = self.blue.upper_row();
        self.blue
            .replace_side(self.green.upper_row(), u64::UPPER_ROW_MASK);
        self.green.replace_side(temp, u64::UPPER_ROW_MASK);
    }

    fn l_clock(&mut self) {
        self.blue = self.blue.rotate_right(16);
        let temp = self.yellow.left_col();
        self.yellow
            .replace_side(self.orange.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.orange
            .replace_side(self.white.left_col().rotate_right(32), u64::RIGHT_COL_MASK);
        self.white
            .replace_side(self.red.left_col(), u64::LEFT_COL_MASK);
        self.red.replace_side(temp, u64::LEFT_COL_MASK);
    }

    fn l_counter_clock(&mut self) {
        self.blue = self.blue.rotate_left(16);
        let temp = self.yellow.left_col();
        self.yellow
            .replace_side(self.red.left_col(), u64::LEFT_COL_MASK);
        self.red
            .replace_side(self.white.left_col(), u64::LEFT_COL_MASK);
        self.white
            .replace_side(self.orange.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.orange
            .replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn l_double(&mut self) {
        self.blue = self.blue.rotate_right(32);
        let temp = self.yellow.left_col();
        self.yellow
            .replace_side(self.white.left_col(), u64::LEFT_COL_MASK);
        self.white.replace_side(temp, u64::LEFT_COL_MASK);
        let temp = self.red.left_col();
        self.red
            .replace_side(self.orange.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.orange
            .replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn f_clock(&mut self) {
        self.red = self.red.rotate_right(16);
        let temp = self.yellow.down_row();
        self.yellow
            .replace_side(self.blue.right_col().rotate_right(16), u64::DOWN_ROW_MASK);
        self.blue
            .replace_side(self.white.upper_row().rotate_right(16), u64::RIGHT_COL_MASK);
        self.white
            .replace_side(self.green.left_col().rotate_right(16), u64::UPPER_ROW_MASK);
        self.green
            .replace_side(temp.rotate_right(16), u64::LEFT_COL_MASK);
    }

    fn f_counter_clock(&mut self) {
        self.red = self.red.rotate_left(16);
        let temp = self.yellow.down_row();
        self.yellow
            .replace_side(self.green.left_col().rotate_left(16), u64::DOWN_ROW_MASK);
        self.green
            .replace_side(self.white.upper_row().rotate_left(16), u64::LEFT_COL_MASK);
        self.white
            .replace_side(self.blue.right_col().rotate_left(16), u64::UPPER_ROW_MASK);
        self.blue
            .replace_side(temp.rotate_left(16), u64::RIGHT_COL_MASK);
    }

    fn f_double(&mut self) {
        self.red = self.red.rotate_right(32);
        let temp = self.yellow.down_row();
        self.yellow
            .replace_side(self.white.upper_row().rotate_right(32), u64::DOWN_ROW_MASK);
        self.white
            .replace_side(temp.rotate_right(32), u64::UPPER_ROW_MASK);
        let temp = self.green.left_col();
        self.green
            .replace_side(self.blue.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.blue
            .replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn r_clock(&mut self) {
        self.green = self.green.rotate_right(16);
        let temp = self.yellow.right_col();
        self.yellow
            .replace_side(self.red.right_col(), u64::RIGHT_COL_MASK);
        self.red
            .replace_side(self.white.right_col(), u64::RIGHT_COL_MASK);
        self.white
            .replace_side(self.orange.left_col().rotate_right(32), u64::RIGHT_COL_MASK);
        self.orange
            .replace_side(temp.rotate_right(32), u64::LEFT_COL_MASK);
    }

    fn r_counter_clock(&mut self) {
        self.green = self.green.rotate_left(16);
        let temp = self.yellow.right_col();
        self.yellow
            .replace_side(self.orange.left_col().rotate_right(32), u64::RIGHT_COL_MASK);
        self.orange
            .replace_side(self.white.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.white
            .replace_side(self.red.right_col(), u64::RIGHT_COL_MASK);
        self.red.replace_side(temp, u64::RIGHT_COL_MASK);
    }

    fn r_double(&mut self) {
        self.green = self.green.rotate_right(32);
        let temp = self.yellow.right_col();
        self.yellow
            .replace_side(self.white.right_col(), u64::RIGHT_COL_MASK);
        self.white.replace_side(temp, u64::RIGHT_COL_MASK);
        let temp = self.orange.left_col();
        self.orange
            .replace_side(self.red.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.red
            .replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn b_clock(&mut self) {
        self.orange = self.orange.rotate_right(16);
        let temp = self.yellow.upper_row();
        self.yellow
            .replace_side(self.green.right_col().rotate_left(16), u64::UPPER_ROW_MASK);
        self.green
            .replace_side(self.white.down_row().rotate_left(16), u64::RIGHT_COL_MASK);
        self.white
            .replace_side(self.blue.left_col().rotate_left(16), u64::DOWN_ROW_MASK);
        self.blue
            .replace_side(temp.rotate_left(16), u64::LEFT_COL_MASK);
    }

    fn b_counter_clock(&mut self) {
        self.orange = self.orange.rotate_right(16);
        let temp = self.yellow.upper_row();
        self.yellow
            .replace_side(self.blue.left_col().rotate_right(16), u64::UPPER_ROW_MASK);
        self.blue
            .replace_side(self.white.down_row().rotate_right(16), u64::LEFT_COL_MASK);
        self.white
            .replace_side(self.green.right_col().rotate_right(16), u64::DOWN_ROW_MASK);
        self.green
            .replace_side(temp.rotate_right(16), u64::RIGHT_COL_MASK);
    }

    fn b_double(&mut self) {
        self.orange = self.orange.rotate_right(32);
        let temp = self.yellow.upper_row();
        self.yellow
            .replace_side(self.white.down_row().rotate_right(32), u64::UPPER_ROW_MASK);
        self.white
            .replace_side(temp.rotate_right(32), u64::DOWN_ROW_MASK);
        let temp = self.blue.left_col();
        self.blue
            .replace_side(self.green.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.green
            .replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn d_clock(&mut self) {
        self.white = self.white.rotate_right(16);
        let temp = self.orange.down_row();
        self.orange
            .replace_side(self.green.down_row(), u64::DOWN_ROW_MASK);
        self.green
            .replace_side(self.red.down_row(), u64::DOWN_ROW_MASK);
        self.red
            .replace_side(self.blue.down_row(), u64::DOWN_ROW_MASK);
        self.blue.replace_side(temp, u64::DOWN_ROW_MASK);
    }

    fn d_counter_clock(&mut self) {
        self.white = self.white.rotate_left(16);
        let temp = self.orange.down_row();
        self.orange
            .replace_side(self.blue.down_row(), u64::DOWN_ROW_MASK);
        self.blue
            .replace_side(self.red.down_row(), u64::DOWN_ROW_MASK);
        self.red
            .replace_side(self.green.down_row(), u64::DOWN_ROW_MASK);
        self.green.replace_side(temp, u64::DOWN_ROW_MASK);
    }

    fn d_double(&mut self) {
        self.white = self.white.rotate_left(32);
        let temp = self.orange.down_row();
        self.orange
            .replace_side(self.red.down_row(), u64::DOWN_ROW_MASK);
        self.red.replace_side(temp, u64::DOWN_ROW_MASK);
        let temp = self.blue.down_row();
        self.blue
            .replace_side(self.green.down_row(), u64::DOWN_ROW_MASK);
        self.green.replace_side(temp, u64::DOWN_ROW_MASK);
    }
}

#[cfg(test)]
mod tests {
    use crate::cube::{enums::Color, traits::MoveCube, Face};

    use super::Cube;

    // To test left and right face movements
    fn new_ref_cube_front_turned_clockwise() -> Cube {
        Cube {
            yellow: Face::new(
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
                Color::Yellow,
                Color::Blue,
                Color::Blue,
                Color::Blue,
            )
            .0,
            blue: Face::new(
                Color::Blue,
                Color::Blue,
                Color::White,
                Color::Blue,
                Color::White,
                Color::Blue,
                Color::Blue,
                Color::White,
            )
            .0,
            red: Face::new_solved(Color::Red).0,
            green: Face::new(
                Color::Yellow,
                Color::Green,
                Color::Green,
                Color::Yellow,
                Color::Green,
                Color::Yellow,
                Color::Green,
                Color::Green,
            )
            .0,
            orange: Face::new_solved(Color::Orange).0,
            white: Face::new(
                Color::Green,
                Color::Green,
                Color::Green,
                Color::White,
                Color::White,
                Color::White,
                Color::White,
                Color::White,
            )
            .0,
        }
    }

    // To test upper, down, front and back face movements
    fn new_ref_cube_right_turned_clockwise() -> Cube {
        Cube {
            yellow: Face::new(
                Color::Yellow,
                Color::Yellow,
                Color::Red,
                Color::Yellow,
                Color::Red,
                Color::Yellow,
                Color::Yellow,
                Color::Red,
            )
            .0,
            blue: Face::new_solved(Color::Blue).0,
            red: Face::new(
                Color::Red,
                Color::Red,
                Color::White,
                Color::Red,
                Color::White,
                Color::Red,
                Color::Red,
                Color::White,
            )
            .0,
            green: Face::new_solved(Color::Green).0,
            orange: Face::new(
                Color::Yellow,
                Color::Orange,
                Color::Orange,
                Color::Yellow,
                Color::Orange,
                Color::Yellow,
                Color::Orange,
                Color::Orange,
            )
            .0,
            white: Face::new(
                Color::White,
                Color::White,
                Color::Orange,
                Color::White,
                Color::Orange,
                Color::White,
                Color::White,
                Color::Orange,
            )
            .0,
        }
    }

    #[test]
    fn upper_clockwise() {
        let mut solved_cube = new_ref_cube_right_turned_clockwise();
        solved_cube.u_clock();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red,
                    Color::Red,
                    Color::Red
                )
                .0,
                blue: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                red: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::Red,
                    Color::White
                )
                .0,
                green: Face::new(
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green
                )
                .0,
                orange: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Yellow,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                white: Face::new(
                    Color::White,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::White,
                    Color::Orange
                )
                .0,
            }
        )
    }

    #[test]
    fn upper_counter_clockwise() {
        let mut solved_cube = new_ref_cube_right_turned_clockwise();
        solved_cube.u_counter_clock();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Yellow
                )
                .0,
                blue: Face::new(
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                red: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::Red,
                    Color::White
                )
                .0,
                green: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green
                )
                .0,
                orange: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                white: Face::new(
                    Color::White,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::White,
                    Color::Orange
                )
                .0,
            }
        )
    }

    #[test]
    fn upper_double() {
        let mut solved_cube = new_ref_cube_right_turned_clockwise();
        solved_cube.u_double();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow
                )
                .0,
                blue: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                red: Face::new(
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::Red,
                    Color::White
                )
                .0,
                green: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green
                )
                .0,
                orange: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::Yellow,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                white: Face::new(
                    Color::White,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::White,
                    Color::Orange
                )
                .0,
            }
        )
    }

    #[test]
    fn left_clockwise() {
        let mut solved_cube = new_ref_cube_front_turned_clockwise();
        solved_cube.l_clock();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Orange,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                blue: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::White,
                    Color::White,
                    Color::White
                )
                .0,
                red: Face::new(
                    Color::Yellow,
                    Color::Red,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Blue,
                    Color::Red,
                    Color::Red
                )
                .0,
                green: Face::new(
                    Color::Yellow,
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Green
                )
                .0,
                orange: Face::new(
                    Color::Orange,
                    Color::Orange,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::Orange,
                    Color::Orange,
                    Color::Green
                )
                .0,
                white: Face::new(
                    Color::Red,
                    Color::Green,
                    Color::Green,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::White,
                    Color::White
                )
                .0,
            }
        )
    }

    #[test]
    fn left_counter_clockwise() {
        let mut solved_cube = new_ref_cube_front_turned_clockwise();
        solved_cube.l_counter_clock();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                blue: Face::new(
                    Color::White,
                    Color::White,
                    Color::White,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                red: Face::new(
                    Color::Green,
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::Red
                )
                .0,
                green: Face::new(
                    Color::Yellow,
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Green
                )
                .0,
                orange: Face::new(
                    Color::Orange,
                    Color::Orange,
                    Color::Blue,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow
                )
                .0,
                white: Face::new(
                    Color::Orange,
                    Color::Green,
                    Color::Green,
                    Color::Orange,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::White
                )
                .0,
            }
        )
    }

    #[test]
    fn left_double() {
        let mut solved_cube = new_ref_cube_front_turned_clockwise();
        solved_cube.l_double();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Green,
                    Color::Yellow,
                    Color::Yellow,
                    Color::White,
                    Color::Yellow,
                    Color::White,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                blue: Face::new(
                    Color::White,
                    Color::Blue,
                    Color::Blue,
                    Color::White,
                    Color::Blue,
                    Color::White,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                red: Face::new(
                    Color::Orange,
                    Color::Red,
                    Color::Red,
                    Color::Orange,
                    Color::Red,
                    Color::Orange,
                    Color::Red,
                    Color::Red
                )
                .0,
                green: Face::new(
                    Color::Yellow,
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Green
                )
                .0,
                orange: Face::new(
                    Color::Orange,
                    Color::Orange,
                    Color::Red,
                    Color::Orange,
                    Color::Red,
                    Color::Orange,
                    Color::Orange,
                    Color::Red
                )
                .0,
                white: Face::new(
                    Color::Yellow,
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::White,
                    Color::Blue,
                    Color::White,
                    Color::White
                )
                .0,
            }
        )
    }

    #[test]
    fn front_clockwise() {
        let mut solved_cube = new_ref_cube_right_turned_clockwise();
        solved_cube.f_clock();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                blue: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::White,
                    Color::Blue,
                    Color::White,
                    Color::Blue,
                    Color::Blue,
                    Color::Orange
                )
                .0,
                red: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::Red,
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::White,
                    Color::White
                )
                .0,
                green: Face::new(
                    Color::Yellow,
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Red,
                    Color::Green,
                    Color::Green
                )
                .0,
                orange: Face::new(
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                white: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::White,
                    Color::Orange
                )
                .0,
            }
        )
    }

    #[test]
    fn front_counter_clockwise() {
        let mut solved_cube = new_ref_cube_right_turned_clockwise();
        solved_cube.f_counter_clock();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Green,
                    Color::Green,
                    Color::Green
                )
                .0,
                blue: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::Red,
                    Color::Blue,
                    Color::Yellow,
                    Color::Blue,
                    Color::Blue,
                    Color::Yellow
                )
                .0,
                red: Face::new(
                    Color::White,
                    Color::White,
                    Color::White,
                    Color::Red,
                    Color::Red,
                    Color::Red,
                    Color::Red,
                    Color::Red
                )
                .0,
                green: Face::new(
                    Color::Orange,
                    Color::Green,
                    Color::Green,
                    Color::White,
                    Color::Green,
                    Color::White,
                    Color::Green,
                    Color::Green
                )
                .0,
                orange: Face::new(
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                white: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::White,
                    Color::Orange
                )
                .0,
            }
        )
    }

    #[test]
    fn front_double() {
        let mut solved_cube = new_ref_cube_right_turned_clockwise();
        solved_cube.f_double();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Orange,
                    Color::White,
                    Color::White
                )
                .0,
                blue: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::Green,
                    Color::Blue,
                    Color::Green,
                    Color::Blue,
                    Color::Blue,
                    Color::Green
                )
                .0,
                red: Face::new(
                    Color::White,
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::Red
                )
                .0,
                green: Face::new(
                    Color::Blue,
                    Color::Green,
                    Color::Green,
                    Color::Blue,
                    Color::Green,
                    Color::Blue,
                    Color::Green,
                    Color::Green
                )
                .0,
                orange: Face::new(
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                white: Face::new(
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::White,
                    Color::Orange
                )
                .0,
            }
        )
    }

    #[test]
    fn right_clockwise() {
        let mut solved_cube = new_ref_cube_front_turned_clockwise();
        solved_cube.r_clock();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Blue,
                    Color::Blue,
                    Color::Red
                )
                .0,
                blue: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::White,
                    Color::Blue,
                    Color::White,
                    Color::Blue,
                    Color::Blue,
                    Color::White
                )
                .0,
                red: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::Green,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::Red,
                    Color::White
                )
                .0,
                green: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green
                )
                .0,
                orange: Face::new(
                    Color::Blue,
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                white: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Orange,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::White,
                    Color::Orange
                )
                .0,
            }
        )
    }

    #[test]
    fn right_counter_clockwise() {
        let mut solved_cube = new_ref_cube_front_turned_clockwise();
        solved_cube.r_counter_clock();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Blue,
                    Color::Blue,
                    Color::Orange
                )
                .0,
                blue: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::White,
                    Color::Blue,
                    Color::White,
                    Color::Blue,
                    Color::Blue,
                    Color::White
                )
                .0,
                red: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Red,
                    Color::Blue
                )
                .0,
                green: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Yellow
                )
                .0,
                orange: Face::new(
                    Color::White,
                    Color::Orange,
                    Color::Orange,
                    Color::White,
                    Color::Orange,
                    Color::Green,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                white: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::White,
                    Color::White,
                    Color::Red
                )
                .0,
            }
        )
    }

    #[test]
    fn right_double() {
        let mut solved_cube = new_ref_cube_front_turned_clockwise();
        solved_cube.r_double();
        assert_eq!(
            solved_cube,
            Cube {
                yellow: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Green,
                    Color::Yellow,
                    Color::White,
                    Color::Blue,
                    Color::Blue,
                    Color::White
                )
                .0,
                blue: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::White,
                    Color::Blue,
                    Color::White,
                    Color::Blue,
                    Color::Blue,
                    Color::White
                )
                .0,
                red: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::Orange,
                    Color::Red,
                    Color::Orange,
                    Color::Red,
                    Color::Red,
                    Color::Orange
                )
                .0,
                green: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Green,
                    Color::Yellow
                )
                .0,
                orange: Face::new(
                    Color::Red,
                    Color::Orange,
                    Color::Orange,
                    Color::Red,
                    Color::Orange,
                    Color::Red,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                white: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::White,
                    Color::Yellow,
                    Color::White,
                    Color::White,
                    Color::Blue
                )
                .0,
            }
        )
    }
}
