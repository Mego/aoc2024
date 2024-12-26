use grid::Grid;
use itertools::Itertools;

use crate::util::adj_squares;

const WIDTH: i16 = 101;
const HEIGHT: i16 = 103;

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: i16,
    y: i16,
    vx: i16,
    vy: i16,
}

impl Robot {
    pub fn parse(line: &str) -> Self {
        let (x, y, vx, vy) = line
            .split_ascii_whitespace()
            .flat_map(|s| s[2..].split(",").map(|x| x.parse::<i16>().unwrap()))
            .collect_tuple()
            .unwrap();
        Self { x, y, vx, vy }
    }

    pub fn step(self) -> Self {
        let x = (self.x + self.vx + WIDTH) % WIDTH;
        let y = (self.y + self.vy + HEIGHT) % HEIGHT;
        Self { x, y, ..self }
    }

    pub fn step_mut(&mut self) {
        self.x = (self.x + self.vx + WIDTH) % WIDTH;
        self.y = (self.y + self.vy + HEIGHT) % HEIGHT;
    }
}

fn count_quads(robots: &Vec<Robot>) -> [u64; 4] {
    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;
    let mut counts = [0; 4];

    for r in robots {
        match ((r.x - mid_x).signum(), (r.y - mid_y).signum()) {
            (1, 1) => counts[0] += 1,
            (1, -1) => counts[1] += 1,
            (-1, -1) => counts[2] += 1,
            (-1, 1) => counts[3] += 1,
            _ => {}
        };
    }

    counts
}

pub fn part1(input: String) -> u64 {
    let mut robots = input.lines().map(Robot::parse).collect_vec();
    for _ in 0..100 {
        robots = robots.into_iter().map(Robot::step).collect_vec();
    }

    count_quads(&robots).into_iter().product::<u64>()
}

pub fn part2(input: String) -> u64 {
    let mut robots = input.lines().map(Robot::parse).collect_vec();
    let mut grid = Grid::<u16>::new(WIDTH as usize, HEIGHT as usize);
    for i in 1..(WIDTH * HEIGHT) as u16 {
        for r in robots.iter_mut() {
            r.step_mut();
            grid[(r.x as usize, r.y as usize)] = i;
        }
        let num_robots_with_adj: usize = robots
            .iter()
            .map(|r| {
                let adj = adj_squares(&grid, (r.x as usize, r.y as usize));
                adj.into_iter().filter(|&pos| grid[pos] == i).count()
            })
            .sum();
        if num_robots_with_adj >= robots.len() / 10 * 9 {
            return i as u64;
        }
    }
    unreachable!();
}
