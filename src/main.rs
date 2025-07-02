
use crate::chess::{State, state_from_fen};

mod search;
mod chess;
mod score;
fn main() {
    let mut state = State::new();
    let fen = "4QQ2/8/8/4k3/5N1P/5K2/P4R2/8 b - - 0 1".to_string();
    state_from_fen(fen, &mut state).unwrap();
    let res = search::find_move(&state, 3);
    println!("{}, {}, {}", res.0 .0, res.0 .1, res.1);
}