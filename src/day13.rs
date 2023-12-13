use crate::util::{Grid, Point};
use std::collections::HashSet;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        solve(input, 0)
    }

    fn solve_2(&self, input: String) -> String {
        solve(input, 1)
    }
}

fn solve(input: String, target_diff: usize) -> String {
    parse_input(&input)
        .map(|ps| {
            let (orient, val) = find_mirror(&ps, target_diff);
            match orient {
                Orientation::Vertical => val,
                Orientation::Horizontal => val * 100,
            }
        })
        .sum::<i32>()
        .to_string()
}

fn parse_input(s: &str) -> impl Iterator<Item = HashSet<Point>> + '_ {
    s.split("\n\n").map(|map_str| {
        map_str
            .parse::<Grid<char>>()
            .unwrap()
            .points_with_item()
            .filter(|(_, c)| c == &&'#')
            .map(|(p, _)| p)
            .collect()
    })
}

fn find_mirror(points: &HashSet<Point>, target_diff: usize) -> (Orientation, i32) {
    let target = points.len() - target_diff;
    {
        let max_x = points.iter().map(|p| p.0).max().unwrap();
        for x in 1..=max_x {
            let mirror_points = points
                .iter()
                .filter(|p| {
                    let d = x - p.0;
                    let mx = x + d - 1;
                    mx < 0 || mx > max_x || points.contains(&Point(mx, p.1))
                })
                .count();
            if mirror_points == target {
                return (Orientation::Vertical, x);
            }
        }
    }

    {
        let max_y = points.iter().map(|p| p.1).max().unwrap();
        for y in 1..=max_y {
            let mirror_points = points
                .iter()
                .filter(|p| {
                    let d = y - p.1;
                    let my = y + d - 1;
                    my < 0 || my > max_y || points.contains(&Point(p.0, my))
                })
                .count();
            if mirror_points == target {
                return (Orientation::Horizontal, y);
            }
        }
    }

    unreachable!()
}

enum Orientation {
    Vertical,
    Horizontal,
}
