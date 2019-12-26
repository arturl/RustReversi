#![allow(dead_code)]

use log::{error, info, set_max_level, trace, warn};
use std::fmt;
use crate::board::*;

pub struct Transcript {
    pub moves: Vec<Pos2D>,
}

impl Transcript {
    pub fn new() -> Transcript {
        Transcript { moves: vec![] }
    }

    pub fn from_trace(trace: &str) -> Transcript {
        let mut t = Transcript::new();
        let bytes = trace.as_bytes();
        let mut index = 0;
        loop {
            let chi = bytes[index];
            index += 1;
            let chj = bytes[index];
            index += 1;

            let i = (chi as usize) - 97;
            let j = (chj as usize) - 48;
            trace!("Mapped {},{} -> {},{}", chi, chj, i, j);

            t.add(Pos2D::new(i, j));
            if index == bytes.len() {
                break;
            }
        }
        t
    }

    pub fn add(&mut self, position: Pos2D) {
        self.moves.push(position)
    }
}

impl fmt::Display for Transcript {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::from("");
        for p in self.moves.clone() {
            output.push(((p.i as u8) + 97) as char);
            output.push_str(&p.j.to_string());
        }
        write!(f, "{}", output)
    }
}
