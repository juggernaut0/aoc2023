use crate::util::{parse_lines, Point};
use itertools::Itertools;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let bricks = settle_bricks(&input);
        bricks
            .bricks
            .iter()
            .filter(|brick| {
                // check for any bricks above that have only a single brick below
                for above in bricks.bricks_above(brick) {
                    if bricks.bricks_below(above).len() == 1 {
                        return false;
                    }
                }
                true
            })
            .count()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let bricks = settle_bricks(&input);
        let dag = bricks.build_dag();
        dag.iter()
            .map(|node| {
                let n = supports(&dag, node.id);
                log::debug!("{} -> {n}", node.id);
                n
            })
            .sum::<usize>()
            .to_string()
    }
}

fn settle_bricks(input: &str) -> BrickStack {
    let mut bricks = BrickStack {
        bricks: parse_lines(input).collect(),
    };
    bricks.bricks.sort_by_key(Brick::bottom);
    bricks
        .bricks
        .iter_mut()
        .enumerate()
        .for_each(|(i, b)| b.id = i);
    bricks.settle();
    log::info!("bricks are settled");
    bricks
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

    fn contains(&self, p: Point3) -> bool {
        self.points().contains(&p)
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

    fn points_below(&self) -> Vec<Point3> {
        self.points()
            .into_iter()
            .map(|p| Point3(p.0, p.1, p.2 - 1))
            .filter(|p| !self.contains(*p))
            .collect()
    }

    fn points_above(&self) -> Vec<Point3> {
        self.points()
            .into_iter()
            .map(|p| Point3(p.0, p.1, p.2 + 1))
            .filter(|p| !self.contains(*p))
            .collect()
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
    fn get(&self, p: Point3) -> Option<&Brick> {
        self.bricks.iter().find(|brick| brick.contains(p))
    }

    fn settle(&mut self) {
        for i in 0..self.bricks.len() {
            let falling = &self.bricks[i];
            log::debug!("dropping brick {i} = {falling:?}");
            let points: HashSet<_> = falling
                .points()
                .into_iter()
                .map(|p| Point(p.0, p.1))
                .collect();

            let floor = points
                .into_iter()
                .map(|fall_p| {
                    self.bricks[0..i]
                        .iter()
                        .rev()
                        .flat_map(|b| {
                            b.points().into_iter().filter_map(|p| {
                                if fall_p == Point(p.0, p.1) {
                                    Some(p.2)
                                } else {
                                    None
                                }
                            })
                        })
                        .max()
                        .unwrap_or_default()
                })
                .max()
                .unwrap_or_default();
            let d = falling.bottom() - floor - 1;

            let falling = &mut self.bricks[i];
            falling.start.2 -= d;
            falling.end.2 -= d;
            log::debug!("dropped: {falling:?}");
        }
    }

    fn bricks_above(&self, b: &Brick) -> Vec<&Brick> {
        b.points_above()
            .into_iter()
            .filter_map(|a| self.get(a))
            .unique_by(|b| b.id)
            .collect()
    }

    fn bricks_below(&self, b: &Brick) -> Vec<&Brick> {
        b.points_below()
            .into_iter()
            .filter_map(|a| self.get(a))
            .unique_by(|b| b.id)
            .collect()
    }

    fn build_dag(&self) -> Vec<Rc<DagNode>> {
        let rcs = self
            .bricks
            .iter()
            .map(|b| {
                Rc::new(DagNode {
                    id: b.id,
                    supported_by: RefCell::new(Vec::new()),
                })
            })
            .collect_vec();
        for brick in &self.bricks {
            let supported_by = self
                .bricks_below(brick)
                .into_iter()
                .map(|a| Rc::clone(&rcs[a.id]))
                .collect_vec();
            rcs[brick.id].supported_by.replace(supported_by);
        }
        log::info!("dag built");
        rcs
    }
}

struct DagNode {
    id: usize,
    supported_by: RefCell<Vec<Rc<DagNode>>>,
}

fn supports(dag: &[Rc<DagNode>], start: usize) -> usize {
    let mut set = HashSet::new();
    set.insert(start);
    loop {
        let Some(next) = dag.iter().find(|node| {
            let supported_by = node.supported_by.borrow();
            !set.contains(&node.id)
                && supported_by.len() > 0
                && supported_by.iter().all(|s| set.contains(&s.id))
        }) else {
            break;
        };
        set.insert(next.id);
    }
    set.len() - 1
}
