use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

use crate::util::{fetch_input, submit};

fn validate(update: &Vec<u64>, rules: &HashSet<(u64, u64)>) -> bool {
    let mut combs = update.iter().combinations(2);
    combs.all(|c| {
        let &a = c[0];
        let &b = c[1];
        !rules.contains(&(b, a))
    })
}

pub fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let mut rules = vec![];
    let mut updates = vec![];
    let mut parsing_rules = true;
    for line in input.lines() {
        if line.len() == 0 {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            rules.push(
                line.split("|")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            );
        } else {
            updates.push(
                line.split(",")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_vec(),
            );
        }
    }
    (rules, updates)
}

fn parse2(input: &str) -> (Vec<Vec<u64>>, Vec<Vec<u64>>, HashSet<(u64, u64)>) {
    let mut rules = HashSet::new();
    let mut valid_updates = vec![];
    let mut invalid_updates = vec![];
    let mut parsing_rules = true;
    for line in input.lines() {
        if line.trim().len() == 0 {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            rules.insert(
                line.trim()
                    .split("|")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            );
        } else {
            let update = line
                .trim()
                .split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec();
            if validate(&update, &rules) {
                valid_updates.push(update);
            } else {
                invalid_updates.push(update);
            }
        }
    }
    (valid_updates, invalid_updates, rules)
}

pub fn part1(input: String) -> u64 {
    let (valid, _, _) = parse2(&input.trim());
    let res: u64 = valid.iter().map(|update| update[update.len() / 2]).sum();
    res
}

pub fn part2(input: String) -> u64 {
    let (_, invalid, rules) = parse2(&input.trim());
    let res: u64 = invalid
        .into_iter()
        .map(|update| {
            let len = update.len();
            update
                .into_iter()
                .sorted_by(|&a, &b| {
                    if rules.contains(&(a, b)) {
                        return Ordering::Less;
                    } else if rules.contains(&(b, a)) {
                        return Ordering::Greater;
                    }
                    Ordering::Equal
                })
                .nth(len / 2)
                .unwrap()
        })
        .sum();
    res
}
