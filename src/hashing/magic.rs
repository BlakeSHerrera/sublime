/*
http://www.talkchess.com/forum3/viewtopic.php?topic_view=threads&p=175834&t=19699
https://www.chessprogramming.org/index.php?title=Looking_for_Magics#Feeding_in_Randoms
https://rhysre.net/fast-chess-move-generation-with-magic-bitboards.html

The actual magic numbers came from my previous project:
https://github.com/BlakeSHerrera/CS-3793-Chess-AI/blob/main/magic.c
*/

use super::bitmask::*;


pub const fn get_rook_moves(square: usize, blockers: u64) -> u64 {
    let i = ROOK_MAGICS[square]
        .wrapping_mul(blockers & ROOK_RELEVANT_OCCUPANCY[square])
        .wrapping_shr((64 - ROOK_BITS[square]) as u32);
    ROOK_TABLE[square][i as usize]
}

pub const fn get_bishop_moves(square: usize, blockers: u64) -> u64 {
    let i = BISHOP_MAGICS[square]
        .wrapping_mul(blockers & BISHOP_RELEVANT_OCCUPANCY[square])
        .wrapping_shr((64 - ROOK_BITS[square]) as u32);
    BISHOP_TABLE[square][i as usize]
}


const ROOK_MAGICS: [u64; 64] = [
    0x80004000976080,
    0x1040400010002000,
    0x4880200210000980,
    0x5280080010000482,
    0x200040200081020,
    0x2100080100020400,
    0x4280008001000200,
    0x1000a4425820300,
    0x29002100800040,
    0x4503400040201004,
    0x209002001004018,
    0x1131000a10002100,
    0x9000800120500,
    0x10e001804820010,
    0x29000402000100,
    0x2002000d01c40292,
    0x80084000200c40,
    0x10004040002002,
    0x201030020004014,
    0x80012000a420020,
    0x129010008001204,
    0x6109010008040002,
    0x950010100020004,
    0x803a0000c50284,
    0x80004100210080,
    0x200240100140,
    0x20004040100800,
    0x4018090300201000,
    0x4802010a00102004,
    0x2001000900040002,
    0x4a02104001002a8,
    0x2188108200204401,
    0x40400020800080,
    0x880402000401004,
    0x10040800202000,
    0x604410a02001020,
    0x200200206a001410,
    0x86000400810080,
    0x428200040600080b,
    0x2001000041000082,
    0x80002000484000,
    0x210002002c24000,
    0x401a200100410014,
    0x5021000a30009,
    0x218000509010010,
    0x4000400410080120,
    0x20801040010,
    0x29040040820011,
    0x4080400024800280,
    0x500200040100440,
    0x2880142001004100,
    0x412020400a001200,
    0x18c028004080080,
    0x884001020080401,
    0x210810420400,
    0x801048745040200,
    0x4401002040120082,
    0x408200210012,
    0x110008200441,
    0x2010002004100901,
    0x801000800040211,
    0x480d000400820801,
    0x820104201280084,
    0x1001040311802142,
];

const BISHOP_MAGICS: [u64; 64] = [
    0x420410208004484,
    0x90100a00a22000,
    0x90028a004b0200,
    0x14304202003100,
    0x84a0a1028080001,
    0x3008244420210004,
    0x104008450580000,
    0x480450400a01d14,
    0x40c0408020408,
    0x100208020c820200,
    0x84805242000,
    0x4091401004940,
    0x2820040420400009,
    0x49c008260200206,
    0x2012404040a1802,
    0x204301180202a4,
    0xc000203c850200,
    0x1010200803880180,
    0x103000810410200,
    0x400a440401020100,
    0x24c000210220801,
    0x8100a08021800,
    0x200a108010402,
    0x4006040022220200,
    0xb50041440080200,
    0x428600804010200,
    0x808080105020020,
    0xa81040040440080,
    0x104040008c10040,
    0x40401a011028,
    0x6020240883100,
    0x2422020640410080,
    0x401084000a30400,
    0x80a1212280800,
    0x44010400204044,
    0x12401821060200,
    0x406008400120020,
    0x4802180602004052,
    0x202220040120800,
    0x8088062810100,
    0x1008010860600800,
    0x1621004001028,
    0x40a010043080803,
    0x1003c010400200,
    0x400012012000100,
    0x4010200041016090,
    0x3040c00800c08,
    0x52020401100021,
    0x1400411008204200,
    0x800240104104840,
    0x11001440c040840,
    0x15120022a080100,
    0x2081101132020148,
    0x14051408020400,
    0x409107040810000,
    0xa0188301182000,
    0x820320304024020,
    0x8844022000,
    0x400001900411054,
    0x2045422010840400,
    0x1000000110021200,
    0x5404044004080082,
    0x40151c0c040400,
    0x2082280124040044,
];


