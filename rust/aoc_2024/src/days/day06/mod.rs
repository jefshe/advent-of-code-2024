use super::Answer;
use crate::AOCUpdate::*;
use crate::BoxedAsync;
use crate::TX;
mod part_a;
mod part_b;
use color_eyre::Result;

#[derive(Debug, Default)]
pub struct Day6 {}

pub async fn run(tx: TX) -> Result<()> {
    let parta = part_a::run(&tx);
    let (idx, s) = tx;
    println!("Day 6a: {}", parta);
    s.send(Done(
        idx,
        Answer {
            parta: Some(parta),
            partb: None,
        },
    ))?;
    Ok(())
}

pub fn wrapped_run(tx: TX) -> BoxedAsync {
    Box::pin(run(tx))
}
