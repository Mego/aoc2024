use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use cached::proc_macro::cached;
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

fn parse(input: String) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines();
    let avail = lines
        .next()
        .unwrap()
        .split(", ")
        .map(String::from)
        .collect_vec();
    lines.next().unwrap();
    let goals = lines.map(String::from).collect_vec();
    (avail, goals)
}

pub fn part1(input: String) -> u64 {
    let (avail, goals) = parse(input);
    goals
        .into_par_iter()
        .filter(|g| can_make_string(g.clone(), avail.clone()) > 0)
        .count() as u64
}

#[cached]
fn can_make_string(goal: String, avail: Vec<String>) -> u64 {
    avail
        .iter()
        .filter_map(|s| {
            if goal == *s {
                return Some(1);
            }
            goal.starts_with(s)
                .then(|| can_make_string(goal[s.len()..].to_owned(), avail.clone()))
        })
        .sum()
}

pub fn part2(input: String) -> u64 {
    let (avail, goals) = parse(input);
    goals
        .into_par_iter()
        .map(|g| can_make_string(g, avail.clone()))
        .sum::<u64>()
}
