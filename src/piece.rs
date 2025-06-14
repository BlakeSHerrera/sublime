
use Color::*;
use ColoredPiece::*;
use Moveset::*;
use Piece::*;


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0, White,
}

impl Color {

    pub const ALL: [Color; 2] = [Black, White];

    const FULL_ARR: [Color; 12] = [
        White, White, White, White, White, White,
        Black, Black, Black, Black, Black, Black,
    ];
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Moveset {
    RookMove, KnightMove, BishopMove, QueenMove, KingMove,
    // The pawns have different movement rules based on color.
    BlackPawnMove, WhitePawnMove,
}

impl Moveset {
    const FULL_ARR: [Moveset; 12] = [
        RookMove, KnightMove, BishopMove, QueenMove, KingMove, WhitePawnMove,
        RookMove, KnightMove, BishopMove, QueenMove, KingMove, BlackPawnMove,
    ];
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Rook = 0, Knight, Bishop, Queen, King, Pawn,
}

impl Piece {

    const FULL_ARR: [Piece; 12] = [
        Rook, Knight, Bishop, Queen, King, Pawn,
        Rook, Knight, Bishop, Queen, King, Pawn,
    ];

    pub fn as_color(self, color: Color) -> ColoredPiece {
        match color {
            White => ColoredPiece::ALL[self as usize],
            Black => ColoredPiece::ALL[6 + self as usize],
        }
    }
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Promotion {
    Rook, Knight, Bishop, Queen
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColoredPiece {
    WhiteRook = 0, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhitePawn,
    BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackPawn,
}

impl ColoredPiece {

    pub const ALL: [ColoredPiece; 12] = [
        WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhitePawn,
        BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackPawn,
    ];

    pub const fn get_color(self) -> Color {
        Color::FULL_ARR[self as usize]
    }

    pub const fn get_piece(self) -> Piece {
        Piece::FULL_ARR[self as usize]
    }

    pub const fn get_moveset(self) -> Moveset {
        Moveset::FULL_ARR[self as usize]
    }
}