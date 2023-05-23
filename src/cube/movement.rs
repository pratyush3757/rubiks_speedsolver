use crate::cube::traits::{FaceBitMask, MoveCube};
use crate::cube::Cube;

impl MoveCube for Cube {
    fn u_clock(&mut self) {
        self.u_face = self.u_face.rotate_right(16);
        let temp = self.b_face.upper_row();
        self.b_face
            .replace_side(self.l_face.upper_row(), u64::UPPER_ROW_MASK);
        self.l_face
            .replace_side(self.f_face.upper_row(), u64::UPPER_ROW_MASK);
        self.f_face
            .replace_side(self.r_face.upper_row(), u64::UPPER_ROW_MASK);
        self.r_face.replace_side(temp, u64::UPPER_ROW_MASK);
    }

    fn u_counter_clock(&mut self) {
        self.u_face = self.u_face.rotate_left(16);
        let temp = self.b_face.upper_row();
        self.b_face
            .replace_side(self.r_face.upper_row(), u64::UPPER_ROW_MASK);
        self.r_face
            .replace_side(self.f_face.upper_row(), u64::UPPER_ROW_MASK);
        self.f_face
            .replace_side(self.l_face.upper_row(), u64::UPPER_ROW_MASK);
        self.l_face.replace_side(temp, u64::UPPER_ROW_MASK);
    }

    fn u_double(&mut self) {
        self.u_face = self.u_face.rotate_left(32);
        let temp = self.b_face.upper_row();
        self.b_face
            .replace_side(self.f_face.upper_row(), u64::UPPER_ROW_MASK);
        self.f_face.replace_side(temp, u64::UPPER_ROW_MASK);
        let temp = self.l_face.upper_row();
        self.l_face
            .replace_side(self.r_face.upper_row(), u64::UPPER_ROW_MASK);
        self.r_face.replace_side(temp, u64::UPPER_ROW_MASK);
    }

