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

use crate::board::Board;
use std::io;

pub fn play_against_engine(engine_player: i8) {
    println!("\n\n\n");
    let mut pos = Board::new();
    if engine_player == -1 {
        loop {
            if !pos.is_in_play() {
                break;
            }
            println!("{}", pos.pgn);
            user_turn(&mut pos);
            if !pos.is_in_play() {
                break;
            }
            println!("{}", pos.pgn);
            engine_turn(&mut pos);
        }
    } else {
        loop {
            if !pos.is_in_play() {
                break;
            }
            println!("{}", pos.pgn);
            engine_turn(&mut pos);
            if !pos.is_in_play() {
                break;
            }
            println!("{}", pos.pgn);
            user_turn(&mut pos);
        }
    }
    println!("\n\nGame Over!");
    pos.show();
    match pos.evaluation() {
        1  =>  println!("\nX won!\n"),
        -1 => println!("\nO won!\n"),
        0  => println!("\nDraw!\n"),
        _ => (),
    }
}

pub fn engine_turn(pos: &mut Board) {
    println!("Calculating...");
    let best_move = pos.best_move().unwrap();
    pos.play(best_move).unwrap();
    println!("Engine's move: {}\n", best_move);
}

pub fn user_turn(pos: &mut Board) {
    pos.show();
    loop {
        let mut square_str = String::new();
        if pos.current_player() == 1 {
            println!("\nEnter move for O:");
        } else {
            println!("\nEnter move for X:");
        }
        io::stdin().read_line(&mut square_str).expect("---");
        square_str.pop();
        if square_str.len() == 1 {
            if square_str.chars().next().unwrap().is_numeric() {
                let square:i8 = square_str.parse().unwrap();
                if pos.is_valid_move(square) {
                    pos.play(square).unwrap();
                    break;
                }
            }
        }
    }
}

pub fn menu() {
    'total: loop {
        println!("\n\n\n\nnorts.\n\n");
        println!("[1] - Play against engine");
        println!("[2] - Play from PGN");
        println!("[3] - Find the best move from PGN\n");
        println!("[4] - Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("---");
        match &choice as &str {
            "1\n" => {
                let engine_player: i8;
                loop {
                    println!("\n\nShould the engine be X or O? (X goes first)");
                    let mut engine_player_choice = String::new();
                    io::stdin().read_line(&mut engine_player_choice).expect("---");
                    if engine_player_choice.to_lowercase() == "x\n" {
                        engine_player = 1;
                        break;
                    } else if engine_player_choice.to_lowercase() == "o\n" {
                        engine_player = -1;
                        break;
                    }
                }
                play_against_engine(engine_player);
                break;
            }

            "4\n" => {
                break 'total;
            }

            _ => {}
        }
    }
}
