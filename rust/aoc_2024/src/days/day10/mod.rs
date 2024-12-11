use std::collections::HashSet;

use super::*;
use crate::griddy::Griddy;
use crate::point::Pt;
use crate::util::D::*;
use crate::util::*;
use color_eyre::Result;
use rayon::prelude::*;

async fn run(mut tx: ItemTX) -> Result<()> {
    let griddy = input();
    let parta = time_run(|| parta(&griddy, &mut tx));
    let partb = time_run(|| partb(&griddy, &mut tx));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta(griddy: &Griddy<char>, _tx: &mut ItemTX) -> String {
    let mut score = 0;
    for pt in griddy.find_all(&'0') {
        let mut soln = HashSet::new();
        count_nines(griddy, &pt, &mut soln);
        score += soln.len();
    }
    format!("{:?}", score)
}

pub fn partb(griddy: &Griddy<char>, _tx: &mut ItemTX) -> String {
    format!(
        "{:?}",
        griddy
            .find_all(&'0')
            .par_iter()
            .map(|pt| count_hikes(griddy, pt))
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
    [Up, Down, Left, Right]
        .iter()
        .map(|d| *curr + *d)
        .filter(|pt| griddy.check(pt) && griddy[pt] as i32 - griddy[curr] as i32 == 1)
        .map(|pt| count_hikes(griddy, &pt))
        .sum()
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
    // #[test]
    // fn it_works() {
    //     let (tx, _rx) = unbounded_channel::<Ev>();
    //     let mut itx = (0, tx);
    //     println!("{}", partb(&mut itx));
    // }
}
