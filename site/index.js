// var config = {
//   draggable: true,
//   dropOffBoard: 'snapback', // this is the default
//   position: 'start'
// }
// var board = Chessboard('myBoard', config)
import initSync, {set_state, get_state, get_state_fen, make_move, get_valid_moves, in_check, change_promotion, _eval, engine_move} from './node_modules/chess/chess.js';

async function setBoard(fen) {
    await initSync();
    set_state(fen);
    displayBoard();
}

async function displayBoard() {
    await initSync();
    state = get_state();
    console.log(state);
    for (let i=0; i<64; i++) {
        let piece = state.board[i];
        board.children[i].innerHTML = '';
        if (piece) {
            let image = document.createElement('img');
            image.style.width = '100%';
            image.style.height = '100%';
            image.src = `./img/${piece.color}${piece.tp}.png`;
            image.onclick = pieceClicked;
            board.children[i].appendChild(image);
        }
    }
    for (let square of board.children) {
        square.classList.remove('valid-move');
    }
    if (activePiece) {
        valid_moves = get_valid_moves(activePiece);
        for (let move of valid_moves) {
            let square = board.children[move];
            square.classList.add('valid-move');
        }
    }
    if (!game_over) {
        if (checkMate()) {
            game_over = true;
            // clearInterval(engine);
            alert(`${state.turn} loses by checkmate!`);
        } else if (stalemate()) {
            game_over = true;
            // clearInterval(engine);
            alert(`Stalemate! The game is a draw.`);
        }
    }
    console.log(`eval: ${_eval()}`);
}

function pieceClicked() {
    let square = this.parentNode;
    let squareNum = (7 - 'h'.charCodeAt(0) + square.id.charCodeAt(0)) + (parseInt(8-square.id[1]))*8;

    if (activePiece === squareNum) {
        activePiece = null; 
        displayBoard();
        return;
    }

    if (activePiece === null) {
        if (state.board[squareNum].color === state.turn) {
            activePiece = squareNum;
        }
        displayBoard();
        return;
    }

    valid_moves = get_valid_moves(activePiece);
    if (valid_moves.includes(squareNum)) {
        make_move(activePiece, squareNum);
        activePiece = null; 
    }
    else {
        if (state.board[squareNum].color === state.turn) {
            activePiece = squareNum;
        } else {
            activePiece = null;
        }
    }
    displayBoard();
}

function emptyClicked() {
    if (this.children.length > 0) {
        return;
    }
    let squareNum = (7 - 'h'.charCodeAt(0) + this.id.charCodeAt(0)) + (parseInt(8-this.id[1]))*8;
    valid_moves = get_valid_moves(activePiece);
    if (valid_moves.includes(squareNum)) {
        make_move(activePiece, squareNum);
    }
    activePiece = null; 
    displayBoard();
}

function checkMate() {
    if (in_check()) {
        for (let i = 0; i < 64; i++) {
            let piece = state.board[i];
            if (piece && piece.color === state.turn) {
                let moves = get_valid_moves(i);
                if (moves.length > 0) {
                    return false; // There are still valid moves
                }
            }
        }
        return true;
    }
    return false;
}
function stalemate() {
    if (!in_check()) {
        for (let i = 0; i < 64; i++) {
            let piece = state.board[i];
            if (piece && piece.color === state.turn) {
                let moves = get_valid_moves(i);
                if (moves.length > 0) {
                    return false; // There are still valid moves
                }
            }
        }
        return true;
    }
    return false;
}

let board = document.getElementById('board');
for (let i = 0; i < 64; i++) {
    let square = document.createElement('div');
    square.id = `${"abcdefgh"[i%8]}${8 - Math.floor(i/8)}`;
    square.classList.add(((i+ Math.floor(i/8)) % 2 === 0) ? 'white' : 'black');
    square.style.width = '75px';
    square.style.height = '75px';
    square.onclick = emptyClicked;
    board.appendChild(square);
}

let activePiece = null;
let valid_moves = [];
let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
let state = null;
let game_over = false;

setBoard(start);

// // Engine vs Player
const engine = setInterval(() => {
    if (state.turn === "White") return; 
    let x = engine_move();
    console.log(`Engine move: ${x}`);
    displayBoard();
}, 1000);

// Engine vs Engine
// engine = setInterval(() => {
//     if (game_over) return;
//     let x = engine_move();
//     console.log(`Engine move: ${x}`);
//     displayBoard();
// }, 100);

document.getElementById('fenSubmit').onclick = function() {
    let fen = document.getElementById('fenInput').value;
    let fallback = get_state_fen();
    console.log(fallback);
    setBoard(fen).catch(err => {
        setBoard(fallback);
        console.error("Error setting board:", err);
        alert("Invalid FEN string. Please check the format.");
    });
    game_over = false;
}

document.getElementById('reset').onclick = function() {
    setBoard(start);
}

document.getElementById('getfen').onclick = function() {
    document.getElementById('outputfen').value = get_state_fen();
}

document.getElementById('promPiece').onchange = function() {
    change_promotion(this.value);
}

// document.getElementById('engineMove').onclick = () => {
//     // await initSync();
//     let x = engine_move();
//     console.log(`Engine move: ${x}`);
//     displayBoard();
// }


