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

// facade for bitboards::Bitboard to hide implementation details

use std::fmt::Formatter;
use std::{fmt, io};

use crate::bitboards::Bitboard;
use crate::search::search;

#[derive(Debug, Clone)]
pub struct PositionAlreadyConcludedError;

impl fmt::Display for PositionAlreadyConcludedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " The position cannot be analysed as it has already concluded."
        )
    }
}

#[derive(Debug, Clone)]
pub struct InvalidPgnError;

impl fmt::Display for InvalidPgnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, " PGN is invalid.")
    }
}

#[derive(Debug, Clone)]
pub struct IllegalMoveError;

impl fmt::Display for IllegalMoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, " Move is illegal.")
    }
}

pub struct Board {
    pub bitboard: Bitboard,
    pub pgn: String,
}

impl Board {
    pub fn new() -> Board {
        Board {
            bitboard: Bitboard::new(),
            pgn: String::new(),
        }
    }

    pub fn evaluation(&self) -> i8 {
        if self.bitboard.x_won() {
            return 1;
        }
        if self.bitboard.o_won() {
            return -1;
        }
        0
    }

    pub fn is_valid_move(&self, square: i8) -> bool {
        (square > 0 || square < 8) && self.bitboard.is_legal(square as u8) && self.is_in_play()
    }

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

    pub fn play(&mut self, square: i8) -> Result<bool, IllegalMoveError> {
        if self.is_valid_move(square) {
            self.bitboard.play(square as u8);
            self.pgn += &*square.to_string();
            Ok(true)
        } else {
            Err(IllegalMoveError)
        }
    }

    pub fn is_in_play(&self) -> bool {
        !(self.bitboard.x_won() || self.bitboard.o_won() || self.bitboard.is_draw())
    }

    pub fn best_move(&mut self) -> Result<i8, PositionAlreadyConcludedError> {
        if self.is_in_play() {
            Ok(search(&mut self.bitboard, i8::MIN, i8::MAX).1 as i8)
        } else {
            Err(PositionAlreadyConcludedError)
        }
    }

    pub fn current_player(&self) -> i8 {
        // converts bool to 1 / -1
        (self.bitboard.current_player() as i8 * -2) + 1
    }

    pub fn show(&self) {
        println!("*-----------------------*");
        println!(" Board:         Squares:");
        for row in 0..3 {
            for col in 0..3 {
                if (self.bitboard.x_bitboard & (2 as u16).pow((row * 3) + col))
                    == (2 as u16).pow((row * 3) + col)
                {
                    print!(" X ");
                } else if (self.bitboard.o_bitboard & (2 as u16).pow((row * 3) + col))
                    == (2 as u16).pow((row * 3) + col)
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
