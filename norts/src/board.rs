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

use std::fmt;
use std::fmt::Formatter;

use crate::bitboards::Bitboard;
use crate::search::search;

#[derive(Debug, Clone)]
pub struct PositionAlreadyConcludedError;

impl fmt::Display for PositionAlreadyConcludedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The position cannot be analysed as it has already concluded."
        )
    }
}

#[derive(Debug, Clone)]
pub struct InvalidPgnError;

impl fmt::Display for InvalidPgnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PGN is invalid.")
    }
}

#[derive(Debug, Clone)]
pub struct IllegalMoveError;

impl fmt::Display for IllegalMoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Move is illegal.")
    }
}

#[derive(Debug, Clone)]
pub struct NoMoveToUndoError;

impl fmt::Display for NoMoveToUndoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "No move to undo!")
    }
}
/// The main representation of the board for end user interaction.
/// See methods for usage.
pub struct Board {
    /// Binary representation of the position used to optimise performance.
    pub bitboard: Bitboard,
    ///
    pub pgn: String,
}

impl Board {
    /// Returns a fresh board in the starting position.
    pub fn new() -> Board {
        Board {
            bitboard: Bitboard::new(),
            pgn: String::new(),
        }
    }

    /// Undoes the last move that was played
    pub fn undo_move(&mut self) -> Result<(), NoMoveToUndoError> {
        if self.pgn.len() == 0 {
            return Err(NoMoveToUndoError);
        }
        self.bitboard.clear_square(
            self.pgn
                .chars()
                .nth(self.pgn.len() - 1)
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
        );
        self.pgn.truncate(self.pgn.len() - 1);
        Ok(())
    }

    /// Returns:
    /// * 1 if X has won
    /// * -1 if O has won
    /// * 0 if the game is ongoing or is a draw
    pub fn situation(&self) -> i8 {
        if self.bitboard.x_won() {
            return 1;
        }
        if self.bitboard.o_won() {
            return -1;
        }
        0
    }

    /// Returns a bool indication whether or not a certain move is possible in the position
    ///
    pub fn is_valid_move(&self, square: i8) -> bool {
        square >= 0 && square < 9 && self.bitboard.is_legal(square as u8) && self.is_in_play()
    }

    /// Tests if a PGN is valid
    ///
    /// ## PGN
    /// To save a certain position or game, you can use the PGN or Portable Game Notation format.
    /// A PGN is a simple concatenation of moves notated in the above manner,
    /// such that "042" yields the following position:
    ///
    ///     X  .  X
    ///     .  O  .
    ///     .  .  .
    ///``` ignore
    /// use crate::norts::board::Board;
    ///
    /// fn main() {
    ///    let mut pos = Board::parse_pgn("172");
    ///    pos.show();
    /// }
    /// ```
    pub fn is_valid_pgn(pgn: &str) -> bool {
        let mut pos = Board::new();
        for c in pgn.chars() {
            if c.is_numeric() {
                if pos.is_valid_move(c.to_string().parse::<i8>().unwrap()) {
                    pos.play(c.to_string().parse::<i8>().unwrap()).unwrap();
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Returns a board object which picks up from the specified PGN string.
    ///
    /// ## PGN
    /// To save a certain position or game, you can use the PGN or Portable Game Notation format.
    /// A PGN is a simple concatenation of moves notated in the above manner,
    /// such that "042" yields the following position:
    ///
    ///     X  .  X
    ///     .  O  .
    ///     .  .  .
    ///``` ignore
    /// use crate::norts::board::Board;
    ///
    /// fn main() {
    ///    let mut pos = Board::parse_pgn("172");
    ///    pos.show();
    /// }
    /// ```
    pub fn parse_pgn(pgn: &str) -> Result<Board, InvalidPgnError> {
        if Board::is_valid_pgn(pgn) {
            let mut pos = Board::new();
            for c in pgn.chars() {
                if c.is_numeric() {
                    if (c.to_string().parse::<i8>().unwrap()) < 9 {
                        pos.play(c.to_string().parse::<i8>().unwrap()).unwrap();
                    }
                } else {
                    ()
                }
            }
            Ok(pos)
        } else {
            Err(InvalidPgnError)
        }
    }

    /// Plays a move to the certain square.
    ///
    /// ## Move Notation
    /// Every square is applied a number by this grid:
    ///
    ///     0  1  2
    ///     3  4  5
    ///     6  7  8
    /// A move is notated by the square on which a piece is placed, X goes first.
    /// The following example creates a board and plays some moves:
    /// ``` ignore
    /// use crate::norts::board::Board;
    ///
    /// fn main() {
    ///     let mut pos = Board::new();
    ///     pos.play(0).unwrap();
    ///     pos.play(4).unwrap();
    ///     pos.play(2).unwrap();
    ///     pos.show();
    ///     println!("Best move: {}", pos.best_move().unwrap());
    /// }
    ///  ```
    pub fn play(&mut self, square: i8) -> Result<bool, IllegalMoveError> {
        if self.is_valid_move(square) {
            self.bitboard.play(square as u8);
            self.pgn += &*square.to_string();
            Ok(true)
        } else {
            Err(IllegalMoveError)
        }
    }

    /// Returns a bool indicating whether or not the game has ended.
    pub fn is_in_play(&self) -> bool {
        !(self.bitboard.x_won() || self.bitboard.o_won() || self.bitboard.is_draw())
    }

    /// Returns the best move in the position.
    pub fn best_move(&mut self) -> Result<i8, PositionAlreadyConcludedError> {
        if self.is_in_play() {
            Ok(search(&mut self.bitboard, i8::MIN, i8::MAX).1 as i8)
        } else {
            Err(PositionAlreadyConcludedError)
        }
    }

    pub fn current_player(&self) -> i8 {
        // converts bool to 1 / -1
        -((self.bitboard.current_player() as i8 * -2) + 1)
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        for row in 0..3 {
            for col in 0..3 {
                if (self.bitboard.x_bitboard & (1 << ((row * 3) + col))) == 1 << ((row * 3) + col) {
                    out += " X "
                } else if (self.bitboard.o_bitboard & (1 << ((row * 3) + col)))
                    == 1 << ((row * 3) + col)
                {
                    out += " O "
                } else {
                    out += " . "
                }
            }
            out += "\n"
        }
        out
    }

    /// Displays a visual representation of the board to the standard output.
    pub fn show(&self) {
        println!("*-----------------------*");
        println!(" Board:         Squares:");
        for row in 0..3 {
            for col in 0..3 {
                if (self.bitboard.x_bitboard & (1 << ((row * 3) + col))) == 1 << ((row * 3) + col) {
                    print!(" X ");
                } else if (self.bitboard.o_bitboard & (1 << ((row * 3) + col)))
                    == (1 << ((row * 3) + col))
                {
                    print!(" O ");
                } else {
                    print!(" . ");
                }
            }
            println!(
                "       {}  {}  {}  ",
                (row * 3),
                (row * 3) + 1,
                (row * 3) + 2
            );
        }
        println!("*-----------------------*");
    }
}
