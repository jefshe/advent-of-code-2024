use std::collections::HashSet;

use crate::{
    griddy::Griddy,
    util::{bigga, parse_chars, D, XY},
    AOCUpdate, TX,
};

const INPUT_DAY: &str = "day06";

fn render_grid(grid: &Vec<Vec<char>>, tx: &TX) {
    let debug: Vec<String> = grid
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect();
    let (idx, s) = tx;
    s.send(AOCUpdate::Render(*idx, debug))
        .expect("Could not send update");
}

pub fn run() -> String {
    let mut grid = parse_chars(INPUT_DAY);
    grid = bigga(&grid, 1, 'Z');
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
        .filter(|g| is_loop(g, HashSet::new(), &pos, &D::Up))
        .count();
    println!("{:?}", ans);
    ans.to_string()
}

pub fn is_loop(grid: &Griddy<char>, mut visited: HashSet<(XY, D)>, pos: &XY, facing: &D) -> bool {
    if visited.contains(&(pos.clone(), facing.clone())) {
        // println!("{grid}");
        return true;
    }
    visited.insert((pos.clone(), facing.clone()));
    let forward = pos.dir(&facing);
    match grid[&forward] {
        'Z' => false,
        '#' | 'O' => is_loop(grid, visited, pos, &facing.cw()),
        _ => is_loop(grid, visited, &forward, facing),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        run();
    }
}
