use std::collections::{HashMap, HashSet};

use grid::Grid;
use itertools::Itertools;

use crate::util::IsValidIndex;

fn find_trails(
    grid: &Grid<u8>,
    start_position: (usize, usize),
) -> Vec<((usize, usize), (usize, usize))> {
    let trails = (b'1'..=b'9').fold(
        vec![vec![start_position]],
        |current_positions, num_to_find| {
            current_positions
                .iter()
                .flat_map(|pos| {
                    let last_pos = pos.iter().last().unwrap();
                    [(0, -1), (0, 1), (1, 0), (-1, 0)]
                        .into_iter()
                        .filter_map(|d| {
                            let new_pos = (last_pos.0 as isize + d.0, last_pos.1 as isize + d.1);
                            (grid.is_valid_index(new_pos)
                                && grid
                                    .get(new_pos.0, new_pos.1)
                                    .is_some_and(|&c| c == num_to_find))
                            .then_some((new_pos.0 as usize, new_pos.1 as usize))
                        })
                        .map(|new_pos| {
                            pos.iter()
                                .copied()
                                .chain([new_pos].into_iter())
                                .collect_vec()
                        })
                })
                .collect_vec()
        },
    );
    trails
        .into_iter()
        .map(|positions| (positions[0], *positions.last().unwrap()))
        .collect()
}

pub fn part1(input: String) -> u64 {
    let grid = Grid::from_vec(
        input.lines().flat_map(|l| l.bytes()).collect_vec(),
        input.lines().next().unwrap().len(),
    );
    let start_positions = grid
        .indexed_iter()
        .filter_map(|((x, y), &c)| (c == b'0').then_some((x, y)))
        .collect_vec();
    let trails = start_positions
        .iter()
        .flat_map(|&start_position| find_trails(&grid, start_position))
        .unique()
        .collect_vec();
    let mut scores = HashMap::new();
    trails.into_iter().for_each(|(start, _)| {
        let score = scores.entry(start).or_default();
        *score += 1;
    });
    scores.values().sum()
}

pub fn part2(input: String) -> u64 {
    let grid = Grid::from_vec(
        input.lines().flat_map(|l| l.bytes()).collect_vec(),
        input.lines().next().unwrap().len(),
    );
    let start_positions = grid
        .indexed_iter()
        .filter_map(|((x, y), &c)| (c == b'0').then_some((x, y)))
        .collect_vec();
    let trails = start_positions
        .iter()
        .flat_map(|&start_position| find_trails(&grid, start_position))
        .collect_vec();
    let mut scores = HashMap::new();
    trails.into_iter().for_each(|(start, _)| {
        let score = scores.entry(start).or_default();
        *score += 1;
    });
    scores.values().sum()
}
