use {
    super::color::{*, Color::*},
    Piece::*,
    Moveset::*,
    GenericPiece::*,
};


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

    pub const BITS: u64 = 3;
    pub const BIT_MAX: u64 = 0b111;
    pub const EMPTY_CODE: u64 = GenericPiece::BIT_MAX;

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
    Rook = 0, Knight, Bishop, Queen
}

impl Promotion {

    pub const ALL: [Promotion; 4] = [
        Promotion::Rook, 
        Promotion::Knight, 
        Promotion::Bishop, 
        Promotion::Queen
    ];

    pub const BITS: u64 = 3;  // Extra bit for no promotion
    pub const BIT_MAX: u64 = 0b111;
    pub const EMPTY_CODE: u64 = Promotion::BIT_MAX;
    
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

    pub const BITS: u64 = 4;
    pub const BIT_MAX: u64 = 0b1111;
    pub const EMPTY_CODE: u64 = Piece::BIT_MAX;

    pub const ALL: [Piece; 12] = [
        WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhitePawn,
        BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackPawn,
    ];

    pub const WHITE_PIECES: [Piece; 6] = [
        WhiteRook, WhiteKnight, WhiteBishop, WhiteQueen, WhiteKing, WhitePawn,
    ];

    pub const BLACK_PIECES: [Piece; 6] = [
        BlackRook, BlackKnight, BlackBishop, BlackQueen, BlackKing, BlackPawn,
    ];

    pub const fn pieces_of_color(color: Color) -> [Piece; 6] {
        match color {
            White => Piece::WHITE_PIECES,
            Black => Piece::BLACK_PIECES,
        }
    }

    pub const fn color(self) -> Color {
        Color::PIECE_ARR[self as usize]
    }

    pub const fn inv(self) -> Piece {
        Piece::ALL[(self as usize + 6) % 12]
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
}
