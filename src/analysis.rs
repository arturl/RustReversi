#![allow(dead_code)]

use log::{error, info, set_max_level, trace, warn};
use std::time::{Duration, Instant};

use crate::board::*;
use crate::color::Color;
use crate::stat::Stat;

pub fn find_best_move(
    board: &Board,
    color: Color,
    level: i32,
    max_level: i32,
    stat: &mut Stat) -> Option<(Pos2D, i32)> {
    let possible_moves = board.get_available_moves_for(color);
    let mut max_score = std::i32::MIN;
    let mut best_move: Option<(Pos2D, i32)> = None;

    for mv in possible_moves {
        let mut board_copy = Board::new_from(board);
        board_copy.place(mv, color);
        stat.nodes_viewed += 1;

        let possible_oppo_moves = board_copy.get_available_moves_for(color.opposite());

        let mut min_score = std::i32::MAX;
        let mut best_oppo_move: Option<(Pos2D, i32)> = None;
        for mv_oppo in possible_oppo_moves {
            let mut board_copy2 = Board::new_from(&board_copy);
            board_copy2.place(mv_oppo, color.opposite());
            stat.nodes_viewed += 1;

            let oppo_score: i32;

            if max_level > level {
                let best2 = find_best_move(&board_copy2, color, level + 1, max_level, stat);
                oppo_score = match best2 {
                    Some(s) => s.1,
                    None => eval(&board_copy2, color),
                };
            } else {
                oppo_score = eval(&board_copy2, color);
            }

            if oppo_score < max_score {
                best_oppo_move = best_move;
                break;
            }
            if oppo_score < min_score {
                min_score = oppo_score;
                best_oppo_move = Some((mv_oppo, oppo_score));
            }
        }

        let score = match best_oppo_move {
            Some((_, s)) => s,
            None => eval(&board_copy, color),
        };

        if score > max_score {
            max_score = score;
            best_move = Some((mv, max_score));
        }
    }
    best_move
}

pub fn caclulate_score(board: &Board) -> (usize, usize) {
    (
        board.num_of_color(Color::Black),
        board.num_of_color(Color::White),
    )
}

fn eval_corner_worker(
    board: &Board,
    color: Color,
    corner: Pos2D,
    precorner1: Pos2D,
    precorner2: Pos2D,
    badprecorner: Pos2D,
) -> i32 {
    let mut score: i32 = 0;
    if board.get_at(corner) == color {
        score += 100;
        if board.get_at(precorner1) == color {
            score += 50;
        }
        if board.get_at(precorner2) == color {
            score += 50;
        }
    } else {
        // Bad pre-corner
        if board.get_at(badprecorner) == color {
            score -= 100;
        }
    }
    score
}

fn eval_corner(
    board: &Board,
    color: Color,
    corner: Pos2D,
    precorner1: Pos2D,
    precorner2: Pos2D,
    badprecorner: Pos2D,
) -> i32 {
    eval_corner_worker(board, color, corner, precorner1, precorner2, badprecorner)
        - eval_corner_worker(
            board,
            color.opposite(),
            corner,
            precorner1,
            precorner2,
            badprecorner,
        )
}

pub fn eval(board: &Board, color: Color) -> i32 {
    let occupied = board.num_occupied();
    let mut score: i32;
    if occupied < 54 {
        score = board.get_available_moves_for(color).count() as i32
            - board.get_available_moves_for(color.opposite()).count() as i32;

        // What matters at this stage is stable cells, plus minimizing number of opponent moves
        score += eval_corner(
            board,
            color,
            Pos2D::new(0,0),
            Pos2D::new(1,0),
            Pos2D::new(0,1),
            Pos2D::new(1,1),
        );

        score += eval_corner(
            board,
            color,
            Pos2D::new(7,0),
            Pos2D::new(7,1),
            Pos2D::new(6,0),
            Pos2D::new(6,1),
        );

        score += eval_corner(
            board,
            color,
            Pos2D::new(7,7),
            Pos2D::new(7,6),
            Pos2D::new(6,7),
            Pos2D::new(6,6),
        );

        score += eval_corner(
            board,
            color,
            Pos2D::new(0,7),
            Pos2D::new(0,6),
            Pos2D::new(1,7),
            Pos2D::new(1,6),
        );
    } else {
        score = board.num_of_color(color) as i32 - board.num_of_color(color.opposite()) as i32;
    }
    score
}
