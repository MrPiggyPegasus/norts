# What's a norts?
norts is a (*clears throat*) BLAZINGLY FAST Noughts and Crosses / Tic Tac Toe solver written in Rust.

## Components
* /norts/ is the Cargo Crate containing the engine itself.
* /bin/ is a binary used to play against the bot in the terminal.

## Engine Design
norts is designed with speed as a priority, and is able to solve any position almost instantly.
The position is stored using 2 16-bit bitboards such that wins and draws can be detected using cpu-efficient bitwise operations
and the Minimax algorithm which is used to decide moves, is enhanced with Alpha-Beta pruning.
Further improvements could be enabled with the implementations of transposition tables but such speeds would likely create a singularity and end the world or something.
