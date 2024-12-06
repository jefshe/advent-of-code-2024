#![feature(let_chains)]
#![feature(if_let_guard)]
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{HighlightSpacing, List, ListItem, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};
use tokio::{self};

pub mod days;
pub mod gfx;
pub mod griddy;
pub mod list;
pub mod util;

use crate::gfx::*;
use crate::list::*;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal).await;
    ratatui::restore();
    app_result
}

struct App {
    should_exit: bool,
    aoc_list: AOCList,
}

impl Default for App {
    fn default() -> Self {
        let mut aoc_list = AOCList::default();
        aoc_list.state.select(Some(0));
        Self {
            should_exit: false,
            aoc_list,
        }
    }
}

impl App {
    async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
            self.aoc_list.update();
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                self.run_exercise();
            }
            _ => {}
        }
    }

    /// Changes the status of the selected list item
    fn run_exercise(&mut self) {
        if let Some(i) = self.aoc_list.state.selected() {
            self.aoc_list.run(i);
        }
    }

    fn select_none(&mut self) {
        self.aoc_list.state.select(None);
    }

    fn select_next(&mut self) {
        self.aoc_list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.aoc_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.aoc_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.aoc_list.state.select_last();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, selected_area] =
            Layout::vertical([Constraint::Fill(TOP_RATIO), Constraint::Fill(BOTTOM_RATIO)])
                .areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(selected_area, buf);
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("AOC 2024 Rust Solutions")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = block("Select A Day");

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .aoc_list
            .items
            .iter()
            .enumerate()
            .map(|(i, day)| {
                let color = alternate_colors(i);
                ListItem::from(day).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.aoc_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        if let Some(i) = self.aoc_list.state.selected() {
            self.aoc_list.items[i].render(area, buf);
        }
    }
}
