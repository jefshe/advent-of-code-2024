use super::{Answer, Day};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Day4 {}

impl Day4 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Day for Day4 {
    fn run(&self) -> Answer {
        Answer {
            parta: Some("Part A".to_string()),
            partb: Some("Part B".to_string()),
        }
    }

    fn pretty_visualization(&self, _buf: &mut ratatui::prelude::Buffer) {}
}
