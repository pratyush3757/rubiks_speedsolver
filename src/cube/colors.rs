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

