use super::time_run;
use super::Answer;
use super::TX;
use crate::BoxedAsync;
use crate::ItemTX;
mod part_a;
mod part_b;
use color_eyre::Result;

#[derive(Debug, Default)]
pub struct Day6 {}

pub async fn run(mut tx: ItemTX) -> Result<()> {
    let parta = time_run(part_a::run);
    let partb = time_run(|| part_b::run(&mut tx));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}
