mod aoc24;
mod math;
mod util;

#[cfg(test)]
use aoc24::*;
#[cfg(test)]
use util::{fetch_input, submit};

#[cfg(test)]
use paste::paste;

#[tokio::main]
async fn main() {
    use aoc24::*;
    use util::fetch_input;
    let input = fetch_input(2024, 20).await;
    let result = day20::part1(input);
    println!("{result}");
}

#[cfg(test)]
macro_rules! day {
    ( $day:ident ) => {
        stringify!($day)[3..].parse::<u8>().unwrap()
    };
}

#[cfg(test)]
macro_rules! run_day_test {
    ( $day:ident ) => {
        paste! {
            #[test]
            fn [<test_ $day>]() {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let day = day!($day);
                let input = runtime.block_on(fetch_input(2024, day)).trim().to_string();
                {
                    let res = $day::part1(input.clone());
                    let output = submit(2024, day, 1, format!("{res}"));
                    println!("{day}::part1 {}", res);
                    assert!(output.starts_with("right"));
                }
                {
                    let res = $day::part2(input);
                    let output = submit(2024, day, 2, format!("{res}"));
                    println!("{day}::part2 {}", res);
                    assert!(output.starts_with("right"));
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    run_day_test!(day01);
    run_day_test!(day02);
    run_day_test!(day03);
    run_day_test!(day04);
    run_day_test!(day05);
    run_day_test!(day06);
    run_day_test!(day07);
    run_day_test!(day08);
    run_day_test!(day09);
    run_day_test!(day10);
    run_day_test!(day11);
    run_day_test!(day12);
    run_day_test!(day13);
    run_day_test!(day14);
    run_day_test!(day15);
    run_day_test!(day16);
    run_day_test!(day17);
    run_day_test!(day18);
    run_day_test!(day19);
    run_day_test!(day20);
    run_day_test!(day21);
    run_day_test!(day22);
    run_day_test!(day23);
    run_day_test!(day24);
    run_day_test!(day25);
}
