use std::collections::HashSet;

use super::{time_run, Answer, TX};
use crate::griddy::Griddy;
use crate::point::Pt;
use crate::util::D::*;
use crate::BoxedAsync;
use crate::{util::*, ItemTX};
use color_eyre::Result;

async fn run(mut tx: ItemTX) -> Result<()> {
    let parta = time_run(|| parta(&mut tx));
    let partb = time_run(|| partb(&mut tx));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta(_tx: &mut ItemTX) -> String {
    let griddy = input();
    let mut score = 0;
    for pt in griddy.find_all(&'0') {
        let mut soln = HashSet::new();
        count_nines(&griddy, &pt, &mut soln);
        score += soln.len();
    }
    format!("{:?}", score)
}

pub fn partb(_tx: &mut ItemTX) -> String {
    let griddy = input();

    format!(
        "{:?}",
        griddy
            .find_all(&'0')
            .iter()
            .map(|pt| count_hikes(&griddy, pt))
            .sum::<usize>()
    )
}

pub fn count_nines(griddy: &Griddy<char>, curr: &Pt, soln: &mut HashSet<Pt>) {
    if griddy[curr] == '9' {
        soln.insert(*curr);
        return;
    }
    for d in [Up, Down, Left, Right] {
        let next = *curr + d;
        if griddy.check(&next) && griddy[&next] as i32 - griddy[curr] as i32 == 1 {
            count_nines(griddy, &next, soln)
        }
    }
}

pub fn count_hikes(griddy: &Griddy<char>, curr: &Pt) -> usize {
    if griddy[curr] == '9' {
        return 1;
    }

    let mut count = 0;
    for d in [Up, Down, Left, Right] {
        let next = *curr + d;
        if griddy.check(&next) && griddy[&next] as i32 - griddy[curr] as i32 == 1 {
            count += count_hikes(griddy, &next)
        }
    }
    count
}

pub fn input() -> Griddy<char> {
    let vec = parse_chars("day10");
    Griddy::new(vec)
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
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
        println!("{}", partb(&mut itx));
    }
}
