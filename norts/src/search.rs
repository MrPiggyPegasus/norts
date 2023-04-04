/*
Copyright (c) 2023. "MrPiggyPegasus"
This file is part of the "norts" Noughts and Crosses engine, see https://github.com/MrPiggyPegasus/norts.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NON INFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::bitboards::Bitboard;

/// Uses a strong solved minimax algorithm with alpha-beta pruning
/// to search the game tree.
/// Moves are played and then undone to avoid the memory intense process
/// of copying the board.
pub fn search(pos: &mut Bitboard, mut alpha: i8, mut beta: i8) -> (i8, u8) {
    if pos.x_won() {
        return (100 - (pos.num_moves() as i8), 9);
    }
    if pos.o_won() {
        return (-100 + (pos.num_moves() as i8), 9);
    }
    if pos.is_draw() {
        return (0, 9);
    }
    // if X is playing, the engine wants to maximise the eval
    return if pos.current_player() {
        let mut max_eval = i8::MIN + 10;
        let mut max_move: u8 = 9;
        for square in 0..9 {
            if !pos.is_legal(square) {
                continue;
            }
            pos.play(square);
            let eval = search(pos, alpha, beta).0;
            pos.clear_square(square);
            if eval >= max_eval {
                max_eval = eval;
                max_move = square;
                if eval > beta {
                    break;
                }
                if eval > alpha {
                    alpha = eval;
                }
            }
        }
        (max_eval, max_move)
    } else {
        // if O is playing, the engine wants to minimise the eval
        let mut min_eval = i8::MAX - 10;
        let mut min_move: u8 = 9;
        for square in 0..9 {
            if !pos.is_legal(square) {
                continue;
            }
            pos.play(square);
            let eval = search(pos, alpha, beta).0;
            pos.clear_square(square);
            if eval < min_eval {
                min_eval = eval;
                min_move = square;
                if eval <= alpha {
                    break;
                }
                if eval < beta {
                    beta = eval;
                }
            }
        }
        (min_eval, min_move)
    };
}
