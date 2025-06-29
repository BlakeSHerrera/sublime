use sublime::{
    board::{
        color::Color::*,
        direction::Direction::*,
    },
    hashing::bitmask::*,
};


fn print_pause(msg: &str, mask: u64) {
    println!("{msg}");
    print(mask);
    use std::io::Write;
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut String::new());
}


fn main() {
    println!("Press enter to continue.");

    for (msg, mask) in [
        ("ALL_SQUARES", ALL_SQUARES),
        ("NO_SQUARES", NO_SQUARES),
        ("DARK_SQUARES", DARK_SQUARES),
        ("LIGHT_SQUARES", LIGHT_SQUARES),
        ("CENTER_FILES", CENTER_FILES),
        ("FLANK_FILES", FLANK_FILES),
        ("WING_FILES", WING_FILES),
        ("EDGE_FILES", EDGE_FILES),
        ("CENTER_RANKS", CENTER_RANKS),
        ("FLANK_RANKS", FLANK_RANKS),
        ("WING_RANKS", WING_RANKS),
        ("EDGE_RANKS", EDGE_RANKS),
        ("CENTER", CENTER),
        ("FLANKS", FLANKS),
        ("WINGS", WINGS),
        ("EDGES", EDGES),
    ] {
        print_pause(msg, mask);
    }

    for (msg, mask_arr) in [
        ("FILE", FILE.to_vec()),
        ("RANK", RANK.to_vec()),
        ("INNER_FILES", INNER_FILES.to_vec()),
        ("OUTER_FILES", OUTER_FILES.to_vec()),
        ("INNER_RANKS", INNER_RANKS.to_vec()),
        ("OUTER_RANKS", OUTER_RANKS.to_vec()),
        ("INNER_SQUARES", INNER_SQUARES.to_vec()),
        ("OUTER_SQUARES", OUTER_SQUARES.to_vec()),
        ("LEFT_FILES", LEFT_FILES.to_vec()),
        ("RIGHT_FILES", RIGHT_FILES.to_vec()),
        ("BOTTOM_RANKS", BOTTOM_RANKS.to_vec()),
        ("TOP_RANKS", TOP_RANKS.to_vec()),
        ("QUADRANTS", QUADRANTS.to_vec()),
        ("DIAGONAL", DIAGONAL.to_vec()),
        ("ANTI_DIAGONAL", ANTI_DIAGONAL.to_vec()),
        ("SQUARE", SQUARE.to_vec()),
        ("CASTLING_TO_CLEAR", CASTLING_TO_CLEAR.to_vec()),
        ("CASTLING_NO_ATTACK", CASTLING_NO_ATTACK.to_vec()),
        ("KING_MOVES", KING_MOVES.to_vec()),
        ("KNIGHT_MOVES", KNIGHT_MOVES.to_vec()),
        ("PAWN_ATTACKS[White]", PAWN_ATTACKS[White as usize].to_vec()),
        ("PAWN_ATTACKS[Black]", PAWN_ATTACKS[Black as usize].to_vec()),
        ("RAY_CAST[Northwest]", RAY_CAST[Northwest as usize].to_vec()),
        ("RAY_CAST[North]", RAY_CAST[North as usize].to_vec()),
        ("RAY_CAST[Northeast]", RAY_CAST[Northeast as usize].to_vec()),
        ("RAY_CAST[East]", RAY_CAST[East as usize].to_vec()),
        ("RAY_CAST[Southeast]", RAY_CAST[Southeast as usize].to_vec()),
        ("RAY_CAST[South]", RAY_CAST[South as usize].to_vec()),
        ("RAY_CAST[Southwest]", RAY_CAST[Southwest as usize].to_vec()),
        ("RAY_CAST[West]", RAY_CAST[West as usize].to_vec()),
        ("ROOK_RAYS", ROOK_RAYS.to_vec()),
        ("BISHOP_RAYS", BISHOP_RAYS.to_vec()),
        ("ROOK_RELEVANCE", ROOK_RELEVANT_OCCUPANCY.to_vec()),
        ("BISHOP_RELEVANCE", BISHOP_RELEVANT_OCCUPANCY.to_vec()),
    ] {
        for (i, v) in mask_arr.iter().enumerate() {
            let msg = std::fmt::format(format_args!("{}[{}]", msg, i));
            print_pause(&msg, *v);
        }
    }
}