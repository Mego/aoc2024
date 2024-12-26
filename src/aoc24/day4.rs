use itertools::Itertools;

pub fn part1(input: String) -> u64 {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let xmas = ['X', 'M', 'A', 'S'];
    let mut res = 0;
    (0..grid.len()).for_each(|x| {
        (0..grid[x].len()).for_each(|y| {
            if y + 3 < grid[x].len() && (grid[x][y..=y + 3] == xmas) {
                res += 1;
            }
            if y >= 3 && ([grid[x][y], grid[x][y - 1], grid[x][y - 2], grid[x][y - 3]] == xmas) {
                res += 1;
            }
            if x + 3 < grid.len()
                && ([grid[x][y], grid[x + 1][y], grid[x + 2][y], grid[x + 3][y]] == xmas)
            {
                res += 1;
            }
            if x >= 3 && ([grid[x][y], grid[x - 1][y], grid[x - 2][y], grid[x - 3][y]] == xmas) {
                res += 1;
            }
            if x + 3 < grid.len()
                && y + 3 < grid[x].len()
                && ([
                    grid[x][y],
                    grid[x + 1][y + 1],
                    grid[x + 2][y + 2],
                    grid[x + 3][y + 3],
                ] == xmas)
            {
                res += 1;
            }
            if x >= 3
                && y >= 3
                && ([
                    grid[x][y],
                    grid[x - 1][y - 1],
                    grid[x - 2][y - 2],
                    grid[x - 3][y - 3],
                ] == xmas)
            {
                res += 1;
            }
            if x + 3 < grid.len()
                && y >= 3
                && ([
                    grid[x][y],
                    grid[x + 1][y - 1],
                    grid[x + 2][y - 2],
                    grid[x + 3][y - 3],
                ] == xmas)
            {
                res += 1;
            }
            if x >= 3
                && y + 3 < grid[x].len()
                && ([
                    grid[x][y],
                    grid[x - 1][y + 1],
                    grid[x - 2][y + 2],
                    grid[x - 3][y + 3],
                ] == xmas)
            {
                res += 1;
            }
        });
    });
    res
}

pub fn part2(input: String) -> u64 {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();
    let mut res = 0;
    (0..grid.len() - 2).for_each(|x| {
        (0..grid[x].len() - 2).for_each(|y| {
            if grid[x + 1][y + 1] == 'A' {
                // conor reference?
                let mut corners = [
                    grid[x][y],
                    grid[x + 2][y],
                    grid[x][y + 2],
                    grid[x + 2][y + 2],
                ];
                corners.sort();
                if corners == ['M', 'M', 'S', 'S'] && grid[x][y] != grid[x + 2][y + 2] {
                    res += 1;
                }
            }
        })
    });
    res
}
