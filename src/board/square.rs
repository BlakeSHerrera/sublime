use {
    crate::hashing::bitmask,
    super::{
        color::*,
        line::*,
    },
    Square::*,
};


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    A1 = 0, B1, C1, D1, E1, F1, G1, H1, 
    A2, B2, C2, D2, E2, F2, G2, H2, 
    A3, B3, C3, D3, E3, F3, G3, H3, 
    A4, B4, C4, D4, E4, F4, G4, H4, 
    A5, B5, C5, D5, E5, F5, G5, H5, 
    A6, B6, C6, D6, E6, F6, G6, H6, 
    A7, B7, C7, D7, E7, F7, G7, H7, 
    A8, B8, C8, D8, E8, F8, G8, H8, 
}

impl Square {

    pub const BITS: u64 = 6;
    pub const BIT_MAX: u64 = 0b111111;

    pub const fn rank(self) -> Rank {
        Rank::ALL[self as usize / 8]
    }

    pub const fn file(self) -> File {
        File::ALL[self as usize % 8]
    }

    pub const fn row(self) -> usize {
        self as usize / 8
    }

    pub const fn col(self) -> usize {
        self as usize % 8
    }

    pub const fn diag(self) -> Diagonal {
        Diagonal::ALL[7 - self.rank() as usize + self.file() as usize]
    }

    pub const fn antidiag(self) -> AntiDiagonal {
        AntiDiagonal::ALL[self.rank() as usize + self.file() as usize]
    }

    pub const fn mask(self) -> u64 {
        bitmask::SQUARE[self as usize]
    }

    pub const fn color(self) -> Color {
        Color::ALL[(self.rank() as usize + self.file() as usize) & 1]
    }

    pub const ALL: [Square; 64] = [
        A1, B1, C1, D1, E1, F1, G1, H1, 
        A2, B2, C2, D2, E2, F2, G2, H2, 
        A3, B3, C3, D3, E3, F3, G3, H3, 
        A4, B4, C4, D4, E4, F4, G4, H4, 
        A5, B5, C5, D5, E5, F5, G5, H5, 
        A6, B6, C6, D6, E6, F6, G6, H6, 
        A7, B7, C7, D7, E7, F7, G7, H7, 
        A8, B8, C8, D8, E8, F8, G8, H8,
    ];

    pub const fn chrs(self) -> [char; 2] {
        [self.file().chr(), self.rank().chr()]
    }

    pub const fn from_file_rank(file: File, rank: Rank) -> Square {
        Square::from_rc(rank as usize, file as usize)
    }

    pub const fn from_rc(row: usize, col: usize) -> Square {
        Square::ALL[8 * row + col]
    }
}

