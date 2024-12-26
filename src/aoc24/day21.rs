use std::sync::{Arc, LazyLock};

use cached::proc_macro::cached;
use itertools::Itertools;
use pathfinding::{grid::Grid, matrix::Matrix, prelude::astar_bag};
use rayon::prelude::*;
use regex::Regex;

use crate::util::Direction;

static NUMPAD_MATRIX: LazyLock<Matrix<Option<u8>>> = LazyLock::new(|| {
    Matrix::from_vec(
        4,
        3,
        vec![
            Some(b'7'),
            Some(b'8'),
            Some(b'9'),
            Some(b'4'),
            Some(b'5'),
            Some(b'6'),
            Some(b'1'),
            Some(b'2'),
            Some(b'3'),
            None,
            Some(b'0'),
            Some(b'A'),
        ],
    )
    .unwrap()
    .transposed()
});

static NUMPAD: LazyLock<Grid> = LazyLock::new(|| {
    Grid::from_coordinates(
        &NUMPAD_MATRIX
            .items()
            .filter_map(|(k, v)| v.map(|_| k))
            .collect_vec(),
    )
    .unwrap()
});

const NUMPAD_COORDS: [(usize, usize); 11] = [
    (1, 3),
    (0, 2),
    (1, 2),
    (2, 2),
    (0, 1),
    (1, 1),
    (2, 1),
    (0, 0),
    (1, 0),
    (2, 0),
    (2, 3),
];

const NUMPAD_START: (usize, usize) = NUMPAD_COORDS[10];

static DPAD: LazyLock<Grid> = LazyLock::new(|| {
    let mut g = Grid::new(3, 2);
    g.add_vertex((0, 0));
    g.invert();
    g
});

const DPAD_UP: (usize, usize) = (1, 0);
const DPAD_LEFT: (usize, usize) = (0, 1);
const DPAD_RIGHT: (usize, usize) = (2, 1);
const DPAD_DOWN: (usize, usize) = (1, 1);
const DPAD_A: (usize, usize) = (2, 0);

#[cached]
fn find_shortest_numpad_paths(from: (usize, usize), to: (usize, usize)) -> Arc<Vec<Vec<u8>>> {
    astar_bag(
        &from,
        |&p| NUMPAD.neighbours(p).into_iter().map(|n| (n, 1)),
        |&p| NUMPAD.distance(p, to),
        |&p| p == to,
    )
    .unwrap()
    .0
    .map(|p| {
        p.windows(2)
            .map(|w| {
                let a = w[0];
                let b = w[1];
                Direction::from_coords(a.into(), b.into()).to_char()
            })
            .chain([b'A'])
            .collect_vec()
    })
    .collect_vec()
    .into()
}

#[cached]
fn find_shortest_dpad_paths(from: (usize, usize), to: (usize, usize)) -> Arc<Vec<Vec<u8>>> {
    astar_bag(
        &from,
        |&p| DPAD.neighbours(p).into_iter().map(|n| (n, 1)),
        |&p| DPAD.distance(p, to),
        |&p| p == to,
    )
    .unwrap()
    .0
    .map(|p| {
        p.windows(2)
            .map(|w| {
                let a = w[0];
                let b = w[1];
                Direction::from_coords(a.into(), b.into()).to_char()
            })
            .chain([b'A'])
            .collect_vec()
    })
    .collect_vec()
    .into()
}

fn u8_to_numpad_coord(c: u8) -> (usize, usize) {
    match c {
        b'A' => NUMPAD_START,
        b'0'..=b'9' => NUMPAD_COORDS[(c - b'0') as usize],
        _ => unimplemented!("{}", String::from_utf8_lossy(&[c])),
    }
}

fn u8_to_dpad_coord(c: u8) -> (usize, usize) {
    match c {
        b'^' => DPAD_UP,
        b'>' => DPAD_RIGHT,
        b'<' => DPAD_LEFT,
        b'v' => DPAD_DOWN,
        b'A' => DPAD_A,
        _ => unimplemented!("{}", String::from_utf8_lossy(&[c])),
    }
}

#[cached]
fn find_shortest_seq(s: Vec<u8>, depth: usize, top: bool) -> usize {
    s.into_iter()
        .fold((b'A', 0), |(cur, mut acc), c| {
            let paths = if top {
                let from = u8_to_numpad_coord(cur);
                let to = u8_to_numpad_coord(c);
                find_shortest_numpad_paths(from, to)
            } else {
                let from = u8_to_dpad_coord(cur);
                let to = u8_to_dpad_coord(c);
                find_shortest_dpad_paths(from, to)
            };
            if depth == 0 {
                acc += paths[0].len();
            } else {
                acc += paths
                    .par_iter()
                    .map(|p| find_shortest_seq(p.clone(), depth - 1, false))
                    .min()
                    .unwrap();
            }
            (c, acc)
        })
        .1
}

pub fn part1(input: String) -> u64 {
    let num_regex = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .map(|line| {
            let numeric_part = num_regex
                .find(line)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            let path_len = find_shortest_seq(line.bytes().collect_vec(), 2, true);
            dbg!(numeric_part, path_len);
            numeric_part * path_len
        })
        .sum::<usize>() as u64
}

pub fn part2(input: String) -> u64 {
    0
}
