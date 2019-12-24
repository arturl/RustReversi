#![allow(dead_code)]

use log::{info, warn, trace, error, set_max_level};
use std::time::{Duration, Instant};
use std::fmt;

pub struct Stat {
    pub nodes_viewed: u32,
    pub start: Instant,
}

impl Stat {
    pub fn new() -> Stat {
        Stat { 
            nodes_viewed: 0,
            start: Instant::now(),
        }
    }
}
