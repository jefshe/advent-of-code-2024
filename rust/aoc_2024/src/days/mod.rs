use crate::TX;
use async_trait::async_trait;
use color_eyre::Result;

#[derive(Debug, Default, Clone)]
pub struct Answer {
    pub parta: Option<String>,
    pub partb: Option<String>,
}

#[async_trait]
pub trait Day: Send + Sync {
    async fn run(tx: TX) -> Result<()>;
}
pub mod day04;
pub mod day05;
pub mod day06;
