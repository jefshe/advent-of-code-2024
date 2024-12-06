use ratatui::{
    prelude::*,
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE, YELLOW},
        Color, Modifier, Style,
    },
    widgets::{Block, Borders, Padding},
};
use style::palette::tailwind::RED;

pub const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
pub const NORMAL_ROW_BG: Color = SLATE.c950;
pub const ALT_ROW_BG_COLOR: Color = SLATE.c900;
pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
pub const TEXT_FG_COLOR: Color = SLATE.c200;
pub const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;
pub const ANSWER_TEXT_COLOR: Color = RED.c500;
pub const INCOMPLETE_TEXT_FG_COLOR: Color = YELLOW.c500;
// Layout
pub const TOP_RATIO: u16 = 1;
pub const BOTTOM_RATIO: u16 = 1;

pub const ANSWER_RATIO: u16 = 1;
pub const VIZ_RATIO: u16 = 2;

pub fn block(title: &str) -> Block {
    Block::new()
        .borders(Borders::TOP)
        .border_set(symbols::border::EMPTY)
        .border_style(TODO_HEADER_STYLE)
        .bg(NORMAL_ROW_BG)
        .padding(Padding::horizontal(1))
}
