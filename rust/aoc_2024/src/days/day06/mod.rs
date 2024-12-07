use super::Answer;
use crate::BoxedAsync;
use crate::Ev;
use crate::ItemTX;
mod part_a;
mod part_b;
use color_eyre::Result;

#[derive(Debug, Default)]
pub struct Day6 {}

pub async fn run(tx: ItemTX) -> Result<()> {
    let (idx, s) = tx;
    s.send(Ev::Done(
        idx,
        Answer {
            parta: Some(part_a::run()),
            partb: Some(part_b::run()),
        },
    ))?;
    Ok(())
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}
