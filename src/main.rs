mod board;
use board::Board;

fn main() {
    let mut board = Board::new();
    board.show();
    board.play(0);
    board.play(2);
    board.play(8);
    board.play(3);
    board.play(4);
    println!("{}", board.x_won());
    board.show();
}
