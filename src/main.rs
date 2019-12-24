#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// #[macro_use] extern crate log;

use log::{info, warn, trace, error, set_max_level};

mod color;
use crate::color::Color;

mod board;
use crate::board::Board;

mod analysis;
use crate::analysis::*;

mod stat;
use crate::stat::*;

mod transcript;
use crate::transcript::*;

use std::io::{self, Read};
use std::io::stdout;
use std::io::Write;

fn main() {

    env_logger::builder()
        .format_timestamp(None)
        .filter_module("reversi", log::LevelFilter::Info)
        .filter_module("reversi::board", log::LevelFilter::Info)
        .filter_module("reversi::analysis", log::LevelFilter::Error)
        .filter_module("reversi::stat", log::LevelFilter::Error)
        .init();

    let mut board = Board::new();

    board.set_at_c('D',3,Color::Black);
    board.set_at_c('D',4,Color::White);
    board.set_at_c('E',3,Color::White);
    board.set_at_c('E',4,Color::Black);

    let mut transcript = Transcript::new();
    //let mut transcript = Transcript::from_trace("c4e5f2c3e6d2b3f6g7d5c5d6c6d7c2f7");
    board.replay_transcript(&transcript);

    board.print();

    let mut color = Color::Black;

    loop {
        let score = caclulate_score(&board);
        println!("Score: Black:{}  White:{}  Total:{}", score.0, score.1, board.num_occupied());
        if board.has_any_moves(color) {
            println!("Enter next move for {:?}:", color);
        }
        else {
            println!("{:?} has no more moves", color);
            color = color.opposite();
            if board.has_any_moves(color) {
                println!("Instead, enter next move for {:?}:", color);
            }
            else {
                println!("{:?} also has no more moves. Gave over.", color);
                return;
            }
        }
        loop {

            if color == Color::White {
                let mut stat = Stat::new();
                let (pos, score) = find_best_move(&board, color, 0, true, &mut stat).unwrap();
                board.place_pos2d(pos, color);
                transcript.add(pos.i, pos.j);
                board.print();
                println!("Computer picked {}. Reviewed {} nodes. Best score {}. Elapsed {:?}", 
                    pos, stat.nodes_viewed, score, stat.start.elapsed());
                break;
            }

            let hints = board.iter_pos2d().filter(|p| board.can_place_pos2d(*p, color));
            println!("transcript: {}", transcript);
            print!("Hint: ");
            for pat in hints {
                 print!("{} ", pat);
            }
            println!();

            print!("> ");
            stdout().flush().unwrap();
            let mut input = String::new();
            let stdin = io::stdin();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            println!("you said '{}'", input);
            if input == "q" {
                return;
            }
            let coords = input.as_bytes();
            if coords.len() == 2 {
                let x = coords[0];
                let y = coords[1];

                let xi = (x - 97) as usize;
                let yi = (y - 48) as usize;

                if board.can_place(xi,yi,color) {
                    board.place(xi,yi,color);
                    transcript.add(xi,yi);
                    board.print();
                    break;
                }

                println!("Error: This position is not valid for {:?}. Try again.", color);
            }
            else {
                println!("Error: Wrong length, must be 2 characters. Try again.");
            }
        }

        color = color.opposite();
    }
}
