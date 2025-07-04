# Intro
A simple chess engine written in Rust that runs on the web (using wasm).

[Try it out!](https://chess-rust-dun.vercel.app/)

# Implementation details

Right now, the engine is a simple minimax implementation with alpha-beta pruning (+negaMax). It uses a basic evaluation function that considers material balance and position based "bonuses" for the pieces only.

# Features implemented
- [x] Basic chess rules
- [x] Move generation
- [x] Basic evaluation function
- [x] NegaMax with alpha-beta pruning
- [x] Zobrist hashing for position representation

# To be added (in order of priority)
- [ ] Transposition tables
- [ ] Iterative deepening
- [ ] Universal chess interface (UCI) integration
- [ ] Move ordering
- [ ] Quiescence search
- [ ] Bitboard representation
- [ ] Move generation using magic bitboards
- [ ] Opening book
- [ ] More advanced evaluation function
- [ ] Endgame tablebases