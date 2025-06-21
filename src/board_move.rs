use Castling::*;

use crate::err::*;
use crate::piece::{*, Color::*};
use crate::square::{*, Square::{*}};


const CASTLING_SQUARES: [[Square; 4]; 4] = [
    // king start, king end, rook start, rook end
    [E1, G1, H1, F1],
    [E1, C1, A1, D1],
    [E8, G8, H8, F8],
    [E8, C8, A8, D8],
];
const KING_START: usize = 0;
const KING_END: usize = 1;
const ROOK_START: usize = 2;
const ROOK_END: usize = 3;


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Castling {
    WhiteKingside = 0,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}

impl Castling {

    pub const ALL: [Castling; 4] = [
        WhiteKingside,
        WhiteQueenside,
        BlackKingside,
        BlackQueenside,
    ];

    pub const fn color(self) -> Color {
        const COLORS: [Color; 4] = [White, White, Black, Black];
        COLORS[self as usize]
    }

    pub const fn chr(self) -> char {
        const CASTLING_CHARS: [char; 4] = ['K', 'Q', 'k', 'q'];
        CASTLING_CHARS[self as usize]
    }

    pub const fn from_char(chr: char) -> Result<Castling, FenError> {
        match chr {
            'K' => Ok(WhiteKingside),
            'Q' => Ok(WhiteQueenside),
            'k' => Ok(BlackKingside),
            'q' => Ok(BlackQueenside),
            _ => Err(FenError::InvalidCastlingChar(chr))
        }
    }

    pub const fn king_start(self) -> Square {
        CASTLING_SQUARES[self as usize][KING_START]
    }

    pub const fn king_end(self) -> Square {
        CASTLING_SQUARES[self as usize][KING_END]
    }

    pub const fn rook_start(self) -> Square {
        CASTLING_SQUARES[self as usize][ROOK_START]
    }

    pub const fn rook_end(self) -> Square {
        CASTLING_SQUARES[self as usize][ROOK_END]
    }

    pub const fn king(self) -> ColoredPiece {
        Piece::King.as_color(self.color())
    }
}
