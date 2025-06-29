use {
    crate::hashing::bitmask,
    super::{
        color::{*, Color::*},
        square::{*, Square::*},
    },
    AntiDiagonal::*,
    Diagonal::*,
    File::*,
    Rank::*,
};


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntiDiagonal {
    A1A1 = 0, A2B1, A3C1, A4D1, A5E1, A6F1, A7G1, A8H1,
    B8H2, C8H3, D8H4, E8H5, F8H6, G8H7, H8H8
}

impl AntiDiagonal {

    pub const fn mask(self) -> u64 {
        bitmask::ANTI_DIAGONAL[self as usize]
    }

    pub const ALL: [AntiDiagonal; 15] = [
        A1A1, A2B1, A3C1, A4D1, A5E1, A6F1, A7G1, A8H1,
        B8H2, C8H3, D8H4, E8H5, F8H6, G8H7, H8H8
    ];

    pub const fn ends(self) -> [Square; 2] {
        const STARTS: [Square; 15] = [
            A1, A2, A3, A4, A5, A6, A7, A8,
            B8, C8, D8, E8, F8, G8, H8,
        ];
        const ENDS: [Square; 15] = [
            A1, B1, C1, D1, E1, F1, G1, H1,
            H2, H3, H4, H5, H6, H7, H8,
        ];
        [STARTS[self as usize], ENDS[self as usize]]
    }
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diagonal {
    A8A8 = 0, A7B8, A6C8, A5D8, A4E8, A3F8, A2G8,
    A1H8, B1H7, C1H6, D1H5, E1H4, F1H3, G1H2, H1H1,
}

impl Diagonal {

    pub const fn mask(self) -> u64 {
        bitmask::DIAGONAL[self as usize]
    }

    pub const ALL: [Diagonal; 15] = [
        A8A8, A7B8, A6C8, A5D8, A4E8, A3F8, A2G8, A1H8,
        B1H7, C1H6, D1H5, E1H4, F1H3, G1H2, H1H1
    ];

    pub const fn ends(self) -> [Square; 2] {
       const STARTS: [Square; 15] = [
            A8, A7, A6, A5, A4, A3, A2, A1, 
            B1, C1, D1, E1, F1, G1, H1
        ];
        const ENDS: [Square; 15] = [
            A8, B8, C8, D8, E8, F8, G8, H8, 
            H7, H6, H5, H4, H3, H2, H1
        ];
        [STARTS[self as usize], ENDS[self as usize]]
    }
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum File {
    AFile = 0, BFile, CFile, DFile, EFile, FFile, GFile, HFile,
}

impl File {

    pub const BITS: u64 = 3;
    pub const BIT_MAX: u64 = 0b111;
    
    pub const fn mask(self) -> u64 {
        bitmask::FILE[self as usize]
    }

    pub const fn offset(self) -> usize {
        self as usize
    }

    pub const ALL: [File; 8] = [
        AFile, BFile, CFile, DFile, EFile, FFile, GFile, HFile
    ];

    pub const fn ep_square(self, color: Color) -> Square {
        Square::from_file_rank(
            self, 
            match color {
                White => Rank3, 
                Black => Rank6
            })
    }
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Rank1 = 0, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8,
}

impl Rank {

    pub const BITS: u64 = 3;
    pub const BIT_MAX: u64 = 0b111;

    pub const fn mask(self) -> u64 {
        bitmask::RANK[self as usize]
    }

    pub const fn offset(self) -> usize {
        8 * self as usize
    }
    
    pub const ALL: [Rank; 8] = [
        Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8
    ];
}
