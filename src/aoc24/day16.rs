use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Display,
};

use exhaust::Exhaust;
use itertools::Itertools;
use pathfinding::{
    grid::Grid,
    matrix::Matrix,
    prelude::{astar_bag, dijkstra_partial},
};
use petgraph::{
    algo::{all_simple_paths, dijkstra},
    data::DataMap,
    dot::Dot,
    prelude::*,
};

use crate::util::{adj_squares, Coordinate, Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node(Coordinate, Direction);

impl<T> From<(T, Direction)> for Node
where
    T: Into<Coordinate>,
{
    fn from(value: (T, Direction)) -> Self {
        Self(value.0.into(), value.1)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

fn parse(input: String) -> (Graph<Node, usize>, NodeIndex, Vec<NodeIndex>) {
    let grid = grid::Grid::from_vec(
        input.lines().flat_map(|l| l.trim().bytes()).collect_vec(),
        input.lines().next().unwrap().len(),
    );
    let mut g = Graph::new();
    let start: Coordinate = grid
        .indexed_iter()
        .find(|&(_, &c)| c == b'S')
        .unwrap()
        .0
        .into();
    let end: Coordinate = grid
        .indexed_iter()
        .find(|&(_, &c)| c == b'E')
        .unwrap()
        .0
        .into();

    let mut node_indexes: HashMap<Node, NodeIndex> = HashMap::new();
    Direction::exhaust().for_each(|dir| {
        let idx1 = g.add_node((start, dir).into());
        let idx2 = g.add_node((end, dir).into());
        node_indexes.insert((start, dir).into(), idx1);
        node_indexes.insert((end, dir).into(), idx2);
    });
    grid.indexed_iter().for_each(|(pos, &c)| {
        if c != b'#' {
            for dir in Direction::exhaust() {
                let coord = (pos, dir).into();
                let idx = *node_indexes
                    .entry(coord)
                    .or_insert_with(|| g.add_node(coord));
                let next_pos: Coordinate = dir.move_dir(pos.into()).into();
                if grid[next_pos] != b'#' {
                    let coord2 = (next_pos, dir).into();
                    let idx2 = *node_indexes
                        .entry(coord2)
                        .or_insert_with(|| g.add_node(coord2));
                    g.add_edge(idx, idx2, 1);
                }
            }
        }
    });
    node_indexes.iter().for_each(|(&node, &idx)| {
        let idx2 = node_indexes[&(node.0, node.1.cw()).into()];
        g.add_edge(idx, idx2, 1000);
        let idx3 = node_indexes[&(node.0, node.1.cw().opposite()).into()];
        g.add_edge(idx, idx3, 1000);
    });

    let start_node = node_indexes[&(start, Direction::RIGHT).into()];
    let end_nodes = node_indexes
        .iter()
        .filter_map(|(&k, &v)| (k.0 == end).then_some(v))
        .collect_vec();
    (g, start_node, end_nodes)
}

pub fn part1(input: String) -> u64 {
    let (g, start, ends) = parse(input);
    // println!("{} {}", g.node_count(), g.edge_count());
    let sol = dijkstra(&g, start, None, |edge| *edge.weight());
    // dbg!(start, &ends, &sol);
    // println!("{}", Dot::with_config(&g, &[]));
    ends.into_iter()
        .filter_map(|end| sol.get(&end).copied())
        .min()
        .unwrap() as u64
    // 0
}

pub fn part2(input: String) -> u64 {
    let (g, start, ends) = parse(input);
    let res: HashSet<_> = astar_bag(
        &start,
        |&idx| {
            let edges = g.edges(idx);
            edges.map(|edge| (edge.target(), *edge.weight()))
        },
        |&idx| {
            let node = g[idx];
            ends.iter()
                .map(|&end_idx| {
                    let end = g[end_idx];
                    node.0.x.abs_diff(end.0.x) + node.0.y.abs_diff(end.0.y)
                })
                .min()
                .unwrap()
        },
        |idx| ends.contains(idx),
    )
    .unwrap()
    .0
    .flat_map(|p| p.into_iter().map(|idx| g[idx].0))
    .collect();
    res.len() as u64
}
