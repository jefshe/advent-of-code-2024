use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub fn parse_chars(day: &str) -> Vec<Vec<char>> {
    read_lines(format!("./input/{}.txt", day))
        .expect("Could not read file")
        .map(|line| line.expect("Could not read line").chars().collect())
        .collect()
}

pub fn bigga<T: Clone>(vec: &Vec<Vec<T>>, by: usize, default: T) -> Vec<Vec<T>> {
    let mut new_vec = vec![vec![default; vec.len() + 2 * by]; vec.len() + 2 * by];
    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            new_vec[i + by][j + by] = vec[i][j].clone();
        }
    }
    new_vec
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub struct XY {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum D {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl XY {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn dir(&self, d: &D) -> Self {
        match d {
            D::Up => self.up(),
            D::Down => self.down(),
            D::Left => self.left(),
            D::Right => self.right(),
            D::UpLeft => self.up_left(),
            D::UpRight => self.up_right(),
            D::DownLeft => self.down_left(),
            D::DownRight => self.down_right(),
        }
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn up_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    pub fn up_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    pub fn down_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    pub fn down_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

pub type Grid<T> = Vec<Vec<T>>;
pub fn grid_get<'a, T>(grid: &'a Grid<T>, xy: &XY, dir: &D) -> &'a T {
    let pos = xy.dir(dir);
    &grid[pos.y][pos.x]
}
