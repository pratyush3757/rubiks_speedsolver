mod colors;
mod traits;

use traits::FaceBitMask;

#[derive(Debug)]
pub struct Cube {
    Red: u64,
    Orange: u64,
    Green: u64,
    Blue: u64,
    Yellow: u64,
    White: u64,
}

impl FaceBitMask for Cube {}

impl Cube {
    pub fn new() -> Self {
        /* let mut red: u64 = 0;
        for i in 0..8 {
            red |= 1 << ((i*8) + Colors::Red as u8);
        } */
        
        // The numbers represent solved faces
        Cube {
            Red: 0x02_02_02_02_02_02_02_02u64, // 144680345676153346
            Orange: 0x04_04_04_04_04_04_04_04u64, // 289360691352306692
            Green: 0x08_08_08_08_08_08_08_08u64, // 578721382704613384
            Blue: 0x10_10_10_10_10_10_10_10u64, // 1157442765409226768
            Yellow: 0x20_20_20_20_20_20_20_20u64, // 2314885530818453536
            White: 0x40_40_40_40_40_40_40_40u64, // 4629771061636907072
        }
    }

    pub fn get_corner_cubie(&self, position: u8) -> Cubie {
        let (color_1, color_2, color_3) = match position {
            1 => { (Self::upper_left(self.Yellow), Self::upper_right(self.Orange), Self::upper_left(self.Blue)) },
            3 => { (Self::upper_right(self.Yellow), Self::upper_left(self.Orange), Self::upper_right(self.Green)) },
            6 => { (Self::down_left(self.Yellow), Self::upper_left(self.Red), Self::upper_right(self.Blue)) },
            8 => { (Self::down_right(self.Yellow), Self::upper_right(self.Red), Self::upper_left(self.Green)) },
            13 => { (Self::down_left(self.White), Self::down_right(self.Orange), Self::down_left(self.Blue)) },
            15 => { (Self::down_right(self.White), Self::down_left(self.Orange), Self::down_right(self.Green)) },
            18 => { (Self::upper_left(self.White), Self::down_left(self.Red), Self::down_right(self.Blue)) },
            20 => { (Self::upper_right(self.White), Self::down_right(self.Red), Self::down_left(self.Green)) },
            _ => unreachable!(),
        };
        Cubie::new(color_1, color_2, color_3)
    }
    
    pub fn get_edge_cubie(&self, position: u8) -> Cubie {
        let (color_1, color_2) = match position {
            2 => { (Self::upper(self.Yellow), Self::upper(self.Orange)) },
            4 => { (Self::left(self.Yellow), Self::upper(self.Blue)) },
            5 => { (Self::right(self.Yellow), Self::upper(self.Green)) },
            7 => { (Self::down(self.Yellow), Self::upper(self.Red)) },
            9 => { (Self::left(self.Blue), Self::right(self.Orange)) },
            10 => { (Self::left(self.Orange), Self::right(self.Green)) },
            11 => { (Self::right(self.Blue), Self::left(self.Red)) },
            12 => { (Self::right(self.Red), Self::left(self.Green)) },
            14 => { (Self::down(self.White), Self::down(self.Orange)) },
            16 => { (Self::left(self.White), Self::down(self.Blue)) },
            17 => { (Self::right(self.White), Self::down(self.Green)) },
            19 => { (Self::upper(self.White), Self::down(self.Red)) },
            _ => unreachable!(),
        };
        Cubie::new(color_1, color_2, 0)
    }
}

#[derive(Debug)]
pub struct Cubie {
    colors: u8,
}

impl Cubie {
    fn new(color_1: u8, color_2: u8, color_3: u8) -> Self {
        let mut colors = 0u8;
        colors |= 1 << color_1;
        colors |= 1 << color_2;
        colors |= 1 << color_3;
        Cubie { colors }
    }
}

pub fn find_cubie(cubie: Cubie) -> u8 {
    unimplemented!();
}
