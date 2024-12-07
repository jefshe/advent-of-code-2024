use crate::days::Answer;
use color_eyre::eyre::Result;
use crossterm::event::*;
use futures::StreamExt;
use ratatui::crossterm::event::{KeyCode, KeyEventKind};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub type TX = UnboundedSender<Ev>;
pub type RX = UnboundedReceiver<Ev>;
pub type ItemTX = (usize, TX);

pub enum Ev {
    InProgress(usize),
    Render(usize, Vec<String>),
    Done(usize, Answer),
    Up,
    Down,
    Quit,
    Run,
    Resize,
}

pub struct EventHandler {
    pub tx: TX,
    pub events: RX,
    pub task: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = unbounded_channel::<Ev>();
        let key_tx = tx.clone();
        let task = tokio::spawn(async move {
            let mut reader = EventStream::new();
            loop {
                match reader.next().await {
                    Some(Ok(Event::Key(k))) if k.kind == KeyEventKind::Press => match k.code {
                        KeyCode::Char('q') | KeyCode::Esc => key_tx.send(Ev::Quit),
                        KeyCode::Char('j') | KeyCode::Down => key_tx.send(Ev::Down),
                        KeyCode::Char('k') | KeyCode::Up => key_tx.send(Ev::Up),
                        KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                            key_tx.send(Ev::Run)
                        }
                        _ => Ok(()),
                    },
                    Some(Ok(Event::Resize(_, _))) => key_tx.send(Ev::Resize),
                    _ => continue,
                }
                .expect("Unable to send event");
            }
        });

        Self {
            events: rx,
            tx,
            task,
        }
    }
    pub async fn next(&mut self) -> Result<Ev> {
        self.events
            .recv()
            .await
            .ok_or(color_eyre::eyre::eyre!("Unable to get event"))
    }

    pub async fn tx(&self) -> TX {
        self.tx.clone()
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        self.task.abort();
    }
}
