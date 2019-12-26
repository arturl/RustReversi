#![allow(dead_code)]

use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    Empty,
    Black,
    White,
    Shadow
}

fn color_short_name(color: Color) -> String {
    match color {
        Color::Empty => " ",
        Color::Black => "B",
        Color::White => "W",
        Color::Shadow => "."
    }
    .to_string()
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", color_short_name(*self))
    }
}

impl Color {
    pub fn is_empty(&self) -> bool {
        match self {
            Color::Empty => true,
            _ => false,
        }
    }

    pub fn opposite(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
            _ => panic!("cannot have opposite of empty"),
        }
    }
}
