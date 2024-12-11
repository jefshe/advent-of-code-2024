use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Add, Index, IndexMut, Sub};
use std::path::Path;

pub fn parse_lines_iter(day: &str) -> impl Iterator<Item = String> {
    read_lines(format!("./input/{day}"))
        .expect("Could not read file")
        .map(|line| line.expect("Could not read line"))
}

pub fn split_lines_iter(day: &str, re: &str) -> Vec<Vec<String>> {
    read_lines(format!("./input/{day}"))
        .expect("Could not read file")
        .map(|line| {
            let separator = Regex::new(re).expect("Invalid regex");
            let unwrapped = &line.expect("Could not read line");
            let split = separator.split(unwrapped).map(|s| s.to_string());
            split.collect()
        })
        .collect()
}

pub fn parse_chars(day: &str) -> Vec<Vec<char>> {
    read_lines(format!("./input/{day}"))
        .expect("Could not read file")
        .map(|line| line.expect("Could not read line").chars().collect())
        .collect()
}

pub fn parse_2_parts(day: &str) -> (Vec<String>, Vec<String>) {
    let mut buf = read_lines(format!("./input/{day}"))
        .expect("Could not read file")
        .map(|line| line.expect("Could not read line"));
    (
        buf.by_ref().take_while(|line| !line.is_empty()).collect(),
        buf.collect(),
    )
}

pub fn bigga<T: Clone>(vec: Vec<Vec<T>>, by: usize, default: T) -> Vec<Vec<T>> {
    let mut new_vec = vec![vec![default; vec.len() + 2 * by]; vec.len() + 2 * by];
    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            new_vec[i + by][j + by] = vec[i][j].clone();
        }
    }
    new_vec
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn digit_count(input: usize) -> usize {
    (0..).take_while(|i| 10_usize.pow(*i) <= input).count()
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct XY {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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

impl D {
    pub fn neg(&self) -> Self {
        match self {
            D::Up => D::Down,
            D::Down => D::Up,
            D::Left => D::Right,
            D::Right => D::Left,
            D::UpLeft => D::UpLeft,
            D::UpRight => D::DownLeft,
            D::DownLeft => D::UpRight,
            D::DownRight => D::UpLeft,
        }
    }

    pub fn cw(&self) -> D {
        match self {
            D::Up => D::Right,
            D::Down => D::Left,
            D::Left => D::Up,
            D::Right => D::Down,
            D::UpLeft => D::UpRight,
            D::UpRight => D::DownRight,
            D::DownLeft => D::UpLeft,
            D::DownRight => D::DownLeft,
        }
    }

    pub fn ccw(&self) -> D {
        match self {
            D::Up => D::Left,
            D::Down => D::Right,
            D::Left => D::Down,
            D::Right => D::Up,
            D::UpLeft => D::DownLeft,
            D::UpRight => D::UpLeft,
            D::DownLeft => D::DownRight,
            D::DownRight => D::UpRight,
        }
    }
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

impl<T> Index<&XY> for Grid<T> {
    type Output = T;
    fn index(&self, xy: &XY) -> &Self::Output {
        &self[xy.y][xy.x]
    }
}

impl<T> IndexMut<&XY> for Grid<T> {
    fn index_mut(&mut self, index: &XY) -> &mut Self::Output {
        &mut self[index.y][index.x]
    }
}

impl Add<D> for XY {
    type Output = Self;
    fn add(self, other: D) -> Self {
        self.dir(&other)
    }
}

impl Sub<D> for XY {
    type Output = Self;
    fn sub(self, other: D) -> Self {
        self.dir(&other.neg())
    }
}

impl From<(usize, usize)> for XY {
    fn from(pair: (usize, usize)) -> Self {
        Self {
            x: pair.0,
            y: pair.1,
        }
    }
}

impl fmt::Debug for XY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub trait GridFn {
    fn check(&self, xy: &XY) -> bool;
}

impl<T> GridFn for Grid<T> {
    fn check(&self, xy: &XY) -> bool {
        xy.x < self[0].len() && xy.y < self.len()
    }
}
