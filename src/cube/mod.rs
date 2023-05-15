mod colors;
mod traits;

use traits::FaceBitMask;
use colors::Color;

use colors::{CornerPosition, EdgePosition};

#[derive(Debug)]
pub struct Cube {
    Yellow: u64,    // U
    Blue: u64,      // L
    Red: u64,       // F
    Green: u64,     // R
    Orange: u64,    // B
    White: u64,     // D
}

impl FaceBitMask for Cube {}

impl Cube {
    pub fn new() -> Self {
        /* let mut color: u64 = 0;
        for i in 0..8 {
            color |= Colors::Yellow << (i*8);
        } */
        
        // The numbers represent solved faces
        Cube {
            Yellow: 0x02_02_02_02_02_02_02_02u64, // 144680345676153346
            Blue: 0x04_04_04_04_04_04_04_04u64, // 289360691352306692
            Red: 0x08_08_08_08_08_08_08_08u64, // 578721382704613384
            Green: 0x10_10_10_10_10_10_10_10u64, // 1157442765409226768
            Orange: 0x20_20_20_20_20_20_20_20u64, // 2314885530818453536
            White: 0x40_40_40_40_40_40_40_40u64, // 4629771061636907072
        }
    }

    pub fn get_corner_cubie(&self, position: CornerPosition) -> Cubie {
        let (color_1, color_2, color_3) = match position {
            UBL => { (Self::upper_left(self.Yellow), Self::upper_right(self.Orange), Self::upper_left(self.Blue)) },
            UBR => { (Self::upper_right(self.Yellow), Self::upper_left(self.Orange), Self::upper_right(self.Green)) },
            UFL => { (Self::down_left(self.Yellow), Self::upper_left(self.Red), Self::upper_right(self.Blue)) },
            UFR => { (Self::down_right(self.Yellow), Self::upper_right(self.Red), Self::upper_left(self.Green)) },
            DBL => { (Self::down_left(self.White), Self::down_right(self.Orange), Self::down_left(self.Blue)) },
            DBR => { (Self::down_right(self.White), Self::down_left(self.Orange), Self::down_right(self.Green)) },
            DFL => { (Self::upper_left(self.White), Self::down_left(self.Red), Self::down_right(self.Blue)) },
            DFR => { (Self::upper_right(self.White), Self::down_right(self.Red), Self::down_left(self.Green)) },
            // _ => unreachable!(),
        };
        Cubie::new(color_1, color_2, color_3)
    }
    
    pub fn get_edge_cubie(&self, position: EdgePosition) -> Cubie {
        let (color_1, color_2) = match position {
            UB => { (Self::upper(self.Yellow), Self::upper(self.Orange)) },
            UL => { (Self::left(self.Yellow), Self::upper(self.Blue)) },
            UR => { (Self::right(self.Yellow), Self::upper(self.Green)) },
            UF => { (Self::down(self.Yellow), Self::upper(self.Red)) },
            BL => { (Self::left(self.Orange), Self::right(self.Blue)) },
            BR => { (Self::left(self.Orange), Self::right(self.Green)) },
            FL => { (Self::right(self.Red), Self::left(self.Blue)) },
            FR => { (Self::right(self.Red), Self::left(self.Green)) },
            DB => { (Self::down(self.White), Self::down(self.Orange)) },
            DL => { (Self::left(self.White), Self::down(self.Blue)) },
            DR => { (Self::right(self.White), Self::down(self.Green)) },
            DF => { (Self::upper(self.White), Self::down(self.Red)) },
            // _ => unreachable!(),
        };
        Cubie::new(color_1, color_2, 0)
    }
}

#[derive(Debug)]
struct Face(u64);

impl Face {
    fn new(facelet_1: Color, facelet_2: Color, facelet_3: Color, facelet_4: Color, facelet_5: Color, facelet_6: Color, facelet_7: Color, facelet_8: Color) -> Self {
       Face(0)
    }
}

#[derive(Debug)]
pub struct Cubie {
    colors: u8,
}

impl Cubie {
    fn new(color_1: u8, color_2: u8, color_3: u8) -> Self {
        Cubie { colors: color_1 | color_2 | color_3 }
    }
}

pub fn find_cubie(cubie: Cubie) -> u8 {
    unimplemented!();
}
