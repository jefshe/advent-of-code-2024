use crate::{
    griddy::Griddy,
    point::Pt,
    util::{parse_chars, D},
};
use rayon::prelude::*;
use std::collections::HashSet;

const INPUT_DAY: &str = "day06";

pub fn run() -> String {
    let grid = parse_chars(INPUT_DAY);
    let griddy = Griddy::new(grid);
    let pos = griddy.find(&'^').unwrap();

    griddy
        .data
        .par_iter()
        .enumerate()
        .filter(|(_, &c)| c == '.')
        .map(|(i, _)| {
            let mut check = griddy.clone();
            check.data[i] = 'O';
            check
        })
        .filter(|g| is_loop(g, pos, D::Up))
        .count()
        .to_string()
}

pub fn is_loop(grid: &Griddy<char>, pos: Pt, facing: D) -> bool {
    let mut visited = HashSet::new();
    let mut d = facing;
    let mut curr = pos;
    loop {
        visited.insert((curr, d));
        let next = curr + d;
        if !grid.check(&next) {
            return false;
        }
        if visited.contains(&(next, d)) {
            return true;
        }
        match grid[&next] {
            '#' | 'O' => {
                d = d.cw();
            }
            _ => curr = next,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("{}", run());
    }
}
