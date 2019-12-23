#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// #[macro_use] extern crate log;

use log::{info, warn, trace, error, set_max_level};

mod color;
use crate::color::Color;

mod board;
use crate::board::Board;

use std::io::{self, Read};
use std::io::stdout;
use std::io::Write;

fn main() {

    env_logger::builder()
        .format_timestamp(None)
        .filter_module("reversi", log::LevelFilter::Info)
        .filter_module("reversi::board", log::LevelFilter::Info)
        .init();

    let mut board = Board::new();

    board.set_at_c('D',3,Color::Black);
    board.set_at_c('D',4,Color::White);
    board.set_at_c('E',3,Color::White);
    board.set_at_c('E',4,Color::Black);

    board.print();

    let mut vecb = Vec::new();
    let mut vecw = Vec::new();
    for i in 0..8 {
        for j in 0..8 {
            if board.can_place(i,j,Color::Black) {
                vecb.push((i,j));
            }
            if board.can_place(i,j,Color::White) {
                vecw.push((i,j));
            }
        }
    }

    // let b2 = board.can_place(3,2,Color::Black);
    println!("for black: {:?}", vecb);
    println!("for white: {:?}", vecw);

    let mut color = Color::Black;

    loop {
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

                trace!("--> {} {}", xi, yi);

                if board.can_place(xi,yi,color) {
                    board.place(xi,yi,color);
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
