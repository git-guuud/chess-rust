mod chess;
use chess::*;

mod score;
mod search;

use wasm_bindgen::prelude::*;
use std::sync::{Mutex, OnceLock};


static STATE: OnceLock<Mutex<State>> = OnceLock::new();


#[wasm_bindgen]
pub fn set_state(fen: String) -> Result<(), String> {
    if fen.is_empty() {
        return Err("FEN string cannot be empty".to_string());
    }
    let state = STATE.get_or_init(|| Mutex::new(State::new()));
    let mut state = state.lock().unwrap();
    match state_from_fen(fen, &mut state) {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid FEN string".to_string()),
    }
}


#[wasm_bindgen]
pub fn get_state() -> JsValue {
    let state = STATE.get_or_init(|| Mutex::new(State::new()));
    let state = state.lock().unwrap();
    serde_wasm_bindgen::to_value(&(*state)).unwrap()
}

#[wasm_bindgen]
pub fn get_state_fen() -> String {
    let state = STATE.get_or_init(|| Mutex::new(State::new()));
    let state = state.lock().unwrap();
    
    let mut fen = String::new();
    let mut empty_count = 0;

    for rank in 0..8 {
        for file in 0..8 {
            match state.board[rank * 8 + file] {
                Some(piece) => {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push(match piece.tp {
                        PieceType::Pawn => if piece.color == PieceColor::White { 'P' } else { 'p' },
                        PieceType::Knight => if piece.color == PieceColor::White { 'N' } else { 'n' },
                        PieceType::Bishop => if piece.color == PieceColor::White { 'B' } else { 'b' },
                        PieceType::Rook => if piece.color == PieceColor::White { 'R' } else { 'r' },
                        PieceType::Queen => if piece.color == PieceColor::White { 'Q' } else { 'q' },
                        PieceType::King => if piece.color == PieceColor::White { 'K' } else { 'k' },
                    });
                }
                None => empty_count += 1,
            }
        }
        if empty_count > 0 {
            fen.push_str(&empty_count.to_string());
            empty_count = 0;
        }
        if rank < 7 {
            fen.push('/');
        }
    }

    fen.push(' ');
    fen.push(if state.turn == PieceColor::White { 'w' } else { 'b' });

    fen.push(' ');
    let castling: Vec<char> = state.castling.iter().enumerate()
        .filter_map(|(i, &can_castle)| if can_castle {
            match i {
                0 => Some('K'), // White kingside
                1 => Some('Q'), // White queenside
                2 => Some('k'), // Black kingside
                3 => Some('q'), // Black queenside
                _ => None,
            }
        } else {
            None
        }).collect();
    if castling.is_empty() {
        fen.push('-');
    } else {
        fen.extend(castling);
    }
    fen.push(' ');
    if state.en_passant == (8, 8) {
        fen.push('-');
    } else {
        let file = (state.en_passant.1 + 'a' as usize) as u8 as char;
        let rank = (8 - state.en_passant.0 + '0' as usize) as u8 as char;
        fen.push(file);
        fen.push(rank);
    }
    fen.push(' ');
    fen.push_str(&state.halfmove_clock.to_string());
    fen.push(' ');
    fen.push_str(&state.fullmove_number.to_string());
    fen
}

#[wasm_bindgen]
pub fn make_move(from: usize, to: usize) {
    let state = STATE.get_or_init(|| Mutex::new(State::new()));
    let mut state = state.lock().unwrap();

    make_move_on(from, to, &mut state);
}

#[wasm_bindgen]
pub fn get_valid_moves(from: usize) -> Vec<usize>{
    if from >= 64 {
        return Vec::new();
    }

    let state = STATE.get_or_init(|| Mutex::new(State::new()));
    let state = state.lock().unwrap();

    get_valid_moves_for(from, &state) 
}

#[wasm_bindgen]
pub fn in_check() -> bool {
    let state = STATE.get_or_init(|| Mutex::new(State::new()));
    let state = state.lock().unwrap();
    in_check_state(&state)
}

#[wasm_bindgen]
pub fn change_promotion(tp: String) {
    let state = STATE.get_or_init(|| Mutex::new(State::new()));
    let mut state = state.lock().unwrap();

    match tp.as_str() {
        "Queen" => state.promotion = PieceType::Queen,
        "Rook" => state.promotion = PieceType::Rook,
        "Bishop" => state.promotion = PieceType::Bishop,
        "Knight" => state.promotion = PieceType::Knight,
        _ => {}
    }
}

#[wasm_bindgen]
pub fn eval() -> i32{
    let state = STATE.get_or_init(|| Mutex::new(State::new()));
    let state = state.lock().unwrap();
    score::evaluate(state.board, state.turn)
}

#[wasm_bindgen]
pub fn engine_move() -> i32 {
    let state = STATE.get_or_init(|| Mutex::new(State::new()));
    let mut state = state.lock().unwrap();

    let res = search::find_move(&state, 4);
    make_move_on(res.0.0, res.0.1, &mut state);
    res.1
}