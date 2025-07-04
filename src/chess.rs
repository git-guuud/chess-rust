use serde::Serialize;
use std::{collections::HashMap, ops::Not};

use crate::zobrist::ZOBRIST_KEYS;


#[derive(Clone, Copy, PartialEq, Serialize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Clone, Copy, PartialEq, Serialize)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Serialize)]
pub struct Piece {
    pub color: PieceColor,
    pub tp: PieceType,
}





#[derive(Clone, Serialize)]
pub struct State {
    #[serde(serialize_with = "<[_]>::serialize")]
    pub board: [Option<Piece>; 64],
    pub turn: PieceColor,
    pub castling: u32, // 0000qkQK
    pub en_passant: (usize, usize), // (rank, file)
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub promotion: PieceType,
    pub zob_hash: u64,
}

impl State {
    pub fn new() -> Self {
        State {
            board: [None; 64],
            turn: PieceColor::White,
            castling: 0,
            en_passant: (8, 8),
            halfmove_clock: 0,
            fullmove_number: 1,
            promotion: PieceType::Queen, 
            zob_hash: ZOBRIST_KEYS.castling[0],
        }
    }

    pub fn set_piece(&mut self, ind: usize, piece: Piece) {
        let mut x = if piece.color == PieceColor::Black {0} else {6};
        x += piece.tp as usize;
        self.zob_hash ^= ZOBRIST_KEYS.pieces[ind][x];
        self.board[ind] = Some(piece);
    }

    pub fn remove_piece(&mut self, ind: usize) {
        if let Some(piece) = self.board[ind] {
            let mut x = if piece.color == PieceColor::Black {0} else {6};
            x += piece.tp as usize;
            self.zob_hash ^= ZOBRIST_KEYS.pieces[ind][x];
            self.board[ind] = None;
        }
    }

    pub fn switch_turn(&mut self) {
        if self.turn == PieceColor::White {
            self.turn = PieceColor::Black;
        } else {
            self.turn = PieceColor::White;
        }
        self.zob_hash ^= ZOBRIST_KEYS.turn;
    }

    pub fn set_castle(&mut self, cs: u32) {
        if cs == self.castling {return;}
        self.zob_hash ^= ZOBRIST_KEYS.castling[self.castling as usize];
        self.castling = cs;
        self.zob_hash ^= ZOBRIST_KEYS.castling[cs as usize];
    }

    pub fn set_en_passant(&mut self, rank: usize, file: usize) {
        if (rank, file) == self.en_passant {return;}
        if self.en_passant.1 != 8 {
            self.zob_hash ^= ZOBRIST_KEYS.enpassant[self.en_passant.1];
        }
        self.en_passant = (rank, file);
        if file != 8 {
            self.zob_hash ^= ZOBRIST_KEYS.enpassant[file];
        }
    }
}

