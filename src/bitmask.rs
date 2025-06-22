use crate::square::Square::*;

use Direction::*;


pub fn print(mask: u64) {
    for i in 0..8 {
        let j = 7 - i;
        let b = (mask & RANK[j]) >> 8 * j;
        for k in 0..8 {
            print!("{} ", match b & FILE[k] != 0 {true => "X", false => "."});
        }
        println!("");
    }
}


pub const fn count_bits(mut mask: u64) -> u32 {
    let mut count = 0;
    while mask != 0 {
        count += 1;
        mask = mask & !(1 << mask.trailing_zeros());
    }
    count
}


pub const fn shift_left(mask: u64, n: usize) -> u64 {
    (mask & !LEFT_FILES[n]) >> n
}

pub const fn shift_right(mask: u64, n: usize) -> u64 {
    (mask & !RIGHT_FILES[n]) << n
}

pub const fn shift_down(mask: u64, n: usize) -> u64 {
    (mask & !BOTTOM_RANKS[n]) >> 8 * n
}

pub const fn shift_up(mask: u64, n: usize) -> u64 {
    (mask & !TOP_RANKS[n]) << 8 * n
}


pub const NO_SQUARES: u64 = 0;
pub const ALL_SQUARES: u64 = !NO_SQUARES;
pub const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;
pub const LIGHT_SQUARES: u64 = !DARK_SQUARES;


const fn gen_lines(seed: u64, is_file: bool) -> [u64; 8] {
    let mut arr: [u64; 8] = [seed; 8];
    let mut i = 0;
    while i < arr.len() {
        arr[i] <<= match is_file {
            true => i,
            false => 8 * i,
        };
        i += 1;
    }
    arr
}

pub const FILE: [u64; 8] = gen_lines(0x0101010101010101, true);
pub const RANK: [u64; 8] = gen_lines(0xFF, false);

pub const CENTER_FILES: u64 = FILE[3] | FILE[4];
pub const FLANK_FILES: u64 = FILE[2] | FILE[5];
pub const WING_FILES: u64 = FILE[1] | FILE[6];
pub const EDGE_FILES: u64 = FILE[0] | FILE[7];

pub const CENTER_RANKS: u64 = RANK[3] | RANK[4];
pub const FLANK_RANKS: u64 = RANK[2] | RANK[5];
pub const WING_RANKS: u64 = RANK[1] | RANK[6];
pub const EDGE_RANKS: u64 = RANK[0] | RANK[7];

pub const EDGES: u64 = EDGE_FILES | EDGE_RANKS; 
pub const WINGS: u64 = (WING_FILES | WING_RANKS) & !EDGES;
pub const FLANKS: u64 = (FLANK_FILES | FLANK_RANKS) & !EDGES & !WINGS;
pub const CENTER: u64 = CENTER_FILES & CENTER_RANKS;

pub const INNER_FILES: [u64; 4] = [
    CENTER_FILES,
    CENTER_FILES | FLANK_FILES,
    ALL_SQUARES & !EDGE_FILES,
    ALL_SQUARES
];
pub const OUTER_FILES: [u64; 4] = [
    EDGE_FILES,
    EDGE_FILES | WING_FILES,
    ALL_SQUARES & !CENTER_FILES,
    ALL_SQUARES
];
pub const INNER_RANKS: [u64; 4] = [
    CENTER_RANKS,
    CENTER_RANKS | FLANK_RANKS,
    ALL_SQUARES & !EDGE_RANKS,
    ALL_SQUARES
];
pub const OUTER_RANKS: [u64; 4] = [
    EDGE_RANKS,
    EDGE_RANKS | WING_RANKS,
    ALL_SQUARES & !CENTER_RANKS,
    ALL_SQUARES
];
pub const INNER_SQUARES: [u64; 4] = [
    CENTER,
    CENTER | FLANKS,
    ALL_SQUARES & !EDGES,
    ALL_SQUARES
];
pub const OUTER_SQUARES: [u64; 4] = [
    EDGES,
    EDGES | WINGS,
    ALL_SQUARES & !CENTER,
    ALL_SQUARES
];


const fn cumulative(arr: [u64; 8], rev: bool) -> [u64; 9] {
    let mut result: [u64; 9] = [0; 9];
    let mut i = 1;
    while i < result.len() {
        let j = match rev {
            true => 8 - i,
            false => i - 1
        };
        result[i] = result[i - 1] | arr[j];
        i += 1;
    }
    result
}

pub const LEFT_FILES: [u64; 9] = cumulative(FILE, false);
pub const RIGHT_FILES: [u64; 9] = cumulative(FILE, true);
pub const BOTTOM_RANKS: [u64; 9] = cumulative(RANK, false);
pub const TOP_RANKS: [u64; 9] = cumulative(RANK, true);


const fn gen_diags(seed: u64) -> [u64; 15] {
    let mut arr: [u64; 15] = [0; 15];
    let mut i = 0;
    while i < arr.len() {
        arr[i] = match i < 7 {
            true => shift_left(seed, 7 - i),
            false => shift_right(seed, i - 7),
        };
        i += 1;
    }
    arr
}

