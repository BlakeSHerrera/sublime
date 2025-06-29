use {
    crate::hashing::bitmask,
    super::{
        color::{*, Color::*},
        piece::*,
        square::{*, Square::*},
    },
    Quadrant::*,
    FileSide::*,
    RankSide::*,
};


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quadrant {
    WhiteKingside = 0,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}

impl Quadrant {

    pub const BITS: u64 = 2;

    pub const ALL: [Quadrant; 4] = [
        WhiteKingside,
        WhiteQueenside,
        BlackKingside,
        BlackQueenside,
    ];

    pub const KINGSIDE: [Quadrant; 2] = [
        WhiteKingside,
        BlackKingside,
    ];

    pub const QUEENSIDE: [Quadrant; 2] = [
        WhiteQueenside,
        BlackQueenside,
    ];

    pub const fn color(self) -> Color {
        const COLORS: [Color; 4] = [White, White, Black, Black];
        COLORS[self as usize]
    }

    pub const fn king_start(self) -> Square {
        const SQUARES: [Square; 4] = [E1, E1, E8, E8];
        SQUARES[self as usize]
    }

    pub const fn king_cross(self) -> Square {
        const SQUARES: [Square; 4] = [F1, D1, F8, D8];
        SQUARES[self as usize]
    }

    pub const fn king_end(self) -> Square {
        const SQUARES: [Square; 4] = [G1, C1, G8, C8];
        SQUARES[self as usize]
    }

    pub const fn rook_start(self) -> Square {
        const SQUARES: [Square; 4] = [H1, A1, H8, A8];
        SQUARES[self as usize]
    }

    pub const fn rook_end(self) -> Square {
        const SQUARES: [Square; 4] = [F1, D1, F8, D8];
        SQUARES[self as usize]
    }

    pub const fn king(self) -> Piece {
        GenericPiece::King.as_color(self.color())
    }

    pub const fn to_clear_mask(self) -> u64 {
        bitmask::CASTLING_TO_CLEAR[self as usize]
    }

    pub const fn no_attack_mask(self) -> u64 {
        bitmask::CASTLING_NO_ATTACK[self as usize]
    }
}


#[repr(u8)]
pub enum FileSide {
    Kingside = 0,
    Queenside,
}

impl FileSide {
    pub const fn to_quadrant(self, color: Color) -> Quadrant {
        Quadrant::ALL[(color as usize) << 1 | self as usize]
    }
}


#[repr(u8)]
pub enum RankSide {
    WhiteSide = 0,
    BlackSide,
}
