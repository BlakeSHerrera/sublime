use crate::color::{Color, Color::*};
use crate::err::*;

use Piece::*;
use Moveset::*;
use GenericPiece::*;



#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Moveset {
    RookMove, KnightMove, BishopMove, QueenMove, KingMove,
    BlackPawnMove, WhitePawnMove,
}

impl Moveset {
    const PIECE_ARR: [Moveset; 12] = [
        RookMove, KnightMove, BishopMove, QueenMove, KingMove, WhitePawnMove,
        RookMove, KnightMove, BishopMove, QueenMove, KingMove, BlackPawnMove,
    ];
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericPiece {
    Rook = 0, Knight, Bishop, Queen, King, Pawn,
}

impl GenericPiece {

    const ALL: [GenericPiece; 6] = [Rook, Knight, Bishop, Queen, King, Pawn];

    const PIECE_ARR: [GenericPiece; 12] = [
        Rook, Knight, Bishop, Queen, King, Pawn,
        Rook, Knight, Bishop, Queen, King, Pawn,
    ];

    pub const fn as_color(self, color: Color) -> Piece {
        match color {
            White => Piece::ALL[self as usize],
            Black => Piece::ALL[6 + self as usize],
        }
    }
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Promotion {
    Rook = 1, Knight, Bishop, Queen
}

impl Promotion {
    
    pub const fn as_generic_piece(self) -> GenericPiece {
        GenericPiece::ALL[self as usize]
    }

    pub const fn as_piece(self, color: Color) -> Piece {
        self.as_generic_piece().as_color(color)
    }
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    WhiteRook = 0, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhitePawn,
    BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackPawn,
}

impl Piece {

    pub const ALL: [Piece; 12] = [
        WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhitePawn,
        BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackPawn,
    ];

    pub const fn color(self) -> Color {
        Color::PIECE_ARR[self as usize]
    }

    pub const fn occ_index(self) -> usize {
        Piece::ALL.len() + self.color() as usize
    }

    pub const fn as_generic(self) -> GenericPiece {
        GenericPiece::PIECE_ARR[self as usize]
    }

    pub const fn moveset(self) -> Moveset {
        Moveset::PIECE_ARR[self as usize]
    }

    pub const fn chr(self) -> char {
        const CHARS: [char; 12] = [
            'R', 'N', 'B', 'Q', 'K', 'P',
            'r', 'n', 'b', 'q', 'k', 'p',
        ];
        CHARS[self as usize]
    }

    pub const fn from_char(chr: char) -> Result<Self, FenError> {
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
            _ => Err(FenError::InvalidPiece(chr)),
        }
    }
}
