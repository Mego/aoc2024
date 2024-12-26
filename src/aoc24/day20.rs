use std::collections::{BTreeMap, HashMap};

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
    // println!(
    //     "{}",
    //     (0..grid.height)
    //         .map(|y| (0..grid.width)
    //             .map(|x| {
    //                 if (x, y) == start {
    //                     return "S";
    //                 }
    //                 if (x, y) == end {
    //                     return "E";
    //                 }
    //                 if grid.has_vertex((x, y)) {
    //                     return ".";
    //                 }
    //                 "#"
    //             })
    //             .join(""))
    //         .join("\n")
    // );
    let legit_time = dijkstra(&start, |&p| grid_successors(&grid, p), |&p| p == end)
        .unwrap()
        .1;
    // dbg!(legit_time);
    let timesaves = (0..grid.width)
        .cartesian_product(0..grid.height)
        .par_bridge()
        .filter(|&p| {
            if !grid.has_vertex(p) {
                let mut g = Grid::from_iter(grid.iter());
                g.add_vertex(p);
                let new_time = dijkstra(&start, |&p| grid_successors(&g, p), |&p| p == end)
                    .unwrap()
                    .1;
                return legit_time - new_time >= 100;
            }
            false
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
