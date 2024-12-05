use super::{Answer, Day};
use ratatui::prelude::*;
mod part_a;
mod part_b;

#[derive(Debug)]
pub struct Day5 {}

impl Day5 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Day for Day5 {
    fn run(&self, _viz: Rect, _buf: &mut Buffer) -> Answer {
        Answer {
            parta: Some(part_a::run()),
            // partb: Some(part_b::run()),
            partb: None,
        }
    }
}
