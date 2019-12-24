#![allow(dead_code)]

use log::{info, warn, trace, error, set_max_level};
use std::time::{Duration, Instant};

use crate::color::Color;
use crate::board::*;
use crate::stat::Stat;

pub fn find_best_move(board: &Board, color: Color, level: i32, recurse: bool, stat: &mut Stat) -> Option<(Pos2D, i32)> {
    let possible_moves = board.iter_pos2d().filter(|p| board.can_place_pos2d(*p, color));
    let mut max_score = -99999;
    let mut best_move: Option<(Pos2D, i32)> = None;
    let mut indent = format!("[{}]", level);
    for _ in 0..level {
        indent += "    ";
    }
    for mv in possible_moves {
        let mut board_copy = board.clone();
        trace!("{}mv: {}", indent, mv);
        board_copy.place(mv.i, mv.j, color);
        stat.nodes_viewed += 1;

        let possible_oppo_moves = board_copy.iter_pos2d().filter(|p| board_copy.can_place_pos2d(*p, color.opposite()));

        let mut min_score = 99999;
        let mut best_oppo_move: Option<(Pos2D, i32)> = None;
        for mv_oppo in possible_oppo_moves {
            let mut board_copy2 = board_copy.clone();
            trace!("{}  mv_oppo: {}", indent, mv_oppo);
            board_copy2.place_pos2d(mv_oppo, color.opposite());
            stat.nodes_viewed += 1;

            let oppo_score: i32;

            if recurse {
                let best2 = find_best_move(&board_copy2, color, level+1, false, stat);
                oppo_score = match best2 {
                    Some(s) => s.1,
                    None => eval(&board_copy2, color)
                };
            }
            else {
                oppo_score = eval(&board_copy2, color);
            }

            trace!("{}  -> {}", indent, oppo_score);
            if oppo_score < max_score {
                trace!("{}  pruning!", indent);
                best_oppo_move = best_move;
                break;
            }
            if oppo_score < min_score {
                min_score = oppo_score;
                best_oppo_move = Some((mv_oppo, oppo_score));
            }
        }

        let score = match best_oppo_move {
            Some((_,s)) => s,
            None => eval(&board_copy, color)
        };

        info!("{}mv: {} -> {}", indent, mv, score);
        trace!("{}-> {}", indent, score);
        if score > max_score {
            trace!("{}it's best: score = {:?}", indent, score);
            max_score = score;
            best_move = Some((mv, max_score));
        }
    }
    best_move
}

pub fn caclulate_score(board: &Board) -> (usize, usize) {
    (board.num_of_color(Color::Black), board.num_of_color(Color::White))
}

#[derive(Debug)]
struct Point {
    i: char,
    j: usize,
}

fn eval_corner_worker(board: &Board, color: Color, corner: &Point, precorner1: &Point, precorner2: &Point, badprecorner: &Point) -> i32 {
    let mut score: i32 = 0;
    if board.get_at_c(corner.i, corner.j) == color {
        score += 100;
        if board.get_at_c(precorner1.i, precorner1.j) == color {
            score += 50;
        }
        if board.get_at_c(precorner2.i, precorner2.j) == color {
            score += 50;
        }
    }
    else {
        // Bad pre-corner
        if board.get_at_c(badprecorner.i, badprecorner.j) == color {
            score -= 100;
        }
    }
    score
}

fn eval_corner(board: &Board, color: Color, corner: &Point, precorner1: &Point, precorner2: &Point, badprecorner: &Point) -> i32 {
    eval_corner_worker(board, color,            corner, precorner1, precorner2, badprecorner) -
    eval_corner_worker(board, color.opposite(), corner, precorner1, precorner2, badprecorner)
}

pub fn eval(board: &Board, color: Color) -> i32 {
    let occupied = board.num_occupied();
    let mut score: i32;
    if occupied < 54 {
        score = 
            board.iter_pos2d().filter(|p| board.can_place_pos2d(*p, color)).count() as i32 -
            board.iter_pos2d().filter(|p| board.can_place_pos2d(*p, color.opposite())).count() as i32;

        // What matters at this stage is stable cells, plus minimizing number of opponent moves
        score += eval_corner(board, color, &Point{i: 'a', j: 0},
                                           &Point{i: 'b', j: 0},
                                           &Point{i: 'a', j: 1},
                                           &Point{i: 'b', j: 1});

        score += eval_corner(board, color, &Point{i: 'h', j: 0},
                                           &Point{i: 'h', j: 1},
                                           &Point{i: 'g', j: 0},
                                           &Point{i: 'g', j: 1});

        score += eval_corner(board, color, &Point{i: 'h', j: 7},
                                           &Point{i: 'h', j: 6},
                                           &Point{i: 'g', j: 7},
                                           &Point{i: 'g', j: 6});

        score += eval_corner(board, color, &Point{i: 'a', j: 7},
                                           &Point{i: 'a', j: 6},
                                           &Point{i: 'b', j: 7},
                                           &Point{i: 'b', j: 6});
    }
    else {
        score = board.num_of_color(color) as i32 - board.num_of_color(color.opposite()) as i32;
    }
    score
}