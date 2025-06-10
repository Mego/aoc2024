use std::collections::{BTreeMap, HashMap, HashSet};

use crate::util::{iter::IteratorExt, ParallelIteratorExt};
use itertools::Itertools;
use pathfinding::{
    grid::Grid,
    matrix::Matrix,
    prelude::{astar_bag, dijkstra},
};
use rayon::prelude::*;

fn parse(input: String) -> (Grid, (usize, usize), (usize, usize)) {
    let m = Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap();
    let start_pos = m
        .items()
        .find_map(|(p, &c)| (c == b'S').then_some(p))
        .unwrap();
    let end_pos = m
        .items()
        .find_map(|(p, &c)| (c == b'E').then_some(p))
        .unwrap();

    (
        Grid::from(m.map(|c| c != b'#')),
        (start_pos.1, start_pos.0),
        (end_pos.1, end_pos.0),
    )
}

fn grid_successors(
    grid: &Grid,
    p: (usize, usize),
) -> impl Iterator<Item = ((usize, usize), usize)> {
    grid.neighbours(p).into_iter().map(|n| (n, 1))
}

pub fn part1(input: String) -> u64 {
    let (grid, start, end) = parse(input);
    let (path, legit_time) =
        dijkstra(&start, |&p| grid_successors(&grid, p), |&p| p == end).unwrap();
    let mut neg_grid = grid.clone();
    neg_grid.invert();
    let path_adj: HashSet<_> = path
        .into_par_iter()
        .flat_map(|vertex| {
            let (x, y) = vertex;
            let mut candidates = Vec::with_capacity(4);
            if x > 0 {
                candidates.push((x - 1, y));
            }
            if x + 1 < neg_grid.width {
                candidates.push((x + 1, y));
            }
            if y > 0 {
                candidates.push((x, y - 1));
            }
            if y + 1 < neg_grid.height {
                candidates.push((x, y + 1));
            }
            candidates.retain(|&v| neg_grid.has_vertex(v));
            candidates
        })
        .duplicates()
        .collect();
    let timesaves = path_adj
        .into_par_iter()
        .filter(|&p| {
            let mut g = grid.clone();
            g.add_vertex(p);
            let new_time = dijkstra(&start, |&p| grid_successors(&g, p), |&p| p == end)
                .unwrap()
                .1;
            legit_time - new_time >= 100
        })
        .count();
    timesaves as u64
}

pub fn part2(input: String) -> u64 {
    let (grid, start, end) = parse(input);
    let path = dijkstra(&start, |&p| grid_successors(&grid, p), |&p| p == end)
        .unwrap()
        .0;

    const MIN_SAVE: usize = 100;
    let timesaves = (0..path.len() - MIN_SAVE)
        .into_par_iter()
        .flat_map(|i| {
            (i + MIN_SAVE..path.len())
                .into_par_iter()
                .map(move |j| (i, j))
        })
        .filter_map(|(i, j)| {
            let a = path[i];
            let b = path[j];
            let dist = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
            (dist <= 20 && j - i - dist >= MIN_SAVE).then_some(j - i - dist)
        })
        .count();

    timesaves as u64
}
