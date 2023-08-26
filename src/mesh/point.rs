use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct PointIter {
    point: Coord,
    index: u8,
}
impl From<[f64; 3]> for Coord {
    fn from(value: [f64; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<Coord> for [f64; 3] {
    fn from(value: Coord) -> Self {
        [value.x, value.y, value.z]
    }
}

impl Iterator for PointIter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                Some(self.point.x)
            }
            1 => {
                self.index += 1;
                Some(self.point.y)
            }
            2 => {
                self.index += 1;
                Some(self.point.z)
            }
            _ => None,
        }
    }
}
impl IntoIterator for Coord {
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y, self.z].into_iter()
    }
}

impl<'a> Coord {
    pub fn iter(&'a self) -> std::array::IntoIter<&'a f64, 3> {
        self.into_iter()
    }

    pub fn dot(self, rhs: Coord) -> f64 {
        let prod = self * rhs;
        prod.into_iter().sum()
    }

    pub fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }

    /// normalize inplace and return the norm
    pub fn normalize(&mut self) -> f64 {
        let norm = self.norm();
        self / norm;
        norm
    }
}

impl<'a> IntoIterator for &'a Coord {
    type Item = &'a f64;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.x, &self.y, &self.z].into_iter()
    }
}

impl Add<f64> for Coord {
    type Output = Coord;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}
impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Coord {
    type Output = Coord;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<Coord> for Coord {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f64> for &mut Coord {
    type Output = ();

    fn div(self, rhs: f64) -> Self::Output {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
impl Div<f64> for Coord {
    type Output = Coord;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Index<usize> for Coord {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index not in range 0..3"),
        }
    }
}
