use {
    crate::hashing::bitmask::*,
    Direction::*,
};


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
            North => (mask & !TOP_RANKS[n]) << 8 * n,
            East => (mask & !RIGHT_FILES[n]) << n,
            South => (mask & !BOTTOM_RANKS[n]) >> 8 * n,
            West => (mask & !LEFT_FILES[n]) >> n,
            Northwest => North.shift(West.shift(mask, n), n),
            Northeast => North.shift(East.shift(mask, n), n),
            Southeast => South.shift(East.shift(mask, n), n),
            Southwest => South.shift(West.shift(mask, n), n),
        }
    }
}
