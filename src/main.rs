
use crate::chess::{State, state_from_fen};

mod search;
mod chess;
mod score;
mod zobrist;

// use crate::zobrist::generate_keys;


fn main() {
    let mut state = State::new();
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    state_from_fen(fen, &mut state).unwrap();
    let res = search::find_move(&state, 3);
    println!("{}, {}, {}", res.0 .0, res.0 .1, res.1);

    // println!("{:#?}", generate_keys());
}