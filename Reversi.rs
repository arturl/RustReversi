#![allow(dead_code)]

use std::fmt;

#[derive(Copy, Clone)]
enum Color {
    Empty,
    Black,
    White
}

fn color_short_name(color : Color) -> String
{
    match color {
            Color::Empty => " ",
            Color::Black => "B",
            Color::White => "W",
        }.to_string()
}

fn color_full_name(color : Color) -> String
{
    match color {
            Color::Empty => "Empty",
            Color::Black => "Black",
            Color::White => "White",
        }.to_string()
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", color_short_name(*self))
    }
}

fn print_board(board: &[[Color; 8]; 8]) {
    println!("      A     B     C     D     E     F     G     H");
    println!("   |-----|-----|-----|-----|-----|-----|-----|-----");
    for i in 0..8 {
        print!("{}  |", i);
        for j in 0..8 {
            print!("  {}  |", board[i][j]);
        }
        println!("");
        println!("   |-----|-----|-----|-----|-----|-----|-----|-----");
    }
}

fn set_piece_at(board: & mut [[Color; 8]; 8], i: usize, j: usize, color: Color) {
    board[i][j] = color;
}

fn main() {
    let mut board: [[Color; 8]; 8] = [[Color::Empty; 8];8];
    board[0][0] = Color::Black;

    print_board(& board);
    set_piece_at(&mut board, 1,1,Color::White);
    println!("---");
    print_board(& board);

    println!("b={}", board[0][0]);
}