use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

fn step_secret(secret: u64) -> u64 {
    let mut res = ((secret * 64) ^ secret) % 16777216;
    res ^= res / 32;
    res %= 16777216;
    res ^= res * 2048;
    res %= 16777216;
    res
}

pub fn part1(input: String) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut secret = l.parse().unwrap();
            for _ in 0..2000 {
                secret = step_secret(secret);
            }
            secret
        })
        .sum()
}

pub fn part2(input: String) -> u64 {
    let mut initial_prices = vec![];
    let deltas = input
        .lines()
        .map(|l| {
            let mut secret = l.parse().unwrap();
            let mut result = vec![];
            for i in 0..2000 {
                secret = step_secret(secret);
                if i == 0 {
                    initial_prices.push(secret % 10);
                }
                result.push(secret % 10);
            }
            result
                .windows(2)
                .map(|x| {
                    let a = x[0];
                    let b = x[1];
                    b - a
                })
                .collect_vec()
        })
        .collect_vec();

    deltas
        .iter()
        .flat_map(|ds| ds.windows(4))
        .collect::<HashSet<_>>()
        .into_par_iter()
        .map(|w| {
            let mut profit = 0;
            for (buyer_deltas, &init_price) in deltas.iter().zip(initial_prices.iter()) {
                let mut pos = 0;
                while pos < buyer_deltas.len() - 4 && buyer_deltas[pos..pos + 4] != *w {
                    pos += 1;
                }
                if pos < buyer_deltas.len() - 4 {
                    profit += buyer_deltas[..pos + 4]
                        .iter()
                        .fold(init_price, |a, v| a + v);
                }
            }
            profit
        })
        .max()
        .unwrap()
}
