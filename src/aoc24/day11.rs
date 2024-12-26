use cached::proc_macro::cached;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[cached]
fn update_stone(stone: u64, count: u8) -> u64 {
    if count == 0 {
        return 1;
    }
    if stone == 0 {
        return update_stone(1, count - 1);
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            return update_stone(stone / 10u64.pow(digits / 2), count - 1)
                + update_stone(stone % 10u64.pow(digits / 2), count - 1);
        } else {
            return update_stone(stone * 2024, count - 1);
        }
    }
}

pub fn part1(input: String) -> u64 {
    let stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();
    return stones
        .into_par_iter()
        .map(|stone| update_stone(stone, 25))
        .sum();
}

pub fn part2(input: String) -> u64 {
    let stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();
    return stones
        .into_par_iter()
        .map(|stone| update_stone(stone, 75))
        .sum();
}
