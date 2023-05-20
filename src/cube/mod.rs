mod enums;
mod traits;
mod movement;

use self::enums::{Color, CornerPosition, EdgePosition};
use self::traits::FaceBitMask;

#[derive(Debug, PartialEq)]
pub struct Cube {
    yellow: u64, // U
    blue: u64,   // L
    red: u64,    // F
    green: u64,  // R
    orange: u64, // B
    white: u64,  // D
}

impl FaceBitMask for Cube {}

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
            yellow: Face::new_solved(Color::Yellow).0,
            blue: Face::new_solved(Color::Blue).0,
            red: Face::new_solved(Color::Red).0,
            green: Face::new_solved(Color::Green).0,
            orange: Face::new_solved(Color::Orange).0,
            white: Face::new_solved(Color::White).0,
        }
    }

    pub fn get_corner_cubie(&self, position: &CornerPosition) -> Cubie {
        let (color_1, color_2, color_3) = match position {
            CornerPosition::UBL => (
                Self::upper_left(self.yellow),
                Self::upper_right(self.orange),
                Self::upper_left(self.blue),
            ),
            CornerPosition::UBR => (
                Self::upper_right(self.yellow),
                Self::upper_left(self.orange),
                Self::upper_right(self.green),
            ),
            CornerPosition::UFL => (
                Self::down_left(self.yellow),
                Self::upper_left(self.red),
                Self::upper_right(self.blue),
            ),
            CornerPosition::UFR => (
                Self::down_right(self.yellow),
                Self::upper_right(self.red),
                Self::upper_left(self.green),
            ),
            CornerPosition::DBL => (
                Self::down_left(self.white),
                Self::down_right(self.orange),
                Self::down_left(self.blue),
            ),
            CornerPosition::DBR => (
                Self::down_right(self.white),
                Self::down_left(self.orange),
                Self::down_right(self.green),
            ),
            CornerPosition::DFL => (
                Self::upper_left(self.white),
                Self::down_left(self.red),
                Self::down_right(self.blue),
            ),
            CornerPosition::DFR => (
                Self::upper_right(self.white),
                Self::down_right(self.red),
                Self::down_left(self.green),
            ),
            // _ => unreachable!(),
        };
        Cubie::new(color_1, color_2, color_3)
    }

    pub fn get_edge_cubie(&self, position: &EdgePosition) -> Cubie {
        let (color_1, color_2) = match position {
            EdgePosition::UB => (Self::upper(self.yellow), Self::upper(self.orange)),
            EdgePosition::UL => (Self::left(self.yellow), Self::upper(self.blue)),
            EdgePosition::UR => (Self::right(self.yellow), Self::upper(self.green)),
            EdgePosition::UF => (Self::down(self.yellow), Self::upper(self.red)),
            EdgePosition::BL => (Self::left(self.orange), Self::right(self.blue)),
            EdgePosition::BR => (Self::left(self.orange), Self::right(self.green)),
            EdgePosition::FL => (Self::right(self.red), Self::left(self.blue)),
            EdgePosition::FR => (Self::right(self.red), Self::left(self.green)),
            EdgePosition::DB => (Self::down(self.white), Self::down(self.orange)),
            EdgePosition::DL => (Self::left(self.white), Self::down(self.blue)),
            EdgePosition::DR => (Self::right(self.white), Self::down(self.green)),
            EdgePosition::DF => (Self::upper(self.white), Self::down(self.red)),
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
