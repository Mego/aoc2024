use std::{fmt::Display, sync::LazyLock};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Operator {
    ADD,
    MUL,
    CAT,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::ADD => a + b,
            Operator::MUL => a * b,
            Operator::CAT => format!("{a}{b}").parse().unwrap(),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::ADD => "+",
                Operator::MUL => "*",
                Operator::CAT => "||",
            }
        )
    }
}

const EXAMPLE_INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

static LINE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+): ((?:\d+(?:\s+)?)+)").unwrap());

pub fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let mut captures_iter = LINE_REGEX.captures_iter(l);
            let capture = captures_iter.next().unwrap();
            let goal = capture.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let nums = capture
                .get(2)
                .unwrap()
                .as_str()
                .split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec();
            (goal, nums)
        })
        .collect_vec()
}

pub fn part1(input: String) -> u64 {
    let v = parse(&input);
    let res: usize = v
        .into_iter()
        .map(|(goal, nums)| {
            let mut acc_vals = Vec::new();
            for i in 1..nums.len() {
                if i == 1 {
                    [Operator::ADD, Operator::MUL].into_iter().for_each(|op| {
                        let val = op.apply(nums[0], nums[1]);
                        if i == nums.len() - 1 {
                            if val == goal {
                                acc_vals.push(val);
                            }
                        } else if val <= goal {
                            acc_vals.push(val);
                        }
                    });
                } else {
                    acc_vals = acc_vals
                        .into_iter()
                        .flat_map(|acc| {
                            let mut res = Vec::new();
                            for op in [Operator::ADD, Operator::MUL] {
                                let val = op.apply(acc, nums[i]);
                                if i == nums.len() - 1 {
                                    if val == goal {
                                        res.push(val);
                                    }
                                } else if val <= goal {
                                    res.push(val);
                                }
                            }
                            res
                        })
                        .collect_vec()
                }
            }
            if acc_vals.len() > 0 {
                return goal as usize;
            }
            0
        })
        .sum();
    res as u64
}

pub fn part2(input: String) -> u64 {
    let v = parse(&input);
    let res: usize = v
        .into_iter()
        .map(|(goal, nums)| {
            let mut acc_vals = Vec::new();
            for i in 1..nums.len() {
                if i == 1 {
                    [Operator::ADD, Operator::MUL, Operator::CAT]
                        .into_iter()
                        .for_each(|op| {
                            let val = op.apply(nums[0], nums[1]);
                            if i == nums.len() - 1 {
                                if val == goal {
                                    acc_vals.push(val);
                                }
                            } else if val <= goal {
                                acc_vals.push(val);
                            }
                        });
                } else {
                    acc_vals = acc_vals
                        .into_iter()
                        .flat_map(|acc| {
                            let mut res = Vec::new();
                            for op in [Operator::ADD, Operator::MUL, Operator::CAT] {
                                let val = op.apply(acc, nums[i]);
                                if i == nums.len() - 1 {
                                    if val == goal {
                                        res.push(val);
                                    }
                                } else if val <= goal {
                                    res.push(val);
                                }
                            }
                            res
                        })
                        .collect_vec()
                }
            }
            if acc_vals.len() > 0 {
                return goal as usize;
            }
            0
        })
        .sum();
    res as u64
}
