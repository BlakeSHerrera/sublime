use {
    crate::board::{
        color::{*, Color::*},
        line::{
            *,
            File::*,
            Rank::*,
        },
        piece::{*, GenericPiece::*},
        square::{*, Square::*},
        zone::{
            *, 
            Quadrant::*, 
            FileSide::*
        },
    },
    super::position::*,
};


#[derive(Debug)]
pub enum IllegalMove {
    InCheck,
    InvalidMove,
    OpponentPieceMove,
    EmptySquareMove,
    AlliedCapture,
    CastleOutOfCheck,
    CastleThroughCheck,
    InvalidPromotion,
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
        board: &GameState, 
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
                    (EFile, GFile) => mv |= 1 << IS_CASTLING_OFFSET
                        | (Kingside.to_quadrant(board.turn()) as u64) << CASTLING_OPTION_OFFSET,
                    (EFile, CFile) => mv |= 1 << IS_CASTLING_OFFSET
                        | (Queenside.to_quadrant(board.turn()) as u64) << CASTLING_OPTION_OFFSET,
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

    pub const fn color(self) -> Color {
        self.origin_piece().color()
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
        self.0 & 1 << DOUBLE_PAWN_PUSH_OFFSET != 0
    }

    pub const fn is_ep_capture(self) -> bool {
        self.0 & 1 << EP_CAPTURE_OFFSET != 0
    }

    pub const fn is_castling(self) -> bool {
        self.0 & 1 << IS_CASTLING_OFFSET != 0
    }

    pub const fn get_castling(self) -> Quadrant {
        Quadrant::ALL[(self.0 >> CASTLING_OPTION_OFFSET & 0b11) as usize]
    }
}


impl GameState {

    pub const fn push_partial(&mut self, mv: Move) {
        // No fen info updates; to check for move legality
        self.remove_piece(mv.origin_piece(), mv.origin_square());
        match mv.captured_piece() {
            Some(p) => self.remove_piece(p, mv.captured_square()),
            None => (),
        };
        self.put_piece(mv.destination_piece(), mv.destination_square());

        match mv.is_castling() {
            true => {
                let q = mv.get_castling();
                self.remove_piece(q.rook(), q.rook_start());
                self.put_piece(q.rook(), q.rook_end());
            },
            false => (),
        }
    }

    pub const fn push(&mut self, mv: Move) {
        self.push_partial(mv);
        
        self.deny_ep();
        match mv.origin_piece().as_generic() {
            Pawn => {
                self.reset_halfmove_ctr();
                match mv.is_double_pawn_push() {
                    true => self.set_ep_target(mv.origin_square().file()),
                    false => (),
                };
            },
            King => {
                let color = self.turn();
                self.inc_halfmove_ctr();
                self.deny_castling(Kingside.to_quadrant(color));
                self.deny_castling(Queenside.to_quadrant(color));
            },
            _ => self.inc_halfmove_ctr(),
        };
        
        match [mv.origin_square(), mv.destination_square()] {
            [H1, _] | [_, H1] => self.deny_castling(WhiteKingside),
            [A1, _] | [_, A1] => self.deny_castling(WhiteQueenside),
            [H8, _] | [_, H8] => self.deny_castling(BlackKingside),
            [A8, _] | [_, A8] => self.deny_castling(BlackQueenside),
            _ => (),
        };

        self.inc_turn();
    }

    pub const fn pop_partial(&mut self, mv: Move) {
        // No fen info updates
        self.put_piece(mv.origin_piece(), mv.origin_square());
        self.remove_piece(mv.destination_piece(), mv.destination_square());
        match mv.captured_piece() {
            None => (),
            Some(p) => self.put_piece(p, mv.captured_square()),
        }
        match mv.is_castling() {
            false => (),
            true => {
                let q = mv.get_castling();
                self.put_piece(q.rook(), q.rook_start());
                self.remove_piece(q.rook(), q.rook_end());
            }
        }
        // It is the caller's responsibility to reset the last fen_info.
    }
}
