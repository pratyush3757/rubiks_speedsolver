#[derive(Debug)]
pub enum Color {
    Blank = 1,
    Yellow = 2,
    Blue = 4,
    Red = 8,
    Green = 16,
    Orange = 32,
    White = 64,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            2 => { Self::Yellow },
            4 => { Self::Blue },
            8 => { Self::Red },
            16 => { Self::Green },
            32 => { Self::Orange },
            64 => { Self::White },
            _ => {Self::Blank},
        }
    }
}

#[derive(Debug)]
pub enum EdgePosition {
    UF,
    UR,
    UB,
    UL,
    DF,
    DR,
    DB,
    DL,
    FR,
    FL,
    BR,
    BL,
}

#[derive(Debug)]
pub enum CornerPosition {
    UFR,
    URB,
    UBL,
    ULF,
    DRF,
    DFL,
    DLB,
    DBR,
}
