use grid::Grid;
use itertools::Itertools;
use reqwest::header;

fn parse(input: String) -> (Vec<Vec<usize>>, Vec<Vec<usize>>, usize) {
    let mut keys = vec![];
    let mut locks = vec![];
    let mut height = 0;

    input.split("\n\n").for_each(|d| {
        let first_line = d.lines().next().unwrap();
        let is_lock = first_line.bytes().all(|b| b == b'#');
        let mut d2 = Grid::from_vec(
            d.lines().flat_map(|l| l.bytes()).collect_vec(),
            first_line.len(),
        );
        d2.transpose();
        height = d2.rows();
        let nums = d2
            .iter_rows()
            .map(|r| r.filter(|&&b| b == b'#').count() - 1)
            .collect_vec();
        if is_lock {
            locks.push(nums);
        } else {
            keys.push(nums);
        }
    });

    (locks, keys, height)
}

pub fn part1(input: String) -> u64 {
    let (locks, keys, height) = parse(input);
    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(l, k)| l.iter().zip(k).all(|(a, b)| a + b <= height))
        .count() as u64
}
