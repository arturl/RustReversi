#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// #[macro_use] extern crate log;

use log::{error, info, set_max_level, trace, warn};

mod color;
use crate::color::Color;

mod board;
use crate::board::*;

mod analysis;
use crate::analysis::*;

mod stat;
use crate::stat::*;

mod transcript;
use crate::transcript::*;

use std::io::stdout;
use std::io::Write;
use std::io::{self, Read};

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .filter_module("reversi", log::LevelFilter::Info)
        .filter_module("reversi::board", log::LevelFilter::Error)
        .filter_module("reversi::analysis", log::LevelFilter::Error)
        .filter_module("reversi::stat", log::LevelFilter::Error)
        .init();

    let mut board = Board::new();

    board.set_at_c('D', 3, Color::Black);
    board.set_at_c('D', 4, Color::White);
    board.set_at_c('E', 3, Color::White);
    board.set_at_c('E', 4, Color::Black);

    //let mut transcript = Transcript::new();
    let mut transcript = Transcript::from_trace("c4c3c2b3a4d5f3b2c5a3a2d2e2d1c0c1d0f4f5e1e5e0f0f1f2g2h2f6f7g3h3g4h4c6c7h5");
    //let mut transcript = Transcript::from_trace("c4e5f2c3e6d2b3f6g7d5c5d6c6d7c2f7");
    board.replay_transcript(&transcript);

    board.print();

    let mut color = Color::Black;

    loop {
        let score = caclulate_score(&board);
        println!(
            "Score: Black:{}  White:{}  Total:{}",
            score.0,
            score.1,
            board.num_occupied()
        );
        if board.has_any_moves(color) {
            println!("Enter next move for {:?}:", color);
        } else {
            println!("{:?} has no more moves", color);
            color = color.opposite();
            if board.has_any_moves(color) {
                println!("Instead, enter next move for {:?}:", color);
            } else {
                println!("{:?} also has no more moves. Gave over.", color);
                return;
            }
        }
        loop {
            if color == Color::White {
                let mut stat = Stat::new();
                let (pos, score) = find_best_move(&board, color, 0, 3, &mut stat).unwrap();
                board.place(pos, color);
                transcript.add(pos);
                board.print();
                let elapsed = stat.start.elapsed();
                println!(
                    "Computer picked {}. Reviewed {} nodes. Best score {}. Elapsed {:?}. Speed: {}.",
                    pos,
                    stat.nodes_viewed,
                    score,
                    elapsed,
                    if elapsed.as_secs() == 0 {
                        format!("{}nodes/ms", ((stat.nodes_viewed as f64 / stat.start.elapsed().as_millis() as f64) as i32))
                    }
                    else {
                        format!("{}Knodes/sec", (((stat.nodes_viewed / 1000) as f64 / stat.start.elapsed().as_secs() as f64) as i32))
                    }
                );
                break;
            }

            let hints = board.get_available_moves_for(color);
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
                let xi = (coords[0] - 97) as usize;
                let yi = (coords[1] - 48) as usize;
                let position = Pos2D::new(xi, yi);
                if board.can_place(position, color) {
                    board.place(position, color);
                    transcript.add(position);
                    board.print();
                    break;
                }

                println!(
                    "Error: This position is not valid for {:?}. Try again.",
                    color
                );
            } else {
                println!("Error: Wrong length, must be 2 characters. Try again.");
            }
        }

        color = color.opposite();
    }
}
