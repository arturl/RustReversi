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

            if level < max_level {
                let best2 = find_best_move(&board_copy2, color, level + 1, max_level, stat);
                oppo_score = match best2 {
                    Some(s) => s.1,
                    None => eval(&board_copy2, color, max_level),
                };
            } else {
                oppo_score = eval(&board_copy2, color, max_level);
            }

            // Alpha-beta pruning
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
            None => eval(&board_copy, color, max_level),
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

pub fn eval(board: &Board, color: Color, level: i32) -> i32 {
    let occupied = board.num_occupied();
    let mut score: i32;
    if occupied < (64 - 2*level) as usize {

        // What matters at this stage is stable cells, plus minimizing number of opponent moves

        score =   board.get_available_moves_for(color).count() as i32
                - board.get_available_moves_for(color.opposite()).count() as i32;

        let corner0 = board.get_at(Pos2D::new(0,0));
        if corner0 == color {
            score += 100;
            if board.get_at(Pos2D::new(1,0)) == color {
                score += 50;
                if board.get_at(Pos2D::new(2,0)) == color {
                    score += 50;
                    if board.get_at(Pos2D::new(3,0)) == color {
                        score += 50;
                    }
                }
            }
            if board.get_at(Pos2D::new(0,1)) == color {
                score += 50;
                if board.get_at(Pos2D::new(0,2)) == color {
                    score += 50;
                    if board.get_at(Pos2D::new(0,3)) == color {
                        score += 50;
                    }
                }
            }
        }
        else if corner0 == Color::Empty {
            if board.get_at(Pos2D::new(1,1)) == color {
                score -= 100;
            }
        }

        let corner1 = board.get_at(Pos2D::new(7,0));
        if corner1 == color {
            score += 100;
            if board.get_at(Pos2D::new(6,0)) == color {
                score += 50;
                if board.get_at(Pos2D::new(5,0)) == color {
                    score += 50;
                    if board.get_at(Pos2D::new(4,0)) == color {
                        score += 50;
                    }
                }
            }
            if board.get_at(Pos2D::new(7,1)) == color {
                score += 50;
                if board.get_at(Pos2D::new(7,2)) == color {
                    score += 50;
                    if board.get_at(Pos2D::new(7,3)) == color {
                        score += 50;
                    }
                }
            }
        }
        else if corner1 == Color::Empty {
            if board.get_at(Pos2D::new(6,1)) == color {
                score -= 100;
            }
        }

        let corner2 = board.get_at(Pos2D::new(0,7));
        if corner2 == color {
            score += 100;
            if board.get_at(Pos2D::new(1,7)) == color {
                score += 50;
                if board.get_at(Pos2D::new(2,7)) == color {
                    score += 50;
                    if board.get_at(Pos2D::new(3,7)) == color {
                        score += 50;
                    }
                }
            }
            if board.get_at(Pos2D::new(0,6)) == color {
                score += 50;
                if board.get_at(Pos2D::new(0,5)) == color {
                    score += 50;
                    if board.get_at(Pos2D::new(0,4)) == color {
                        score += 50;
                    }
                }
            }
        }
        else if corner2 == Color::Empty {
            if board.get_at(Pos2D::new(1,6)) == color {
                score -= 100;
            }
        }

        let corner3 = board.get_at(Pos2D::new(7,7));
        if corner3 == color {
            score += 100;
            if board.get_at(Pos2D::new(6,7)) == color {
                score += 50;
                if board.get_at(Pos2D::new(5,7)) == color {
                    score += 50;
                    if board.get_at(Pos2D::new(4,7)) == color {
                        score += 50;
                    }
                }
            }
            if board.get_at(Pos2D::new(7,6)) == color {
                score += 50;
                if board.get_at(Pos2D::new(7,5)) == color {
                    score += 50;
                    if board.get_at(Pos2D::new(7,4)) == color {
                        score += 50;
                    }
                }
            }
        }
        else if corner3 == Color::Empty {
            if board.get_at(Pos2D::new(6,6)) == color {
                score -= 100;
            }
        }

    } else {
        score = board.num_of_color(color) as i32 - board.num_of_color(color.opposite()) as i32;
    }
    score
}
