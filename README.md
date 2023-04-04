# norts
A perfect Noughts and Crosses / Tic Tac Toe engine written in Rust.

## Directories
* /norts/ is the Cargo Crate containing the engine itself
* /bin/ is a binary used to play against the bot in the terminal.

## Engine Design
norts is designed with speed as a priority, and is able to solve any position in under half of a second.
The position is stored using 2 16-bit bitboards such that wins and draws can be detected using cpu-friendly bitwise operations 
and the Minimax algorithm is used to decide moves, is enhanced with Alpha-Beta pruning.
Further improvements could be enabled with the implementations of transposition tables. (W.I.P.)

