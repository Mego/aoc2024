use itertools::Itertools;

pub fn parse(input: String) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
                .unwrap()
        })
        .unzip()
}

pub fn part1(input: String) -> u64 {
    let (mut a, mut b) = parse(input);
    a.sort();
    b.sort();
    a.iter().zip(b).map(|(&a, b)| a.abs_diff(b)).sum()
}

pub fn part2(input: String) -> u64 {
    let (a, b) = parse(input);
    a.iter()
        .map(|x| b.iter().filter(|&y| x == y).count() * (*x as usize))
        .sum::<usize>() as u64
}
