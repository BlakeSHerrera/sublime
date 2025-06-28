use crate::bitmask;
use crate::color::{Color, Color::*};
use crate::err::*;

use File::*;
use Rank::*;
use Diagonal::*;
use AntiDiagonal::*;
use Square::*;



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

    pub const fn chr(self) -> char {
        const CHARS: [char; 8] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'
        ];
        CHARS[self as usize]
    }

    pub const fn from_chr(chr: char) -> Result<File, CoordinateError> {
        match 'a' <= chr && chr <= 'h' {
            true => Ok(File::ALL[(chr as u8 - 'a' as u8) as usize]),
            false => Err(CoordinateError::InvalidFile(chr)),
        }
    }

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

    pub const fn chr(self) -> char {
        const CHARS: [char; 8] = [
            '1', '2', '3', '4', '5', '6', '7', '8'
        ];
        CHARS[self as usize]
    }

    pub const fn from_chr(chr: char) -> Result<Rank, CoordinateError> {
        match '1' <= chr && chr <= '8' {
            true => Ok(Rank::ALL[(chr as u8 - '1' as u8) as usize]),
            false => Err(CoordinateError::InvalidRank(chr)),
        }
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

    pub const fn from_chrs(rank: char, file: char) -> Result<Square, CoordinateError> {
        match (Rank::from_chr(rank), File::from_chr(file)) {
            (Ok(r), Ok(f)) => Ok(Square::from_file_rank(f, r)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}