pub fn state_from_fen(fen: String, state:&mut State) -> Result<(),()> {

    let piece_from_char = HashMap::from([
        ('p', Piece {color: PieceColor::Black, tp: PieceType::Pawn}),
        ('P', Piece {color: PieceColor::White, tp: PieceType::Pawn}),
        ('n', Piece {color: PieceColor::Black, tp: PieceType::Knight}),
        ('N', Piece {color: PieceColor::White, tp: PieceType::Knight}),
        ('b', Piece {color: PieceColor::Black, tp: PieceType::Bishop}),
        ('B', Piece {color: PieceColor::White, tp: PieceType::Bishop}),
        ('r', Piece {color: PieceColor::Black, tp: PieceType::Rook}),
        ('R', Piece {color: PieceColor::White, tp: PieceType::Rook}),
        ('q', Piece {color: PieceColor::Black, tp: PieceType::Queen}),
        ('Q', Piece {color: PieceColor::White, tp: PieceType::Queen}),
        ('k', Piece {color: PieceColor::Black, tp: PieceType::King}),
        ('K', Piece {color: PieceColor::White, tp: PieceType::King}),
    ]);
    let parts = fen.split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 6 {
        return Err(());
    }
    let mut rank = 7;
    let mut file = 0;
    for c in parts[0].chars() {
        match c {
            '1'..='9' => {
                let emp = c.to_digit(10).unwrap() as usize;
                for i in 0..emp {
                    state.remove_piece((7-rank)*8 + file + i);
                }
                file += emp;
            }
            '/' => {
                if file != 8 {
                    return Err(());
                }
                rank -= 1;
                file = 0;
            }
            'p' | 'P' | 'n' | 'N' | 'b' | 'B' | 'r' | 'R' | 'q' | 'Q' | 'k' | 'K' => {
                state.set_piece((7-rank)*8 + file, piece_from_char[&c]);
                file += 1;
            }
            _ => {return Err(())}
        }
    }
    if rank != 0 || file != 8 {
        return Err(());
    }

    state.turn = PieceColor::White;
    match parts[1] {
        "w" => {},
        "b" => state.switch_turn(),
        _ => return Err(()),
    };

    state.set_castle(0);
    if parts[2] != "-" {

        let mut cs = 0;
        for c in parts[2].chars() {
            match c {
                'K' => cs |= 1, // White kingside
                'Q' => cs |= 1<<1, // White queenside
                'k' => cs |= 1<<2, // Black kingside
                'q' => cs |= 1<<3, // Black queenside
                _ => return Err(()),
            }
        }
        state.set_castle(cs);
    }

    state.set_en_passant(8, 8);
    if parts[3] != "-" {
        if parts[3].len() != 2 {
            return Err(());
        }
        let file = match parts[3].chars().nth(0) {
            Some(c) if c.is_ascii_lowercase() => c as usize - 'a' as usize,
            _ => return Err(()),
        };
        let rank = match parts[3].chars().nth(1) {
            Some(c) if c.is_ascii_digit() => 8 - (c.to_digit(10).unwrap() as usize),
            _ => return Err(()),
        };
        if file > 7 || rank > 7 {
            return Err(());
        }
        state.set_en_passant(rank, file);
    }

    if let Ok(halfmove_clock) = parts[4].parse::<u32>() {
        state.halfmove_clock = halfmove_clock;
    } else {
        return Err(());
    }

    if let Ok(fullmove_number) = parts[5].parse::<u32>() {
        state.fullmove_number = fullmove_number;
    } else {
        return Err(());
    }

    if state.halfmove_clock > 100 {
        return Err(());
    }

    return Ok(());
}

