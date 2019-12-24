#![allow(dead_code)]

use log::{info, warn, trace, error, set_max_level};

use crate::color::Color;
use crate::board::Board;

pub fn find_best_move(board: &Board, color: Color) -> Option<(usize, usize)> {
    let possible_moves = board.iter_pos2d().filter(|p| board.can_place_pos2d(*p, color));

    let mut best_score = -99999;
    let mut best_move: Option<(usize, usize)> = None;
    for mv in possible_moves {
        let mut board_copy = board.clone();
        trace!("mv: {}{}", ((mv.i as u8)+97) as char, mv.j);
        board_copy.place(mv.i, mv.j, color);

        let possible_oppo_moves = board_copy.iter_pos2d().filter(|p| board_copy.can_place_pos2d(*p, color.opposite()));

        let mut best_oppo_score = -99999;
        let mut best_oppo_move: Option<(usize, usize)> = None;
        for mv_oppo in possible_oppo_moves {
            let mut board_copy2 = board_copy.clone();
            trace!("  mv_oppo: {}{}", ((mv_oppo.i as u8)+97) as char, mv_oppo.j);
            board_copy2.place(mv_oppo.i, mv_oppo.j, color.opposite());
            let oppo_score = eval(&board_copy2, color.opposite());
            trace!("  -> {}", oppo_score);
            if oppo_score > best_oppo_score {
                best_oppo_score = oppo_score;
                best_oppo_move = Some((mv_oppo.i, mv_oppo.j));
            }
        }

        let score = match best_oppo_move {
            Some(_) => - best_oppo_score,
            None => eval(&board_copy, color)
        };

        info!("mv: {}{} -> {}", ((mv.i as u8)+97) as char, mv.j, score);
        trace!("-> {}", score);
        if score > best_score {
            trace!("it's best: score = {:?}", score);
            best_score = score;
            best_move = Some((mv.i, mv.j));
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
    if occupied < 48 {
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