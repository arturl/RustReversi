#![allow(dead_code)]

use log::{error, info, set_max_level, trace, warn};
use std::fmt;
use std::time::{Duration, Instant};

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