    fn l_clock(&mut self) {
        self.l_face = self.l_face.rotate_right(16);
        let temp = self.u_face.left_col();
        self.u_face
            .replace_side(self.b_face.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.b_face
            .replace_side(self.d_face.left_col().rotate_right(32), u64::RIGHT_COL_MASK);
        self.d_face
            .replace_side(self.f_face.left_col(), u64::LEFT_COL_MASK);
        self.f_face.replace_side(temp, u64::LEFT_COL_MASK);
    }

    fn l_counter_clock(&mut self) {
        self.l_face = self.l_face.rotate_left(16);
        let temp = self.u_face.left_col();
        self.u_face
            .replace_side(self.f_face.left_col(), u64::LEFT_COL_MASK);
        self.f_face
            .replace_side(self.d_face.left_col(), u64::LEFT_COL_MASK);
        self.d_face
            .replace_side(self.b_face.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.b_face
            .replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn l_double(&mut self) {
        self.l_face = self.l_face.rotate_right(32);
        let temp = self.u_face.left_col();
        self.u_face
            .replace_side(self.d_face.left_col(), u64::LEFT_COL_MASK);
        self.d_face.replace_side(temp, u64::LEFT_COL_MASK);
        let temp = self.f_face.left_col();
        self.f_face
            .replace_side(self.b_face.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.b_face
            .replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn f_clock(&mut self) {
        self.f_face = self.f_face.rotate_right(16);
        let temp = self.u_face.down_row();
        self.u_face
            .replace_side(self.l_face.right_col().rotate_right(16), u64::DOWN_ROW_MASK);
        self.l_face
            .replace_side(self.d_face.upper_row().rotate_right(16), u64::RIGHT_COL_MASK);
        self.d_face
            .replace_side(self.r_face.left_col().rotate_right(16), u64::UPPER_ROW_MASK);
        self.r_face
            .replace_side(temp.rotate_right(16), u64::LEFT_COL_MASK);
    }

    fn f_counter_clock(&mut self) {
        self.f_face = self.f_face.rotate_left(16);
        let temp = self.u_face.down_row();
        self.u_face
            .replace_side(self.r_face.left_col().rotate_left(16), u64::DOWN_ROW_MASK);
        self.r_face
            .replace_side(self.d_face.upper_row().rotate_left(16), u64::LEFT_COL_MASK);
        self.d_face
            .replace_side(self.l_face.right_col().rotate_left(16), u64::UPPER_ROW_MASK);
        self.l_face
            .replace_side(temp.rotate_left(16), u64::RIGHT_COL_MASK);
    }

    fn f_double(&mut self) {
        self.f_face = self.f_face.rotate_right(32);
        let temp = self.u_face.down_row();
        self.u_face
            .replace_side(self.d_face.upper_row().rotate_right(32), u64::DOWN_ROW_MASK);
        self.d_face
            .replace_side(temp.rotate_right(32), u64::UPPER_ROW_MASK);
        let temp = self.r_face.left_col();
        self.r_face
            .replace_side(self.l_face.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.l_face
            .replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn r_clock(&mut self) {
        self.r_face = self.r_face.rotate_right(16);
        let temp = self.u_face.right_col();
        self.u_face
            .replace_side(self.f_face.right_col(), u64::RIGHT_COL_MASK);
        self.f_face
            .replace_side(self.d_face.right_col(), u64::RIGHT_COL_MASK);
        self.d_face
            .replace_side(self.b_face.left_col().rotate_right(32), u64::RIGHT_COL_MASK);
        self.b_face
            .replace_side(temp.rotate_right(32), u64::LEFT_COL_MASK);
    }

    fn r_counter_clock(&mut self) {
        self.r_face = self.r_face.rotate_left(16);
        let temp = self.u_face.right_col();
        self.u_face
            .replace_side(self.b_face.left_col().rotate_right(32), u64::RIGHT_COL_MASK);
        self.b_face
            .replace_side(self.d_face.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.d_face
            .replace_side(self.f_face.right_col(), u64::RIGHT_COL_MASK);
        self.f_face.replace_side(temp, u64::RIGHT_COL_MASK);
    }

    fn r_double(&mut self) {
        self.r_face = self.r_face.rotate_right(32);
        let temp = self.u_face.right_col();
        self.u_face
            .replace_side(self.d_face.right_col(), u64::RIGHT_COL_MASK);
        self.d_face.replace_side(temp, u64::RIGHT_COL_MASK);
        let temp = self.b_face.left_col();
        self.b_face
            .replace_side(self.f_face.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.f_face
            .replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn b_clock(&mut self) {
        self.b_face = self.b_face.rotate_right(16);
        let temp = self.u_face.upper_row();
        self.u_face
            .replace_side(self.r_face.right_col().rotate_left(16), u64::UPPER_ROW_MASK);
        self.r_face
            .replace_side(self.d_face.down_row().rotate_left(16), u64::RIGHT_COL_MASK);
        self.d_face
            .replace_side(self.l_face.left_col().rotate_left(16), u64::DOWN_ROW_MASK);
        self.l_face
            .replace_side(temp.rotate_left(16), u64::LEFT_COL_MASK);
    }

    fn b_counter_clock(&mut self) {
        self.b_face = self.b_face.rotate_left(16);
        let temp = self.u_face.upper_row();
        self.u_face
            .replace_side(self.l_face.left_col().rotate_right(16), u64::UPPER_ROW_MASK);
        self.l_face
            .replace_side(self.d_face.down_row().rotate_right(16), u64::LEFT_COL_MASK);
        self.d_face
            .replace_side(self.r_face.right_col().rotate_right(16), u64::DOWN_ROW_MASK);
        self.r_face
            .replace_side(temp.rotate_right(16), u64::RIGHT_COL_MASK);
    }

    fn b_double(&mut self) {
        self.b_face = self.b_face.rotate_right(32);
        let temp = self.u_face.upper_row();
        self.u_face
            .replace_side(self.d_face.down_row().rotate_right(32), u64::UPPER_ROW_MASK);
        self.d_face
            .replace_side(temp.rotate_right(32), u64::DOWN_ROW_MASK);
        let temp = self.l_face.left_col();
        self.l_face
            .replace_side(self.r_face.right_col().rotate_right(32), u64::LEFT_COL_MASK);
        self.r_face
            .replace_side(temp.rotate_right(32), u64::RIGHT_COL_MASK);
    }

    fn d_clock(&mut self) {
        self.d_face = self.d_face.rotate_right(16);
        let temp = self.b_face.down_row();
        self.b_face
            .replace_side(self.r_face.down_row(), u64::DOWN_ROW_MASK);
        self.r_face
            .replace_side(self.f_face.down_row(), u64::DOWN_ROW_MASK);
        self.f_face
            .replace_side(self.l_face.down_row(), u64::DOWN_ROW_MASK);
        self.l_face.replace_side(temp, u64::DOWN_ROW_MASK);
    }

    fn d_counter_clock(&mut self) {
        self.d_face = self.d_face.rotate_left(16);
        let temp = self.b_face.down_row();
        self.b_face
            .replace_side(self.l_face.down_row(), u64::DOWN_ROW_MASK);
        self.l_face
            .replace_side(self.f_face.down_row(), u64::DOWN_ROW_MASK);
        self.f_face
            .replace_side(self.r_face.down_row(), u64::DOWN_ROW_MASK);
        self.r_face.replace_side(temp, u64::DOWN_ROW_MASK);
    }

    fn d_double(&mut self) {
        self.d_face = self.d_face.rotate_left(32);
        let temp = self.b_face.down_row();
        self.b_face
            .replace_side(self.f_face.down_row(), u64::DOWN_ROW_MASK);
        self.f_face.replace_side(temp, u64::DOWN_ROW_MASK);
        let temp = self.l_face.down_row();
        self.l_face
            .replace_side(self.r_face.down_row(), u64::DOWN_ROW_MASK);
        self.r_face.replace_side(temp, u64::DOWN_ROW_MASK);
    }
}

#[cfg(test)]
mod tests {
    use crate::cube::{enums::Color, traits::MoveCube, Face};

    use super::Cube;

    // To test left and right face movements
    fn new_ref_cube_front_turned_clockwise() -> Cube {
        Cube {
            u_face: Face::new(
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
            l_face: Face::new(
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
            f_face: Face::new_solved(Color::Red).0,
            r_face: Face::new(
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
            b_face: Face::new_solved(Color::Orange).0,
            d_face: Face::new(
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
            u_face: Face::new(
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
            l_face: Face::new_solved(Color::Blue).0,
            f_face: Face::new(
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
            r_face: Face::new_solved(Color::Green).0,
            b_face: Face::new(
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
            d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.u_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.u_counter_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.u_double();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_front_turned_clockwise();
        ref_cube.l_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_front_turned_clockwise();
        ref_cube.l_counter_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_front_turned_clockwise();
        ref_cube.l_double();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.f_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.f_counter_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.f_double();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_front_turned_clockwise();
        ref_cube.r_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_front_turned_clockwise();
        ref_cube.r_counter_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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
        let mut ref_cube = new_ref_cube_front_turned_clockwise();
        ref_cube.r_double();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
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
                l_face: Face::new(
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
                f_face: Face::new(
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
                r_face: Face::new(
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
                b_face: Face::new(
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
                d_face: Face::new(
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

    #[test]
    fn back_clockwise() {
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.b_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red
                )
                .0,
                l_face: Face::new(
                    Color::Red,
                    Color::Blue,
                    Color::Blue,
                    Color::Yellow,
                    Color::Blue,
                    Color::Yellow,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                f_face: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::Red,
                    Color::White
                )
                .0,
                r_face: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Orange,
                    Color::Green,
                    Color::White,
                    Color::Green,
                    Color::Green,
                    Color::White
                )
                .0,
                b_face: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Orange,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                d_face: Face::new(
                    Color::White,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::Orange,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue
                )
                .0,
            }
        )
    }

    #[test]
    fn back_counter_clockwise() {
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.b_counter_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red
                )
                .0,
                l_face: Face::new(
                    Color::White,
                    Color::Blue,
                    Color::Blue,
                    Color::White,
                    Color::Blue,
                    Color::Orange,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                f_face: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::Red,
                    Color::White
                )
                .0,
                r_face: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Green,
                    Color::Red
                )
                .0,
                b_face: Face::new(
                    Color::Orange,
                    Color::Orange,
                    Color::Orange,
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Yellow
                )
                .0,
                d_face: Face::new(
                    Color::White,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::Orange,
                    Color::Green,
                    Color::Green,
                    Color::Green
                )
                .0,
            }
        )
    }

    #[test]
    fn back_double() {
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.b_double();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
                    Color::Orange,
                    Color::White,
                    Color::White,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red
                )
                .0,
                l_face: Face::new(
                    Color::Green,
                    Color::Blue,
                    Color::Blue,
                    Color::Green,
                    Color::Blue,
                    Color::Green,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                f_face: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::Red,
                    Color::White
                )
                .0,
                r_face: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Blue,
                    Color::Green,
                    Color::Blue,
                    Color::Green,
                    Color::Green,
                    Color::Blue
                )
                .0,
                b_face: Face::new(
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow
                )
                .0,
                d_face: Face::new(
                    Color::White,
                    Color::White,
                    Color::Orange,
                    Color::White,
                    Color::Orange,
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow
                )
                .0,
            }
        )
    }

    #[test]
    fn down_clockwise() {
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.d_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red
                )
                .0,
                l_face: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                f_face: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::White,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                r_face: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Red,
                    Color::Red,
                    Color::White
                )
                .0,
                b_face: Face::new(
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Green,
                    Color::Green,
                    Color::Green
                )
                .0,
                d_face: Face::new(
                    Color::White,
                    Color::White,
                    Color::White,
                    Color::White,
                    Color::White,
                    Color::Orange,
                    Color::Orange,
                    Color::Orange
                )
                .0,
            }
        )
    }

    #[test]
    fn down_counter_clockwise() {
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.d_counter_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red
                )
                .0,
                l_face: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Red,
                    Color::Red,
                    Color::White
                )
                .0,
                f_face: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::White,
                    Color::Green,
                    Color::Green,
                    Color::Green
                )
                .0,
                r_face: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                b_face: Face::new(
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                d_face: Face::new(
                    Color::Orange,
                    Color::Orange,
                    Color::Orange,
                    Color::White,
                    Color::White,
                    Color::White,
                    Color::White,
                    Color::White
                )
                .0,
            }
        )
    }

    #[test]
    fn down_double() {
        let mut ref_cube = new_ref_cube_right_turned_clockwise();
        ref_cube.d_double();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Red,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Red
                )
                .0,
                l_face: Face::new(
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Green,
                    Color::Green,
                    Color::Green
                )
                .0,
                f_face: Face::new(
                    Color::Red,
                    Color::Red,
                    Color::White,
                    Color::Red,
                    Color::White,
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange
                )
                .0,
                r_face: Face::new(
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Green,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue
                )
                .0,
                b_face: Face::new(
                    Color::Yellow,
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow,
                    Color::Orange,
                    Color::Red,
                    Color::Red,
                    Color::White
                )
                .0,
                d_face: Face::new(
                    Color::Orange,
                    Color::White,
                    Color::White,
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
    fn all_moves_random_cube() {
        let mut ref_cube = Cube {
            u_face: Face::new(
                Color::Orange,
                Color::Red,
                Color::Orange,
                Color::Blue,
                Color::White,
                Color::Blue,
                Color::Red,
                Color::Green,
            )
            .0,
            l_face: Face::new(
                Color::White,
                Color::Yellow,
                Color::Red,
                Color::Green,
                Color::Green,
                Color::Orange,
                Color::White,
                Color::Blue,
            )
            .0,
            f_face: Face::new(
                Color::Yellow,
                Color::Yellow,
                Color::Red,
                Color::White,
                Color::Red,
                Color::Red,
                Color::Blue,
                Color::Blue,
            )
            .0,
            r_face: Face::new(
                Color::White,
                Color::Orange,
                Color::Yellow,
                Color::Blue,
                Color::Yellow,
                Color::Orange,
                Color::Orange,
                Color::Green,
            )
            .0,
            b_face: Face::new(
                Color::Green,
                Color::White,
                Color::Green,
                Color::Orange,
                Color::Red,
                Color::Red,
                Color::Green,
                Color::White,
            )
            .0,
            d_face: Face::new(
                Color::White,
                Color::Orange,
                Color::Yellow,
                Color::Blue,
                Color::Green,
                Color::Blue,
                Color::Yellow,
                Color::Yellow,
            )
            .0,
        };
        ref_cube.d_double();
        ref_cube.r_double();
        ref_cube.u_counter_clock();
        ref_cube.f_clock();
        ref_cube.l_double();
        ref_cube.b_counter_clock();
        ref_cube.r_clock();
        ref_cube.d_clock();
        ref_cube.b_double();
        ref_cube.r_counter_clock();
        ref_cube.f_double();
        ref_cube.u_double();
        ref_cube.l_clock();
        ref_cube.f_counter_clock();
        ref_cube.l_counter_clock();
        ref_cube.u_clock();
        ref_cube.b_clock();
        ref_cube.d_counter_clock();
        assert_eq!(
            ref_cube,
            Cube {
                u_face: Face::new(
                    Color::Red,
                    Color::Orange,
                    Color::Orange,
                    Color::Green,
                    Color::Red,
                    Color::Orange,
                    Color::Green,
                    Color::Yellow
                )
                .0,
                l_face: Face::new(
                    Color::White,
                    Color::Red,
                    Color::Blue,
                    Color::Blue,
                    Color::Blue,
                    Color::Green,
                    Color::Blue,
                    Color::Red
                )
                .0,
                f_face: Face::new(
                    Color::Yellow,
                    Color::Orange,
                    Color::Blue,
                    Color::Yellow,
                    Color::White,
                    Color::Yellow,
                    Color::Orange,
                    Color::White
                )
                .0,
                r_face: Face::new(
                    Color::Red,
                    Color::White,
                    Color::White,
                    Color::Green,
                    Color::Yellow,
                    Color::Green,
                    Color::Red,
                    Color::Green
                )
                .0,
                b_face: Face::new(
                    Color::Blue,
                    Color::Yellow,
                    Color::Blue,
                    Color::Green,
                    Color::White,
                    Color::White,
                    Color::Red,
                    Color::Orange
                )
                .0,
                d_face: Face::new(
                    Color::Green,
                    Color::White,
                    Color::Orange,
                    Color::Orange,
                    Color::Yellow,
                    Color::Yellow,
                    Color::Blue,
                    Color::Red
                )
                .0,
            }
        )
    }
}
