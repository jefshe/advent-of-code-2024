use crate::{block, days::*, gfx::*, Ev, ItemTX, TX};
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{ListItem, ListState, Paragraph, Widget, Wrap},
};
use std::{future::Future, pin::Pin};

pub struct AOCList {
    pub items: Vec<AOCDay>,
    pub state: ListState,
}
impl Default for AOCList {
    fn default() -> Self {
        Self {
            items: vec![
                AOCDay::new("Day 7", day07::wrapped_run),
                AOCDay::new("Day 6", day06::wrapped_run),
                AOCDay::new("Day 5", day05::wrapped_run),
                AOCDay::new("Day 4", day04::wrapped_run),
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
pub type BoxedAsync = Pin<Box<dyn Future<Output = Result<()>> + Send>>;
pub type AsyncCall = fn(ItemTX) -> BoxedAsync;

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
        match self.runner {
            Some(r) => {
                let fut = r(tx);
                self.task = Some(tokio::spawn(async move {
                    fut.await.unwrap();
                }));
            }
            None => (),
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(match &self.answer {
            RunState::Done(ans) => vec![
                Line::styled(format!("Part A: {:?}", ans.partb), ANSWER_TEXT_COLOR),
                Line::styled(format!("Part B: {:?}", ans.partb), ANSWER_TEXT_COLOR),
            ],
            RunState::InProgress => vec![Line::styled("In Progress...", INPROGRESS_TEXT_COLOR)],
            RunState::NotRun => vec![],
        })
        .block(block("Answer"))
        .wrap(Wrap { trim: false })
        .render(area, buf);
        if let Some(txt) = &self.viz {
            Paragraph::new(txt.iter().map(Line::raw).collect::<Vec<Line>>())
                .block(block(""))
                .render(area, buf);
        }
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
