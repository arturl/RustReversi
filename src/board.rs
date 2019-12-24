#![allow(dead_code)]

use crate::color::Color;
use crate::stat::*;
use crate::transcript::*;
use std::fmt;
use log::{info, warn, trace, error, set_max_level};

#[derive(Clone, Copy)]
pub struct Pos2D {
    pub i: usize,
    pub j: usize
}

impl Pos2D {
    fn new(ii: usize, jj: usize) -> Pos2D {
        Pos2D { i: ii, j: jj }
    }
}

impl fmt::Display for Pos2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", ((self.i as u8)+97) as char, self.j )
    }
}

impl Iterator for Pos2D {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 7 {
            self.i += 1;
        }else{
            if self.j < 7 {
                self.i = 0;
                self.j += 1;
            }
            else {
                return None;
            }
        }
        Some((self.i, self.j))
    }
}

#[derive(Clone)]
pub struct Board {
    board: [Color; 64]
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [Color::Empty; 64]
        }
    }

    pub fn num_occupied(&self) -> usize {
        64-self.num_of_color(Color::Empty)
    }

    pub fn num_of_color(&self, color: Color) -> usize {
        self.board.iter().filter(|x| **x == color).count()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Color> {
        self.board.iter()
    }

    pub fn iter_pos2d(&self) -> impl Iterator<Item = Pos2D> + '_ {
        self.board.iter().enumerate().map(|(x,y)| Pos2D::new(x/8, x %8))
    }

    pub fn replay_transcript(&mut self, transcript: &Transcript) {
        let mut color = Color::Black;
        for mv in transcript.moves.clone() {
            self.place(mv.0, mv.1, color);
            color = color.opposite();
        }
    }

    const DIRS:  [[i32;2]; 8] = [   [-1,-1],
                                    [-1, 0],
                                    [-1, 1],
                                    [ 0, 1],
                                    [ 0,-1],
                                    [ 1,-1],
                                    [ 1, 0],
                                    [ 1, 1]];

    pub fn print(&self) {
        println!("      A     B     C     D     E     F     G     H");
        //println!("      0     1     2     3     4     5     6     7");
        println!("   |-----|-----|-----|-----|-----|-----|-----|-----");
        for i in 0..8 {
            print!("{}  |", i);
            for j in 0..8 {
                print!("  {}  |", self.get_at(j, i));
            }
            println!("");
            println!("   |-----|-----|-----|-----|-----|-----|-----|-----");
        }
    }

    pub fn set_at(&mut self, i: usize, j: usize, color: Color) {
        self.set_at_pos(j*8+i, color)
    }

    pub fn get_at(&self, i: usize, j: usize) -> Color {
        self.get_at_pos(j*8+i)
    }

    pub fn get_at_pos(&self, pos: usize) -> Color {
        self.board[pos]
    }

    pub fn set_at_pos(&mut self, pos: usize, color: Color) {
        self.board[pos] = color
    }

    pub fn set_at_c(&mut self, i: char, j: usize, color: Color) {
        let lc = i.to_ascii_lowercase();
        let ii = (lc.to_digit(36).unwrap() - 10) as usize;
        self.set_at(ii,j,color);
    }

    pub fn get_at_c(&self, i: char, j: usize) -> Color {
        let lc = i.to_ascii_lowercase();
        let ii = (lc.to_digit(36).unwrap() - 10) as usize;
        self.get_at(ii,j)
    }

    pub fn place_pos2d(&mut self, pos: Pos2D, color: Color) {
        self.place(pos.i, pos.j, color)
    }

    pub fn place(&mut self, i: usize, j: usize, color: Color) {
        trace!("initial pos: {:?}", (i,j, color));

        if self.get_at(i,j) != Color::Empty {
            panic!("Cannot place to non-empty cell");
        }

        let mut total_flipped = 0;

        let opposite = color.opposite();

        // Scan in 8 directions:

        for direction in &Board::DIRS {
            trace!("--> {:?}", direction);
            let mut flipped = 0;
            let mut new_position = (i as i32, j as i32);
            loop {
                new_position = (new_position.0 + direction[0], new_position.1 + direction[1]);
                if new_position.0 >= 0 && new_position.0 < 8 && new_position.1 >= 0 && new_position.1 < 8 {
                    let bounds_checked_position = (new_position.0 as usize, new_position.1 as usize);
                    trace!("{:?}", bounds_checked_position);
                    let color_at_this_position = self.get_at(bounds_checked_position.0, bounds_checked_position.1);
                    trace!("c: {:?}", color_at_this_position);
                    if color_at_this_position == opposite {
                        trace!("continue going in this direction");
                        flipped += 1;
                    } else {
                        if color_at_this_position == color && flipped > 0 {
                            trace!("success: can flip {} in this direction", flipped);
                            // Go back to the beginning and flip all cells in this direction
                            let mut flipping_position = (i as i32, j as i32);
                            for _ in 0..flipped {
                                flipping_position = (flipping_position.0 + direction[0], flipping_position.1 + direction[1]);
                                self.set_at(flipping_position.0 as usize, flipping_position.1 as usize, color);
                                total_flipped += 1;
                            }
                            break;
                        }
                        trace!("dropping this direction");
                        break;
                    }
                } else {
                    // this direction got us out of bounds, exit
                    trace!("out of bounds for this direction");
                    break;
                }
            }
        }

        if total_flipped > 0 {
            self.set_at(i, j, color);
        } else {
            panic!("{:?} is not a valid position for {:?}", (i,j), color)
        }
    }

    pub fn can_place_pos2d(&self, pos: Pos2D, color: Color) -> bool {
        self.can_place(pos.i, pos.j, color)
    }

    pub fn can_place(&self, i: usize, j: usize, color: Color) -> bool {
        trace!("initial pos: {:?}", (i,j, color));

        if self.get_at(i,j) != Color::Empty {
            return false;
        }

        let opposite = color.opposite();

        // Scan in 8 directions:

        for direction in &Board::DIRS {
            trace!("--> {:?}", direction);
            let mut flipped = 0;
            let mut new_position = (i as i32, j as i32);
            loop {
                new_position = (new_position.0 + direction[0], new_position.1 + direction[1]);
                if new_position.0 >= 0 && new_position.0 < 8 && new_position.1 >= 0 && new_position.1 < 8 {
                    let bounds_checked_position = (new_position.0 as usize, new_position.1 as usize);
                    trace!("{:?}", bounds_checked_position);
                    let color_at_this_position = self.get_at(bounds_checked_position.0, bounds_checked_position.1);
                    trace!("c: {:?}", color_at_this_position);
                    if color_at_this_position == opposite {
                        trace!("continue going in this direction");
                        flipped += 1;
                    } else {
                        if color_at_this_position == color && flipped > 0 {
                            trace!("success: can flip {} in this direction", flipped);
                            return true;
                        }
                        trace!("dropping this direction");
                        break;
                    }
                } else {
                    // this direction got us out of bounds, exit
                    trace!("out of bounds for this direction");
                    break;
                }
            }
        }
        false
    }

    pub fn has_any_moves(&self, color: Color) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                if self.can_place(i,j,color) {
                    return true;
                }
            }
        }
        false
    }
}
