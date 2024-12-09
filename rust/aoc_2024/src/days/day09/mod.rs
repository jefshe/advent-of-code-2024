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

pub fn parta(tx: &mut ItemTX) -> String {
    let mut next_file_id = 0;
    let compressed = input();
    let mut disk = vec![-1; 100000];
    let mut size: usize = 0;
    for (i, s) in compressed.iter().enumerate() {
        let new_size = size + (*s as usize);
        if i % 2 == 0 {
            disk[size..new_size].fill(next_file_id);
            next_file_id += 1;
        }
        size = new_size;
    }

    let final_size = size;

    // 2ptrs baby
    let (mut i, mut j) = (0, final_size - 1);
    while (i != j) {
        if disk[i] != -1 {
            i += 1;
            continue;
        }
        if disk[j] == -1 {
            j -= 1;
            continue;
        }
        disk.swap(i, j);
        i += 1;
        j -= 1;
    }
    //tx.update(vec![format!("{:?}", &disk[0..final_size])])
    //     .unwrap();

    format!(
        "{:?}",
        disk.iter()
            .filter(|&&x| x >= 0)
            .enumerate()
            .map(|(i, &x)| i * (x as usize))
            .sum::<usize>()
    )
}

pub fn partb(tx: &mut ItemTX) -> String {
    0.to_string()
}

fn input() -> Vec<u32> {
    parse_lines_iter("day09")
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
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
