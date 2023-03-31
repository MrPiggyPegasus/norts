mod board;
mod search;

use board::Board;

fn main() {
    let mut board = Board::new();
    board.play(1);
    board.play(2);
    board.play(3);
    board.show();
    println!("");
    board.undo_move();
    board.undo_move();
    board.undo_move();
    board.undo_move();
    println!("{}", board.x_bitboard);
    println!("{}", board.o_bitboard);
    board.show()
}
