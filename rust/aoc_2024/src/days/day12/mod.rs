use super::*;
use crate::{griddy::Griddy, util::*};
use color_eyre::Result;
const FILE: &str = "day12_ex";

async fn run(mut tx: ItemTX) -> Result<()> {
    let griddy = input();
    let parta = time_run(|| parta(&griddy));
    let partb = time_run(|| partb(&griddy));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta(griddy: &Griddy<char>) -> String {
    griddy.strings().join("\n");
    println!("{:?}", griddy);
    0.to_string()
}
pub fn partb(griddy: &Griddy<char>) -> String {
    "todo".to_string()
}

pub fn input() -> Griddy<char> {
    Griddy::new(parse_chars(FILE))
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}

#[cfg(test)]
mod tests {
    use super::input;

    #[test]
    fn it_works() {
        let griddy = input();
        println!("{}", griddy);
    }
}
