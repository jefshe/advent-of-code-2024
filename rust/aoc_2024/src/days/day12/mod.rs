use std::collections::HashSet;

use super::*;
use crate::util::D::*;
use crate::{griddy::Griddy, point::Pt, util::*};
use color_eyre::Result;
const FILE: &str = "day12";

async fn run(mut tx: ItemTX) -> Result<()> {
    let mut griddy = input();
    let parta = time_run(|| parta(&mut griddy));
    let partb = time_run(|| partb(&mut griddy));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta(griddy: &mut Griddy<char>) -> String {
    let mut visited = HashSet::new();
    format!(
        "{}",
        griddy
            .pts()
            .into_iter()
            .map(|pt| grow_a(griddy, pt, &mut visited))
            .map(|(area, perimeter)| area * perimeter)
            .sum::<usize>()
    )
}
pub fn partb(griddy: &mut Griddy<char>) -> String {
    let mut visited = HashSet::new();
    let pts = griddy.pts().into_iter().collect::<Vec<Pt>>();
    let ans = pts
        .into_iter()
        .map(|pt| grow_b(griddy, pt, &mut visited))
        .map(|(area, perimeter)| area * perimeter)
        .sum::<usize>();
    format!("{}", ans)
}

pub fn grow_b(griddy: &mut Griddy<char>, pt: Pt, visited: &mut HashSet<Pt>) -> (usize, usize) {
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
            }

            let neighbour_d = match d {
                Up | Down => [Left, Right],
                Left | Right => [Up, Down],
                _ => panic!("unexpected"),
            };

            for d2 in neighbour_d {
                let neighbour = pt + d2;
                if !griddy.check(&neighbour) || griddy[&neighbour] != griddy[&pt] {
                    griddy[&pt] = '!';
                    perimeter += 1
                }
            }
        }
    }
    (area, perimeter)
}

pub fn grow_a(griddy: &Griddy<char>, pt: Pt, visited: &mut HashSet<Pt>) -> (usize, usize) {
    let mut stack = vec![pt];
    let mut area = 0;
    let mut perimeter = 0;
    while let Some(pt) = stack.pop() {
        if visited.contains(&pt) {
            continue;
        }
        visited.insert(pt);
        area += 1;
        for d in [Up, Left, Down, Right] {
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
pub fn input() -> Griddy<char> {
    Griddy::new(parse_chars(FILE))
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let mut griddy = input();
    //     let ans = partb(&mut griddy);
    //     // println!("{}", griddy);
    // }
}
