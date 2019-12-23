#![allow(dead_code)]

use std::fmt;

#[derive(Copy, Clone)]
pub enum Color {
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
