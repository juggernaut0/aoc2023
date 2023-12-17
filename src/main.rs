#![warn(clippy::pedantic)]
// TODO fix casts
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]

use clap::{Arg, Command};
use log::Level;
use std::time::Instant;

#[cfg(test)]
mod answers;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

const DAYS: [&dyn Solution; 25] = [
    &day1::Solution,
    &day2::Solution,
    &day3::Solution,
    &day4::Solution,
    &day5::Solution,
    &day6::Solution,
    &day7::Solution,
    &day8::Solution,
    &day9::Solution,
    &day10::Solution,
    &day11::Solution,
    &day12::Solution,
    &day13::Solution,
    &day14::Solution,
    &day15::Solution,
    &day16::Solution,
    &day17::Solution,
    &day18::Solution,
    &day19::Solution,
    &day20::Solution,
    &day21::Solution,
    &day22::Solution,
    &day23::Solution,
    &day24::Solution,
    &day25::Solution,
];

fn main() {
    let matches = Command::new("aoc2023")
        .about("Advent of Code 2023")
        .arg(
            Arg::new("day")
                .value_parser(clap::value_parser!(u32).range(1..=25))
                .required(true),
        )
        .arg(Arg::new("puzzle").value_parser(["1", "2"]))
        .arg(
            Arg::new("log_level")
                .long("level")
                .help("Logging level")
                .value_parser(["trace", "debug", "info", "warn", "error"])
                .default_value("warn"),
        )
        .get_matches();

    let level_match = &matches
        .get_one("log_level")
        .map(|it: &String| it.to_lowercase())
        .unwrap();
    let log_level = match level_match.as_str() {
        "trace" => Level::Trace,
        "debug" => Level::Debug,
        "info" => Level::Info,
        "warn" => Level::Warn,
        "error" => Level::Error,
        _ => unreachable!(),
    };

    simple_logger::init_with_level(log_level).unwrap();

    let day: usize = matches.get_one("day").map(|it: &u32| *it as usize).unwrap();
    let puzzle = matches.get_one("puzzle").map(String::as_str);

    let solution = DAYS[day - 1];

    let (answer, time) = match puzzle {
        Some("1") => {
            let input = read_input(day, "1");
            time(|| solution.solve_1(input))
        }
        Some("2") => {
            let input = read_input(day, "2");
            time(|| solution.solve_2(input))
        }
        Some(_) => unreachable!(),
        None => {
            let input_1 = read_input(day, "1");
            let input_2 = read_input(day, "2");
            let ((a1, a2), time) = time(|| (solution.solve_1(input_1), solution.solve_2(input_2)));
            (a1 + "\n" + &a2, time)
        }
    };

    println!("{answer}");
    println!("Elapsed: {time}ms");
}

fn read_input(day: usize, puzzle: &str) -> String {
    let dss = format!("input/{day}-{puzzle}.txt");
    let ds = std::path::Path::new(&dss);
    if ds.exists() {
        std::fs::read_to_string(ds).unwrap()
    } else {
        std::fs::read_to_string(format!("input/{day}.txt")).unwrap()
    }
}

fn time<T>(f: impl FnOnce() -> T) -> (T, u128) {
    let start = Instant::now();
    let res = f();
    (res, start.elapsed().as_millis())
}

trait Solution {
    fn solve_1(&self, input: String) -> String;
    fn solve_2(&self, input: String) -> String;
}

#[cfg(test)]
fn init_test_logging() {
    use std::sync::Once;
    static TEST_INIT: Once = Once::new();
    TEST_INIT.call_once(|| simple_logger::init_with_level(Level::Trace).unwrap());
}
