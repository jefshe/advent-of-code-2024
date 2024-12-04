
use super::{Answer, Day};
use ratatui::prelude::*;
mod part_a;
mod part_b;

#[derive(Debug)]
pub struct Day4 {}

impl Day4 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Day for Day4 {
    fn run(&self, _viz: Rect, _buf: &mut Buffer) -> Answer {
        Answer {
            parta: Some(part_a::run()),
            partb: Some(part_b::run()),
        }
    }
}

