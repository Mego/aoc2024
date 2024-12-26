use std::{
    borrow::Borrow,
    cell::Ref,
    cmp::Ordering,
    collections::HashMap,
    fmt::Display,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    ops::{Deref, Index, IndexMut, Range, RangeFrom},
    slice::SliceIndex,
    sync::LazyLock,
};

use exhaust::Exhaust;
use grid::Grid;
use itertools::Itertools;
#[cfg(not(test))]
use regex::Regex;
use reqwest::header::COOKIE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Exhaust)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

impl Direction {
    pub fn to_delta(&self) -> CoordinateOffset {
        match self {
            Self::LEFT => (0, -1),
            Self::RIGHT => (0, 1),
            Self::UP => (-1, 0),
            Self::DOWN => (1, 0),
        }
        .into()
    }

    pub fn from_coords(a: Coordinate, b: Coordinate) -> Self {
        assert!(
            (a.x == b.x) ^ (a.y == b.y),
            "coordinates must be unique and lie on a cardinal line {},{}",
            a,
            b
        );
        match (a.x.cmp(&b.x), a.y.cmp(&b.y)) {
            (_, Ordering::Less) => Self::RIGHT,
            (_, Ordering::Greater) => Self::LEFT,
            (Ordering::Less, _) => Self::DOWN,
            (Ordering::Greater, _) => Self::UP,
            _ => unreachable!(),
        }
    }

    pub fn from_char(c: u8) -> Self {
        match c {
            b'<' => Self::LEFT,
            b'>' => Self::RIGHT,
            b'^' => Self::UP,
            b'v' => Self::DOWN,
            _ => unreachable!(),
        }
    }

    pub fn to_char(&self) -> u8 {
        match self {
            Self::LEFT => b'<',
            Self::RIGHT => b'>',
            Self::UP => b'^',
            Self::DOWN => b'v',
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::DOWN => Self::UP,
            Self::LEFT => Self::RIGHT,
            Self::UP => Self::DOWN,
            Self::RIGHT => Self::LEFT,
        }
    }

    pub fn cw(&self) -> Self {
        match self {
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
        }
    }

    pub fn cw_turns(&self, other: Self) -> usize {
        (0..4)
            .find_map(|n| {
                let mut dir = *self;
                for _ in 0..n {
                    dir = dir.cw();
                }
                (dir == other).then_some(n)
            })
            .unwrap()
    }

