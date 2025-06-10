use aoc2024::aoc24::*;
use aoc2024::util::fetch_input;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use tokio::runtime::Runtime;

macro_rules! bench_part {
    ( $c:expr, $input:expr, $day:ident, $part:ident ) => {
        $c.bench_function(
            &format!("{} - {}", stringify!($day), stringify!($part)),
            |b| {
                b.iter_batched(|| $input.clone(), |i| $day::$part(i), BatchSize::LargeInput);
            },
        );
    };
}

macro_rules! day {
    ( $day:ident ) => {
        stringify!($day)[3..].parse::<u8>().unwrap()
    };
}

macro_rules! create_benches {
    ( $( $day:ident ),* ) => {
        $(
            fn $day(c: &mut Criterion) {
                let day = day!($day);
                let input = Runtime::new().unwrap().block_on(fetch_input(2024, day));

                bench_part!(c, input, $day, part1);
                bench_part!(c, input, $day, part2);
            }
        )*

        criterion_group!{
            name = benches;
            config = Criterion::default().sample_size(10);
            targets = $( $day, )*
        }
    };
}

create_benches!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

criterion_main!(benches);
