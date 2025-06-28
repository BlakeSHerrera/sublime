use crate::color::{*, Color::*};
use crate::err::*;
use crate::piece::{*, GenericPiece::*};
use crate::position::*;
use crate::square::{*, Rank::*, File::*, Square::*};

use Castling::*;


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

    pub const BITS: u64 = 3;  // TODO rename to MOVE_BITS
    pub const BIT_MAX: u64 = 0b111;
    pub const EMPTY_CODE: u64 = Castling::BIT_MAX;

    pub const ALL: [Castling; 4] = [
        WhiteKingside,
        WhiteQueenside,
        BlackKingside,
        BlackQueenside,
    ];

    pub const KINGSIDE: [Castling; 2] = [
        WhiteKingside,
        BlackKingside,
    ];

    pub const QUEENSIDE: [Castling; 2] = [
        WhiteQueenside,
        BlackQueenside,
    ];

    pub const fn code(self) -> u64 {
        1 << IS_CASTLING_OFFSET | (self as u64) << CASTLING_OPTION_OFFSET
    }

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

    pub const fn king(self) -> Piece {
        GenericPiece::King.as_color(self.color())
    }
}


const ORIGIN_SQUARE_OFFSET: u64 = 0;
const ORIGIN_PIECE_OFFSET: u64 = ORIGIN_SQUARE_OFFSET + Square::BITS;
const DESTINATION_SQUARE_OFFSET: u64 = ORIGIN_PIECE_OFFSET + Piece::BITS;
const DESTINATION_PIECE_OFFSET: u64 = DESTINATION_SQUARE_OFFSET + Square::BITS;  // Covers promotion
const CAPTURED_SQUARE_OFFSET: u64 = DESTINATION_PIECE_OFFSET + Piece::BITS;
const CAPTURED_PIECE_OFFSET: u64 = CAPTURED_SQUARE_OFFSET + Square::BITS;
const DOUBLE_PAWN_PUSH_OFFSET: u64 = CAPTURED_PIECE_OFFSET + Piece::BITS;
const EP_FILE_OFFSET: u64 = DOUBLE_PAWN_PUSH_OFFSET + 1;
const EP_CAPTURE_OFFSET: u64 = EP_FILE_OFFSET + File::BITS;
const IS_CASTLING_OFFSET: u64 = EP_CAPTURE_OFFSET + 1;
const CASTLING_OPTION_OFFSET: u64 = IS_CASTLING_OFFSET + 1;

const BITS_TO_SPARE: u64 = 64 - CASTLING_OPTION_OFFSET - 2;


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Move(pub u64);

impl Move {

    pub const fn new(
        board: GameState, 
        origin: Square, 
        destination: Square, 
        promotion: Option<Promotion>,
    ) -> Self {
        let moved = board.piece_at(origin);
        let captured = board.piece_at(destination);
        
        let mut mv: u64 = (origin as u64)  << ORIGIN_SQUARE_OFFSET
            | match moved {
                None => Piece::EMPTY_CODE,
                Some(p) => p as u64,
            } << ORIGIN_PIECE_OFFSET
            
            | (destination as u64) << DESTINATION_SQUARE_OFFSET
            | match promotion {
                None => Piece::EMPTY_CODE,
                Some(p) => p.as_piece(board.turn()) as u64,
            } << DESTINATION_PIECE_OFFSET;
        
        match [moved, captured] {
            [Some(p), None] => match p.as_generic() {
                Pawn => match origin.file() as u8 == destination.file() as u8 {
                    true => // En passant
                        mv |= (p.inv() as u64) << CAPTURED_PIECE_OFFSET
                            | match p.color() {
                                White => destination as u64 - 8,
                                Black => destination as u64 + 8,
                            } << CAPTURED_SQUARE_OFFSET
                            | 1 << EP_CAPTURE_OFFSET,
                    false => match [origin.rank(), destination.rank()] {
                        [Rank2, Rank4] | [Rank7, Rank5] => 
                            mv |= 1 << DOUBLE_PAWN_PUSH_OFFSET
                                | (origin.file() as u64) << EP_FILE_OFFSET,
                        _ => (),
                    }
                },
                King => match (origin.file(), destination.file()) {
                    // TODO maybe make a generic castling
                    (EFile, GFile) => mv |= Castling::KINGSIDE[board.turn() as usize].code(),
                    (EFile, CFile) => mv |= Castling::QUEENSIDE[board.turn() as usize].code(),
                    _ => (),
                },
                _ => (),
            },
            [Some(_), Some(p)] => 
                // Regular capture
                mv |= (destination as u64) << CAPTURED_SQUARE_OFFSET
                    | (p as u64) << CAPTURED_PIECE_OFFSET,
            _ =>  // No capture
                mv |= Piece::EMPTY_CODE << CAPTURED_PIECE_OFFSET,
            
        }

        Move(mv)
    }

    pub const fn origin_square(self) -> Square {
        Square::ALL[(self.0 >> ORIGIN_SQUARE_OFFSET & Square::BIT_MAX) as usize]
    }

    pub const fn origin_piece(self) -> Piece {
        Piece::ALL[(self.0 >> ORIGIN_PIECE_OFFSET & Piece::BIT_MAX) as usize]
    }

    pub const fn destination_square(self) -> Square {
        Square::ALL[(self.0 >> DESTINATION_SQUARE_OFFSET & Square::BIT_MAX) as usize]
    }

    pub const fn destination_piece(self) -> Piece {
        Piece::ALL[(self.0 >> DESTINATION_PIECE_OFFSET & Piece::BIT_MAX) as usize]
    }

    pub const fn is_capture(self) -> bool {
        self.0 >> CAPTURED_PIECE_OFFSET & Piece::BIT_MAX != Piece::EMPTY_CODE
    }

    pub const fn captured_square(self) -> Square {
        Square::ALL[(self.0 >> CAPTURED_SQUARE_OFFSET & Square::BIT_MAX) as usize]
    }

    pub const fn captured_piece(self) -> Option<Piece> {
        match self.0 >> CAPTURED_PIECE_OFFSET & Piece::BIT_MAX {
            Piece::EMPTY_CODE => None,
            code => Some(Piece::ALL[code as usize]),
        }
    }

    pub const fn is_double_pawn_push(self) -> bool {
        self.0 >> DOUBLE_PAWN_PUSH_OFFSET & 1 == 1
    }

    pub const fn is_ep_capture(self) -> bool {
        self.0 >> EP_CAPTURE_OFFSET & 1 == 1
    }
}


