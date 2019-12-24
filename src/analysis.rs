#![allow(dead_code)]

use crate::color::Color;
use crate::board::Board;

use log::{info, warn, trace, error, set_max_level};

pub fn find_best_move(board: &Board, color: Color) -> Option<(usize, usize)> {
    for i in 0..8 {
        for j in 0..8 {
            if board.can_place(i,j,color) {
                return Some((i,j));
            }
        }
    }
    None
}

pub fn caclulate_score(board: &Board) -> (i32, i32) {
    let mut num_blacks = 0;
    let mut num_whites = 0;
    for i in 0..8 {
        for j in 0..8 {
            if board.get_at(i,j) == Color::Black {
                num_blacks += 1;
            }
            if board.get_at(i,j) == Color::White {
                num_whites += 1;
            }
        }
    }
    (num_blacks, num_whites)
}