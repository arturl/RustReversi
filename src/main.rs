#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// #[macro_use] extern crate log;

use log::{info, warn, trace, error, set_max_level};

mod color;
use crate::color::Color;

mod board;
use crate::board::Board;

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

}
