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
  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NON INFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  SOFTWARE.
 */

use crate::board;

pub fn search(pos: &board::Board, alpha: i8, beta: i8) -> i8 {
    let eval = node_eval(pos);
    if eval.1 {
        return eval.0;
    }
    // if X is playing, and therefore wants a higher eval
    if pos.current_player() {
        let max_eval = i8::MIN;
    }
    0
}

/// Returns the eval + bool indicating whether or not the position is concluded.
/// X plays the first move, so a positive score is better for X and a negative one
/// is better for O, zero is a draw.
/// The eval returned is moved closer to zero by every piece on the board, such that
/// the engine prefers to win as quickly as possible.
pub fn node_eval(pos: &board::Board) -> (i8, bool) {
    if pos.x_won() {
        return (
            100 - (pos.num_moves() as i8),
            true,
        );
    } else if pos.o_won() {
        return (
            -100 + (pos.num_moves() as i8),
            true,
        );
    } else if pos.is_draw() {
        return (0, true);
    }
    return (0, false);
}
