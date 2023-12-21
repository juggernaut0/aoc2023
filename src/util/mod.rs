#![allow(dead_code)]
mod counter;
mod dir;
mod grid;
mod point;
mod search;

pub use counter::*;
pub use dir::*;
pub use grid::*;
pub use point::*;
pub use search::*;
use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_lines<'a, T: FromStr + 'a>(input: &'a str) -> impl Iterator<Item = T> + 'a
where
    T::Err: Debug,
{
    parse_lines_with(input, |line| line.parse().unwrap())
}

pub fn parse_lines_with<'a, T, P: FnMut(&'a str) -> T + 'a>(
    input: &'a str,
    parser: P,
) -> impl Iterator<Item = T> + 'a {
    input.lines().map(parser)
}

pub fn rev_chars(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    loop {
        if b == 0 {
            return a;
        }
        let t = a % b;
        a = b;
        b = t;
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn circ_mod(a: i32, b: i32) -> i32 {
    let r = a % b;
    if r < 0 {
        r + b
    } else {
        r
    }
}
