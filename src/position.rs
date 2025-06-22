use std::cmp::max;

use crate::bitmask;
use crate::color::{Color, Color::*};
use crate::board_move::Castling;
use crate::err::*;
use crate::piece::{
    GenericPiece,
    GenericPiece::*,
    Piece,
};
use crate::square::*;
use crate::zobrist;


const CASTLING_OFFSET: u32 = 0;
const TURN_OFFSET: u32 = 4;
const EP_LEGAL_OFFSET: u32 = 5;
const EP_TARGET_OFFSET: u32 = 6;
const HALFMOVE_CTR_OFFSET: u32 = 9;
const FULLMOVE_CTR_OFFSET: u32 = 17;

const HALFMOVE_CTR_BITS: u32 = FULLMOVE_CTR_OFFSET - HALFMOVE_CTR_OFFSET;
const FULLMOVE_CTR_BITS: u32 = 32 - FULLMOVE_CTR_OFFSET;

pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


const WHITE_OCCUPANCY: usize = Piece::ALL.len();
const BLACK_OCCUPANCY: usize = WHITE_OCCUPANCY + 1;
const FULL_OCCUPANCY: usize = BLACK_OCCUPANCY + 1;


#[derive(Debug)]
pub enum FenSection {
    Board,
    SideToMove,
    Castling,
    EnPassant,
    HalfmoveCounter,
    FullmoveCounter,
}


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
    bitboard: [u64; Piece::ALL.len() + 3],
    fen_info: u32,
    zobrist_hash: u64,  // TODO set in methods
}

const fn occ_mismatch(mask: u64) -> Result<(), IllegalPosition> {
    match mask {
        0 => Ok(()),
        _ => Err(
            IllegalPosition::CorruptedBitboard(
                CorruptedBitboard::OccupancyMismatch(mask)))
    }
}

impl GameState {

    fn empty() -> GameState {
        GameState {
            bitboard: [0; 15],
            fen_info: 0,
            zobrist_hash: 0,
        }
    }


    pub const fn count_piece(&self, piece: Piece) -> u32 {
        bitmask::count_bits(self.bitboard[piece as usize])
    }

    pub const fn count_piece_validate(&self, piece: Piece, limit: u32) -> Result<u32, IllegalPosition> {
        let n = self.count_piece(piece);
        if n > limit {
            return Err(IllegalPosition::TooManyPieces(piece, n));
        }
        Ok(n)
    }

