use crate::event_handler::{Ev, TX};
use color_eyre::Result;
use ratatui::{
    text::{Line, Span},
    widgets::Paragraph,
};
use std::{future::Future, pin::Pin, time::Duration};

pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

pub type BoxedAsync = Pin<Box<dyn Future<Output = Result<()>> + Send>>;
pub type ItemTX = (usize, TX);
pub type AsyncCall = fn(ItemTX) -> BoxedAsync;

#[derive(Debug, Default, Clone)]
pub struct Answer {
    pub parta: (String, Duration),
    pub partb: (String, Duration),
}

impl Answer {
    pub fn into_paragraph(&self) -> Paragraph {
        Paragraph::new(vec![
            self.as_line(&self.parta, "Part A"),
            self.as_line(&self.partb, "Part B"),
        ])
    }

    fn as_line(&self, part: &(String, Duration), title: &str) -> Line {
        Line::from(vec![
            Span::raw(format!("{title}: {}", part.0)),
            Span::raw(format!("   [{}]", format_duration(&part.1))),
        ])
    }
}
fn format_duration(d: &Duration) -> String {
    if d.as_secs() > 100 {
        format!("{}s", d.as_secs())
    } else if d.as_millis() > 1000 {
        format!("{}.{}s", d.as_secs(), d.subsec_millis() / 1000)
    } else {
        format!("{:.3}ms", d.as_millis_f32())
    }
}

pub trait Comms {
    fn update(&mut self, str: Vec<String>) -> Result<()>;
    fn append(&mut self, str: String) -> Result<()>;
    fn done(&mut self, ans: Answer) -> Result<()>;
    fn send(&mut self, ev: Ev) -> Result<()>;
}

impl Comms for ItemTX {
    fn update(&mut self, str: Vec<String>) -> Result<()> {
        self.send(Ev::Render(self.0, str))
    }

    fn append(&mut self, str: String) -> Result<()> {
        self.send(Ev::RenderAppend(self.0, str))
    }

    fn done(&mut self, ans: Answer) -> Result<()> {
        self.send(Ev::Done(self.0, ans))
    }

    fn send(&mut self, ev: Ev) -> Result<()> {
        let send = &self.1;
        if !send.is_closed() {
            send.send(ev)?;
        }
        Ok(())
    }
}

fn time_run<F: FnOnce() -> String>(cb: F) -> (String, Duration) {
    let start = std::time::Instant::now();
    let ans = cb();
    let duration = start.elapsed();
    (ans, duration)
}
