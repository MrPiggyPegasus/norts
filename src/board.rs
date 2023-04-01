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

// board representation:
// 1   2   4
// 8   16  32
// 64  128 256

// or 2^x:

// 0   1   2
// 3   4   5
// 6   7   8

/// Representation of the board using 2 u16 bitboards.
pub struct Board {
    pub x_bitboard: u16,
    pub o_bitboard: u16,
    pub move_history: [u8; 9],
    pub legal_moves: [bool; 9],
}

impl Board {
    pub fn new() -> Board {
        Board {
            x_bitboard: 0b000_0000_0000_0000,
            o_bitboard: 0b0000_0000_0000_0000,
            // If a slot is 10, the move has not been played
            move_history: [10; 9],
            legal_moves: [true; 9],
        }
    }

    pub fn current_player(&self) -> bool {
        self.x_bitboard.count_ones() <= self.o_bitboard.count_ones()
    }

    pub fn num_moves(&self) -> u8 {
        self.x_bitboard.count_ones() as u8 + self.o_bitboard.count_ones() as u8
    }

    pub fn last_move(&self) -> u8 {
        self.move_history[(self.num_moves() - 1) as usize]
    }

    pub fn undo_move(&mut self) {
        if self.num_moves() < 1 {
            return;
        }
        self.move_history[self.num_moves() as usize] = 10;
        if self.current_player() {
            println!(
                "{} , {}",
                (2 as u32).pow(self.last_move() as u32),
                self.o_bitboard
            );
            self.o_bitboard -= (2 as u32).pow(self.last_move() as u32) as u16
        } else {
            self.x_bitboard -= (2 as u32).pow(self.last_move() as u32) as u16
        }
    }

    pub fn play(&mut self, square: u8) {
        self.legal_moves[square as usize] = false;
        self.move_history[self.num_moves() as usize] = square;
        if self.current_player() {
            self.x_bitboard |= (2 as u16).pow(square as u32)
        } else {
            self.o_bitboard |= (2 as u16).pow(square as u32)
        }
    }

    pub fn show(&self) {
        for row in 0..3 {
            for col in 0..3 {
                if (self.x_bitboard & (2 as u16).pow((row * 3) + col))
                    == (2 as u16).pow((row * 3) + col)
                {
                    print!("X  ");
                } else if (self.o_bitboard & (2 as u16).pow((row * 3) + col))
                    == (2 as u16).pow((row * 3) + col)
                {
                    print!("O  ");
                } else {
                    print!(".  ");
                }
            }
            println!();
        }
    }

    pub fn is_draw(&self) -> bool {
        self.x_bitboard | self.o_bitboard == 0b0000_0001_1111_1111
    }

    pub fn o_won(&self) -> bool {
        // check diagonals by matching bit patterns
        if self.o_bitboard & 0b0000_0001_0001_0001 == 0b0000_0001_0001_0001 {
            return true;
        }
        if self.o_bitboard & 0b0000_0000_0101_0100 == 0b0000_0001_0001_0001 {
            return true;
        }

        for row in 0..3 {
            // check horizontals
            if self.o_bitboard & 0b0000_0000_0000_0111 << (row * 3)
                == 0b0000_0000_0000_0101 << (row * 3)
            {
                return true;
            }
            // check verticals
            if self.o_bitboard & 0b0000_0000_0100_1001 << row == 0b0000_0000_0100_1001 << row {
                return true;
            }
        }
        return false;
    }

    pub fn x_won(&self) -> bool {
        // check diagonals by matching bit patterns
        if self.x_bitboard & 0b0000_0001_0001_0001 == 0b0000_0001_0001_0001 {
            return true;
        }
        if self.x_bitboard & 0b0000_0000_0101_0100 == 0b0000_0001_0001_0001 {
            return true;
        }

        for row in 0..3 {
            // check horizontals
            if self.x_bitboard & 0b0000_0000_0000_0111 << (row * 3)
                == 0b0000_0000_0000_0111 << (row * 3)
            {
                return true;
            }
            // check verticals
            if self.x_bitboard & 0b0000_0000_0100_1001 << row == 0b0000_0000_0100_1001 << row {
                return true;
            }
        }
        return false;
    }

    pub fn is_legal(&self, square: u8) -> bool {
        self.x_bitboard & (2 as u16).pow(square as u32)
            != (2 as u16).pow(square as u32)
            && (self.o_bitboard & (2 as u16).pow(square as u32))
            != (2 as u16).pow(square as u32)
    }
}
