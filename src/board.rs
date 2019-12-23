#![allow(dead_code)]

use crate::color::Color;

pub struct Board {
    board: [[Color; 8]; 8]
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[Color::Empty; 8];8]
        }
    }

    pub fn print(&self) {
        println!("      A     B     C     D     E     F     G     H");
        println!("   |-----|-----|-----|-----|-----|-----|-----|-----");
        for i in 0..8 {
            print!("{}  |", i);
            for j in 0..8 {
                print!("  {}  |", self.board[i][j]);
            }
            println!("");
            println!("   |-----|-----|-----|-----|-----|-----|-----|-----");
        }
    }

    pub fn set_at(&mut self, i: usize, j: usize, color: Color) {
        self.board[j][i] = color;
    }

    pub fn set_at_c(&mut self, i: char, j: usize, color: Color) {
        let lc = i.to_ascii_lowercase();
        let ii = (lc.to_digit(36).unwrap() - 10) as usize;
        self.set_at(ii,j,color);
    }

}
