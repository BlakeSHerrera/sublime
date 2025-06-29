use crate::board::{
    color::{*, Color::*},
    line::*,
    piece::{*, Piece::*}, 
    square::Square,
    zone::{*, Quadrant::*},
};

use ConversionError::*;


#[derive(Debug)]
pub enum ConversionError {
    InvalidColor(char),
    InvalidRank(char),
    InvalidFile(char),
    InvalidPiece(char),
    InvalidCastling(char),
    IncompleteSquare,
}


impl Color {
    
    pub const fn chr(self) -> char {
        const CHARS: [char; 2] = ['w', 'b'];
        CHARS[self as usize]
    }

    pub const fn from_chr(chr: char) -> Result<Color, ConversionError> {
        match chr {
            'w' => Ok(White),
            'b' => Ok(Black),
            _ => Err(InvalidColor(chr)),
        }
    }
}


impl File {

    pub const fn chr(self) -> char {
        const CHARS: [char; 8] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'
        ];
        CHARS[self as usize]
    }

    pub const fn from_chr(chr: char) -> Result<File, ConversionError> {
        match 'a' <= chr && chr <= 'h' {
            true => Ok(File::ALL[(chr as u8 - 'a' as u8) as usize]),
            false => Err(InvalidFile(chr)),
        }
    }
}


impl Rank {
    
    pub const fn chr(self) -> char {
        const CHARS: [char; 8] = [
            '1', '2', '3', '4', '5', '6', '7', '8'
        ];
        CHARS[self as usize]
    }

    pub const fn from_chr(chr: char) -> Result<Rank, ConversionError> {
        match '1' <= chr && chr <= '8' {
            true => Ok(Rank::ALL[(chr as u8 - '1' as u8) as usize]),
            false => Err(InvalidRank(chr)),
        }
    }
}



impl Square {
    pub const fn from_chrs(file: char, rank: char) -> Result<Square, ConversionError> {
        match (Rank::from_chr(rank), File::from_chr(file)) {
            (Ok(r), Ok(f)) => Ok(Square::from_file_rank(f, r)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}


impl Piece {
    
    pub const fn chr(self) -> char {
        const CHARS: [char; 12] = [
            'R', 'N', 'B', 'Q', 'K', 'P',
            'r', 'n', 'b', 'q', 'k', 'p',
        ];
        CHARS[self as usize]
    }

    pub const fn from_char(chr: char) -> Result<Self, ConversionError> {
        match chr {
            'R' => Ok(WhiteRook),
            'N' => Ok(WhiteKnight),
            'B' => Ok(WhiteBishop),
            'Q' => Ok(WhiteQueen),
            'K' => Ok(WhiteKing),
            'P' => Ok(WhitePawn),
            'r' => Ok(BlackRook),
            'n' => Ok(BlackKnight),
            'b' => Ok(BlackBishop),
            'q' => Ok(BlackQueen),
            'k' => Ok(BlackKing),
            'p' => Ok(BlackPawn),
            _ => Err(InvalidPiece(chr)),
        }
    }
}


impl Quadrant {

    pub const fn chr(self) -> char {
        const CASTLING_CHARS: [char; 4] = ['K', 'Q', 'k', 'q'];
        CASTLING_CHARS[self as usize]
    }

    pub const fn from_char(chr: char) -> Result<Quadrant, ConversionError> {
        match chr {
            'K' => Ok(WhiteKingside),
            'Q' => Ok(WhiteQueenside),
            'k' => Ok(BlackKingside),
            'q' => Ok(BlackQueenside),
            _ => Err(InvalidCastling(chr))
        }
    }
}