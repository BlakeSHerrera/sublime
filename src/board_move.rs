use Castling::*;

use crate::err::*;


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

    pub const fn get_char(self) -> char {
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
}
