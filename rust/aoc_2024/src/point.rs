use std::{
    fmt,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use crate::util::D;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Pt {
    pub x: i32,
    pub y: i32,
}

impl Add<Pt> for Pt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<Pt> for Pt {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub<Pt> for Pt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign<Pt> for Pt {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<i32> for Pt {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl From<(i32, i32)> for Pt {
    fn from(pair: (i32, i32)) -> Self {
        Self {
            x: pair.0,
            y: pair.1,
        }
    }
}

impl Add<D> for Pt {
    type Output = Self;
    fn add(self, other: D) -> Self {
        match other {
            D::Up => self + Pt::from((0, -1)),
            D::Down => self + Pt::from((0, 1)),
            D::Left => self + Pt::from((-1, 0)),
            D::Right => self + Pt::from((1, 0)),
            D::UpLeft => self + Pt::from((-1, -1)),
            D::UpRight => self + Pt::from((1, -1)),
            D::DownLeft => self + Pt::from((-1, 1)),
            D::DownRight => self + Pt::from((1, 1)),
        }
    }
}

impl Add<(i32, i32)> for Pt {
    type Output = Self;
    fn add(self, other: (i32, i32)) -> Self {
        self + Pt::from(other)
    }
}

impl fmt::Debug for Pt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
