use core::fmt;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use ratatui::{text::Line, widgets::Paragraph};

use crate::{point::Pt, util::XY};

#[derive(Debug, Clone)]
pub struct Griddy<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: Eq + ToString> Griddy<T> {
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

    pub fn check_pt(&self, pt: &Pt) -> bool {
        pt.x >= 0 && pt.y >= 0 && pt.x < self.width as i32 && pt.y < self.height as i32
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

    pub fn to_pair(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    pub fn to_pt(&self, i: usize) -> Pt {
        Pt {
            x: (i % self.width) as i32,
            y: (i / self.width) as i32,
        }
    }

    pub fn strings(&self) -> Vec<String> {
        self.data
            .chunks(self.width)
            .map(|row| {
                row.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
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

impl<T> Index<&Pt> for Griddy<T> {
    type Output = T;
    fn index(&self, pt: &Pt) -> &Self::Output {
        &self.data[(pt.y as usize) * self.height + (pt.x as usize)]
    }
}

impl<T> IndexMut<&Pt> for Griddy<T> {
    fn index_mut(&mut self, pt: &Pt) -> &mut Self::Output {
        &mut self.data[(pt.y as usize) * self.height + (pt.x as usize)]
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

impl<T> Index<&(usize, usize)> for Griddy<T> {
    type Output = T;
    fn index(&self, pair: &(usize, usize)) -> &Self::Output {
        &self.data[pair.1 * self.height + pair.0]
    }
}

impl<T> IndexMut<&(usize, usize)> for Griddy<T> {
    fn index_mut(&mut self, pair: &(usize, usize)) -> &mut Self::Output {
        &mut self.data[pair.1 * self.height + pair.0]
    }
}
