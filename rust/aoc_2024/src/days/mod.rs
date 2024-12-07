use crate::{Ev, ItemTX};
use color_eyre::Result;

#[derive(Debug, Default, Clone)]
pub struct Answer {
    pub parta: Option<String>,
    pub partb: Option<String>,
}

pub trait TX {
    fn update(&mut self, str: Vec<String>) -> Result<()>;
    fn append(&mut self, str: Vec<String>) -> Result<()>;
    fn done(&mut self, ans: Answer) -> Result<()>;
}

impl TX for ItemTX {
    fn update(&mut self, str: Vec<String>) -> Result<()> {
        let (i, send) = self;
        send.send(Ev::Render(*i, str))?;
        Ok(())
    }

    fn append(&mut self, str: Vec<String>) -> Result<()> {
        let (i, send) = self;
        send.send(Ev::RenderAppend(*i, str))?;
        Ok(())
    }

    fn done(&mut self, ans: Answer) -> Result<()> {
        let (i, send) = self;
        send.send(Ev::Done(*i, ans))?;
        Ok(())
    }
}

pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
