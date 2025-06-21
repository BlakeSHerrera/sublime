use sublime::perft;
use sublime::position::*;


fn print_board(fen: &str) {
    println!("Fen is: {}", fen);
    match GameState::from_fen(fen) {
        Err(e) => println!("{:?}", e),
        Ok(state) => match state.validate() {
            Err(e) => println!("{:?}", e),
            Ok(_) => {
                println!("Board (pretty):");
                state.print_pretty();
            }
        }
    }
    println!("");
}

fn main() {
    for i in perft::FENS {
        print_board(i);
    }
}