use crate::util::Point;
use itertools::Itertools;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let mut galaxies = parse_input(&input);
        expand(&mut galaxies, 2);
        galaxies
            .into_iter()
            .combinations(2)
            .map(|c| c[0].l1dist(c[1]))
            .sum::<i32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let mut galaxies = parse_input(&input);
        expand(&mut galaxies, 1_000_000);
        galaxies
            .into_iter()
            .combinations(2)
            .map(|c| i64::from(c[0].l1dist(c[1])))
            .sum::<i64>()
            .to_string()
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut res = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                res.push(Point::of(x, y));
            }
        }
    }
    res
}

fn expand(points: &mut [Point], times: i32) {
    let exp = times - 1;

    let mut max_x = points.iter().map(|p| p.0).max().unwrap();
    let mut x = 0;
    while x <= max_x {
        if !points.iter().any(|p| p.0 == x) {
            for p in points.iter_mut().filter(|p| p.0 > x) {
                p.0 += exp;
            }
            x += exp;
            max_x += exp;
        }
        x += 1;
    }

    let mut max_y = points.iter().map(|p| p.1).max().unwrap();
    let mut y = 0;
    while y <= max_y {
        if !points.iter().any(|p| p.1 == y) {
            for p in points.iter_mut().filter(|p| p.1 > y) {
                p.1 += exp;
            }
            y += exp;
            max_y += exp;
        }
        y += 1;
    }
}