    pub fn move_dir(&self, pos: Coordinate) -> CoordinateOffset {
        let d = self.to_delta();
        (pos.x as isize + d.x, pos.y as isize + d.y).into()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DOWN => "v",
                Self::LEFT => "<",
                Self::RIGHT => ">",
                Self::UP => "^",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<&(usize, usize)> for Coordinate {
    fn from(value: &(usize, usize)) -> Self {
        Self::from(*value)
    }
}

impl From<Coordinate> for (usize, usize) {
    fn from(value: Coordinate) -> Self {
        (value.x, value.y)
    }
}

impl From<&Coordinate> for (usize, usize) {
    fn from(value: &Coordinate) -> Self {
        Self::from(*value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoordinateOffset {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for CoordinateOffset {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<CoordinateOffset> for Coordinate {
    fn from(value: CoordinateOffset) -> Self {
        Self {
            x: value.x as usize,
            y: value.y as usize,
        }
    }
}

pub struct GridRef<'a, T: Sized> {
    grid: &'a Grid<T>,
}

impl<T> Deref for GridRef<'_, T> {
    type Target = Grid<T>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<'a, T> From<&'a Grid<T>> for GridRef<'a, T> {
    fn from(value: &'a Grid<T>) -> Self {
        Self { grid: value }
    }
}

impl<T> Display for GridRef<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn print_grid(grid: &Grid<u8>) {
    println!(
        "{}",
        grid.iter_rows()
            .map(|row| row
                .map(|&i| format!("{}", String::from_utf8_lossy(&[i])))
                .join(""))
            .join("\n")
    );
}

const MY_COOKIE: &str = include_str!("../cookie.txt");

pub trait Gridtools<T: Sized> {
    fn try_move_dir(&self, pos: Coordinate, dir: Direction) -> Option<Coordinate>;
}

impl<T> Gridtools<T> for Grid<T> {
    fn try_move_dir(&self, pos: Coordinate, dir: Direction) -> Option<Coordinate> {
        let new_pos = dir.move_dir(pos);
        self.is_valid_index(new_pos).then_some(Coordinate {
            x: new_pos.x as usize,
            y: new_pos.y as usize,
        })
    }
}

pub trait IsValidIndex<T: Sized> {
    fn is_valid_index(&self, index: T) -> bool;
}

impl<T: Sized> IsValidIndex<(isize, isize)> for Grid<T> {
    fn is_valid_index(&self, index: (isize, isize)) -> bool {
        index.0 >= 0 && index.1 >= 0 && self.is_valid_index((index.0 as usize, index.1 as usize))
    }
}

impl<T: Sized> IsValidIndex<(usize, usize)> for Grid<T> {
    fn is_valid_index(&self, index: (usize, usize)) -> bool {
        index.0 < self.rows() && index.1 < self.cols()
    }
}

impl<T: Sized> IsValidIndex<CoordinateOffset> for Grid<T> {
    fn is_valid_index(&self, index: CoordinateOffset) -> bool {
        index.x >= 0 && index.y >= 0 && self.is_valid_index((index.x as usize, index.y as usize))
    }
}

impl<T: Sized> IsValidIndex<Coordinate> for Grid<T> {
    fn is_valid_index(&self, index: Coordinate) -> bool {
        index.x < self.rows() && index.y < self.cols()
    }
}

impl<T: Sized> Index<Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self[(index.x, index.y)]
    }
}

impl<T: Sized> IndexMut<Coordinate> for Grid<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        &mut self[(index.x, index.y)]
    }
}

const DELTAS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const EIGHT_DELTAS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

pub fn adj_squares<T: Sized>(grid: &Grid<T>, cur: (usize, usize)) -> Vec<(usize, usize)> {
    let cur_i = (cur.0 as isize, cur.1 as isize);
    DELTAS
        .iter()
        .filter_map(|(dx, dy)| {
            let sq = (cur_i.0 + dx, cur_i.1 + dy);
            grid.is_valid_index(sq)
                .then_some((sq.0 as usize, sq.1 as usize))
        })
        .collect_vec()
}

pub fn add_delta(cur: (usize, usize), delta: (isize, isize)) -> (isize, isize) {
    (cur.0 as isize + delta.0, cur.1 as isize + delta.1)
}

pub struct Neighborhood {
    pub up: Option<u8>,
    pub up_right: Option<u8>,
    pub right: Option<u8>,
    pub down_right: Option<u8>,
    pub down: Option<u8>,
    pub down_left: Option<u8>,
    pub left: Option<u8>,
    pub up_left: Option<u8>,
}

pub fn adj_squares8(grid: &Grid<u8>, cur: (usize, usize)) -> Neighborhood {
    Neighborhood {
        up: {
            let pos = add_delta(cur, EIGHT_DELTAS[0]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        up_right: {
            let pos = add_delta(cur, EIGHT_DELTAS[1]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        right: {
            let pos = add_delta(cur, EIGHT_DELTAS[2]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        down_right: {
            let pos = add_delta(cur, EIGHT_DELTAS[3]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        down: {
            let pos = add_delta(cur, EIGHT_DELTAS[4]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        down_left: {
            let pos = add_delta(cur, EIGHT_DELTAS[5]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        left: {
            let pos = add_delta(cur, EIGHT_DELTAS[6]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
        up_left: {
            let pos = add_delta(cur, EIGHT_DELTAS[7]);
            grid.is_valid_index(pos)
                .then(|| *grid.get(pos.0, pos.1).unwrap())
        },
    }
}

pub fn diff(c: u8, sq: Option<u8>) -> bool {
    sq.is_none_or(|x| x != c)
}

#[cfg(not(test))]
static WAIT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"You gave an answer too recently; you have to wait after submitting an answer before trying again\.  You have (.+) left to wait\.",
    ).unwrap()
});

#[cfg(not(test))]
static WRONG_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"That's not the right answer; your answer is (.+ .+).").unwrap());

pub async fn fetch_input(year: u16, day: u8) -> String {
    let fname = format!("inputs/{}/day{}.txt", year, day);
    if let Ok(contents) = read_to_string(&fname) {
        return contents;
    }
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let data = reqwest::Client::new()
        .get(url)
        .header(COOKIE, MY_COOKIE)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let f = File::options()
        .create(true)
        .write(true)
        .open(&fname)
        .unwrap();
    let mut buf = BufWriter::new(f);
    write!(&mut buf, "{}", data).unwrap();
    data
}

static CORRECT_ANSWERS: LazyLock<HashMap<(u8, u64), u64>> = LazyLock::new(|| {
    [
        ((1, 1), 1590491),
        ((1, 2), 22588371),
        ((2, 1), 516),
        ((2, 2), 561),
        ((3, 1), 174561379),
        ((3, 2), 106921067),
        ((4, 1), 2549),
        ((4, 2), 2003),
        ((5, 1), 4578),
        ((5, 2), 6179),
        ((6, 1), 5305),
        ((6, 2), 2143),
        ((7, 1), 1620690235709),
        ((7, 2), 145397611075341),
        ((8, 1), 413),
        ((8, 2), 1417),
        ((9, 1), 6607511583593),
        ((9, 2), 6636608781232),
        ((10, 1), 825),
        ((10, 2), 1805),
        ((11, 1), 218956),
        ((11, 2), 259593838049805),
        ((12, 1), 1396298),
        ((12, 2), 853588),
        ((13, 1), 29023),
        ((13, 2), 96787395375634),
        ((14, 1), 230686500),
        ((14, 2), 7672),
        ((15, 1), 1446158),
        ((15, 2), 1446175),
        ((16, 1), 107468),
        ((16, 2), 533),
        ((17, 2), 190593310997519),
        ((18, 1), 314),
        ((19, 1), 233),
        ((19, 2), 691316989225259),
        ((20, 1), 1367),
        ((20, 2), 1006850),
    ]
    .into_iter()
    .collect()
});

static CORRECT_STR_ANSWERS: LazyLock<HashMap<(u8, u64), &str>> = LazyLock::new(|| {
    [((17, 1), "3,1,5,3,7,4,2,7,5"), ((18, 2), "15,20")]
        .into_iter()
        .collect()
});

#[cfg(test)]
pub async fn submit(_year: u16, day: u8, level: u64, answer: u64) -> String {
    if let Some(&val) = CORRECT_ANSWERS.get(&(day, level)) {
        if answer == val {
            return format!("right ({answer})");
        }
        return format!("wrong (expected {val}, received {answer})");
    }
    return "unsolved".to_owned();
}

#[cfg(not(test))]
pub async fn submit(year: u16, day: u8, level: u64, answer: u64) -> String {
    if let Some(&val) = CORRECT_ANSWERS.get(&(day, level)) {
        if answer == val {
            return format!("right ({answer})");
        }
        return format!("wrong (expected {val}, received {answer})");
    }
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let resp = reqwest::Client::new()
        .post(url)
        .header(COOKIE, MY_COOKIE)
        .form(&HashMap::from([("level", level), ("answer", answer)]))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    if let Some(captures) = WAIT_REGEX.captures(&resp) {
        return format!("wait {}", captures.get(1).unwrap().as_str());
    }
    if let Some(captures) = WRONG_REGEX.captures(&resp) {
        return format!("wrong, {}", captures.get(1).unwrap().as_str());
    }
    if resp.contains("That's the right answer!") {
        return "good".to_owned();
    }
    if resp.contains("please wait 5 minutes before trying again") {
        return "wrong; please wait 5 minutes before trying again".to_owned();
    }
    if resp.contains("That's not the right answer.  If you're stuck") {
        return "wrong".to_owned();
    }
    if resp.contains("Did you already complete it?") {
        return "already done".to_owned();
    }
    resp
}

#[cfg(test)]
pub async fn submit_str(_year: u16, day: u8, level: u64, answer: String) -> String {
    if let Some(&val) = CORRECT_STR_ANSWERS.get(&(day, level)) {
        if answer == val {
            return format!("right ({answer})");
        }
        return format!("wrong (expected {val}, received {answer})");
    }
    return "unsolved".to_owned();
}

#[cfg(not(test))]
pub async fn submit_str(year: u16, day: u8, level: u64, answer: String) -> String {
    if let Some(&val) = CORRECT_STR_ANSWERS.get(&(day, level)) {
        if answer == val {
            return format!("right ({answer})");
        }
        return format!("wrong (expected {val}, received {answer})");
    }
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let resp = reqwest::Client::new()
        .post(url)
        .header(COOKIE, MY_COOKIE)
        .form(&HashMap::from([
            ("level", format!("{level}")),
            ("answer", answer),
        ]))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    if let Some(captures) = WAIT_REGEX.captures(&resp) {
        return format!("wait {}", captures.get(1).unwrap().as_str());
    }
    if let Some(captures) = WRONG_REGEX.captures(&resp) {
        return format!("wrong, {}", captures.get(1).unwrap().as_str());
    }
    if resp.contains("That's the right answer!") {
        return "good".to_owned();
    }
    if resp.contains("please wait 5 minutes before trying again") {
        return "wrong; please wait 5 minutes before trying again".to_owned();
    }
    if resp.contains("That's not the right answer.  If you're stuck") {
        return "wrong".to_owned();
    }
    if resp.contains("Did you already complete it?") {
        return "already done".to_owned();
    }
    resp
}
