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
