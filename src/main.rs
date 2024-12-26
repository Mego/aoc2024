mod aoc24;
mod math;
mod util;

use aoc24::*;
use util::{fetch_input, submit, submit_str};

#[cfg(test)]
use paste::paste;

#[tokio::main]
async fn main() {
    // let input = fetch_input(2024, 25).await;
    let input = r"".to_owned();

    let res = day24::part1(input);
    println!("{res}");
    // println!("{}", submit(2024, 25, 1, res).await);
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
                // let input = "2333133121414131402".to_owned();
                {
                    let res = $day::part1(input.clone());
                    let output = runtime.block_on(submit(2024, day, 1, res));
                    println!("{day}::part1 {}", res);
                    assert!(output.starts_with("right"));
                    // assert!(res == 1928);
                }
                {
                    let res = $day::part2(input);
                    let output = runtime.block_on(submit(2024, day, 2, res));
                    println!("{day}::part2 {}", res);
                    assert!(output.starts_with("right"));
                    // assert!(res == 2858);
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    run_day_test!(day1);
    run_day_test!(day2);
    run_day_test!(day3);
    run_day_test!(day4);
    run_day_test!(day5);
    run_day_test!(day6);
    run_day_test!(day7);
    run_day_test!(day8);
    run_day_test!(day9);
    run_day_test!(day10);
    run_day_test!(day11);
    run_day_test!(day12);
    run_day_test!(day13);
}