pub fn make_move_on(from: usize, to: usize, state: &mut State) {
    if from >= 64 || to >= 64 || from == to {
        return;
    }

    let mut piece = match state.board[from] {
        Some(p) if p.color == state.turn => p,
        _ => return,
    };

    //en passant check
    if piece.tp == PieceType::Pawn && state.en_passant.0 * 8 + state.en_passant.1 == to {
        if piece.color == PieceColor::White {
            state.remove_piece(to + 8); 
        }
        else {
            state.remove_piece(to - 8); 
        }
    }

    state.set_en_passant(8, 8);
    if piece.tp == PieceType::Pawn {
        if from.abs_diff(to) == 16 {
            state.set_en_passant(
                if piece.color == PieceColor::White {from/8-1} else {from/8+1}, 
                from%8
            );
        }
    } 

    //Pawn promotion
    if piece.tp == PieceType::Pawn {
        if (piece.color == PieceColor::White && to < 8) || (piece.color == PieceColor::Black && to >= 56) {
            piece = Piece {color: piece.color, tp: state.promotion};
        }
    }

    //Castling check
    if piece.tp == PieceType::King {
        if from == 60 && to == 62 { // White kingside castling
            if (state.castling & 1) == 1 {
                state.set_piece(61, Piece {color: PieceColor::White, tp: PieceType::Rook});
                state.remove_piece(63);
            }
        } else if from == 60 && to == 58 { // White queenside castling
            if (state.castling & 2) == 2 {
                state.set_piece(59, Piece {color: PieceColor::White, tp: PieceType::Rook});
                state.remove_piece(56);
            }
        } else if from == 4 && to == 6 { // Black kingside castling
            if (state.castling & 4) == 4{
                state.set_piece(5, Piece {color: PieceColor::Black, tp: PieceType::Rook});
                state.remove_piece(7);
            }
        } else if from == 4 && to == 2 { // Black queenside castling
            if (state.castling&8) == 8 {
                state.set_piece(3,Piece {color: PieceColor::Black, tp: PieceType::Rook});
                state.remove_piece(0);
            }
        } 
        if piece.color == PieceColor::White {
            state.set_castle(state.castling & 3u32.not());
        } else {
            state.set_castle(state.castling & 12u32.not());
        }
    } else if piece.tp == PieceType::Rook {
        if from == 63 { 
            state.set_castle(state.castling & 1u32.not()); 
        } else if from == 56 { 
            state.set_castle(state.castling & 2u32.not()); 
        } else if from == 7 { 
            state.set_castle(state.castling & 4u32.not()); 
        } else if from == 0 { 
            state.set_castle(state.castling & 8u32.not());
        }
    }
    // disable castling if rook captured
    if to == 63 { 
        state.set_castle(state.castling & 1u32.not()); 
    } else if to == 56 { 
        state.set_castle(state.castling & 2u32.not()); 
    } else if to == 7 { 
        state.set_castle(state.castling & 4u32.not()); 
    } else if to == 0 { 
        state.set_castle(state.castling & 8u32.not()); 
    }



    state.set_piece(to, piece);
    state.remove_piece(from);

    state.switch_turn();
}

pub fn in_check_state(state: & State) -> bool {
    for i in 0..64 {
        match state.board[i] {
            Some(piece) if piece.color != state.turn => {
                for &to in &get_valid_moves_for_raw(i, state) {
                    if let Some(target_piece) = state.board[to] {
                        if target_piece.tp == PieceType::King && target_piece.color == state.turn {
                            return true
                        }
                    }
                }
            }
            _ => continue,
        }
    }
    false
}

