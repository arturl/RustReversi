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

mod board_tests;
mod analysis_tests;

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

    let board_orig = Board::new_from(&board);

    let mut transcript = Transcript::new();
    //let mut transcript = Transcript::  from_trace("bc4wc3bc2wb3ba4wd5bf3wb2bc5wa3ba2wd2be2wd1bc0wc1bd0wf4bf5we1be5we0bf0wf1bf2wg2bh2wf6bf7wg3bh3wg4bh4wc6bc7wh5be6wh1bg5wd6bd7wh6bg1");
    //let mut transcript = Transcript::from_trace("bc4wc5bc6wc3bc2wb3ba2wa3ba4we5bf2wd5be2wb5ba6");
    //let mut transcript = Transcript::from_trace("bc4wc5bf3wb4ba4wc3bb2wc2bb3wa2ba3wa5bd5wb5bc1wc0bb6wc6bb7wd7bd6we5be6wd2bd1we1bd0we0bf0wg0bb1wa7ba6wc7ba1wf7be7wa0bg7wf4bf5wg2bf2wh7bh2wb0wf1wg6bg4wh3bf6we2bg1wh0bh4wg3bh6wg5wh5");

    let mut color = board.replay_transcript(&transcript).opposite();

    board.print();

    loop {
        println!("transcript: {}", transcript);
        let score = caclulate_score(&board);
        println!(
            "Score: Black:{}  White:{}  Total:{}",
            score.0,
            score.1,
            board.num_occupied()
        );
        if board.has_any_moves(color) {
            println!("{:?} moves next", color);
        } else {
            println!("{:?} has no more moves", color);
            color = color.opposite();
            if board.has_any_moves(color) {
                println!("Instead, enter next move for {:?}:", color);
            } else {
                println!("{:?} also has no more moves. Game over.", color);
                return;
            }
        }
        loop {
            if color == Color::White {
                let mut stat = Stat::new();
                let (pos, score) = negamax(&board, color, 6, 52, &mut stat).unwrap();

                // let (pos, score) = if color == Color::White 
                //     {
                //         negamax(&board, color, 4, 4, &mut stat).unwrap()
                //     } else {
                //         minimax(&board, color, 3, 3, &mut stat).unwrap()
                //     };

                board.place(pos, color);
                transcript.add(pos, color);
                board.print();
                let elapsed = stat.start.elapsed();
                println!(
                    "Computer ({}) picked {}. Reviewed {} nodes. Best score {}. Elapsed {:?}. Speed: {}.",
                    color,
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
            print!("Options: ");
            for pat in hints {
                print!("{} ", pat);
            }
            let (pos, score) = minimax(&board, color, 3, 58, &mut Stat::new()).unwrap();
            print!(". Hint: {} (score: {})", pos, score);
            println!();

            print!("> ");
            stdout().flush().unwrap();
            let mut input = String::new();
            let stdin = io::stdin();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            if input == "q" {
                return;
            }
            else if input == "back" {
                board = Board::new_from(&board_orig);
                transcript.back();
                color = board.replay_transcript(&transcript);
                board.print();
                break;
            }
            let coords = input.as_bytes();
            if coords.len() == 2 {
                let xi = (coords[0] - 97) as usize;
                let yi = (coords[1] - 48) as usize;
                let position = Pos2D::new(xi, yi);
                if board.can_place(position, color) {
                    board.place(position, color);
                    transcript.add(position, color);
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
