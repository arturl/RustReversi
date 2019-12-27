#![allow(dead_code)]

use crate::color::Color;
use crate::stat::*;
use crate::transcript::*;
use log::{error, info, set_max_level, trace, warn};
use std::fmt;
use std::mem;

#[derive(Clone, Copy, Debug)]
pub struct Pos2D {
    pub i: usize,
    pub j: usize,
}

impl Pos2D {
    pub fn new(ii: usize, jj: usize) -> Pos2D {
        Pos2D { i: ii, j: jj }
    }
}

impl fmt::Display for Pos2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", ((self.i as u8) + 97) as char, self.j)
    }
}

impl PartialEq for Pos2D {
    fn eq(&self, other: &Self) -> bool {
        (self.i == other.i) && (self.j == other.j)
    }
}

//#[derive(Clone)]
pub struct Board {
    board_data: [Color; 64],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board_data: [Color::Empty; 64],
        }
    }

    pub fn new_from(other: &Board) -> Board {
        Board {
            board_data: other.board_data.clone()
        }
    }

    pub fn num_occupied(&self) -> usize {
        64 - self.num_of_color(Color::Empty)
    }

    const POSITIONS_NORM : [usize; 64] = [ 0, 1, 2, 3, 4, 5, 6, 7,
                                           8, 9,10,11,12,13,14,15,
                                          16,17,18,19,20,21,22,23,
                                          24,25,26,27,28,29,30,31,
                                          32,33,34,35,36,37,38,39,
                                          40,41,42,43,44,45,46,47,
                                          48,49,50,51,52,53,54,55,
                                          56,57,58,59,60,61,62,63 ];

    // This order is optimized for alpha-beta: moves that tend to be better
    // (such as corners) are considered first
    const POSITIONS_OPT : [usize; 60] =  [    0, 7,56,63, 2, 5,16,23,
                                             40,47,58,61, 1, 3, 4, 6,
                                              8,10,11,12,13,15,17,18,
                                             19,20,21,22,24,25,26,29,
                                             30,31,32,33,34,37,38,39,
                                             41,42,43,44,45,46,48,50,
                                             51,52,53,55,57,59,60,62,
                                             9,14,49,54];

    fn get_positions(&self) -> impl Iterator<Item = Pos2D> {
        Board::POSITIONS_OPT.iter()
            .map(|i| Pos2D::new(i / 8, i % 8))
    }

    pub fn num_of_color(&self, color: Color) -> usize {
        self.get_positions()
            .filter(|p| self.get_at(*p) == color)
            .count()
    }

    pub fn has_any_moves(&self, color: Color) -> bool {
        self.get_positions()
            .any(|p| self.can_place(p, color))
    }

    pub fn get_available_moves_for(&self, color: Color) -> impl Iterator<Item = Pos2D> + '_ {
        self.get_positions()
            .filter(move |p| self.can_place(*p, color))
    }

    pub fn count_available_moves(&self, color1: Color, color2: Color) -> (i32, i32) {
        self.get_positions()
            .map(|p| (if self.can_place(p, color1) { 1 } else { 0 }, if self.can_place(p, color2) { 1 } else { 0 }))
            .fold((0,0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
    }

    const DIRS: [[i32; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, 1],
        [0, -1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    pub fn print(&self) {
        println!("      A     B     C     D     E     F     G     H");
        //println!("      0     1     2     3     4     5     6     7");
        println!("   |-----|-----|-----|-----|-----|-----|-----|-----");
        for i in 0..8 {
            print!("{}  |", i);
            for j in 0..8 {
                print!("  {}  |", self.get_at(Pos2D::new(j, i)));
            }
            println!("");
            println!("   |-----|-----|-----|-----|-----|-----|-----|-----");
        }
    }

    fn char_to_index(c: char) -> usize {
        (c.to_ascii_lowercase() as i8 - 97) as usize
    }

    pub fn get_at_c(&self, i: char, j: usize) -> Color {
        self.get_at(Pos2D::new(Board::char_to_index(i), j))
    }

    pub fn set_at_c(&mut self, i: char, j: usize, color: Color) {
        self.set_at(Pos2D::new(Board::char_to_index(i), j), color)
    }

    pub fn set_at(&mut self, p: Pos2D, color: Color) {
        self.set_at_pos_internal(p.j * 8 + p.i, color)
    }

    pub fn get_at(&self, p: Pos2D) -> Color {
        self.get_at_pos_internal(p.j * 8 + p.i)
    }

    fn get_at_pos_internal(&self, index: usize) -> Color {
        // turning off bounds checking appears to have no impact on speed
        // unsafe {
        //     *self.board_data.get_unchecked(index)
        // }
        self.board_data[index]
    }

    fn set_at_pos_internal(&mut self, index: usize, color: Color) {
        // unsafe {
        //     *self.board_data.get_unchecked_mut(index) = color
        // }
        self.board_data[index] = color
    }

    pub fn place(&mut self, position: Pos2D, color: Color) {
        if self.get_at(position) != Color::Empty {
             panic!("Cannot place to non-empty cell");
        }
        let mut total_flipped = 0;
        let opposite = color.opposite();
        for direction in &Board::DIRS {
            let mut flipped = 0;
            let mut new_position = position;
            loop {
                new_position.i = (new_position.i as i32 + direction[0]) as usize;
                new_position.j = (new_position.j as i32 + direction[1]) as usize;
                if new_position.i < 8 && new_position.j < 8 {
                    let color_at_this_position = self.get_at(new_position);
                    if color_at_this_position == opposite {
                        flipped += 1;
                    } else {
                        if color_at_this_position == color && flipped > 0 {
                            // Go back to the beginning and flip all cells in this direction
                            let mut flipping_position = position;
                            for _ in 0..flipped {
                                flipping_position = Pos2D::new(
                                    (flipping_position.i as i32 + direction[0]) as usize,
                                    (flipping_position.j as i32 + direction[1]) as usize,
                                );
                                self.set_at(flipping_position, color);
                                total_flipped += 1;
                            }
                            break;
                        }
                        break;
                    }
                } else {
                    // this direction got us out of bounds, exit
                    break;
                }
            }
        }

        if total_flipped > 0 {
            self.set_at(position, color);
        } else {
            let can = self.can_place(position, color);
            self.print();
            panic!(
                "{} is not a valid position for {:?}. Can=={}",
                position,
                color,
                can
            )
        }
    }

    pub fn can_place(&self, position: Pos2D, color: Color) -> bool {
        if self.get_at(position) != Color::Empty {
            return false;
        }
        let opposite = color.opposite();
        for direction in &Board::DIRS {
            let mut flipped = 0;
            let mut new_position = position;
            loop {
                new_position.i = (new_position.i as i32 + direction[0]) as usize;
                new_position.j = (new_position.j as i32 + direction[1]) as usize;
                if new_position.i < 8 && new_position.j < 8 {
                    let color_at_this_position = self.get_at(new_position);
                    if color_at_this_position == opposite {
                        flipped += 1;
                    } else {
                        if color_at_this_position == color && flipped > 0 {
                            return true;
                        }
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        false
    }

    pub fn replay_transcript(&mut self, transcript: &Transcript) -> Color {
        let mut last_mover = Color::White; // ??
        for mv in transcript.moves.clone() {
            self.place(mv.1, mv.0);
            last_mover = mv.0;
        }
        last_mover
    }
}
