#![allow(dead_code)]

use crate::color::Color;
use crate::board::Board;

use log::{info, warn, trace, error, set_max_level};

pub fn find_best_move(board: &Board, color: Color) -> Option<(usize, usize)> {
    board.iter_pos2D().find(|p| board.can_place(p.ij().0, p.ij().1, color)).map(|p| p.ij())
}

pub fn caclulate_score(board: &Board) -> (usize, usize) {
    (board.num_of_color(Color::Black), board.num_of_color(Color::White))
}