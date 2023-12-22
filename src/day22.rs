use crate::util::{parse_lines, Point};
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let graph = settle_bricks(&input);
        graph
            .supports
            .iter()
            .filter(|support| {
                !support
                    .iter()
                    .copied()
                    .any(|s| graph.supported_by[s].len() == 1)
            })
            .count()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let graph = settle_bricks(&input);
        (0..graph.supports.len())
            .map(|base| {
                let n = supports(&graph, base);
                log::debug!("{base} -> {n}");
                n
            })
            .sum::<usize>()
            .to_string()
    }
}

fn settle_bricks(input: &str) -> BrickGraph {
    let mut bricks = BrickStack {
        bricks: parse_lines(input).collect(),
    };
    bricks.bricks.sort_by_key(Brick::bottom);
    bricks
        .bricks
        .iter_mut()
        .enumerate()
        .for_each(|(i, b)| b.id = i);
    bricks.settle()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point3(i32, i32, i32);

#[derive(Debug)]
struct Brick {
    id: usize,
    start: Point3,
    end: Point3,
}

impl Brick {
    fn bottom(&self) -> i32 {
        min(self.start.2, self.end.2)
    }

    fn points(&self) -> Vec<Point3> {
        if self.start.0 == self.end.0 && self.start.1 == self.end.1 {
            let range = if self.start.2 < self.end.2 {
                self.start.2..=self.end.2
            } else {
                self.end.2..=self.start.2
            };
            range
                .map(|z| Point3(self.start.0, self.start.1, z))
                .collect()
        } else if self.start.0 == self.end.0 && self.start.2 == self.end.2 {
            let range = if self.start.1 < self.end.1 {
                self.start.1..=self.end.1
            } else {
                self.end.1..=self.start.1
            };
            range
                .map(|y| Point3(self.start.0, y, self.start.2))
                .collect()
        } else {
            let range = if self.start.0 < self.end.0 {
                self.start.0..=self.end.0
            } else {
                self.end.0..=self.start.0
            };
            range
                .map(|x| Point3(x, self.start.1, self.start.2))
                .collect()
        }
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.split_once('~').unwrap();
        let start = {
            let mut iter = start_str.split(',');
            Point3(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        };
        let end = {
            let mut iter = end_str.split(',');
            Point3(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        };
        let id = 0;
        Ok(Brick { id, start, end })
    }
}

struct BrickStack {
    bricks: Vec<Brick>,
}

impl BrickStack {
    fn settle(&mut self) -> BrickGraph {
        let mut graph = BrickGraph::default();
        let mut floors: HashMap<Point, (i32, usize)> = HashMap::new();
        for i in 0..self.bricks.len() {
            let falling = &self.bricks[i];
            log::debug!("dropping brick {i} = {falling:?}");
            let points: HashSet<_> = falling
                .points()
                .into_iter()
                .map(|p| Point(p.0, p.1))
                .collect();

            let floor_bricks = points
                .iter()
                .filter_map(|fall_p| floors.get(fall_p))
                .copied()
                .max_set_by_key(|(h, _)| *h);
            let floor = floor_bricks.first().map(|(h, _)| *h).unwrap_or_default();
            let d = falling.bottom() - floor - 1;

            let falling = &mut self.bricks[i];
            falling.start.2 -= d;
            falling.end.2 -= d;

            let top = max(falling.start.2, falling.end.2);
            for p in points {
                floors.insert(p, (top, i));
            }
            let floor_bricks: HashSet<_> = floor_bricks.into_iter().map(|(_, fb)| fb).collect();
            for fb in &floor_bricks {
                graph.supports[*fb].insert(i);
            }
            graph.supports.push(HashSet::new());
            graph.supported_by.push(floor_bricks);
            log::debug!("dropped: {falling:?}");
        }
        graph
    }
}

#[derive(Default)]
struct BrickGraph {
    supports: Vec<HashSet<usize>>,
    supported_by: Vec<HashSet<usize>>,
}

fn supports(graph: &BrickGraph, start: usize) -> usize {
    let mut candidates = graph.supports[start].clone();
    let mut set = HashSet::new();
    set.insert(start);
    loop {
        let next = candidates
            .iter()
            .copied()
            .filter(|node| {
                let supported_by = &graph.supported_by[*node];
                !set.contains(node)
                    && !supported_by.is_empty()
                    && supported_by.iter().all(|s| set.contains(s))
            })
            .collect_vec();
        if next.is_empty() {
            break;
        }
        for n in &next {
            candidates.remove(n);
            candidates.extend(&graph.supports[*n]);
        }
        set.extend(next);
    }
    set.len() - 1
}
