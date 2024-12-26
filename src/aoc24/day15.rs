use std::{cell::RefCell, collections::HashMap, fmt::Display};

use grid::Grid;
use itertools::Itertools;

use crate::util::{Coordinate, Direction, GridRef, IsValidIndex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Box {
    left: Coordinate,
    right: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GridSpace {
    EMPTY,
    WALL,
    ROBOT,
    BoxLeft(Box),
    BoxRight(Box),
}

impl GridSpace {
    fn can_move(
        &self,
        grid: &Grid<GridSpace>,
        dir: Direction,
        pos: Coordinate,
        check_box_pair: bool,
    ) -> Option<Vec<Coordinate>> {
        let next_pos_offset = dir.move_dir(pos);
        if grid.is_valid_index(next_pos_offset) {
            let next_pos: Coordinate = next_pos_offset.into();
            return match (self, dir) {
                (Self::EMPTY, _) => Some(vec![]),
                (Self::WALL, _) => None,
                (Self::BoxLeft(b) | Self::BoxRight(b), Direction::UP | Direction::DOWN) => {
                    let pair_pos = match self {
                        Self::BoxLeft(_) => b.right,
                        Self::BoxRight(_) => b.left,
                        _ => unreachable!(),
                    };

                    if let Some(v1) = grid[next_pos].can_move(grid, dir, next_pos, true) {
                        if check_box_pair {
                            if let Some(v2) = grid[pair_pos].can_move(grid, dir, pair_pos, false) {
                                return Some(
                                    vec![pos].into_iter().chain(v1).chain(v2).collect_vec(),
                                );
                            }
                        } else {
                            return Some(vec![pos].into_iter().chain(v1).collect_vec());
                        }
                    }

                    None
                }
                _ => grid[next_pos]
                    .can_move(grid, dir, next_pos, true)
                    .map(|v| vec![pos].into_iter().chain(v).collect_vec()),
            };
        }
        None
    }
}

impl Display for GridSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EMPTY => ".",
                Self::ROBOT => "@",
                Self::WALL => "#",
                Self::BoxLeft(_) => "[",
                Self::BoxRight(_) => "]",
            }
        )
    }
}

fn parse(input: String) -> (Grid<u8>, Vec<Direction>, (usize, usize)) {
    let (map, moves) = input.split("\n\n").collect_tuple().unwrap();
    let grid = Grid::from_vec(
        map.lines().flat_map(|l| l.bytes()).collect_vec(),
        map.lines().next().unwrap().len(),
    );
    let robot_pos = grid
        .indexed_iter()
        .find_map(|(pos, &c)| (c == b'@').then_some(pos))
        .unwrap();
    (
        grid,
        moves
            .lines()
            .flat_map(|l| l.bytes())
            .map(Direction::from_char)
            .collect_vec(),
        robot_pos,
    )
}

fn step(grid: &mut Grid<u8>, robot_pos: (usize, usize), dir: Direction) -> (usize, usize) {
    let mut prev_pos = robot_pos;
    let mut next_pos = dir.move_dir(robot_pos.into());
    let robot_target_pos = (next_pos.x as usize, next_pos.y as usize);
    let mut found_space = false;
    let mut updates = HashMap::new();
    while grid.is_valid_index(next_pos) {
        let pos = (next_pos.x as usize, next_pos.y as usize);
        if grid[pos] == b'#' {
            break;
        }
        updates.insert(pos, grid[prev_pos]);
        if grid[pos] == b'.' {
            found_space = true;
            break;
        }

        prev_pos = pos;
        next_pos = dir.move_dir(pos.into());
    }
    if found_space {
        for (pos, c) in updates {
            grid[pos] = c;
        }
        grid[robot_pos] = b'.';
        return robot_target_pos;
    }
    robot_pos
}

