use crate::bitmask;

use File::*;
use Rank::*;
use Diagonal::*;
use AntiDiagonal::*;
use Square::*;


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum File {
    FA = 0, FB, FC, FD, FE, FF, FG, FH,
}

impl File {
    
    pub fn mask(self) -> u64 {
        bitmask::FILE[self as usize]
    }

    pub const ALL: [File; 8] = [
        FA, FB, FC, FD, FE, FF, FG, FH
    ];
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    R1 = 0, R2, R3, R4, R5, R6, R7, R8,
}

impl Rank {

    pub fn mask(self) -> u64 {
        bitmask::RANK[self as usize]
    }
    
    pub const ALL: [Rank; 8] = [
        R1, R2, R3, R4, R5, R6, R7, R8
    ];
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diagonal {
    A8A8 = 0, A7B8, A6C8, A5D8, A4E8, A3F8, A2G8,
    A1H8, B1H7, C1H6, D1H5, E1H4, F1H3, G1H2, H1H1,
}

impl Diagonal {

    pub fn mask(self) -> u64 {
        bitmask::DIAGONAL[self as usize]
    }

    pub const ALL: [Diagonal; 15] = [
        A8A8, A7B8, A6C8, A5D8, A4E8, A3F8, A2G8, A1H8,
        B1H7, C1H6, D1H5, E1H4, F1H3, G1H2, H1H1
    ];
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntiDiagonal {
    A1A1 = 0, A2B1, A3C1, A4D1, A5E1, A6F1, A7G1, A8H1,
    B8H2, C8H3, D8H4, E8H5, F8H6, G8H7, H8H8
}

impl AntiDiagonal {

    pub fn mask(self) -> u64 {
        bitmask::ANTI_DIAGONAL[self as usize]
    }

    pub const ALL: [AntiDiagonal; 15] = [
        A1A1, A2B1, A3C1, A4D1, A5E1, A6F1, A7G1, A8H1,
        B8H2, C8H3, D8H4, E8H5, F8H6, G8H7, H8H8
    ];
}


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

    pub const fn rank(self) -> Rank {
        Rank::ALL[self as usize / 8]
    }

    pub const fn file(self) -> File {
        File::ALL[self as usize % 8]
    }

    pub const fn diagonal(self) -> Diagonal {
        Diagonal::ALL[7 - self.rank() as usize + self.file() as usize]
    }

    pub const fn antidiagonal(self) -> AntiDiagonal {
        AntiDiagonal::ALL[self.rank() as usize + self.file() as usize]
    }

    pub const fn mask(self) -> u64 {
        bitmask::SQUARE[self as usize]
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
}

