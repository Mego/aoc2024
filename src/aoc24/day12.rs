use rustc_hash::FxHashSet as HashSet;

use grid::Grid;
use itertools::Itertools;

use crate::util::{adj_squares, adj_squares8, diff};

fn find_regions(grid: &Grid<u8>, include_sides: bool) -> Vec<(usize, usize, usize)> {
    let mut regions = Vec::new();
    let mut checked = HashSet::default();

    grid.indexed_iter().for_each(|((x, y), &c)| {
        if checked.insert((x, y)) {
            let mut adj = adj_squares(grid, (x, y))
                .into_iter()
                .filter(|pos| grid[*pos] == c)
                .collect_vec();
            let mut per = 4 - adj.len();
            let mut area = 1;
            let mut sides = 0;
            let mut squares = HashSet::default();
            squares.insert((x, y));
            while adj.len() > 0 {
                adj = adj
                    .into_iter()
                    .flat_map(|pos| {
                        squares.insert(pos);
                        if checked.insert(pos) {
                            area += 1;
                            let all_adj = adj_squares(grid, pos);
                            let this_adj = all_adj
                                .iter()
                                .copied()
                                .filter(|pos| grid[*pos] == c)
                                .collect_vec();
                            per += 4 - this_adj.len();
                            return this_adj;
                        }
                        vec![]
                    })
                    .collect_vec();
            }
            if include_sides {
                for pos in squares {
                    let n = adj_squares8(grid, pos);
                    if diff(c, n.up) && diff(c, n.left) {
                        sides += 1;
                    }
                    if diff(c, n.down) && diff(c, n.left) {
                        sides += 1;
                    }
                    if diff(c, n.up) && diff(c, n.right) {
                        sides += 1;
                    }
                    if diff(c, n.down) && diff(c, n.right) {
                        sides += 1;
                    }
                    if !diff(c, n.up) && !diff(c, n.left) && diff(c, n.up_left) {
                        sides += 1;
                    }
                    if !diff(c, n.down) && !diff(c, n.left) && diff(c, n.down_left) {
                        sides += 1;
                    }
                    if !diff(c, n.up) && !diff(c, n.right) && diff(c, n.up_right) {
                        sides += 1;
                    }
                    if !diff(c, n.down) && !diff(c, n.right) && diff(c, n.down_right) {
                        sides += 1;
                    }
                }
            }
            regions.push((per, area, sides));
        }
    });

    regions
}

pub fn part1(input: String) -> u64 {
    let grid = Grid::from_vec(
        input.lines().flat_map(|l| l.bytes()).collect_vec(),
        input.lines().next().unwrap().len(),
    );
    let regions = find_regions(&grid, false);
    regions.into_iter().map(|(p, a, _)| p * a).sum::<usize>() as u64
}
pub fn part2(input: String) -> u64 {
    let grid = Grid::from_vec(
        input.lines().flat_map(|l| l.bytes()).collect_vec(),
        input.lines().next().unwrap().len(),
    );
    let regions = find_regions(&grid, true);
    regions.into_iter().map(|(_, a, s)| s * a).sum::<usize>() as u64
}
