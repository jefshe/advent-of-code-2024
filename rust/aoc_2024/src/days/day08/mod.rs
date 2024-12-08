use super::{time_run, Answer, TX};
use crate::griddy::Griddy;
use crate::BoxedAsync;
use crate::{util::*, ItemTX};
use color_eyre::Result;

async fn run(mut tx: ItemTX) -> Result<()> {
    let parta = time_run(|| parta(&mut tx));
    let partb = time_run(|| partb(&mut tx));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta(tx: &mut ItemTX) -> String {
    let griddy = input();
    tx.update(griddy.strings()).expect("Unable to update");
    0.to_string()
}

pub fn partb(_tx: &mut ItemTX) -> String {
    0.to_string()
}

fn input() -> Griddy<char> {
    let grid = parse_chars("day08");
    Griddy::new(grid)
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc::unbounded_channel;

    use crate::Ev;

    use super::*;

    #[test]
    fn it_works() {
        let (tx, _rx) = unbounded_channel::<Ev>();
        let mut itx = (0, tx);
        println!("{}", parta(&mut itx));
    }
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}
