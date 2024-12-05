use ratatui::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Answer {
    pub parta: Option<String>,
    pub partb: Option<String>,
}

pub trait Day {
    fn run(&self, viz_area: Rect, buf: &mut Buffer) -> Answer;
}
pub mod day04;
pub mod day05;
pub use day04::*;
pub use day05::*;
