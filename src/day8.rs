use crate::util::lcm;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (dirs, nodes) = parse_input(&input);

        let mut pos = "AAA";
        let mut i = 0;
        let mut count = 0;
        while pos != "ZZZ" {
            let dir = dirs[i];
            count += 1;
            let (left, right) = &nodes[pos];
            pos = match dir {
                Dir::L => left.as_str(),
                Dir::R => right.as_str(),
            };
            i = (i + 1) % dirs.len();
        }

        count.to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let (dirs, nodes) = parse_input(&input);

        let starts: HashSet<_> = nodes
            .keys()
            .filter(|it| it.ends_with('A'))
            .map(|it| get_end(it.as_str(), &dirs, &nodes))
            .collect();

        starts.into_iter().reduce(lcm).unwrap().to_string()
    }
}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new("(.+) = \\((.+), (.+)\\)").unwrap());

fn parse_input(input: &str) -> (Vec<Dir>, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let dirs = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => panic!("{c}"),
        })
        .collect();

    lines.next().unwrap();

    let mut nodes = HashMap::new();
    for line in lines {
        let matches = RE.captures(line).unwrap();
        let name = matches[1].to_string();
        let left = matches[2].to_string();
        let right = matches[3].to_string();
        nodes.insert(name, (left, right));
    }

    (dirs, nodes)
}

#[derive(Copy, Clone)]
enum Dir {
    L,
    R,
}

fn get_end(start: &str, dirs: &[Dir], nodes: &HashMap<String, (String, String)>) -> u64 {
    let mut pos = start;
    let mut i = 0;
    let mut count = 0;
    // input guarantees that you only hit one Z per start
    while !pos.ends_with('Z') {
        let dir = dirs[i];
        count += 1;
        let (left, right) = &nodes[pos];
        pos = match dir {
            Dir::L => left.as_str(),
            Dir::R => right.as_str(),
        };
        i = (i + 1) % dirs.len();
    }
    count
}
