mod enums;
mod movement;
mod traits;

use self::enums::{Color, CornerPosition, EdgePosition};
use self::traits::FaceBitMask;

#[derive(Debug, PartialEq)]
pub struct Cube {
    u_face: u64,    // yellow
    l_face: u64,    // blue
    f_face: u64,    // red
    r_face: u64,    // green
    b_face: u64,    // orange
    d_face: u64,    // white
}

impl Cube {
    pub fn new() -> Self {
        /* let mut color: u64 = 0;
        for i in 0..8 {
            color |= Color::Yellow << (i*8);
        } */

        // The numbers represent solved faces
        Cube {
            /* yellow: 0x02_02_02_02_02_02_02_02u64, // 144680345676153346
            blue: 0x04_04_04_04_04_04_04_04u64, // 289360691352306692
            red: 0x08_08_08_08_08_08_08_08u64, // 578721382704613384
            green: 0x10_10_10_10_10_10_10_10u64, // 1157442765409226768
            orange: 0x20_20_20_20_20_20_20_20u64, // 2314885530818453536
            white: 0x40_40_40_40_40_40_40_40u64, // 4629771061636907072 */
            u_face: Face::new_solved(Color::Yellow).0,
            l_face: Face::new_solved(Color::Blue).0,
            f_face: Face::new_solved(Color::Red).0,
            r_face: Face::new_solved(Color::Green).0,
            b_face: Face::new_solved(Color::Orange).0,
            d_face: Face::new_solved(Color::White).0,
        }
    }

    pub fn get_corner_cubie(&self, position: &CornerPosition) -> Cubie {
        let (color_1, color_2, color_3) = match position {
            CornerPosition::UBL => (
                self.u_face.upper_left(),
                self.b_face.upper_right(),
                self.l_face.upper_left(),
            ),
            CornerPosition::UBR => (
                self.u_face.upper_right(),
                self.b_face.upper_left(),
                self.r_face.upper_right(),
            ),
            CornerPosition::UFL => (
                self.u_face.down_left(),
                self.f_face.upper_left(),
                self.l_face.upper_right(),
            ),
            CornerPosition::UFR => (
                self.u_face.down_right(),
                self.f_face.upper_right(),
                self.r_face.upper_left(),
            ),
            CornerPosition::DBL => (
                self.d_face.down_left(),
                self.b_face.down_right(),
                self.l_face.down_left(),
            ),
            CornerPosition::DBR => (
                self.d_face.down_right(),
                self.b_face.down_left(),
                self.r_face.down_right(),
            ),
            CornerPosition::DFL => (
                self.d_face.upper_left(),
                self.f_face.down_left(),
                self.l_face.down_right(),
            ),
            CornerPosition::DFR => (
                self.d_face.upper_right(),
                self.f_face.down_right(),
                self.r_face.down_left(),
            ),
            // _ => unreachable!(),
        };
        Cubie::new(color_1, color_2, color_3)
    }

    pub fn get_edge_cubie(&self, position: &EdgePosition) -> Cubie {
        let (color_1, color_2) = match position {
            EdgePosition::UB => (self.u_face.upper(), self.b_face.upper()),
            EdgePosition::UL => (self.u_face.left(), self.l_face.upper()),
            EdgePosition::UR => (self.u_face.right(), self.r_face.upper()),
            EdgePosition::UF => (self.u_face.down(), self.f_face.upper()),
            EdgePosition::BL => (self.b_face.left(), self.l_face.right()),
            EdgePosition::BR => (self.b_face.left(), self.r_face.right()),
            EdgePosition::FL => (self.f_face.right(), self.l_face.left()),
            EdgePosition::FR => (self.f_face.right(), self.r_face.left()),
            EdgePosition::DB => (self.d_face.down(), self.b_face.down()),
            EdgePosition::DL => (self.d_face.left(), self.l_face.down()),
            EdgePosition::DR => (self.d_face.right(), self.r_face.down()),
            EdgePosition::DF => (self.d_face.upper(), self.f_face.down()),
            // _ => unreachable!(),
        };
        Cubie::new(color_1, color_2, 0)
    }
}

#[derive(Debug)]
struct Face(u64);

impl Face {
    #![allow(clippy::too_many_arguments)]
    fn new(
        facelet_1: Color,
        facelet_2: Color,
        facelet_3: Color,
        facelet_4: Color,
        facelet_5: Color,
        facelet_6: Color,
        facelet_7: Color,
        facelet_8: Color,
    ) -> Self {
        Face(
            (facelet_1 as u64) << 56
                | (facelet_2 as u64) << 48
                | (facelet_3 as u64) << 40
                | (facelet_5 as u64) << 32
                | (facelet_8 as u64) << 24
                | (facelet_7 as u64) << 16
                | (facelet_6 as u64) << 8
                | (facelet_4 as u64),
        )
    }

    fn new_solved(facelet: Color) -> Self {
        Self::new(
            facelet, facelet, facelet, facelet, facelet, facelet, facelet, facelet,
        )
    }
}

#[derive(Debug)]
pub struct Cubie {
    colors: u8,
}

impl Cubie {
    fn new(color_1: u8, color_2: u8, color_3: u8) -> Self {
        Cubie {
            colors: color_1 | color_2 | color_3,
        }
    }
}

pub fn find_cubie(cubie: &Cubie) -> u8 {
    unimplemented!();
}
