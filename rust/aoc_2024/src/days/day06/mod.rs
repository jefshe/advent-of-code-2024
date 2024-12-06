use std::collections::{HashMap, HashSet};

use crate::util::{parse_2_parts, parse_chars, parse_lines};

use super::{Answer, Day};
use itertools::Itertools;
use ratatui::prelude::*;
mod part_a;

pub type Rulebook = HashMap<u32, HashSet<u32>>;
#[derive(Debug)]
pub struct Day6 {}

impl Day6 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Day for Day6 {
    fn run(&self, viz: Rect, buf: &mut Buffer) -> Answer {
        Answer {
            parta: Some(part_a::run(viz, buf)),
            partb: None,
        }
    }
}
