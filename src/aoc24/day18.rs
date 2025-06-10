use itertools::Itertools;
use pathfinding::{
    grid::Grid,
    prelude::{bfs_bidirectional, dijkstra},
};

pub fn parse(input: &[&str], size: usize) -> Grid {
    let mut g = Grid::new(size, size);
    input.iter().for_each(|l| {
        g.add_vertex(
            l.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap(),
        );
    });
    g.invert();
    g
}

pub fn parse2(input: &[(usize, usize)], size: usize) -> Grid {
    let mut g = Grid::new(size, size);
    input.iter().for_each(|&p| {
        g.add_vertex(p);
    });
    g.invert();
    g
}

pub fn process_block(g: &mut Grid, input: (usize, usize)) {
    g.invert();
    g.add_vertex(input);
    g.invert();
}

pub fn part1(input: String) -> u64 {
    const BLOCKS: usize = 1024;
    const SIZE: usize = 71;
    let g = parse(&input.lines().take(BLOCKS).collect_vec(), SIZE);
    const START: (usize, usize) = (0, 0);
    const GOAL: (usize, usize) = (SIZE - 1, SIZE - 1);
    let sol = dijkstra(
        &START,
        |&p| g.neighbours(p).into_iter().map(|n| (n, 1)),
        |&p| p == GOAL,
    )
    .unwrap();

    sol.1 as u64
}

pub fn part2(input: String) -> String {
    const SIZE: usize = 71;
    let coords = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect_vec();
    let mut g = Grid::new(SIZE, SIZE);
    g.invert();
    for block in 0.. {
        process_block(&mut g, coords[block]);
        const START: (usize, usize) = (0, 0);
        const GOAL: (usize, usize) = (SIZE - 1, SIZE - 1);
        let successors = |p: &(usize, usize)| g.neighbours(*p).into_iter();
        if let None = bfs_bidirectional(&START, &GOAL, successors, successors) {
            return format!("{},{}", coords[block].0, coords[block].1);
        }
    }
    "".to_owned()
}