pub fn part1(input: String) -> u64 {
    let (mut grid, moves, mut robot_pos) = parse(input);
    // print_grid(&grid);
    // println!();
    for dir in moves {
        robot_pos = step(&mut grid, robot_pos, dir);
        // print_grid(&grid);
        // println!();
    }
    grid.indexed_iter()
        .filter_map(|(pos, &c)| (c == b'O').then_some(pos))
        .fold(0, |acc, pos| acc + (100 * pos.0 + pos.1) as u64)
}

fn widen(input: String) -> String {
    let (map, moves) = input.split("\n\n").collect_tuple().unwrap();
    format!(
        "{}\n\n{}",
        map.bytes()
            .map(|b| match b {
                b'#' => "##",
                b'O' => "[]",
                b'.' => "..",
                b'@' => "@.",
                b'\n' => "\n",
                _ => unreachable!("{}", String::from_utf8_lossy(&[b])),
            })
            .join(""),
        moves
    )
}

fn parse2(input: String) -> (Grid<GridSpace>, Vec<Direction>, Coordinate) {
    let (map, moves) = input.split("\n\n").collect_tuple().unwrap();
    let robot_pos = RefCell::new((0, 0));
    let grid = Grid::from_vec(
        map.lines()
            .enumerate()
            .flat_map(|(x, l)| {
                let robot_pos = &robot_pos;
                l.bytes().enumerate().filter_map(move |(y, b)| match b {
                    b'#' => Some(GridSpace::WALL),
                    b'@' => {
                        robot_pos.replace((x, y));
                        Some(GridSpace::ROBOT)
                    }
                    b'.' => Some(GridSpace::EMPTY),
                    b'[' => Some(GridSpace::BoxLeft(Box {
                        left: (x, y).into(),
                        right: (x, y + 1).into(),
                    })),
                    b']' => Some(GridSpace::BoxRight(Box {
                        left: (x, y - 1).into(),
                        right: (x, y).into(),
                    })),
                    _ => unreachable!(),
                })
            })
            .collect_vec(),
        map.lines().next().unwrap().trim().len(),
    );
    (
        grid,
        moves
            .lines()
            .flat_map(|l| l.bytes())
            .map(Direction::from_char)
            .collect_vec(),
        robot_pos.take().into(),
    )
}

fn step2(grid: &mut Grid<GridSpace>, robot_pos: Coordinate, dir: Direction) -> Coordinate {
    if let Some(v) = grid[robot_pos].can_move(grid, dir, robot_pos, true) {
        let mut updates = HashMap::new();
        for &pos in v.iter() {
            let next_pos: Coordinate = dir.move_dir(pos).into();
            updates.insert(next_pos, grid[pos]);
        }
        for pos in v {
            grid[pos] = GridSpace::EMPTY;
        }
        for (c, g) in updates {
            if let GridSpace::BoxLeft(_) = g {
                let b = Box {
                    left: c,
                    right: (c.x, c.y + 1).into(),
                };
                grid[c] = GridSpace::BoxLeft(b);
            } else if let GridSpace::BoxRight(_) = g {
                let b = Box {
                    left: (c.x, c.y - 1).into(),
                    right: c,
                };
                grid[c] = GridSpace::BoxRight(b);
            } else {
                grid[c] = g;
            }
        }
        return dir.move_dir(robot_pos).into();
    }
    robot_pos
}

pub fn part2(input: String) -> u64 {
    let (mut grid, moves, mut robot_pos) = parse2(widen(input));
    // print_grid(&grid);
    // println!();
    // println!("Initial state:");
    // print_grid2(&grid);
    // println!();
    for dir in moves {
        // println!("Move {dir}:");
        robot_pos = step2(&mut grid, robot_pos, dir);
        // print_grid2(&grid);
        // println!();
    }
    // print_grid2(&grid);
    grid.indexed_iter()
        .filter_map(|(pos, &ref c)| {
            if let GridSpace::BoxLeft(_) = c {
                return Some(pos);
            }
            None
        })
        .fold(0, |acc, pos| acc + (100 * pos.0 + pos.1) as u64)
}

fn print_grid2(grid: &Grid<GridSpace>) {
    println!("{}", GridRef::from(grid));
}
