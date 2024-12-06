use crate::{
    block,
    util::{bigga, parse_chars, parse_lines, Grid, GridFn, D, XY},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Paragraph, Widget},
};

const INPUT_DAY: &str = "day06";

fn render_grid(grid: &Vec<Vec<char>>, viz: &Rect, buf: &mut Buffer) {
    let debug: Vec<Line> = grid
        .iter()
        .map(|row| {
            Line::styled(
                row.iter().collect::<String>(),
                Style::default().fg(Color::Blue),
            )
        })
        .collect();
    Paragraph::new(debug)
        .block(block("Input"))
        .render(*viz, buf);
}

pub fn run(viz: Rect, buf: &mut Buffer) -> String {
    let mut grid = parse_chars(INPUT_DAY);
    grid = bigga(&grid, 1, 'Z');
    let y = grid.iter().position(|r| r.contains(&'^')).unwrap();
    let x = grid[y].iter().position(|c| c == &'^').unwrap();
    step(&mut grid, &XY { x, y }, D::Up, &viz, buf);
    grid.into_iter()
        .map(|r| r.into_iter().filter(|c| c == &'X').count())
        .sum::<usize>()
        .to_string()
}

pub fn step(grid: &mut Grid<char>, pos: &XY, facing: D, viz: &Rect, buf: &mut Buffer) {
    grid[pos] = 'X';
    render_grid(&grid, viz, buf);

    let forward = pos.dir(&facing);
    if grid[&forward] == 'Z' {
        return;
    } else if grid[&forward] == '#' {
        return step(grid, pos, facing.cw(), viz, buf);
    } else {
        step(grid, &forward, facing, viz, buf);
    }
}
