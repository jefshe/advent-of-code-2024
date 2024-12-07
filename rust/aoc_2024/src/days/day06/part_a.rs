use crate::util::{bigga, parse_chars, Grid, D, XY};

const INPUT_DAY: &str = "day06";
pub fn run() -> String {
    let mut grid = parse_chars(INPUT_DAY);
    grid = bigga(grid, 1, 'Z');
    let y = grid.iter().position(|r| r.contains(&'^')).unwrap();
    let x = grid[y].iter().position(|c| c == &'^').unwrap();
    step(&mut grid, &XY { x, y }, D::Up);
    grid.into_iter()
        .map(|r| r.into_iter().filter(|c| c == &'X').count())
        .sum::<usize>()
        .to_string()
}

pub fn step(grid: &mut Grid<char>, pos: &XY, facing: D) {
    grid[pos] = 'X';
    let forward = pos.dir(&facing);
    if grid[&forward] == 'Z' {
    } else if grid[&forward] == '#' {
        return step(grid, pos, facing.cw());
    } else {
        step(grid, &forward, facing);
    }
}
