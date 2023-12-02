mod counter;
mod point;
mod search;

pub use counter::*;
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

pub fn split_once<'a>(s: &'a str, splitter: &str) -> Option<(&'a str, &'a str)> {
    let mut parts = s.splitn(2, splitter);
    let a = parts.next().unwrap();
    let b = parts.next()?;
    Some((a, b))
}
