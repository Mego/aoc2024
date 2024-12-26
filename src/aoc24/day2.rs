use itertools::Itertools;

pub fn part1(input: String) -> u64 {
    let lines = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    lines
        .iter()
        .filter(|line| {
            (line.iter().is_sorted() || line.iter().rev().is_sorted())
                && line
                    .windows(2)
                    .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)
        })
        .count() as u64
}

pub fn part2(input: String) -> u64 {
    let lines = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    lines
        .iter()
        .filter(|line| {
            line.iter().combinations(line.len() - 1).any(|c| {
                (c.iter().is_sorted() || c.iter().rev().is_sorted())
                    && c.windows(2)
                        .all(|w| w[0].abs_diff(*w[1]) >= 1 && w[0].abs_diff(*w[1]) <= 3)
            })
        })
        .count() as u64
}
