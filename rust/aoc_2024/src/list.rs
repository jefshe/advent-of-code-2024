use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{ListItem, ListState, Paragraph, Widget, Wrap},
};
use std::{future::Future, pin::Pin};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{block, days::*, gfx::*};

pub type TX = (usize, UnboundedSender<AOCUpdate>);
pub type RX = UnboundedReceiver<AOCUpdate>;

pub enum AOCUpdate {
    InProgress(usize),
    Render(usize, Vec<String>),
    Done(usize, Answer),
}

pub struct AOCList {
    pub items: Vec<AOCDay>,
    pub state: ListState,
    pub events: UnboundedReceiver<AOCUpdate>,
    pub sender: UnboundedSender<AOCUpdate>,
}
impl Default for AOCList {
    fn default() -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<AOCUpdate>();
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
            events: rx,
            sender: tx,
        }
    }
}
impl AOCList {
    pub fn run(&mut self, i: usize) {
        self.sender.send(AOCUpdate::InProgress(i)).expect("gg");
        self.items[i].run((i, self.sender.clone()));
    }

    pub fn update(&mut self) {
        while let Ok(event) = self.events.try_recv() {
            match event {
                AOCUpdate::Render(i, txt) => {
                    // println!("Received {:?}", txt);
                    self.items[i].viz = Some(txt);
                }
                AOCUpdate::Done(i, ans) => {
                    // println!("Received {:?}", ans);
                    self.items[i].answer = Some(ans);
                }
                AOCUpdate::InProgress(i) => {
                    self.items[i].viz = Some(vec!["In Progress...".to_string()]);
                }
            }
        }
        self.state.select(self.state.selected());
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

    pub fn run(&mut self, tx: TX) {
        match self.runner {
            Some(r) => {
                let fut = r(tx);
                tokio::spawn(async move {
                    fut.await.unwrap();
                });
            }
            None => {
                self.answer = Some(Answer::default());
            }
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if let Some(txt) = &self.viz {
            let [answer, viz] =
                Layout::horizontal([Constraint::Fill(ANSWER_RATIO), Constraint::Fill(VIZ_RATIO)])
                    .areas(area);

            Paragraph::new(match &self.answer {
                Some(ans) => vec![
                    Line::styled(format!("Part A: {:?}", ans.partb), ANSWER_TEXT_COLOR),
                    Line::styled(format!("Part B: {:?}", ans.partb), ANSWER_TEXT_COLOR),
                ],
                None => vec![Line::styled("In Progress...", INPROGRESS_TEXT_COLOR)],
            })
            .block(block("Answer"))
            .wrap(Wrap { trim: false })
            .render(answer, buf);
            Paragraph::new(txt.iter().map(Line::raw).collect::<Vec<Line>>())
                .block(block(""))
                .render(viz, buf);
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