const fn count_bits_64(arr: [u64; 64]) -> [u32; 64] {
    let mut result: [u32; 64] = [0; 64];
    let mut i = 0;
    while i < result.len() {
        result[i] = count_bits(arr[i]);
        i += 1;
    }
    result
}

pub const ROOK_BITS: [u32; 64] = count_bits_64(ROOK_RELEVANT_OCCUPANCY);
pub const BISHOP_BITS: [u32; 64] = count_bits_64(BISHOP_RELEVANT_OCCUPANCY);


const fn sliding_move(blockers: u64, r: i32, c: i32, dr: i32, dc: i32) -> u64 {
    let mut moves = NO_SQUARES;
    let mut r = r + dr;
    let mut c = c + dc;
    while 0 <= r && r < 8 && 0 <= c && c < 8 {
        let s = SQUARE[(8 * r + c) as usize];
        moves |= s;
        if s & blockers != 0 {
            break;
        }
        r += dr;
        c += dc;
    }
    moves
}

const fn sliding_moves(is_rook: bool, square: usize, blockers: u64) -> u64 {
    let mut moves = NO_SQUARES;
    let r = (square / 8) as i32;
    let c = (square % 8) as i32;
    let m = match is_rook {
        true => 0,
        false => 1
    };
    moves |= sliding_move(blockers, r, c, 1 * m, 1);
    moves |= sliding_move(blockers, r, c, 1, -1 * m);
    moves |= sliding_move(blockers, r, c, -1 * m, -1);
    moves |= sliding_move(blockers, r, c, -1, 1 * m);
    moves
}


const fn gen_magic_rook(
    &(mut arr): &[u64; 1 << 12],
    square: usize,
    blockers: u64, 
    mut i: usize
) -> [u64; 1 << 12] {
    assert!(square < 64);
    let j = ROOK_MAGICS[square]
        .wrapping_mul(blockers & ROOK_RELEVANT_OCCUPANCY[square])
        .wrapping_shr((64 - ROOK_BITS[square]) as u32);
    arr[j as usize] = sliding_moves(true, i, blockers);
    while i < 64 {
        if blockers & SQUARE[i] != 0 {
            gen_magic_rook(&mut arr, square, blockers, i + 1);
        }
        i += 1
    }
    arr
}

const fn gen_magic_bishop(
    &(mut arr): &[u64; 1 << 9],
    square: usize,
    blockers: u64,
    mut i: usize
) -> [u64; 1 << 9] {
    let j = BISHOP_MAGICS[square]
        .wrapping_mul(blockers & BISHOP_RELEVANT_OCCUPANCY[square])
        .wrapping_shr((64 - BISHOP_BITS[square]) as u32);
    arr[j as usize] = sliding_moves(false, i, blockers);
    while i < 64 {
        if blockers & SQUARE[square] != 0 {
            gen_magic_bishop(&mut arr, square, blockers, i + 1);
        }
        i += 1;
    }
    arr
}

pub const ROOK_TABLE: [[u64; 1 << 12]; 64] = {
    let mut arr: [[u64; 1 << 12]; 64] = [[0; 1 << 12]; 64];
    let mut i = 0;
    while i < arr.len() {
        arr[i] = gen_magic_rook(&arr[i], i, ROOK_RELEVANT_OCCUPANCY[i], 0);
        i += 1;
    }
    arr
};

pub const BISHOP_TABLE: [[u64; 1 << 9]; 64] = {
    let mut arr: [[u64; 1 << 9]; 64] = [[0; 1 << 9]; 64];
    let mut i = 0;
    while i < arr.len() {
        arr[i] = gen_magic_bishop(&arr[i], i, BISHOP_RELEVANT_OCCUPANCY[i], 0);
        i += 1
    }
    arr
};
