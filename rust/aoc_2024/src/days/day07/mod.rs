use super::{time_run, Answer, TX};
use crate::BoxedAsync;
use crate::{util::*, ItemTX};
use color_eyre::Result;
use rayon::prelude::*;
use regex::Regex;
use std::cmp;

async fn run(mut tx: ItemTX) -> Result<()> {
    let parta = time_run(|| parta(&mut tx));
    let partb = time_run(|| partb(&mut tx));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta(_tx: &mut ItemTX) -> String {
    let maths = input();
    maths
        .par_iter()
        .filter(|(target, rest)| can_math_a(*target, 0, rest))
        .map(|(t, _)| t)
        .sum::<usize>()
        .to_string()
}

pub fn partb(_tx: &mut ItemTX) -> String {
    let maths = input();
    maths
        .par_iter()
        .filter(|(target, rest)| can_math_b(*target, 0, rest))
        .map(|(t, _)| t)
        .sum::<usize>()
        .to_string()
}

fn can_math_a(target: usize, total: usize, rest: &[usize]) -> bool {
    if total > target {
        return false;
    }
    match rest {
        [] => total == target,
        [x, xs @ ..] => {
            can_math_a(target, total + x, xs) || can_math_a(target, cmp::max(total, 1) * x, xs)
        }
    }
}

fn can_math_b(target: usize, total: usize, rest: &[usize]) -> bool {
    if total > target {
        return false;
    }
    match rest {
        [] => total == target,
        [x, xs @ ..] => {
            can_math_b(target, concat(total, *x), xs)
                || can_math_b(target, total + x, xs)
                || can_math_b(target, cmp::max(total, 1) * x, xs)
        }
    }
}

fn concat(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut remainder = b;
    while remainder > 0 {
        a *= 10;
        remainder /= 10;
    }
    a + b
}

fn input() -> Vec<(usize, Vec<usize>)> {
    parse_lines_iter("day07")
        .map(|line| {
            let splitter = Regex::new(r":?\s+").expect("Invalid regex");
            splitter
                .split(&line)
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|n| (n[0], n[1..].to_vec()))
        .collect::<Vec<(usize, Vec<usize>)>>()
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
