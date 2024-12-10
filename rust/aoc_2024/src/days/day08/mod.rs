use std::collections::HashSet;

use super::{time_run, Answer, TX};
use crate::griddy::Griddy;
use crate::point::Pt;
use crate::BoxedAsync;
use crate::{util::*, ItemTX};
use color_eyre::Result;
use itertools::Itertools;

async fn run(mut tx: ItemTX) -> Result<()> {
    let parta = time_run(|| parta(&mut tx));
    let partb = time_run(|| partb(&mut tx));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta(_tx: &mut ItemTX) -> String {
    let mut griddy = input();
    let groups = griddy
        .data
        .iter()
        .enumerate()
        .filter(|(_, c)| **c != '.')
        .map(|(i, c)| (c, griddy.to_pt(i)))
        .into_group_map();

    let mut solutions = HashSet::<Pt>::new();
    for (_, grp) in groups {
        // tx.append(format!("{} {:?}", c, grp)).unwrap();
        for pair in grp.as_slice().iter().combinations(2) {
            let (a, b) = (pair[0], pair[1]);
            let distance = *b - *a;
            let above = *a - distance;
            let below = *b + distance;
            if griddy.check(&above) {
                solutions.insert(above);
            }
            if griddy.check(&below) {
                solutions.insert(below);
            }
        }
    }
    for i in &solutions {
        griddy[i] = '#'
    }

    format!("{:?}", solutions.len())
}

pub fn partb(tx: &mut ItemTX) -> String {
    let mut griddy = input();
    let groups = griddy
        .data
        .iter()
        .enumerate()
        .filter(|(_, c)| **c != '.')
        .map(|(i, c)| (c, griddy.to_pt(i)))
        .into_group_map();

    let mut solutions = HashSet::<Pt>::new();
    for (_, grp) in groups {
        // tx.append(format!("{} {:?}", c, grp)).unwrap();
        for pair in grp.as_slice().iter().combinations(2) {
            let (a, b) = (pair[0], pair[1]);
            let distance = *b - *a;
            solutions.insert(*a);
            solutions.insert(*b);

            let mut above = *a - distance;
            let mut below = *b + distance;
            while griddy.check(&above) {
                solutions.insert(above);
                above -= distance;
            }
            while griddy.check(&below) {
                solutions.insert(below);
                below += distance;
            }
        }
    }
    for i in &solutions {
        griddy[i] = '#'
    }

    tx.update(griddy.strings()).unwrap();

    format!("{:?}", solutions.len())
}

fn input() -> Griddy<char> {
    let grid = parse_chars("day08");
    Griddy::new(grid)
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}

// #[cfg(test)]
// mod tests {
//     use tokio::sync::mpsc::unbounded_channel;

//     use crate::Ev;

//     use super::*;

//     #[test]
//     fn it_works() {
//         let (tx, _rx) = unbounded_channel::<Ev>();
//         let mut itx = (0, tx);
//         println!("{}", parta(&mut itx));
//     }
// }
