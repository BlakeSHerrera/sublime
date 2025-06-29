use {
    super::direction::*,
    Color::*,
};


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White = 0, Black,
}

impl Color {

    pub const BITS: u64 = 1;

    pub const ALL: [Color; 2] = [Black, White];

    pub const PIECE_ARR: [Color; 12] = [
        White, White, White, White, White, White,
        Black, Black, Black, Black, Black, Black,
    ];

    pub const fn inv(self) -> Color {
        match self {
            White => Black,
            Black => White
        }
    }

    pub const fn bb_offset(self) -> usize {
        6 * self as usize
    }

    pub const fn occ_offset(self) -> usize {
        12 + self as usize
    }

    pub const fn pawn_direction(self) -> Direction {
        match self {
            White => Direction::North,
            Black => Direction::South,
        }
    }
}
