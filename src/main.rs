#![allow(dead_code)]
#![allow(unused_variables)]

mod color;
use crate::color::Color;

mod board;
use crate::board::Board;

fn main() {

    let mut board = Board::new();

    board.set_at_c('D',3,Color::Black);
    board.set_at_c('D',4,Color::White);
    board.set_at_c('E',3,Color::White);
    board.set_at_c('E',4,Color::Black);

    board.print();

}
