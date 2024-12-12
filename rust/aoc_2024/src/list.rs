use crate::{block, days::*, gfx::*, Ev, TX};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::Line,
    widgets::{ListItem, ListState, Padding, Paragraph, Widget, Wrap},
};

pub struct AOCList {
    pub items: Vec<AOCDay>,
    pub state: ListState,
}
impl Default for AOCList {
    fn default() -> Self {
        Self {
            items: vec![
                AOCDay::new("Day 12", day12::wrapped_run),
                AOCDay::new("Day 11", day11::wrapped_run),
                AOCDay::new("Day 10", day10::wrapped_run),
                AOCDay::new("Day 9", day09::wrapped_run),
                AOCDay::new("Day 8", day08::wrapped_run),
                AOCDay::new("Day 7", day07::wrapped_run),
                AOCDay::new("Day 6", day06::wrapped_run),
                AOCDay::new("Day 5", day05::wrapped_run),
                AOCDay::new("Day 4", day04::wrapped_run),
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
        }
    }
}
impl AOCList {
    pub fn run(&mut self, i: usize, tx: TX) {
        tx.send(Ev::InProgress(i))
            .expect("Unable to send progress update");
        self.items[i].run((i, tx));
    }
}
pub enum RunState {
    InProgress,
    Done(Answer),
    NotRun,
}
pub struct AOCDay {
    pub title: String,
    pub runner: Option<AsyncCall>,
    pub viz: Option<Vec<String>>,
    pub answer: RunState,
    pub task: Option<tokio::task::JoinHandle<()>>,
}

impl AOCDay {
    fn new(title: &str, runner: AsyncCall) -> Self {
        Self {
            title: title.to_string(),
            viz: None,
            answer: RunState::NotRun,
            runner: Some(runner),
            task: None,
        }
    }
    fn todo(title: &str) -> Self {
        Self {
            title: title.to_string(),
            viz: None,
            runner: None,
            answer: RunState::NotRun,
            task: None,
        }
    }

    pub fn run(&mut self, tx: ItemTX) {
        if let Some(r) = self.runner {
            let fut = r(tx);
            self.task = Some(tokio::spawn(async move {
                fut.await.unwrap();
            }));
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if let Some(txt) = &self.viz {
            Paragraph::new(txt.iter().map(Line::raw).collect::<Vec<Line>>())
                .block(block("Answer").padding(Padding::new(
                    0,               // left
                    0,               // right
                    area.height / 4, // top
                    0,               // bottom
                )))
                .alignment(Alignment::Center)
                .render(area, buf);
        }
        match &self.answer {
            RunState::Done(ans) => ans.into_paragraph(),
            RunState::InProgress => Paragraph::new("In Progress...").style(INPROGRESS_TEXT_COLOR),
            RunState::NotRun => Paragraph::new(""),
        }
        .block(block("Answer"))
        .wrap(Wrap { trim: false })
        .render(area, buf);
    }
}

impl From<&AOCDay> for ListItem<'_> {
    fn from(value: &AOCDay) -> Self {
        match value.runner {
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

impl Drop for AOCDay {
    fn drop(&mut self) {
        if let Some(task) = &self.task {
            task.abort();
        }
    }
}
