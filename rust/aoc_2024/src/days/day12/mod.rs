use std::collections::HashSet;

use super::*;
use crate::util::D::*;
use crate::{griddy::Griddy, point::Pt, util::*};
use color_eyre::Result;
const FILE: &str = "day12";

async fn run(mut tx: ItemTX) -> Result<()> {
    let griddy = input();
    let parta = time_run(|| parta(&griddy));
    let partb = time_run(|| partb(&griddy));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta(griddy: &Griddy<char>) -> String {
    let mut visited = HashSet::new();
    // for pt in griddy.pts() {
    //     let ans = grow(griddy, pt, &mut visited);
    // }

    format!(
        "{}",
        griddy
            .pts()
            .into_iter()
            .map(|pt| grow(griddy, pt, &mut visited))
            .map(|(area, perimeter)| area * perimeter)
            .sum::<usize>()
    )
}

pub fn grow(griddy: &Griddy<char>, pt: Pt, visited: &mut HashSet<Pt>) -> (usize, usize) {
    let mut stack = vec![pt];
    let mut area = 0;
    let mut perimeter = 0;
    while let Some(pt) = stack.pop() {
        if visited.contains(&pt) {
            continue;
        }
        visited.insert(pt);
        area += 1;
        for d in [Up, Down, Left, Right] {
            let next = pt + d;
            if griddy.check(&next) && griddy[&next] == griddy[&pt] {
                stack.push(next);
            } else {
                perimeter += 1;
            }
        }
    }
    (area, perimeter)
}

pub fn partb(griddy: &Griddy<char>) -> String {
    "todo".to_string()
}

pub fn input() -> Griddy<char> {
    Griddy::new(parse_chars(FILE))
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let griddy = input();
        println!("{}", parta(&griddy));
    }
}
