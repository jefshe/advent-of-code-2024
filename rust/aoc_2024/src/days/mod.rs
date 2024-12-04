use ratatui::buffer::Buffer;

#[derive(Debug, Default, Clone)]
pub struct Answer {
    pub parta: Option<String>,
    pub partb: Option<String>,
}

pub trait Day {
    fn run(&self) -> Answer;
    fn pretty_visualization(&self, buf: &mut Buffer);
}
pub mod day04;
pub use day04::*;
