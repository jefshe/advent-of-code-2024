use core::time;
use std::{collections::HashSet, thread};

use super::*;
use crate::{point::Pt, util::*};
use color_eyre::Result;
use itertools::Itertools;
const FILE: &str = "day14";
const WIDTH: usize = 101;
const HEIGHT: usize = 103;

type PV = (Pt, Pt);

async fn run(mut tx: ItemTX) -> Result<()> {
    let inputs = input();
    let parta = time_run(|| parta(&mut tx, &inputs));
    let partb = time_run(|| partb(&mut tx, &inputs));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta(tx: &mut ItemTX, pvs: &Vec<PV>) -> String {
    let positions = pvs
        .iter()
        .map(|(p, v)| *p + *v * 100)
        .map(|pt| pt.fit_in_grid(WIDTH, HEIGHT))
        .collect::<Vec<_>>();
    let counts = positions
        .iter()
        .filter(|pt| pt.x != (WIDTH) as i32 / 2 && pt.y != (HEIGHT) as i32 / 2)
        .into_group_map_by(|pt| (pt.x < (WIDTH as i32) / 2, pt.y < (HEIGHT as i32) / 2));
    format!(
        "{:?}",
        counts
            .values()
            .map(|v| v.len())
            .reduce(|a, b| a * b)
            .unwrap()
    )
}
pub fn partb(tx: &mut ItemTX, pvs: &Vec<PV>) -> String {
    let (mut ps, vs): (Vec<_>, Vec<_>) = pvs.clone().into_iter().unzip();
    let middle_y = HEIGHT as i32 / 2;
    let middle_x = WIDTH as i32 / 2;
    let mut steps = 0;
    loop {
        step(&mut ps, &vs);
        if (steps % 100_000) == 0 {
            tx.append(format!("{steps}")).unwrap();
        }
        //    let regions = ps
        // .iter()
        // .unique()
        // .filter(|pt| pt.x != middle_x / 2 && pt.y != middle_y as i32 / 2)
        // .into_group_map_by(|pt| (pt.x < middle_x, pt.y < middle_y / 2));
        if ps.iter().unique().filter(|pt| pt.x == middle_x).count() > 70
        // && regions[&(true, true)].len() == regions[&(false, true)].len()
        // && regions[&(true, false)].len() == regions[&(false, false)].len()
        // && regions[&(true, true)].len() >= regions[&(true, false)].len()
        // && regions[&(false, true)].len() >= regions[&(false, false)].len()
        {
            debug(tx, &ps);
            break;
        }
        steps += 1;
    }
    format!("{}", steps)
}

pub fn input() -> Vec<PV> {
    split_lines_iter(FILE, r"((\s*(p|v)=)|,)")
        .iter()
        .map(|l| {
            let (p, v) = l
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .tuples()
                .collect_tuple()
                .expect("Invalid input");
            (Pt::new(&p), Pt::new(&v))
        })
        .collect()
}

fn step(ps: &mut Vec<Pt>, vs: &Vec<Pt>) {
    for i in 0..ps.len() {
        ps[i] = (ps[i] + vs[i]).fit_in_grid(WIDTH, HEIGHT);
    }
}

fn debug(tx: &mut ItemTX, positions: &Vec<Pt>) {
    tx.update(
        (0..HEIGHT)
            .map(|i| {
                (0..WIDTH)
                    .map(|j| {
                        if positions.contains(&Pt {
                            x: j as i32,
                            y: i as i32,
                        }) {
                            "#"
                        } else {
                            "."
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();
}
pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let inp = input();
    //     parta(&inp);
    // }
}
