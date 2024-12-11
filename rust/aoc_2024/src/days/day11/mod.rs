use super::{time_run, Answer, TX};
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
    let mut stones = input();
    blink(&mut stones, 25);
    format!("{:?}", stones.len())
}

pub fn partb(_tx: &mut ItemTX) -> String {
    let mut stones = input();
    blink(&mut stones, 75);
    format!("{:?}", stones.len())
}

pub fn blink(stones: &mut Vec<usize>, blinks: usize) {
    if blinks == 0 {
        return;
    }
    let mut blinks_left = blinks;

    while blinks_left > 0 {
        let mut i = 0;
        while i < stones.len() {
            match stones[i] {
                0 => stones[i] = 1,
                num if digit_count(num) % 2 == 0 => {
                    let split_at = digit_count(num) / 2;
                    let left = num / 10_usize.pow(split_at as u32);
                    let right = num % 10_usize.pow(split_at as u32);
                    stones.insert(i, left);
                    stones[i + 1] = right;
                    i += 1
                }
                _ => stones[i] *= 2024,
            }
            i += 1
        }
        blinks_left -= 1
    }
}

pub fn input() -> Vec<usize> {
    let mut vec = parse_lines_iter("day11")
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.parse().expect("Invalid number"))
        .collect::<Vec<usize>>();
    vec.reserve(1_000_000);
    vec
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
