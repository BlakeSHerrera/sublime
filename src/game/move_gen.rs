use {
    crate::{
        board::{
            color::{*, Color::*},
            direction::Direction::*,
            piece::{
                *,
                GenericPiece::*,
                Moveset::*,
            },
            square::*,
            zone::*,
        },
        hashing::{
            bitmask::*,
            magic::*,
        },
    },
    super::{
        board_move::*,
        position::*,
    },
};


pub const MAX_LEGAL_MOVES: usize = 321;


impl Moveset {

    pub const fn pseudo_legal_threats(self, state: &GameState, square: Square) -> u64 {
        match self {
            RookMove => get_rook_moves(square as usize, state.bitboard[FULL_OCCUPANCY]),
            KnightMove => KNIGHT_MOVES[square as usize],
            BishopMove => get_bishop_moves(square as usize, state.bitboard[FULL_OCCUPANCY]),
            QueenMove => get_queen_moves(square as usize, state.bitboard[FULL_OCCUPANCY]),
            KingMove => KING_MOVES[square as usize],
            WhitePawnMove => PAWN_ATTACKS[White as usize][square as usize] 
                & (state.bitboard[BLACK_OCCUPANCY] | state.ep_mask()),
            BlackPawnMove => PAWN_ATTACKS[Black as usize][square as usize] 
                & (state.bitboard[WHITE_OCCUPANCY] | state.ep_mask()),
        }
    }

    pub fn pseudo_legal_nonthreats(self, state: &mut GameState, square: Square) -> u64 {
        match self {
            KingMove => {
                // The turn is temporarily flipped to check threats
                state.set_turn(state.turn().inv());
                let mut mask: u64 = 0;
                for q in Quadrant::ALL {
                    let blockers = state.bitboard[FULL_OCCUPANCY];
                    let castling_cond = state.has_castling_rights(q)
                        && state.turn() != q.color()  // Recall the turn has been flipped
                        && q.to_clear_mask() & blockers == 0
                        && q.no_attack_mask() & state.pseudo_legal_threats(state.turn()) == 0;
                    mask |= match castling_cond {
                        true => q.king_end().mask(),
                        false => 0,
                    };
                }
                state.set_turn(state.turn().inv());
                mask
            },
            WhitePawnMove => {
                let blockers =  state.bitboard[FULL_OCCUPANCY] & !square.mask();
                PAWN_MOVES[White as usize][square as usize] 
                    & !blockers 
                    & !North.shift(blockers, 1)
            },
            BlackPawnMove => {
                let blockers = state.bitboard[FULL_OCCUPANCY] & !square.mask();
                PAWN_MOVES[Black as usize][square as usize]
                    & !blockers
                    & !South.shift(blockers, 1)
            },
            _ => 0,
        }
    }

    pub fn pseudo_legal_moves(self, state: &mut GameState, square: Square) -> u64 {
        // Moves are all legal unless it puts self in check.
        !state.self_occ() & (
            self.pseudo_legal_threats(state, square) 
            | self.pseudo_legal_nonthreats(state, square))
    }
}


impl GameState {

    pub fn pseudo_legal_threats(&self, color: Color) -> u64 {
        let mut mask = 0;
        for piece in Piece::pieces_of_color(color) {
            let mut piece_mask = self.bitboard[piece as usize];
            while piece_mask != 0 {
                let square = Square::ALL[piece_mask.trailing_zeros() as usize];
                mask |= piece.moveset().pseudo_legal_threats(self, square);
                piece_mask ^= square.mask();
            }
        }
        mask
    }

    pub fn generate_pseudo_legal_moves(&mut self, moves: &mut [Move; MAX_LEGAL_MOVES]) -> usize {
        // Returns number of pseudo-legal moves
        let mut i = 0;
        for piece in Piece::pieces_of_color(self.turn()) {
            let mut piece_mask = self.bitboard[piece as usize];
            while piece_mask != 0 {
                let origin = Square::ALL[piece_mask.trailing_zeros() as usize];
                let mut moves_mask = piece.moveset().pseudo_legal_moves(self, origin);
                while moves_mask != 0 {
                    let dest = Square::ALL[moves_mask.trailing_zeros() as usize];
                    match (piece.as_generic(), dest.is_promotion_square()) {
                        (Pawn, true) => for promo in Promotion::ALL {
                            moves[i] = Move::new(self, origin, dest, Some(promo));
                            i += 1;
                        },
                        _ => {
                            moves[i] = Move::new(self, origin, dest, None);
                            i += 1;
                        },
                    }
                    moves_mask ^= dest.mask();
                }
                piece_mask ^= origin.mask();
            }
        }
        i
    }

    pub fn in_check(&self, color: Color) -> bool {
        0 != self.bitboard[King.as_color(color) as usize] & self.pseudo_legal_threats(color.inv())
    }

    pub fn is_legal(&mut self, mv: Move) -> bool {
        // Only check for self-checks. Else guaranteed to be legal.
        self.push_partial(mv);
        let result = self.in_check(self.turn());
        self.pop_partial(mv);
        result
    }

    pub fn generate_legal_moves(&mut self, moves: &mut [Move; MAX_LEGAL_MOVES]) -> usize {
        // Returns number of legal moves
        let mut n = self.generate_pseudo_legal_moves(moves);
        let mut i = 0;
        while i < n {
            match self.is_legal(moves[i]) {
                true => i += 1,
                false => {
                    moves[i] = moves[n - 1];
                    n -= 1;
                }
            }
        }
        n
    }
}
