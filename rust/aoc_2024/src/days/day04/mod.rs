use super::*;
use crate::days::{BoxedAsync, ItemTX};
mod part_a;
mod part_b;
use color_eyre::Result;

async fn run(mut tx: ItemTX) -> Result<()> {
    tx.done(Answer {
        parta: time_run(part_a::run),
        partb: time_run(part_b::run),
    })?;
    Ok(())
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}