    pub fn validate(&self) -> Result<(), IllegalPosition> {
        let mut w_occ: u64 = 0;
        let mut b_occ: u64 = 0;
        let mut i = 0;
        while i < 6 {
            occ_mismatch(w_occ & self.bitboard[i])?;
            w_occ |= self.bitboard[i];
            occ_mismatch(b_occ & self.bitboard[i + 6])?;
            b_occ |= self.bitboard[i + 6];
            i += 1;
        }
        occ_mismatch(w_occ & b_occ)?;
        occ_mismatch(w_occ ^ self.occ(White))?;
        occ_mismatch(b_occ ^ self.occ(Black))?;
        occ_mismatch(w_occ & b_occ ^ self.full_occ())?;
        // TODO zobrist mismatch

        match self.ep_code() {
            0 => (),
            0b0010..0b1110 | 0b10000.. => return Err(
                IllegalPosition::CorruptedBitboard(
                    CorruptedBitboard::InvalidEnPassantCode(self.ep_code()))),
            _ => {
                let s = self.ep_square();
                let enemy_expected = self.turn().inv().pawn_direction().shift(s.mask(), 1);
                let enemy_actual = self.bitboard[Pawn.as_color(self.turn().inv()) as usize];
                let allied_expected = bitmask::PAWN_ATTACKS[self.turn() as usize][s as usize];
                let allied_actual = self.bitboard[Pawn.as_color(self.turn()) as usize];
                if s.mask() & self.full_occ() != 0 {
                    return Err(IllegalPosition::EnPassantSquareOccupied);
                } else if enemy_expected & enemy_actual == 0 {
                    return Err(IllegalPosition::NoEnPassantDefender);
                } else if allied_expected & allied_actual == 0 {
                    return Err(IllegalPosition::NoEnPassantAttacker);
                }
            }
        }

        const STARTING_COUNTS: [(GenericPiece, u32); 4] = [
            (Rook, 2),
            (Knight, 2),
            (Bishop, 2),
            (Queen, 1)
        ];
        for color in Color::ALL {
            if 0 == self.count_piece_validate(King.as_color(color), 1)? {
                return Err(IllegalPosition::MissingKing(color));
            }
            let pawns = self.count_piece_validate(Pawn.as_color(color), 8)?;
            let mut promotions_left = 8 - pawns;
            for (piece, starting_count) in STARTING_COUNTS {
                let count = self.count_piece_validate(piece.as_color(color), promotions_left + starting_count)?;
                promotions_left -= max(0, count - starting_count);
            }
            // TODO check for same color bishops with no pawn promotions
            let p = Bishop.as_color(color);
            if pawns == 8 && self.count_piece(p) >= 2 {
                let mask = self.bitboard[p as usize];
                if (mask & bitmask::LIGHT_SQUARES).count_ones() > 1 {
                    return Err(IllegalPosition::SameColorBishops(p, White));
                } else if (mask & bitmask::DARK_SQUARES).count_ones() > 1 {
                    return Err(IllegalPosition::SameColorBishops(p, Black));
                }
            }
        }

        for castling in Castling::ALL {
            if !self.can_castle(castling) {
                continue
            }
            let king_actual = self.piece_at(castling.king_start());
            let king_expected = Some(King.as_color(castling.color()));
            let rook_actual = self.piece_at(castling.rook_start());
            let rook_expected =  Some(Rook.as_color(castling.color()));
            if king_actual != king_expected || rook_actual != rook_expected {
                return Err(IllegalPosition::InvalidCastling(castling));
            }
        }

        // TODO
        let zobrist = self.compute_zobrist();
        if self.zobrist_hash != zobrist {
            return Err(
                IllegalPosition::CorruptedBitboard(
                    CorruptedBitboard::ZobristMismatch(zobrist, self.zobrist_hash)));
        }

        Ok(())
    }


    pub const fn piece_at_rc(&self, row: usize, col: usize) -> Option<Piece> {
        self.piece_at(Square::from_rc(row, col))
    }

    pub const fn piece_at(&self, square: Square) -> Option<Piece> {
        let mut i = 0;
        while i < Piece::ALL.len() {
            if self.bitboard[i] & square.mask() != 0 {
                return Some(Piece::ALL[i]);
            }
            i += 1;
        }
        None
    }

    const fn full_occ(&self) -> u64 {
        self.bitboard[FULL_OCCUPANCY]
    }

    const fn occ(&self, color: Color) -> u64 {
        self.bitboard[Piece::ALL.len() + color as usize]
    }
    

