use std::{cmp::Ordering, fmt::Display};

use exhaust::Exhaust;

use super::coordinate::{Coordinate, CoordinateOffset};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Exhaust)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

impl Direction {
    pub fn to_delta(&self) -> CoordinateOffset {
        match self {
            Self::LEFT => (0, -1),
            Self::RIGHT => (0, 1),
            Self::UP => (-1, 0),
            Self::DOWN => (1, 0),
        }
        .into()
    }

    pub fn from_coords(a: Coordinate, b: Coordinate) -> Self {
        assert!(
            (a.x == b.x) ^ (a.y == b.y),
            "coordinates must be unique and lie on a cardinal line {},{}",
            a,
            b
        );
        match (a.x.cmp(&b.x), a.y.cmp(&b.y)) {
            (_, Ordering::Less) => Self::RIGHT,
            (_, Ordering::Greater) => Self::LEFT,
            (Ordering::Less, _) => Self::DOWN,
            (Ordering::Greater, _) => Self::UP,
            _ => unreachable!(),
        }
    }

    pub fn from_char(c: u8) -> Self {
        match c {
            b'<' => Self::LEFT,
            b'>' => Self::RIGHT,
            b'^' => Self::UP,
            b'v' => Self::DOWN,
            _ => unreachable!(),
        }
    }

    pub fn to_char(&self) -> u8 {
        match self {
            Self::LEFT => b'<',
            Self::RIGHT => b'>',
            Self::UP => b'^',
            Self::DOWN => b'v',
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::DOWN => Self::UP,
            Self::LEFT => Self::RIGHT,
            Self::UP => Self::DOWN,
            Self::RIGHT => Self::LEFT,
        }
    }

    pub fn cw(&self) -> Self {
        match self {
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
        }
    }

    pub fn cw_turns(&self, other: Self) -> usize {
        (0..4)
            .find_map(|n| {
                let mut dir = *self;
                for _ in 0..n {
                    dir = dir.cw();
                }
                (dir == other).then_some(n)
            })
            .unwrap()
    }

    pub fn move_dir(&self, pos: Coordinate) -> CoordinateOffset {
        let d = self.to_delta();
        (pos.x as isize + d.x, pos.y as isize + d.y).into()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DOWN => "v",
                Self::LEFT => "<",
                Self::RIGHT => ">",
                Self::UP => "^",
            }
        )
    }
}
