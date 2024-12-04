use ratatui::{prelude::*, widgets::{Block, Borders, Padding}};

#[derive(Debug, Default, Clone)]
pub struct Answer {
    pub parta: Option<String>,
    pub partb: Option<String>,
}

pub trait Day {
    fn run(&self, viz_area: Rect, buf: &mut Buffer) -> Answer;
}
pub mod day04;
pub use day04::*;

pub fn render_extra(viz_area: Rect, buf: &mut Buffer) {
    Block::new()
        .title(Line::raw("Extra").centered())
        .borders(Borders::TOP)
        .border_set(symbols::border::EMPTY)
        .padding(Padding::horizontal(1)).render(viz_area, buf);
}