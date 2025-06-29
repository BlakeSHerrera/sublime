

use crate::{
    board::{
        color::{*, Color::*},
        line::*,
        piece::*,
        square::*,
        zone::*,
    },
    hashing::{
        bitmask,
        zobrist,
    },
};


const CASTLING_OFFSET: u32 = 0;
const TURN_OFFSET: u32 = 4;
const EP_LEGAL_OFFSET: u32 = 5;
const EP_TARGET_OFFSET: u32 = 6;
const HALFMOVE_CTR_OFFSET: u32 = 9;
const FULLMOVE_CTR_OFFSET: u32 = 17;

const HALFMOVE_CTR_BITS: u32 = FULLMOVE_CTR_OFFSET - HALFMOVE_CTR_OFFSET;
const FULLMOVE_CTR_BITS: u32 = 32 - FULLMOVE_CTR_OFFSET;

pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


// TODO make these private to fen
pub const WHITE_OCCUPANCY: usize = Piece::ALL.len();
pub const BLACK_OCCUPANCY: usize = WHITE_OCCUPANCY + 1;
pub const FULL_OCCUPANCY: usize = BLACK_OCCUPANCY + 1;


/*
fen_info bits:
0-3     (4)     Castling rights
4       (1)     Black to move?
5       (1)     Has EP target?
6-8     (3)     EP target file
9-16    (8)     Half-move counter
17-31   (15)    Full-move counter
*/
pub struct GameState {
    // See constants above for last 3 indicies
    pub bitboard: [u64; Piece::ALL.len() + 3],
    pub piece_on_square: [Option<Piece>; 64],
    // TODO piece on square array?
    pub fen_info: u32,
    pub zobrist_hash: u64,  // TODO set in methods
}

impl GameState {

    // TODO make private to fen module
    pub fn empty() -> GameState {
        GameState {
            bitboard: [0; 15],
            piece_on_square: [None; 64],
            fen_info: 0,
            zobrist_hash: 0,
        }
    }


    pub const fn count_piece(&self, piece: Piece) -> u32 {
        bitmask::count_bits(self.bitboard[piece as usize])
    }


    pub const fn piece_at_rc(&self, row: usize, col: usize) -> Option<Piece> {
        self.piece_at(Square::from_rc(row, col))
    }

    pub const fn piece_at(&self, square: Square) -> Option<Piece> {
        self.piece_on_square[square as usize]
    }

    // TODO make private to fen
    pub const fn piece_at_bitboard(&self, square: Square) -> Option<Piece> {
        let mut i = 0;
        while i < Piece::ALL.len() {
            if self.bitboard[i] & square.mask() != 0 {
                return Some(Piece::ALL[i]);
            }
            i += 1;
        }
        None
    }

    pub const fn full_occ(&self) -> u64 {
        self.bitboard[FULL_OCCUPANCY]
    }

    pub const fn occ(&self, color: Color) -> u64 {
        self.bitboard[Piece::ALL.len() + color as usize]
    }
    
    pub const fn self_occ(&self) -> u64 {
        self.occ(self.turn())
    }

    pub const fn enemy_occ(&self) -> u64 {
        self.occ(self.turn().inv())
    }

    // TODO make private to fen module
    pub fn compute_zobrist(&self) -> u64 {
        let mut hash: u64 = 0;
        for s in Square::ALL {
            hash ^= match self.piece_at(s) {
                None => 0,
                Some(p) => zobrist::PIECES[p as usize][s as usize],
            };
        }
        hash ^= match self.turn() {
            White => 0,
            Black => zobrist::BLACK_TO_MOVE,
        };
        hash ^= zobrist::CASTLING[self.castling_code() as usize];
        hash ^ match self.ep_legal() {
            false => 0,
            true => zobrist::EP_FILE[self.ep_file_num()],
        }
    }


    const fn set_fen_bits(&mut self, offset: u32, num_bits: u32, bits: u32) {
        self.fen_info = self.fen_info & !((1 << num_bits) - 1 << offset) | bits << offset;
    }

    
    // TODO make private to fen
    pub const fn castling_code(&self) -> u32 {
        self.fen_info & 0b1111
    }

    pub const fn can_castle(&self, castling: Quadrant) -> bool {
        self.castling_code() & (1 << castling as u32) != 0
    }

    pub const fn can_castle_any(&self) -> bool {
        self.castling_code() != 0
    }

    pub const fn deny_castling(&mut self, castling: Quadrant) {
        // TODO perhaps optimize
        self.zobrist_hash ^= zobrist::CASTLING[self.castling_code() as usize];
        // Optimization over set_castling
        self.fen_info &= !(1 << CASTLING_OFFSET + castling as u32);
        self.zobrist_hash ^= zobrist::CASTLING[self.castling_code() as usize];
    }

