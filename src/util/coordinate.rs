use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<&(usize, usize)> for Coordinate {
    fn from(value: &(usize, usize)) -> Self {
        Self::from(*value)
    }
}

impl From<Coordinate> for (usize, usize) {
    fn from(value: Coordinate) -> Self {
        (value.x, value.y)
    }
}

impl From<&Coordinate> for (usize, usize) {
    fn from(value: &Coordinate) -> Self {
        Self::from(*value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoordinateOffset {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for CoordinateOffset {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<CoordinateOffset> for Coordinate {
    fn from(value: CoordinateOffset) -> Self {
        Self {
            x: value.x as usize,
            y: value.y as usize,
        }
    }
}

pub fn add_delta(cur: (usize, usize), delta: (isize, isize)) -> (isize, isize) {
    (cur.0 as isize + delta.0, cur.1 as isize + delta.1)
}
