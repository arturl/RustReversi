#![allow(dead_code)]

use crate::color::Color;

use log::{info, warn, trace, error, set_max_level};

pub struct Pos {
    index: usize,
}

impl Pos {
    fn new(i: usize) -> Pos {
        Pos { index: i }
    }

    fn i(&self) -> usize {
        self.index
    }
}

impl Iterator for Pos {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        if self.index < 64 {
            Some(self.index)
        } else {
            None
        }
    }
}

pub struct Pos2D {
    i: usize,
    j: usize
}

impl Pos2D {
    fn new(ii: usize, jj: usize) -> Pos2D {
        Pos2D { i: ii, j: jj }
    }

    pub fn ij(&self) -> (usize, usize) {
        (self.i, self.j)
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

    pub fn iter_pos(&self) -> impl Iterator<Item = Pos> + '_ {
        self.board.iter().enumerate().map(|(x,y)| Pos::new(x))
    }

    #[allow(non_snake_case)]
    pub fn iter_pos2D(&self) -> impl Iterator<Item = Pos2D> + '_ {
        self.board.iter().enumerate().map(|(x,y)| Pos2D::new(x/8, x %8))
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
        self.set_at_pos(Pos::new(j*8+i),color)
    }

    pub fn get_at(&self, i: usize, j: usize) -> Color {
        self.get_at_pos(Pos::new(j*8+i))
    }

    pub fn get_at_pos(&self, pos: Pos) -> Color {
        self.board[pos.i()]
    }

    pub fn set_at_pos(&mut self, pos: Pos, color: Color) {
        self.board[pos.i()] = color
    }

    pub fn set_at_c(&mut self, i: char, j: usize, color: Color) {
        let lc = i.to_ascii_lowercase();
        let ii = (lc.to_digit(36).unwrap() - 10) as usize;
        self.set_at(ii,j,color);
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