    // TODO make private to fen
    pub const fn set_castling(&mut self, castling: Quadrant, can_castle: bool) {
        self.set_fen_bits(CASTLING_OFFSET + castling as u32, 1, can_castle as u32);
    }

    
    // TODO make private to fen
    pub const fn turn_code(&self) -> u32 {
        self.fen_info >> TURN_OFFSET & 1
    }

    pub const fn turn(&self) -> Color {
        match self.turn_code() == 0 {
            true => Color::White,
            false => Color::Black,
        }
    }

    pub const fn flip_turn(&mut self) {
        self.set_fullmove_ctr(self.fullmove_ctr() + self.turn() as u32);
        self.fen_info ^= 1 << TURN_OFFSET;
        self.zobrist_hash ^= zobrist::BLACK_TO_MOVE;
    }

    // TODO make private to fen module
    pub const fn set_turn(&mut self, color: Color) {
        self.set_fen_bits(TURN_OFFSET, 1, color as u32);
    }


    // TODO make private to fen module
    pub const fn ep_legal_code(&self) -> u32 {
        self.fen_info & 1 << EP_LEGAL_OFFSET
    }

    pub const fn ep_legal(&self) -> bool {
        self.ep_legal_code() != 0
    }

    pub const fn deny_ep(&mut self) {
        self.zobrist_hash ^= match self.ep_legal() {
            true => zobrist::EP_FILE[self.ep_file_num()],
            false => 0,
        };
        self.fen_info &= !0b1111 << EP_LEGAL_OFFSET;
    }


    // TODO make private to fen
    pub const fn ep_code(&self) -> u32 {
        self.fen_info >> EP_TARGET_OFFSET & 0b111
    }

    pub const fn ep_file_num(&self) -> usize {
        self.ep_code() as usize
    }

    pub const fn ep_file(&self) -> File {
        File::ALL[self.ep_file_num()]
    }

    pub const fn ep_rank(&self) -> Rank {
        match self.turn() {
            Color::White => Rank::Rank6,
            Color::Black => Rank::Rank3,
        }
    }

    pub const fn ep_rank_i(&self) -> Rank {
        match self.turn() {
            Color::White => Rank::Rank3,
            Color::Black => Rank::Rank6,
        }
    }

    pub const fn ep_square_num(&self) -> usize {
        self.ep_file_num() + match self.turn() {
            Color::White => Rank::Rank3.offset(),
            Color::Black => Rank::Rank6.offset(),
        }
    }

    pub const fn ep_square(&self) -> Square {
        Square::ALL[self.ep_square_num()]
    }

    pub const fn set_ep_target(&mut self, file: File) {
        self.deny_ep();
        self.zobrist_hash ^= zobrist::EP_FILE[file as usize];
        self.fen_info |= ((file as u32) << 1) + 1 << EP_LEGAL_OFFSET;
    }
    

    pub const fn halfmove_ctr(&self) -> u32 {
        self.fen_info >> HALFMOVE_CTR_OFFSET & !((1 << HALFMOVE_CTR_BITS + 1) - 1)
    }

    pub const fn is_50_move_rule(&self) -> bool {
        self.halfmove_ctr() >= 100
    }

    pub const fn set_halfmove_ctr(&mut self, ctr: u32) {
        self.set_fen_bits(HALFMOVE_CTR_OFFSET, HALFMOVE_CTR_BITS, ctr);
    }

    pub const fn reset_halfmove_ctr(&mut self) {
        self.set_halfmove_ctr(0);
    }

    pub const fn inc_halfmove_ctr(&mut self) {
        self.set_halfmove_ctr(self.halfmove_ctr() + 1);
    }


    pub const fn fullmove_ctr(&self) -> u32 {
        self.fen_info >> FULLMOVE_CTR_OFFSET
    }

    // TODO make private to fen module
    pub const fn set_fullmove_ctr(&mut self, ctr: u32) {
        self.set_fen_bits(FULLMOVE_CTR_OFFSET, FULLMOVE_CTR_BITS, ctr);
    }


    const fn change_piece_bitboard(&mut self, piece: Piece, square: Square) {
        // Remove or put is the same. Should not put piece on occupied square.
        // Need to update self.piece_on_square on function exit.
        self.bitboard[piece as usize] |= square.mask();
        self.bitboard[piece.occ_index()] ^= square.mask();
        self.bitboard[FULL_OCCUPANCY] ^= square.mask();
        self.zobrist_hash ^= zobrist::PIECES[piece as usize][square as usize];
    }

    pub const fn remove_piece(&mut self, piece: Piece, square: Square) {
        self.change_piece_bitboard(piece, square);
        self.piece_on_square[piece as usize] = None;
    }

    pub const fn put_piece(&mut self, piece: Piece, square: Square) {
        self.change_piece_bitboard(piece, square);
        self.piece_on_square[piece as usize] = Some(piece);
    }
}
