use std::collections::HashMap;
use std::rc::Rc;

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
    // let mut stones = input();
    // let mut cache: HashMap<usize, Rc<Vec<usize>>> = HashMap::new();
    // cache.insert(0, Rc::new(vec![1]));
    // cache.insert(1, Rc::new(vec![2024]));
    // cache.insert(2024, Rc::new(vec![20, 24]));
    // cache.insert(20, Rc::new(vec![2, 0]));
    // cache.insert(24, Rc::new(vec![2, 4]));
    // format!("{:?}", blink(&mut stones, &mut cache, 25))
    "see part a".to_string()
}

pub fn partb(tx: &mut ItemTX) -> String {
    let mut stones = input();
    let mut cache: HashMap<usize, Rc<Vec<usize>>> = HashMap::new();
    cache.insert(0, Rc::new(vec![1]));
    cache.insert(1, Rc::new(vec![2024]));
    cache.insert(2024, Rc::new(vec![20, 24]));
    cache.insert(20, Rc::new(vec![2, 0]));
    cache.insert(24, Rc::new(vec![2, 4]));
    format!("{:?}", blink(&mut stones, &mut cache, 75, tx))
}

///
/// 0
/// 1
/// 2024
/// 20 24
/// 2 0 2 4

pub fn blink(
    stones: &Vec<usize>,
    cache: &mut HashMap<usize, Rc<Vec<usize>>>,
    blinks: usize,
    tx: &mut ItemTX,
) -> usize {
    if blinks == 0 {
        return stones.len();
    }

    let mut i = 0;
    let mut total_length = 0;
    tx.append(format!("{:?}  @ blink {blinks}", stones))
        .expect("Unable to append");
    while i < stones.len() {
        match (stones[i], cache.get(&stones[i])) {
            (_, Some(next_stones)) => {
                total_length += blink(&next_stones.clone(), cache, blinks - 1, tx);
            }
            (num, _) if digit_count(num) % 2 == 0 => {
                let split_at = digit_count(num) / 2;
                let left = num / 10_usize.pow(split_at as u32);
                let right = num % 10_usize.pow(split_at as u32);
                let next_stones = Rc::new(vec![left, right]);
                cache.insert(num, next_stones.clone());
                total_length += blink(&next_stones, cache, blinks - 1, tx);
            }
            (num, _) => total_length += blink(&vec![num * 2024], cache, blinks - 1, tx),
        }
        i += 1
    }
    total_length
}

pub fn input() -> Vec<usize> {
    let vec = parse_lines_iter("day11")
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.parse().expect("Invalid number"))
        .collect::<Vec<usize>>();
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
