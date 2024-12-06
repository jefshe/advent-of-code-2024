use crate::{
    util::{bigga, parse_chars, Grid, D, XY},
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

pub fn run(tx: &TX) -> String {
    let mut grid = parse_chars(INPUT_DAY);
    grid = bigga(&grid, 1, 'Z');
    let y = grid.iter().position(|r| r.contains(&'^')).unwrap();
    let x = grid[y].iter().position(|c| c == &'^').unwrap();
    step(&mut grid, &XY { x, y }, D::Up, tx);
    grid.into_iter()
        .map(|r| r.into_iter().filter(|c| c == &'X').count())
        .sum::<usize>()
        .to_string()
}

pub fn step(grid: &mut Grid<char>, pos: &XY, facing: D, tx: &TX) {
    grid[pos] = 'X';
    render_grid(&grid, tx);

    let forward = pos.dir(&facing);
    if grid[&forward] == 'Z' {
        return;
    } else if grid[&forward] == '#' {
        return step(grid, pos, facing.cw(), tx);
    } else {
        step(grid, &forward, facing, tx);
    }
}
