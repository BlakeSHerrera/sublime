use std::cmp::max;

use crate::board::{
    color::{*, Color::*},
    line::*,
    piece::{*, GenericPiece::*},
    square::*,
    zone::Quadrant,
};
use crate::hashing::bitmask;
use crate::game::position::*;

use super::chr::ConversionError;
use super::err::*;


#[derive(Debug)]
pub enum FenSection {
    Board,
    SideToMove,
    Castling,
    EnPassant,
    HalfmoveCounter,
    FullmoveCounter,
}


const fn occ_mismatch(mask: u64) -> Result<(), IllegalPosition> {
    match mask {
        0 => Ok(()),
        _ => Err(
            IllegalPosition::CorruptedBitboard(
                CorruptedBitboard::OccupancyMismatch(mask)))
    }
}

fn converts<T>(result: Result<T, ConversionError>)  -> Result<T, FenError> {
    match result {
        Err(e) => Err(FenError::ConversionError(e)),
        Ok(r) => Ok(r),
    }
}


impl GameState {
    
    fn castling_str(&self) -> String {
        let mut chars: Vec<char> = Vec::new();
        for i in Quadrant::ALL {
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
        match self.can_castle_any() {
            false => s.push('-'),
            true => for castle_option in Quadrant::ALL {
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
                        let piece = converts(Piece::from_char(chr))? as usize;
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
            Some(chr) => state.set_turn(converts(Color::from_chr(chr))?)
        }

        i += 1;
        match chars.next() {
            None => return Err(FenError::MissingSection(FenSection::Castling)),
            Some(' ') => (),
            _ => return Err(FenError::ExpectedSpace(i)),
        }

        // Castling
        for castling in Quadrant::ALL {
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
                    let castling = converts(Quadrant::from_char(chr))?;
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
            (_, None) => return Err(FenError::ConversionError(ConversionError::IncompleteSquare)),
            (Some('-'), Some(' ')) => (),
            (Some(f), Some(r)) => match state.ep_rank() == converts(Rank::from_chr(r))? {
                true => state.set_ep_target(converts(File::from_chr(f))?),
                false => return Err(FenError::InvalidEnPassant),
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

        println!("Piece on square");
        self.print_pretty();
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
                let s = match self.piece_at_rc(r, c) {
                    None => '.',
                    Some(p) => p.chr(),
                };
                print!("{} ", s);
            }
            println!("");
        }
        println!("{}", self.fen());
    }


    const fn count_piece_validate(&self, piece: Piece, limit: u32) -> Result<u32, IllegalPosition> {
        let n = self.count_piece(piece);
        if n > limit {
            return Err(IllegalPosition::TooManyPieces(piece, n));
        }
        Ok(n)
    }

    pub fn validate(&self) -> Result<(), IllegalPosition> {
        let mut occ: [u64; 2] = [0; 2];
        for piece in Piece::ALL {
            occ_mismatch(occ[piece.color() as usize] & self.bitboard[piece as usize])?;
            occ[piece.color() as usize] |= self.bitboard[piece as usize];
        }
        occ_mismatch(occ[White as usize] & occ[Black as usize])?;
        occ_mismatch(occ[White as usize] ^ self.occ(White))?;
        occ_mismatch(occ[Black as usize] ^ self.occ(Black))?;
        occ_mismatch(self.occ(White) & self.occ(Black) ^ self.full_occ())?;

        for s in  Square::ALL {
            if self.piece_at(s) != self.piece_at_bitboard(s) {
                return Err(
                    IllegalPosition::CorruptedBitboard(
                        CorruptedBitboard::OccupancyMismatch(s.mask())))
            }
        }

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

        for castling in Quadrant::ALL {
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

        let zobrist = self.compute_zobrist();
        if self.zobrist_hash != zobrist {
            return Err(
                IllegalPosition::CorruptedBitboard(
                    CorruptedBitboard::ZobristMismatch(zobrist, self.zobrist_hash)));
        }

        Ok(())
    }
}