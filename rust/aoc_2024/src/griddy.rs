use core::fmt;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::util::XY;

#[derive(Debug, Clone)]
pub struct Griddy<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: Eq> Griddy<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        let mut data = Vec::with_capacity(width * height);
        for row in grid {
            data.extend(row);
        }
        Griddy {
            width,
            height,
            data,
        }
    }

    pub fn check(&self, xy: &XY) -> bool {
        xy.x < self.width && xy.y < self.height
    }

    pub fn find(&self, value: &T) -> Option<XY> {
        self.data.iter().position(|x| x == value).map(|i| XY {
            x: i % self.width,
            y: i / self.width,
        })
    }

    pub fn to_xy(&self, i: usize) -> XY {
        XY {
            x: i % self.width,
            y: i / self.width,
        }
    }
}

impl<T> Index<&XY> for Griddy<T> {
    type Output = T;
    fn index(&self, xy: &XY) -> &Self::Output {
        &self.data[xy.y * self.height + xy.x]
    }
}

impl<T> IndexMut<&XY> for Griddy<T> {
    fn index_mut(&mut self, xy: &XY) -> &mut Self::Output {
        &mut self.data[xy.y * self.height + xy.x]
    }
}

impl<T: Display> fmt::Display for Griddy<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[&XY::new(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
