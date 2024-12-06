use super::Answer;
use crate::{AOCUpdate::*, BoxedAsync, TX};
mod part_a;
mod part_b;
use color_eyre::Result;

async fn run(tx: TX) -> Result<()> {
    let (idx, s) = tx;
    s.send(Done(
        idx,
        Answer {
            parta: Some(part_a::run()),
            partb: Some(part_b::run()),
        },
    ))?;
    Ok(())
}

pub fn wrapped_run(tx: TX) -> BoxedAsync {
    Box::pin(run(tx))
}
