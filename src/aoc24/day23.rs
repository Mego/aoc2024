use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use petgraph::{
    algo::{connected_components, tarjan_scc},
    dot::Dot,
    prelude::*,
};

fn parse(input: String) -> UnGraph<String, ()> {
    let cnxs = input.lines().map(|l| l.split("-"));
    let mut g = UnGraph::default();
    let mut nodes = HashMap::new();
    for mut cnx in cnxs {
        let a = cnx.next().unwrap().to_owned();
        let b = cnx.next().unwrap().to_owned();
        let an = *nodes.entry(a.clone()).or_insert_with(|| g.add_node(a));
        let bn = *nodes.entry(b.clone()).or_insert_with(|| g.add_node(b));
        g.add_edge(an, bn, ());
    }
    g
}

pub fn part1(input: String) -> u64 {
    let g = parse(input);
    // println!("{:?}", Dot::with_config(&g, &[]));
    g.node_indices()
        .tuple_combinations()
        .filter(|&(a, b, c)| {
            g.contains_edge(a, b)
                && g.contains_edge(c, b)
                && g.contains_edge(a, c)
                && (g[a].starts_with("t") || g[b].starts_with("t") || g[c].starts_with("t"))
        })
        .count() as u64
}

pub fn part2(input: String) -> String {
    let g = parse(input);
    g.node_indices()
        .par_bridge()
        .filter_map(|i| {
            let neighbors = g.neighbors(i).chain([i]).collect_vec();
            (0..neighbors.len()).find_map(|k| {
                neighbors
                    .iter()
                    .copied()
                    .combinations(neighbors.len() - k)
                    .find(|n| {
                        n.iter()
                            .copied()
                            .tuple_combinations()
                            .all(|(a, b)| g.contains_edge(a, b))
                    })
            })
        })
        .max_by_key(|n| n.len())
        .unwrap()
        .into_iter()
        .map(|i| &g[i])
        .sorted()
        .join(",")
}
