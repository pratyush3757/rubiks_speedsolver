#![allow(clippy::upper_case_acronyms)]

#[derive(Debug, Clone, Copy)]
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
            2 => Self::Yellow,
            4 => Self::Blue,
            8 => Self::Red,
            16 => Self::Green,
            32 => Self::Orange,
            64 => Self::White,
            _ => Self::Blank,
        }
    }
}

#[derive(Debug)]
pub enum EdgePosition {
    UB,
    UL,
    UR,
    UF,
    BL,
    BR,
    FL,
    FR,
    DB,
    DL,
    DR,
    DF,
}

#[derive(Debug)]
pub enum CornerPosition {
    UBL,
    UBR,
    UFL,
    UFR,
    DBL,
    DBR,
    DFL,
    DFR,
}
