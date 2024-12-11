use super::{time_run, Answer, TX};
use crate::BoxedAsync;
use crate::{util::*, ItemTX};
use std::collections::HashMap;
use color_eyre::Result;


const MAX_BLINKS: usize = 75;
type Blink = usize;
type Cache = HashMap<(usize, Blink), usize>;

async fn run(mut tx: ItemTX) -> Result<()> {
    let parta = time_run(|| parta(&mut tx));
    let partb = time_run(|| partb(&mut tx));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta(_tx: &mut ItemTX) -> String {
    let stones = input();
    let mut cache: Cache = HashMap::new();
    cache.insert((0,1), 1);
    cache.insert((1,1), 1);
    format!("{:?}", stones.iter().map(|s| blink(*s, 25, &mut cache)).sum::<usize>())
}
pub fn partb(_tx: &mut ItemTX) -> String {
    let stones = input();
    let mut cache: Cache = HashMap::new();
    cache.insert((0,1), 1);
    cache.insert((1,1), 1);
    format!("{:?}", stones.iter().map(|s| blink(*s, MAX_BLINKS, &mut cache)).sum::<usize>())
}

pub fn blink(stone: usize, blinks: usize, cache: &mut Cache) -> usize{
    if blinks == 1 {
        return if digit_count(stone) % 2 == 0 { 2 } else { 1 }
    }

    if let Some(ans) = cache.get(&(stone, blinks)) {
        return *ans;
    }

    let ans = match stone {
        0 => blink(1, blinks - 1, cache),
        num if digit_count(num) % 2 == 0 => {
            let split_at = digit_count(num) / 2;
            let left = num / 10_usize.pow(split_at as u32);
            let right = num % 10_usize.pow(split_at as u32);
            blink(left, blinks - 1, cache) + blink(right, blinks - 1, cache)
        }
        _ => blink(stone * 2024, blinks - 1, cache)
    };
    cache.insert((stone, blinks), ans);
    ans
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
