#![feature(let_chains)]
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE, YELLOW},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal,
};

pub mod days;
pub mod util;
use crate::days::*;

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;
const INCOMPLETE_TEXT_FG_COLOR: Color = YELLOW.c500;
const TOP_RATIO: u16 = 3;
const BOTTOM_RATIO: u16 = 3;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

/// This struct holds the current state of the app. In particular, it has the `aoc_list` field
/// which is a wrapper around `ListState`. Keeping track of the state lets us render the
/// associated widget with its state and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events. Check
/// the drawing logic for items on how to specify the highlighting style for selected items.
struct App {
    should_exit: bool,
    aoc_list: AOCList,
}

struct AOCList {
    items: Vec<AOCDay>,
    state: ListState,
}

struct AOCDay {
    title: String,
    day: Option<Box<dyn Day>>,
}

impl Default for App {
    fn default() -> Self {
        let mut aoc_list = AOCList {
            items: vec![
                AOCDay::new("Day 4", Box::new(Day4::new())),
                AOCDay::todo("Day 5"),
                AOCDay::todo("Day 6"),
                AOCDay::todo("Day 7"),
                AOCDay::todo("Day 8"),
                AOCDay::todo("Day 9"),
                AOCDay::todo("Day 10"),
                AOCDay::todo("Day 11"),
                AOCDay::todo("Day 12"),
                AOCDay::todo("Day 13"),
                AOCDay::todo("Day 14"),
                AOCDay::todo("Day 15"),
                AOCDay::todo("Day 16"),
                AOCDay::todo("Day 17"),
                AOCDay::todo("Day 18"),
                AOCDay::todo("Day 19"),
                AOCDay::todo("Day 20"),
            ],
            state: ListState::default(),
        };
        aoc_list.state.select(Some(0));
        Self {
            should_exit: false,
            aoc_list,
        }
    }
}

impl AOCDay {
    fn new(title: &str, day: Box<dyn Day>) -> Self {
        Self {
            title: title.to_string(),
            day: Some(day),
        }
    }
    fn todo(title: &str) -> Self {
        Self {
            title: title.to_string(),
            day: None,
        }
    }
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
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
    fn run_exercise(&mut self) {}

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

        let [list_area, item_area, viz_area] =
            Layout::vertical([Constraint::Fill(TOP_RATIO), Constraint::Fill(1), Constraint::Fill(BOTTOM_RATIO)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, viz_area, buf);
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
        let block = Block::new()
            .title(Line::raw("Select A Day").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .aoc_list
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.aoc_list.state);
    }

    fn render_selected_item(&self, area: Rect, viz_area: Rect, buf: &mut Buffer) {
        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("Answer").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        if let Some(i) = self.aoc_list.state.selected()
            && let Some(day) = &self.aoc_list.items[i].day
        {
            let answer = day.run(viz_area, buf);
            Paragraph::new(vec![
                Line::styled(
                    format!("Part A: {}", &answer.parta.unwrap_or("NA".into())),
                    TEXT_FG_COLOR,
                ),
                Line::styled(
                    format!("Part B: {}", &answer.partb.unwrap_or("NA".into())),
                    TEXT_FG_COLOR,
                ),
            ])
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
        }
        // We can now render the item info
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&AOCDay> for ListItem<'_> {
    fn from(value: &AOCDay) -> Self {
        match value.day {
            Some(_) => Self::new(Line::styled(
                format!(" ✓ {}", value.title),
                COMPLETED_TEXT_FG_COLOR,
            )),
            None => Self::new(Line::styled(
                format!(" x {}", value.title),
                INCOMPLETE_TEXT_FG_COLOR,
            )),
        }
    }
}