pub const MAIN_DIAGONAL: u64 = 0x8040201008040201;
pub const MAIN_ANTIDIAGONAL: u64 = 0x0102040810204080;

pub const DIAGONAL: [u64; 15] = gen_diags(MAIN_DIAGONAL);
pub const ANTI_DIAGONAL: [u64; 15] = gen_diags(MAIN_ANTIDIAGONAL);


pub const SQUARE: [u64; 64] = {
    let mut arr: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < arr.len() {
        arr[i] = 1 << i;
        i += 1;
    }
    arr
};


pub const CASTLING_TO_CLEAR: [u64; 4] = [
    F1.mask() | G1.mask(),
    D1.mask() | C1.mask() | B1.mask(),
    F8.mask() | G8.mask(),
    D8.mask() | C8.mask() | B8.mask(),
];
pub const CASTLING_NO_ATTACK: [u64; 4] = [
    E1.mask() | F1.mask() | G1.mask(),
    E1.mask() | D1.mask() | C1.mask(),
    E8.mask() | F8.mask() | G8.mask(),
    E8.mask() | D8.mask() | C8.mask(),
];


pub const KING_MOVES: [u64; 64] = {
    let mut arr: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < arr.len() {
        arr[i] = SQUARE[i];
        arr[i] |= shift_up(arr[i], 1);
        arr[i] |= shift_down(arr[i], 1);
        arr[i] |= shift_left(arr[i], 1);
        arr[i] |= shift_right(arr[i], 1);
        arr[i] ^= SQUARE[i];
        i += 1;
    }
    arr
};

pub const KNIGHT_MOVES: [u64; 64] = {
    let mut arr: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < arr.len() {
        let s = SQUARE[i];
        arr[i] = shift_left(shift_down(s, 2), 1)
            | shift_right(shift_down(s, 2), 1)
            | shift_left(shift_up(s, 2), 1)
            | shift_right(shift_up(s, 2), 1)
            | shift_up(shift_left(s, 2), 1)
            | shift_down(shift_left(s, 2), 1)
            | shift_up(shift_right(s, 2), 1)
            | shift_down(shift_right(s, 2), 1);
        i += 1;
    }
    arr
};

const fn gen_pawn_attacks(is_white: bool) -> [u64; 64] {
    let mut arr: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < arr.len() {
        let s = SQUARE[i];
        arr[i] = match is_white {
            true => Northwest.shift(s, 1) 
                | Northeast.shift(s, 1),
            false => Southwest.shift(s, 1) 
                | Southeast.shift(s, 1)
        };
        i += 1;
    }
    arr
}

pub const PAWN_ATTACKS: [[u64; 64]; 2] = [
    gen_pawn_attacks(true), 
    gen_pawn_attacks(false)
];


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Northwest = 0, North, Northeast, East,
    Southeast, South, Southwest, West
}

impl Direction {
    
    pub const fn rays(self) -> [u64; 64] {
        RAY_CAST[self as usize]
    }

    pub const fn ray(self, i: usize) -> u64 {
        self.rays()[i]
    }

    pub const ALL: [Direction; 8] = [
        Northwest, North, Northeast, East,
        Southeast, South, Southwest, West
    ];

    pub const fn shift(self, mask: u64, n: usize) -> u64 {
        match self {
            North => shift_up(mask, n),
            East => shift_right(mask, n),
            South => shift_down(mask, n),
            West => shift_left(mask, n),
            Northwest => shift_up(shift_left(mask, n), n),
            Northeast => shift_up(shift_right(mask, n), n),
            Southeast => shift_down(shift_right(mask, n), n),
            Southwest => shift_down(shift_left(mask, n), n),
        }
    }
}

// Ray cast includes starting square
pub const RAY_CAST: [[u64; 64]; 8] = {
    let mut arr: [[u64; 64]; 8] = [[0; 64]; 8];
    let mut i = 0;
    while i < Direction::ALL.len() {
        let mut j =  0;
        while j < 64 {
            let mut k = 0;
            while k < 8 {
                arr[i][j] |= Direction::ALL[i].shift(SQUARE[j], k);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    arr
};


const fn gen_piece_rays(relevant_occupancy_only: bool, is_rook: bool) -> [u64; 64] {
    let mut arr: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < arr.len() {
        let rays = match is_rook {
            true => North.ray(i)
                | East.ray(i)
                | South.ray(i)
                | West.ray(i),
            false => Northwest.ray(i)
                | Northeast.ray(i)
                | Southeast.ray(i)
                | Southwest.ray(i)
        };
        arr[i] = match relevant_occupancy_only {
            true => rays & !EDGES & !SQUARE[i],
            false => rays,
        };
        i += 1;
    }
    arr
}

pub const ROOK_RAYS: [u64; 64] = gen_piece_rays(false, true);
pub const BISHOP_RAYS: [u64; 64] = gen_piece_rays(false, false);

// Relevant occupied squares
pub const ROOK_RELEVANT_OCCUPANCY: [u64; 64] = gen_piece_rays(true, true);
pub const BISHOP_RELEVANT_OCCUPANCY: [u64; 64] = gen_piece_rays(true, false);
