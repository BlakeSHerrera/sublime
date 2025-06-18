use crate::bitmask;
use crate::piece::*;
use crate::zobrist;
use crate::board_move::Castling;
use crate::square::*;
use crate::err::*;


const CASTLING_OFFSET: u32 = 0;
const TURN_OFFSET: u32 = 4;
const EP_LEGAL_OFFSET: u32 = 5;
const EP_TARGET_OFFSET: u32 = 6;
const HALFMOVE_CTR_OFFSET: u32 = 9;
const FULLMOVE_CTR_OFFSET: u32 = 17;

const HALFMOVE_CTR_BITS: u32 = FULLMOVE_CTR_OFFSET - HALFMOVE_CTR_OFFSET;
const FULLMOVE_CTR_BITS: u32 = 32 - FULLMOVE_CTR_OFFSET;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


/*
fen_info bits:
0-3     (4)     Castling rights
4       (1)     Black to move?
5       (1)     Has EP target?
6-8     (3)     EP target file
9-16    (8)     Half-move counter
17-31   (15)    Full-move counter
*/
struct GameState {
    bitboard: [u64; 12],
    white_occupancy: u64,
    black_occupancy: u64,
    full_occupancy: u64,
    fen_info: u32,
    zobrist_hash: u64,  // TODO set in methods
}

impl GameState {

    pub const fn piece_at(&self, row: u64, col: u64) -> Option<ColoredPiece> {
        let mut i = 0;
        while i < self.bitboard.len() {
            if self.bitboard[i] & 8 * row + col != 0 {
                return Some(ColoredPiece::ALL[i]);
            }
            i += 1;
        }
        None
    }


    const fn set_zobrist(&self) {
        // TODO
    }


    const fn set_fen_bits(&mut self, offset: u32, num_bits: u32, bits: u32) {
        self.fen_info = self.fen_info & !((1 << num_bits) - 1) << offset | bits << offset;
    }


    const fn castling_code(&self) -> u32 {
        self.fen_info & 0b1111
    }

    pub const fn can_castle(&self, castling: Castling) -> bool {
        self.castling_code() & 1 << (castling as u32) != 0
    }

    pub const fn deny_castling(&mut self, castling: Castling) {
        self.fen_info &= !(1 << CASTLING_OFFSET + castling as u32);
    }

    const fn set_castling(&mut self, castling: Castling, can_castle: bool) {
        self.set_fen_bits(CASTLING_OFFSET + castling as u32, 1, can_castle as u32);
    }

    
    const fn turn_code(&self) -> u32 {
        self.fen_info >> TURN_OFFSET & 1
    }

    pub const fn turn(&self) -> Color {
        match self.turn_code() == 0 {
            true => Color::White,
            false => Color::Black,
        }
    }

    pub const fn flip_turn(&mut self) {
        self.fen_info ^= 1 << TURN_OFFSET;
    }

    pub const fn set_turn(&mut self, color: Color) {
        self.set_fen_bits(TURN_OFFSET, 1, color as u32);
    }


    const fn ep_legal_code(&self) -> u32 {
        self.fen_info & 1 << EP_LEGAL_OFFSET
    }

    pub const fn ep_legal(&self) -> bool {
        self.ep_legal_code() != 0
    }

    pub const fn deny_ep(&mut self) {
        self.fen_info &= !0b1111 << EP_LEGAL_OFFSET;
    }


    const fn ep_code(&self) -> u32 {
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
            Color::White => Rank::R6,
            Color::Black => Rank::R3,
        }
    }

    pub const fn ep_rank_i(&self) -> Rank {
        match self.turn() {
            Color::White => Rank::R3,
            Color::Black => Rank::R6,
        }
    }

    pub const fn ep_square_num(&self) -> usize {
        self.ep_file_num() + match self.turn() {
            Color::White => Rank::R3.offset(),
            Color::Black => Rank::R6.offset(),
        }
    }

    pub const fn ep_square(&self) -> Square {
        Square::ALL[self.ep_square_num()]
    }

    pub const fn set_ep_target(&mut self, file: File) {
        self.set_fen_bits(EP_LEGAL_OFFSET, 4, ((file as u32) << 1) + 1);
    }
    

    pub const fn halfmove_ctr(&self) -> u32 {
        self.fen_info >> HALFMOVE_CTR_OFFSET & !((1 << HALFMOVE_CTR_BITS) - 1)
    }

    pub const fn is_50_move_rule(&self) -> bool {
        return self.halfmove_ctr() >= 100
    }

    pub const fn set_halfmove_ctr(&mut self, ctr: u32) {
        self.set_fen_bits(HALFMOVE_CTR_OFFSET, HALFMOVE_CTR_BITS, ctr);
    }


    pub const fn fullmove_ctr(&self) -> u32 {
        self.fen_info >> FULLMOVE_CTR_OFFSET
    }

    pub const fn set_fullmove_ctr(&mut self, ctr: u32) {
        self.set_fen_bits(FULLMOVE_CTR_OFFSET, FULLMOVE_CTR_BITS, ctr);
    }


    fn castling_str(&self) -> String {
        let mut chars: Vec<char> = Vec::new();
        for i in Castling::ALL {
            if self.can_castle(i) {
                chars.push(i.get_char());
            }
        }
        if chars.len() == 0 {
            chars.push('-');
        }
        String::from_iter(chars)
    }
}