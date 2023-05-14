#[derive(Debug)]
enum Colors {
    Blank = 0,
    Red = 1,
    Orange = 2,
    Green = 3,
    Blue = 4,
    Yellow = 5,
    White = 6,
}

impl From<u8> for Colors {
    fn from(value: u8) -> Self {
        match value {
            2 => { Self::Red },
            4 => { Self::Orange },
            8 => { Self::Green },
            16 => { Self::Blue },
            32 => { Self::Yellow },
            64 => { Self::White },
            _ => {Self::Blank},
        }
    }
}

trait FaceBitMask {
    // const upper_left: u64 = 0xff_00_00_00_00_00_00_00;
    // const U: u64 = 0x00_ff_00_00_00_00_00_00;
    // const upper_right: u64 = 0x00_00_ff_00_00_00_00_00;
    // const L: u64 = 0x00_00_00_ff_00_00_00_00;
    // const R: u64 = 0x00_00_00_00_ff_00_00_00;
    // const down_left: u64 = 0x00_00_00_00_00_ff_00_00;
    // const D: u64 = 0x00_00_00_00_00_00_ff_00;
    // const down_right: u64 = 0x00_00_00_00_00_00_00_ff;

    fn upper_left(x: u64) -> u8 { ((x >> 56) & 0xff_u64) as u8 }
    fn upper(x: u64) -> u8 { ((x >> 48) & 0xff_u64) as u8 }
    fn upper_right(x: u64) -> u8 { ((x >> 40) & 0xff_u64) as u8 }
    fn left(x: u64) -> u8 { ((x >> 32) & 0xff_u64) as u8 }
    fn right(x: u64) -> u8 { ((x >> 24) & 0xff_u64) as u8 }
    fn down_left(x: u64) -> u8 { ((x >> 16) & 0xff_u64) as u8 }
    fn down(x: u64) -> u8 { ((x >> 8) & 0xff_u64) as u8 }
    fn down_right(x: u64) -> u8 { (x & 0xff_u64) as u8 }
}

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
