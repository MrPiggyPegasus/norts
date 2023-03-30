

// board representation:
// 1   2   4
// 8   16  32
// 64  128 256

// or 2^x:

// 0   1   3
// 4   5   6
// 7   8   9

pub struct Board {
    pub x_bitboard:u16,
    pub o_bitboard:u16,
}

impl Board {
    pub fn new() -> Board {
        Board {x_bitboard: 0b000_0000_0000_0000, o_bitboard: 0b0000_0000_0000_0000}
    }

    pub fn play(&mut self, square:u8) {
        println!("x: {} o: {}", self.x_bitboard.count_ones(), self.o_bitboard.count_ones());
        if self.x_bitboard.count_ones() <= self.o_bitboard.count_ones() {
            println!("x");
            self.x_bitboard |= (2 as u16).pow(square as u32)
        } else {
            println!("o");
            self.o_bitboard |= (2 as u16).pow(square as u32)
        }
    }

    pub fn show(&self) {
        // iterate over bitboards and convert to array
        for row in 0..3 {
            for col in 0..3 {
                if (self.x_bitboard & (2 as u16).pow((row*3)+col)) == (2 as u16).pow((row*3)+col)  {
                    print!("X  ");
                } else if (self.o_bitboard & (2 as u16).pow((row*3)+col)) == (2 as u16).pow((row*3)+col) {
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
            if self.o_bitboard & 0b0000_0000_0000_0111 << (row * 3) == 0b0000_0000_0000_0111 << (row * 3){
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
            if self.x_bitboard & 0b0000_0000_0000_0111 << (row * 3) == 0b0000_0000_0000_0111 << (row * 3){
                return true;
            }
            // check verticals
            if self.x_bitboard & 0b0000_0000_0100_1001 << row == 0b0000_0000_0100_1001 << row {
                return true;
            }
        }
        return false;
    }
}