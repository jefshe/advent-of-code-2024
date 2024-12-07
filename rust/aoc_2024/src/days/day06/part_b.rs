use crate::{
    days::TX,
    griddy::Griddy,
    util::{bigga, parse_chars, D, XY},
    ItemTX,
};
use rayon::prelude::*;
use std::{collections::HashSet, time::Instant};

const INPUT_DAY: &str = "day06";

pub fn run(tx: &mut ItemTX) -> String {
    let mut now = Instant::now();
    let mut grid = parse_chars(INPUT_DAY);
    grid = bigga(grid, 1, 'Z');
    let griddy = Griddy::new(grid);
    let pos = griddy.find(&'^').unwrap();
    let ans = griddy
        .data
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == '.')
        .map(|(i, _)| {
            let mut check = griddy.clone();
            let xy = &check.to_xy(i);
            check[xy] = 'O';
            check
        })
        .collect::<Vec<_>>();
    tx.append(format!("chunking {:?}", now.elapsed())).unwrap();
    now = Instant::now();
    let ret = ans
        .par_iter()
        .filter(|g| is_loop(g, HashSet::new(), &pos, &D::Up))
        .count()
        .to_string();
    tx.append(format!("filtering {:?}", now.elapsed())).unwrap();
    ret
}

pub fn is_loop(grid: &Griddy<char>, mut visited: HashSet<(XY, D)>, pos: &XY, facing: &D) -> bool {
    if visited.contains(&(pos.clone(), facing.clone())) {
        // println!("{grid}");
        return true;
    }
    visited.insert((pos.clone(), facing.clone()));
    let forward = pos.dir(facing);
    match grid[&forward] {
        'Z' => false,
        '#' | 'O' => is_loop(grid, visited, pos, &facing.cw()),
        _ => is_loop(grid, visited, &forward, facing),
    }
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
        run(&mut itx);
    }
}