pub fn get_valid_moves_for_raw(from: usize, state: &State) -> Vec<usize> {
    if from >= 64 {
        return Vec::new();
    }

    let piece = match state.board[from] {
        Some(p) => p,
        None => return Vec::new(),
    };

    let mut valid_moves = Vec::new();

    match piece.tp {
        PieceType::Rook | PieceType::Bishop | PieceType::Queen => {

            let dir = if piece.tp == PieceType::Rook {
                vec![(-1, 0), (1, 0), (0, -1), (0, 1)] 
            } else if piece.tp == PieceType::Bishop {
                vec![(-1, -1), (-1, 1), (1, -1), (1, 1)]
            } else {
                vec![(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)]
            };

            for (dx, dy) in dir {
                let mut x = from as i16 % 8;
                let mut y = from as i16 / 8;

                loop {
                    x += dx;
                    y += dy;

                    if x < 0 || x >= 8 || y < 0 || y >= 8 {
                        break;
                    }

                    let pos = (y * 8 + x) as usize;

                    match state.board[pos] {
                        Some(p) if p.color != piece.color => {
                            valid_moves.push(pos);
                            break;
                        }
                        None => valid_moves.push(pos),
                        _ => break,
                    }
                }
            }
        }

        PieceType::Knight => {
            let knight_moves = vec![
                (-2, -1), (-1, -2), (1, -2), (2, -1),
                (2, 1), (1, 2), (-1, 2), (-2, 1)
            ];
            for (dx, dy) in knight_moves {
                let x = from as i16 % 8 + dx;
                let y = from as i16 / 8 + dy;

                if x >= 0 && x < 8 && y >= 0 && y < 8 {
                    let pos = (y * 8 + x) as usize;
                    match state.board[pos] {
                        Some(p) if p.color != piece.color => valid_moves.push(pos),
                        None => valid_moves.push(pos),
                        _ => {}
                    }
                }
            }
        }

        PieceType::King => {
            let king_moves = vec![
                (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)
            ];
            for (dx, dy) in king_moves {
                let x = from as i16 % 8 + dx;
                let y = from as i16 / 8 + dy;

                if x >= 0 && x < 8 && y >= 0 && y < 8 {
                    let pos = (y * 8 + x) as usize;
                    match state.board[pos] {
                        Some(p) if p.color != piece.color => valid_moves.push(pos),
                        None => valid_moves.push(pos),
                        _ => {}
                    }
                }
            }

            // Castling
            if piece.color == PieceColor::White {
                if (state.castling&1)==1 && state.board[61].is_none() && state.board[62].is_none() {
                    valid_moves.push(62); // White kingside castling
                }
                if state.castling&2 == 2 && state.board[59].is_none() && state.board[58].is_none() && state.board[57].is_none() {
                    valid_moves.push(58); // White queenside castling
                }
            } else {
                if state.castling&4 == 4 && state.board[5].is_none() && state.board[6].is_none() {
                    valid_moves.push(6); // Black kingside castling
                }
                if state.castling&8 == 8 && state.board[3].is_none() && state.board[2].is_none() && state.board[1].is_none() {
                    valid_moves.push(2); // Black queenside castling
                }
            }
        }

        PieceType::Pawn => {
            let en_p = state.en_passant.0 * 8 + state.en_passant.1;
            if piece.color == PieceColor::White {
                if from >= 8 && state.board[from - 8].is_none() {
                    valid_moves.push(from - 8);
                    if from / 8 == 6 && state.board[from - 16].is_none() {
                        valid_moves.push(from - 16);
                    }
                }
                if from % 8 > 0 && from >= 8 && state.board[from - 9].map_or(from-9 == en_p, |p| p.color == PieceColor::Black) {
                    valid_moves.push(from - 9);
                }
                if from % 8 < 7 && from >= 8 && state.board[from - 7].map_or(from-7 == en_p, |p| p.color == PieceColor::Black) {
                    valid_moves.push(from - 7);
                }
            } else {
                if from < 56 && state.board[from + 8].is_none() {
                    valid_moves.push(from + 8);
                    if from / 8 == 1 && state.board[from + 16].is_none() {
                        valid_moves.push(from + 16);
                    }
                }
                if from % 8 > 0 && from < 56 && state.board[from + 7].map_or(from+7 == en_p, |p| p.color == PieceColor::White) {
                    valid_moves.push(from + 7);
                }
                if from % 8 < 7 && from < 56 && state.board[from + 9].map_or(from+9 == en_p, |p| p.color == PieceColor::White) {
                    valid_moves.push(from + 9);
                }
            }
        }
    }

    valid_moves
}

pub fn get_valid_moves_for(from: usize, state: &State) -> Vec<usize> {
    if from >= 64 {
        return Vec::new();
    }

    let _piece = match state.board[from] {
        Some(p) => p,
        None => return Vec::new(),
    };

    let mut valid_moves = Vec::new();

    // Get all potential moves for the piece
    let potential_moves = get_valid_moves_for_raw(from, state);

    // Filter out moves that would leave the king in check
    for &to in &potential_moves {
        let mut temp_state = state.clone();
        make_move_on(from, to, &mut temp_state);
        temp_state.switch_turn();

        if !in_check_state(&temp_state) {
            valid_moves.push(to);
        }
    }

    valid_moves
}

pub fn get_all_valid_moves_raw(state: &State) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for i in 0..63 {
        if let Some(p) = state.board[i] {
            if p.color != state.turn {continue;}
        }
        for &to in get_valid_moves_for_raw(i, state).iter() {
            res.push((i, to));
        }
    }
    res
}

pub fn get_all_valid_moves(state: &State) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for i in 0..63 {
        if let Some(p) = state.board[i] {
            if p.color != state.turn {continue;}
        }
        for &to in get_valid_moves_for(i, state).iter() {
            res.push((i, to));
        }
    }
    res
}