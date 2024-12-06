use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{ListState, Paragraph, Widget, Wrap},
};
use std::{future::Future, pin::Pin};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{block, days::*, ANSWER_RATIO, ANSWER_TEXT_COLOR, VIZ_RATIO};

pub type TX = (usize, UnboundedSender<AOCUpdate>);
pub type RX = UnboundedReceiver<AOCUpdate>;

pub enum AOCUpdate {
    Render(usize, Vec<String>),
    Done(usize, Answer),
}

pub struct AOCList {
    pub items: Vec<AOCDay>,
    pub state: ListState,
    pub events: UnboundedReceiver<AOCUpdate>,
    pub sender: UnboundedSender<AOCUpdate>,
}

impl AOCList {
    pub fn default() -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<AOCUpdate>();
        Self {
            items: vec![
                AOCDay::new("Day 6", day06::wrapped_run),
                AOCDay::new("Day 5", day05::wrapped_run),
                AOCDay::new("Day 4", day04::wrapped_run),
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
            events: rx,
            sender: tx,
        }
    }
}
pub type BoxedAsync = Pin<Box<dyn Future<Output = Result<()>> + Send>>;
pub type AsyncCall = fn(TX) -> BoxedAsync;
pub struct AOCDay {
    pub title: String,
    pub runner: Option<AsyncCall>,
    pub viz: Option<Vec<String>>,
    pub answer: Option<Answer>,
}

impl AOCDay {
    fn new(title: &str, runner: AsyncCall) -> Self {
        Self {
            title: title.to_string(),
            viz: None,
            answer: None,
            runner: Some(runner),
        }
    }
    fn todo(title: &str) -> Self {
        Self {
            title: title.to_string(),
            viz: None,
            answer: None,
            runner: None,
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let [answer, viz] =
            Layout::horizontal([Constraint::Fill(ANSWER_RATIO), Constraint::Fill(VIZ_RATIO)])
                .areas(area);
        if let Some(ans) = &self.answer {
            Paragraph::new(vec![
                Line::styled(format!("Part A: {:?}", ans.partb), ANSWER_TEXT_COLOR),
                Line::styled(format!("Part B: {:?}", ans.partb), ANSWER_TEXT_COLOR),
            ])
            .block(block("Answer"))
            .wrap(Wrap { trim: false })
            .render(answer, buf);
        }

        if let Some(txt) = &self.viz {
            Paragraph::new(txt.iter().map(|row| Line::raw(row)).collect::<Vec<Line>>())
                .block(block("Progress"))
                .render(viz, buf);
        }
    }
}
