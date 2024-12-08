use std::{
    fmt,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

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

impl fmt::Debug for Pt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
