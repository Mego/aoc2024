use std::collections::HashSet;

use grid::Grid;
use itertools::Itertools;

use crate::util::{fetch_input, submit};

const EXAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

fn antinode_locs(
    a: (usize, usize),
    b: (usize, usize),
    rows: usize,
    cols: usize,
    max_mul: usize,
) -> Vec<(usize, usize)> {
    let mut locs = Vec::new();

    [1, -1].into_iter().for_each(|d| {
        let mut mul = 1;
        while mul <= max_mul {
            let diff = (
                ((a.0 as isize) - (b.0 as isize)) * d * mul as isize,
                ((a.1 as isize) - (b.1 as isize)) * d * mul as isize,
            );
            let mut c = ((a.0 as isize) + diff.0, (a.1 as isize) + diff.1);
            if (c.0 as usize, c.1 as usize) == b {
                c = ((b.0 as isize) + diff.0, (b.1 as isize) + diff.1);
            }
            if c.0 >= 0 && (c.0 as usize) < rows && c.1 >= 0 && (c.1 as usize) < cols {
                locs.push((c.0 as usize, c.1 as usize));
                mul += 1;
            } else {
                break;
            }
        }
    });

    locs
}

pub fn part1(input: String) -> u64 {
    let g = {
        Grid::from_vec(
            input.lines().flat_map(|l| l.chars()).collect_vec(),
            input.find("\n").unwrap(),
        )
    };
    let antenna_locs = g
        .indexed_iter()
        .filter_map(|(i, c)| c.is_ascii_alphanumeric().then_some(i))
        .collect_vec();

    let rows = g.rows();
    let cols = g.cols();
    let antinode_locs = antenna_locs
        .iter()
        .cartesian_product(antenna_locs.iter())
        .filter_map(|(&a, &b)| {
            (a != b && *g.get(a.0, a.1).unwrap() == *g.get(b.0, b.1).unwrap())
                .then(|| antinode_locs(a, b, rows, cols, 1))
        })
        .flatten()
        .collect::<HashSet<_>>();
    let res = antinode_locs.len();
    res as u64
}

pub fn part2(input: String) -> u64 {
    // let input = Grid::from_vec(
    //     EXAMPLE_INPUT.lines().flat_map(|l| l.chars()).collect_vec(),
    //     EXAMPLE_INPUT.find("\n").unwrap(),
    // );
    let g = {
        Grid::from_vec(
            input.lines().flat_map(|l| l.chars()).collect_vec(),
            input.find("\n").unwrap(),
        )
    };
    let antenna_locs = g
        .indexed_iter()
        .filter_map(|(i, c)| c.is_ascii_alphanumeric().then_some(i))
        .collect_vec();

    let rows = g.rows();
    let cols = g.cols();
    let mut antinode_locs = antenna_locs
        .iter()
        .cartesian_product(antenna_locs.iter())
        .filter_map(|(&a, &b)| {
            (a != b && *g.get(a.0, a.1).unwrap() == *g.get(b.0, b.1).unwrap())
                .then(|| antinode_locs(a, b, rows, cols, 50))
        })
        .flatten()
        .collect::<HashSet<_>>();
    antenna_locs.iter().copied().for_each(|loc| {
        antinode_locs.insert(loc);
    });
    let res = antinode_locs.len();
    res as u64
}