    fn compute_zobrist(&self) -> u64 {
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


    const fn castling_code(&self) -> u32 {
        self.fen_info & 0b1111
    }

    pub const fn can_castle(&self, castling: Castling) -> bool {
        self.castling_code() & (1 << castling as u32) != 0
    }

    pub const fn deny_castling(&mut self, castling: Castling) {
        // TODO perhaps optimize
        self.zobrist_hash ^= zobrist::CASTLING[self.castling_code() as usize];
        // Optimization over set_castling
        self.fen_info &= !(1 << CASTLING_OFFSET + castling as u32);
        self.zobrist_hash ^= zobrist::CASTLING[self.castling_code() as usize];
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
        self.zobrist_hash ^= zobrist::BLACK_TO_MOVE;
        self.fen_info ^= 1 << TURN_OFFSET;
    }

    const fn set_turn(&mut self, color: Color) {
        self.set_fen_bits(TURN_OFFSET, 1, color as u32);
    }


    const fn ep_legal_code(&self) -> u32 {
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
        return self.halfmove_ctr() >= 100
    }

    pub const fn set_halfmove_ctr(&mut self, ctr: u32) {
        self.set_fen_bits(HALFMOVE_CTR_OFFSET, HALFMOVE_CTR_BITS, ctr);
    }


    pub const fn fullmove_ctr(&self) -> u32 {
        self.fen_info >> FULLMOVE_CTR_OFFSET
    }

    const fn set_fullmove_ctr(&mut self, ctr: u32) {
        self.set_fen_bits(FULLMOVE_CTR_OFFSET, FULLMOVE_CTR_BITS, ctr);
    }


    const fn change_piece(&mut self, piece: Piece, square: Square) {
        // Remove or put is the same. Should not put piece on occupied square.
        self.bitboard[piece as usize] |= square.mask();
        self.bitboard[piece.occ_index()] ^= square.mask();
        self.bitboard[FULL_OCCUPANCY] ^= square.mask();
        self.zobrist_hash ^= zobrist::PIECES[piece as usize][square as usize];
    }

    const fn remove_piece(&mut self, piece: Piece, square: Square) {
        // This function is for readability; algorithm is the same as put_piece
        self.change_piece(piece, square)
    }

    const fn put_piece(&mut self, piece: Piece, square: Square) {
        // This function is for readability; the algorithm is the same as remove_piece
        self.change_piece(piece, square)
    }




    fn castling_str(&self) -> String {
        let mut chars: Vec<char> = Vec::new();
        for i in Castling::ALL {
            if self.can_castle(i) {
                chars.push(i.chr());
            }
        }
        if chars.len() == 0 {
            chars.push('-');
        }
        String::from_iter(chars)
    }

    pub fn fen(&self) -> String {
        let mut s = String::new();

        // Piece configuration
        for r in (0..8).rev() {
            let mut blanks: u8 = 0;
            for c in 0..8 {
                match self.piece_at_rc(r, c) {
                    None => blanks += 1,
                    Some(piece) => {
                        if blanks > 0 {
                            s.push((blanks + b'0') as char);
                            blanks = 0;
                        }
                        s.push(piece.chr());
                    }
                }
            }
            if blanks > 0 {
                s.push((blanks + b'0') as char);
            }
            if r != 0 {
                s.push('/');
            }
        }

        // Side to move
        s.push(' ');
        s.push(self.turn().chr());

        // Castling options
        s.push(' ');
        match self.castling_code() == 0 {
            true => s.push('-'),
            false => for castle_option in Castling::ALL {
                if self.can_castle(castle_option) {
                    s.push(castle_option.chr());
                }
            }
        }
        
        // En passant
        s.push(' ');
        match self.ep_legal() {
            false => s.push('-'),
            true => s.extend(self.ep_square().chrs()),
        }

        // Halfmove counter
        s.push_str(&format!(" {}", self.halfmove_ctr()));

        // Fullmove counter
        s.push_str(&format!(" {}", self.fullmove_ctr()));
        
        s
    }

    pub fn from_fen(fen: &str) -> Result<GameState, FenError> {
        let mut state = GameState::empty();
        let mut chars = fen.chars();
        let mut i = 0;

        // Board
        let mut row = 7;
        let mut col = 0;
        loop {
            i += 1;
            match chars.next() {
                None => break,
                Some(chr) => match chr {
                    ' ' => break,
                    '/' => match (row, col) {
                        (0, _) => return Err(FenError::TooManyRows),
                        (_, 8) => (row, col) = (row - 1, 0),
                        (_, _) => return Err(FenError::IncompleteRow(row)),
                    },
                    '1'..'9' => match col + chr as u8 - b'0' {
                        9.. => return Err(FenError::TooManyColumns(row)),
                        c => col = c,
                    },
                    _ => {
                        let piece = Piece::from_char(chr)? as usize;
                        state.bitboard[piece] |= 1u64 << 8 * row + col as i32;
                        col += 1;
                    },
                }
            }
        }
        match (row, col) {
            (0, 8) => (),
            (0, _) => return Err(FenError::TooManyRows),
            (1.., 8) => return Err(FenError::IncompleteRow(row - 1)),
            _ => return Err(FenError::IncompleteRow(row)),
        }

        // Side to move
        i += 1;
        match chars.next() {
            None => return Err(FenError::MissingSection(FenSection::SideToMove)),
            Some(chr) => state.set_turn(Color::from_chr(chr)?),
        }

        i += 1;
        match chars.next() {
            None => return Err(FenError::MissingSection(FenSection::Castling)),
            Some(' ') => (),
            _ => return Err(FenError::ExpectedSpace(i)),
        }

        // Castling
        for castling in Castling::ALL {
            state.deny_castling(castling);
        }
        let mut min_index: usize = 0;
        loop {
            i += 1;
            match chars.next() {
                None => return Err(FenError::MissingSection(FenSection::Castling)),
                Some(' ') => break,
                Some('-') => match min_index {
                    0 => min_index = 5,
                    _ => return Err(FenError::InvalidCastling),
                },
                Some(chr) => {
                    let castling = Castling::from_char(chr)?;
                    match (castling as usize) < min_index {
                        true => return Err(FenError::CastlingOutOfOrder),
                        false => {
                            state.set_castling(castling, true);
                            min_index = 1 + castling as usize;
                        },
                    }
                }
            }
        }
        if min_index == 0 {
            return Err(FenError::MissingSection(FenSection::Castling));
        }
        
        // En Passant
        i += 2;
        match (chars.next(), chars.next()) {
            (None, _) => return Err(FenError::MissingSection(FenSection::EnPassant)),
            (Some('-'), None) => return Err(FenError::MissingSection(FenSection::HalfmoveCounter)),
            (_, None) => return Err(FenError::CoordinateError(CoordinateError::IncompleteSquare)),
            (Some('-'), Some(' ')) => (),
            (Some(f), Some(r)) => match (File::from_chr(f), Rank::from_chr(r)) {
                (Err(e), _) => return Err(FenError::CoordinateError(e)),
                (_, Err(e)) => return Err(FenError::CoordinateError(e)),
                (Ok(f), Ok(r)) => match state.ep_rank() == r {
                    true => state.set_ep_target(f),
                    false => return Err(FenError::InvalidEnPassant),
                }
            }
        }

        // Halfmove counter
        let mut s = String::new();
        loop {
            match chars.next() {
                None => return Err(FenError::MissingSection(FenSection::FullmoveCounter)),
                Some(' ') => break,
                Some(chr) => s.push(chr),
            }
        }
        match s.parse::<u32>() {
            Err(_) => return Err(FenError::InvalidHalfmove),
            Ok(n) => state.set_halfmove_ctr(n),
        }

        // Fullmove counter
        let s = String::from_iter(chars);
        if s.len() == 0 {
            return Err(FenError::MissingSection(FenSection::FullmoveCounter));
        }
        match s.parse::<u32>() {
            Err(_) => return Err(FenError::InvalidFullMove),
            Ok(n) => state.set_fullmove_ctr(n),
        }
        
        // Set metadata
        for i in 0..6 {
            state.bitboard[WHITE_OCCUPANCY] |= state.bitboard[i];
            state.bitboard[BLACK_OCCUPANCY] |= state.bitboard[i + 6];
        }
        state.bitboard[FULL_OCCUPANCY] = state.occ(White) + state.occ(Black);

        state.zobrist_hash = state.compute_zobrist();

        Ok(state)
    }

    pub fn print_verbose(&self) {
        for i in 0..12 {
            println!("{:?}", Piece::ALL[i]);
            bitmask::print(self.bitboard[i]);
            println!("");
        }

        println!("White occupancy");
        bitmask::print(self.occ(White));
        println!("");

        println!("Black occupancy");
        bitmask::print(self.occ(Black));
        println!("");

        println!("Full occupancy");
        bitmask::print(self.full_occ());
        println!("");

        println!(
            "To move: {} {:01b}", 
            self.turn().chr(), 
            self.turn_code());
        println!(
            "Castling: {} {:04b}", 
            self.castling_str(), 
            self.castling_code());
        println!(
            "EP: Legal = {} {:01b}, Target = {:?} {:03b}", 
            self.ep_legal(),
            self.ep_legal_code(),
            self.ep_square(),
            self.ep_code());
        println!(
            "Halfmove ctr: {}",
            self.halfmove_ctr());
        println!(
            "Fullmove ctr: {}",
            self.fullmove_ctr());
        
        println!("Fen {}", self.fen());
        println!("Fen bits {:032b}", self.fen_info);
        println!(
            "(turn, castling, ep): {:b} {:b} {:b}", 
            self.turn_code(),
            self.castling_code(),
            self.ep_code());
        println!("Zobrist hash: {:x}", self.zobrist_hash);
    }

    pub fn print_pretty(&self) {
        for r in 0..8 {
            let r = 7 - r;
            for c in 0..8 {
                let mut s = '.';
                for p in 0..12 {
                    if self.bitboard[p] & 1 << 8 * r + c != 0 {
                        s = Piece::ALL[p].chr();
                        break;
                    }
                }
                print!("{} ", s);
            }
            println!("");
        }
        println!("{}", self.fen());
    }
}