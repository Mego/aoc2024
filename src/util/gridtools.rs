use std::{
    fmt::Display,
    ops::{Deref, Index, IndexMut},
};

use grid::Grid;
use itertools::Itertools;

use super::{
    coordinate::{add_delta, Coordinate, CoordinateOffset},
    direction::Direction,
};

pub struct GridRef<'a, T: Sized> {
    grid: &'a Grid<T>,
}

impl<T> Deref for GridRef<'_, T> {
    type Target = Grid<T>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<'a, T> From<&'a Grid<T>> for GridRef<'a, T> {
    fn from(value: &'a Grid<T>) -> Self {
        Self { grid: value }
    }
}

impl<T> Display for GridRef<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn print_grid(grid: &Grid<u8>) {
    println!(
        "{}",
        grid.iter_rows()
            .map(|row| row
                .map(|&i| format!("{}", String::from_utf8_lossy(&[i])))
                .join(""))
            .join("\n")
    );
}

pub trait Gridtools<T: Sized> {
    fn try_move_dir(&self, pos: Coordinate, dir: Direction) -> Option<Coordinate>;
}

impl<T> Gridtools<T> for Grid<T> {
    fn try_move_dir(&self, pos: Coordinate, dir: Direction) -> Option<Coordinate> {
        let new_pos = dir.move_dir(pos);
        self.is_valid_index(new_pos).then_some(Coordinate {
            x: new_pos.x as usize,
            y: new_pos.y as usize,
        })
    }
}

pub trait IsValidIndex<T: Sized> {
    fn is_valid_index(&self, index: T) -> bool;
}

impl<T: Sized> IsValidIndex<(isize, isize)> for Grid<T> {
    fn is_valid_index(&self, index: (isize, isize)) -> bool {
        index.0 >= 0 && index.1 >= 0 && self.is_valid_index((index.0 as usize, index.1 as usize))
    }
}

impl<T: Sized> IsValidIndex<(usize, usize)> for Grid<T> {
    fn is_valid_index(&self, index: (usize, usize)) -> bool {
        index.0 < self.rows() && index.1 < self.cols()
    }
}

impl<T: Sized> IsValidIndex<CoordinateOffset> for Grid<T> {
    fn is_valid_index(&self, index: CoordinateOffset) -> bool {
        index.x >= 0 && index.y >= 0 && self.is_valid_index((index.x as usize, index.y as usize))
    }
}

impl<T: Sized> IsValidIndex<Coordinate> for Grid<T> {
    fn is_valid_index(&self, index: Coordinate) -> bool {
        index.x < self.rows() && index.y < self.cols()
    }
}

impl<T: Sized> Index<Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self[(index.x, index.y)]
    }
}

impl<T: Sized> IndexMut<Coordinate> for Grid<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        &mut self[(index.x, index.y)]
    }
}

const DELTAS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const EIGHT_DELTAS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

pub fn adj_squares<T: Sized>(grid: &Grid<T>, cur: (usize, usize)) -> Vec<(usize, usize)> {
    let cur_i = (cur.0 as isize, cur.1 as isize);
    DELTAS
        .iter()
        .filter_map(|(dx, dy)| {
            let sq = (cur_i.0 + dx, cur_i.1 + dy);
            grid.is_valid_index(sq)
                .then_some((sq.0 as usize, sq.1 as usize))
        })
        .collect_vec()
}

pub struct Neighborhood {
    pub up: Option<u8>,
    pub up_right: Option<u8>,
    pub right: Option<u8>,
    pub down_right: Option<u8>,
    pub down: Option<u8>,
    pub down_left: Option<u8>,
    pub left: Option<u8>,
    pub up_left: Option<u8>,
}

pub fn adj_squares8(grid: &Grid<u8>, cur: (usize, usize)) -> Neighborhood {
    Neighborhood {
        up: {
            let pos = add_delta(cur, EIGHT_DELTAS[0]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        up_right: {
            let pos = add_delta(cur, EIGHT_DELTAS[1]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        right: {
            let pos = add_delta(cur, EIGHT_DELTAS[2]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        down_right: {
            let pos = add_delta(cur, EIGHT_DELTAS[3]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        down: {
            let pos = add_delta(cur, EIGHT_DELTAS[4]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        down_left: {
            let pos = add_delta(cur, EIGHT_DELTAS[5]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        left: {
            let pos = add_delta(cur, EIGHT_DELTAS[6]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        up_left: {
            let pos = add_delta(cur, EIGHT_DELTAS[7]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
    }
}
