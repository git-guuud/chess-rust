use std::i32;

use crate::chess::{self, in_check_state, make_move_on, State};
use crate::score::{evaluate};

pub fn find_move(state: &State, depth: usize) -> ((usize, usize), i32) {
    if depth == 0 {
        return ((64, 64), evaluate(state.board, state.turn));
    }

    let mut best_move = (64,64);
    let mut max = i32::MIN+1;

    let mut move_possible = false;

    
    for &(from, to) in chess::get_all_valid_moves(state).iter() {
        println!("{} {}", from, to);
        move_possible = true;
        let mut temp_state = state.clone();
        make_move_on(from, to, &mut temp_state);

        // let eval = -nega_max(&temp_state, depth-1);

        let eval = -alpha_beta(&temp_state, i32::MIN+1, -max, depth);

        if eval>max {
            max = eval;
            best_move = (from, to);
        }
    }

    if !move_possible {
        if in_check_state(state) {
            max += 1;
        } else {
            max = 0;
        }
    }
    (best_move, max)
}

fn nega_max(state: &State, depth: usize) -> i32 {
    if depth==0 {return evaluate(state.board, state.turn)}

    let mut max = i32::MIN;
    for &(from, to) in chess::get_all_valid_moves(state).iter() {
        let mut temp_state = state.clone();
        make_move_on(from, to, &mut temp_state);
        let eval = -nega_max(&temp_state, depth - 1);
        if max<eval {
            max = eval;
        }
    }
    max
}

// fn alpha_beta(state: &State, depth: usize) {
//     let max = i32::MIN;
//     let mut best_move = (64,64);

// }

fn alpha_beta(state: &State, mut alpha: i32, beta: i32, depth: usize) -> i32 {
    if depth == 0 {return evaluate(state.board, state.turn)}


    let mut max = i32::MIN+1;
    let moves = chess::get_all_valid_moves_raw(state);
    for &(from, to) in moves.iter() {
        let mut temp_state = state.clone();
        make_move_on(from, to, &mut temp_state);
        let eval = -alpha_beta(&temp_state, -beta, -alpha, depth-1);
        if eval>alpha {alpha = eval;}
        if eval>max {max = eval;}
        if eval>=beta {return eval;}
    }
    // if moves.len()==0 {
    //     if in_check_state(state) {
    //         max += 1;
    //     } else {
    //         max = 0;
    //     }
    // }
    max
}

