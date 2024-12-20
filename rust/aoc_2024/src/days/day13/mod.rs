use super::*;
use crate::util::*;
use color_eyre::Result;
use itertools::Itertools;
const FILE: &str = "day13";

type XY = (usize, usize);
#[derive(Debug)]
struct Problem {
    a: XY,
    b: XY,
    prize: XY,
}

async fn run(mut tx: ItemTX) -> Result<()> {
    let parta = time_run(|| parta());
    let partb = time_run(|| partb());
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta() -> String {
    format!("{}", 0)
}
pub fn partb() -> String {
    format!("{}", 0)
}

fn input() -> Vec<Problem> {
    split_lines_iter(FILE, r"[^0-9]+")
        .chunks(4)
        .map(|chnk| Problem {
            a: chnk[0]
                .iter()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap(),
            b: chnk[1]
                .iter()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap(),
            prize: chnk[1]
                .iter()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap(),
        })
        .collect()
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", input());
    }
}
