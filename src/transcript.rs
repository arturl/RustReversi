#![allow(dead_code)]

use log::{info, warn, trace, error, set_max_level};
use std::fmt;

pub struct Transcript {
    pub moves: Vec<(usize,usize)>
}

impl Transcript {
    pub fn new() -> Transcript {
        Transcript { 
            moves: vec![]
        }
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

            t.add(i,j);
            if index == bytes.len() {
                break;
            }
        }
        t
    }

    pub fn add(&mut self, i: usize, j: usize) {
        self.moves.push((i,j))
    }

}

impl fmt::Display for Transcript {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::from("");
        for (i,j) in self.moves.clone() {
            output.push(((i as u8)+97) as char);
            output.push_str(&j.to_string());
        }
        write!(f, "{}", output )
    }
}
