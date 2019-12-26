#![allow(dead_code)]

use crate::color::Color;
use crate::stat::*;
use crate::transcript::*;
use std::fmt;
use std::mem;
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

#[derive(Clone)]
pub struct Board {
    board_data: [Color; 64]
}

impl Board {
    pub fn new() -> Board {
        Board {
            board_data: [Color::Empty; 64]
        }
    }

    pub fn num_occupied(&self) -> usize {
        64-self.num_of_color(Color::Empty)
    }

    pub fn num_of_color(&self, color: Color) -> usize {
        (0..64usize)
            .map(|i| (i/8, i%8) )
            .filter(|(i,j)| self.get_at(*i, *j) == color)
            .count()
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

    fn get_at_pos(&self, pos: usize) -> Color {
        // turning off bounds checking appears to have no impact on speed
        // unsafe {
        //     *self.board_data.get_unchecked(pos)
        // }
        self.board_data[pos]
    }

    fn set_at_pos(&mut self, pos: usize, color: Color) {
        // unsafe {
        //     *self.board_data.get_unchecked_mut(pos) = color
        // }
        self.board_data[pos] = color
    }

    fn char_to_index(c: char) -> usize {
        (c.to_ascii_lowercase() as i8 - 97) as usize
    }

    pub fn set_at_c(&mut self, i: char, j: usize, color: Color) {
        self.set_at(Board::char_to_index(i),j,color);
    }

    pub fn get_at_c(&self, i: char, j: usize) -> Color {
        self.get_at(Board::char_to_index(i),j)
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
            let mut new_position = Pos2D::new(i, j);
            loop {
                new_position.i = (new_position.i as i32 + direction[0]) as usize; 
                new_position.j = (new_position.j as i32 + direction[1]) as usize;
                if new_position.i < 8 && new_position.j < 8 {
                    let color_at_this_position = self.get_at(new_position.i, new_position.j);
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
            let can = self.can_place(i, j, color);
            self.print();
            panic!("{:?} is not a valid position for {:?}, can=={}", (i,j), color, can)
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

        for direction in &Board::DIRS {
            trace!("--> {:?}", direction);
            let mut flipped = 0;
            let mut new_position = Pos2D::new(i,j);
            loop {
                new_position.i = (new_position.i as i32 + direction[0]) as usize; 
                new_position.j = (new_position.j as i32 + direction[1]) as usize;
                if new_position.i < 8 && new_position.j < 8 {
                    let color_at_this_position = self.get_at(new_position.i, new_position.j);
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
        (0..64usize)
            .map(|i| (i/8, i%8) )
            .any(move |(i,j)| self.can_place(i, j, color))
    }

    pub fn get_available_moves_for(&self, color: Color) -> impl Iterator<Item = Pos2D> + '_ {
        (0..64usize)
            .map(|i| (i/8, i%8) )
            .filter(move |(i,j)| self.can_place(*i, *j, color))
            .map(|(i,j)| Pos2D::new(i, j))
    }

    pub fn replay_transcript(&mut self, transcript: &Transcript) {
        let mut color = Color::Black;
        for mv in transcript.moves.clone() {
            self.place(mv.0, mv.1, color);
            color = color.opposite();
        }
    }

}
