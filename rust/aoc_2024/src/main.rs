#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(duration_millis_float)]
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{HighlightSpacing, List, ListItem, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};
use tokio::{self, runtime::Builder};

pub mod days;
pub mod event_handler;
pub mod gfx;
pub mod griddy;
pub mod list;
pub mod point;
pub mod util;

use crate::event_handler::*;
use crate::gfx::*;
use crate::list::*;

fn main() {
    Builder::new_multi_thread()
        .thread_stack_size(5 * 1024 * 1024)
        .build()
        .expect("Unable to create runtime")
        .block_on(start_app())
        .expect("Unable to start app");
}

async fn start_app() -> Result<()> {
    color_eyre::install()?;
    let events = EventHandler::new();
    let terminal = ratatui::init();
    let app_result = App::new(events.tx().await).run(events, terminal).await;
    ratatui::restore();
    app_result
}

struct App {
    should_exit: bool,
    aoc_list: AOCList,
    tx: TX,
}

impl App {
    fn new(tx: TX) -> Self {
        let mut aoc_list = AOCList::default();
        aoc_list.state.select(Some(0));
        Self {
            should_exit: false,
            aoc_list,
            tx,
        }
    }

    async fn run(mut self, mut events: EventHandler, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            let ev = events.next().await?;
            self.update_state(ev);
        }
        events.task.abort();
        Ok(())
    }

    fn update_state(&mut self, event: Ev) {
        match event {
            Ev::Quit => self.should_exit = true,
            Ev::Up => self.aoc_list.state.select_previous(),
            Ev::Down => self.aoc_list.state.select_next(),
            Ev::Run if let Some(i) = self.aoc_list.state.selected() => {
                self.aoc_list.run(i, self.tx.clone())
            }
            Ev::InProgress(i) => self.aoc_list.items[i].answer = RunState::InProgress,
            Ev::Render(i, txt) => self.aoc_list.items[i].viz = Some(txt),
            Ev::RenderAppend(i, txt) => match self.aoc_list.items[i].viz {
                Some(ref mut v) => {
                    v.push(txt);
                }
                None => {
                    self.aoc_list.items[i].viz = Some(vec![txt]);
                }
            },
            Ev::Done(i, ans) => self.aoc_list.items[i].answer = RunState::Done(ans),
            _ => {}
        }
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
