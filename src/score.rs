use crate::chess::{Piece, PieceType, PieceColor};

const WHITE_KNIGHT_BONUS: [i32;64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50
];

const BLACK_KNIGHT_BONUS: [i32;64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50
];
    
const WHITE_BISHOP_BONUS: [i32;64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  0, 10, 15, 15, 10,  0,-10,
    -10,  5, 10, 15, 15, 10,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10,  5,  0,  0,  5, 10,-10,
    -20,-10,-10,-10,-10,-10,-10,-20
];

const BLACK_BISHOP_BONUS: [i32;64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10, 10,  5,  0,  0,  5, 10,-10, 
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10,  5, 10, 15, 15, 10,  5,-10,
    -10,  0, 10, 15, 15, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20
];

const WHITE_ROOK_BONUS: [i32;64] = [
    0,  0,  0,  5,  5,  0,  0,  0,   // Back rank connection bonus
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  5,  5,  0,  0, -5,   // Central files bonus
    5, 20, 20, 25, 25, 20, 20,  5,   // 7th rank extra strong
    0,  0,  5, 10, 10,  5,  0,  0 
];

const BLACK_ROOK_BONUS: [i32;64] = [
    0,  0,  5, 10, 10,  5,  0,  0,   // d/e files preferred
    5, 20, 20, 25, 25, 20, 20,  5,   // 2nd rank extra strong
    -5,  0,  0,  5,  5,  0,  0, -5,   // Central files bonus
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    5, 10, 10, 10, 10, 10, 10,  5,
    0,  0,  0,  5,  5,  0,  0,  0 
];

const WHITE_QUEEN_BONUS: [i32;64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  5,  0,-10,  // Slight kingside preference
    -10,  0,  5,  5,  5,  5,  5,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

const BLACK_QUEEN_BONUS: [i32;64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -10,  5,  5,  5,  5,  5,  0,-10,
    0,  0,  5,  5,  5,  5,  0, -5,
    -5,  0,  5,  5,  5,  5,  0, -5,
    -10,  0,  5,  5,  5,  5,  5,-10,
    -10,  0,  0,  0,  0,  5,  0,-10,  // Slight kingside preference
    -20,-10,-10, -5, -5,-10,-10,-20
];

const WHITE_KING_BONUS: [i32;64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20 
];

const BLACK_KING_BONUS: [i32;64] = [
    20, 30, 10,  0,  0, 10, 30, 20,   
    20, 20,  0,  0,  0,  0, 20, 20,   
    -10,-20,-20,-20,-20,-20,-20,-10,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30
];

const WHITE_PAWN_BONUS: [i32;64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    90, 90, 90, 90, 90, 90, 90, 90,
    30, 30, 40, 60, 60, 40, 30, 30,
    10, 10, 20, 40, 40, 20, 10, 10,
    5,  5, 10, 20, 20, 10,  5,  5,
    0,  0,  0,-10,-10,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

const BLACK_PAWN_BONUS: [i32;64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    0,  0,  0,-10,-10,  0,  0,  0,
    5,  5, 10, 20, 20, 10,  5,  5,
    10, 10, 20, 40, 40, 20, 10, 10,
    30, 30, 40, 60, 60, 40, 30, 30,
    90, 90, 90, 90, 90, 90, 90, 90,
    0,  0,  0,  0,  0,  0,  0,  0
];

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 300;
const BISHOP_VALUE: i32 = 300;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 1000000;



pub fn evaluate(board: [Option<Piece>; 64], turn: PieceColor) -> i32 {
    let mut eval = 0;

    for i in 0..64 {
        if let Some(piece) = board[i] {
            match piece.tp {
                PieceType::Pawn => {
                    if piece.color == PieceColor::White {
                        eval += PAWN_VALUE + WHITE_PAWN_BONUS[i];
                    } else {
                        eval -= PAWN_VALUE + BLACK_PAWN_BONUS[i];
                    }
                }

                PieceType::Knight => {
                    if piece.color == PieceColor::White {
                        eval += KNIGHT_VALUE + WHITE_KNIGHT_BONUS[i];
                    } else {
                        eval -= KNIGHT_VALUE + BLACK_KNIGHT_BONUS[i];
                    }
                }

                PieceType::Bishop => {
                    if piece.color == PieceColor::White {
                        eval += BISHOP_VALUE + WHITE_BISHOP_BONUS[i];
                    } else {
                        eval -= BISHOP_VALUE + BLACK_BISHOP_BONUS[i];
                    }
                }

                PieceType::Rook => {
                    if piece.color == PieceColor::White {
                        eval += ROOK_VALUE + WHITE_ROOK_BONUS[i];
                    } else {
                        eval -= ROOK_VALUE + BLACK_ROOK_BONUS[i];
                    }
                }

                PieceType::Queen => {
                    if piece.color == PieceColor::White {
                        eval += QUEEN_VALUE + WHITE_QUEEN_BONUS[i];
                    } else {
                        eval -= QUEEN_VALUE + BLACK_QUEEN_BONUS[i];
                    }
                }

                PieceType::King => {
                    if piece.color == PieceColor::White {
                        eval += KING_VALUE + WHITE_KING_BONUS[i];
                    } else {
                        eval -= KING_VALUE + BLACK_KING_BONUS[i];
                    }
                }
            }
        } else {}
    }
    if turn == PieceColor::White {eval} else {-eval}
}