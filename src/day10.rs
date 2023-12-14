use crate::util::{Grid, Point};
use std::collections::HashSet;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let grid = input.parse().unwrap();
        let d = get_loop_points(&grid).0.len();
        (d / 2).to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let mut grid: Grid<Pipe> = input.parse().unwrap();
        let (loop_points, start_pos) = get_loop_points(&grid);
        for p in grid.points() {
            if !loop_points.contains(&p) {
                grid[p] = Pipe::Ground;
            }
        }
        let start_pipe = Pipe::NE; // TODO generalize
        grid[start_pos] = start_pipe;

        let mut lefts = HashSet::new();
        let mut rights = HashSet::new();

        let mut pos = start_pos;
        let mut dir = Dir::E; // TODO generalize
        loop {
            log::debug!("{pos} {dir:?}");
            let pipe = grid[pos];
            let (left, right) = pipe.left_right(pos, dir);
            flood(&grid, left, &mut lefts);
            flood(&grid, right, &mut rights);
            let (new_pos, new_dir) = pipe.go(pos, dir);
            if new_pos == start_pos {
                break;
            }
            pos = new_pos;
            dir = new_dir;
        }

        // wat - assumes the outside set will contain something on the left or top borders
        let inside = if lefts.iter().any(|p| p.0 == 0 || p.1 == 0) {
            rights
        } else {
            lefts
        };
        inside.len().to_string()
    }
}

fn get_loop_points(grid: &Grid<Pipe>) -> (HashSet<Point>, Point) {
    let start_pos = grid
        .points_with_item()
        .find(|(_p, t)| **t == Pipe::Start)
        .unwrap()
        .0;
    let start_pipe = Pipe::NE; // TODO generalize
    let [mut this, last] = start_pipe.adj(start_pos);
    let mut res = HashSet::new();
    res.insert(start_pos);
    while this != last {
        //log::debug!("this = {this} d = {}", res.len());
        let pipe = grid[this];
        for a in pipe.adj(this) {
            if res.contains(&a) {
                continue;
            } else {
                res.insert(this);
                this = a;
                break;
            }
        }
    }
    res.insert(last);
    (res, start_pos)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Ground,
    Start,
}

impl Pipe {
    fn adj(self, p: Point) -> [Point; 2] {
        match self {
            Pipe::NS => [p + Point(0, -1), p + Point(0, 1)],
            Pipe::EW => [p + Point(1, 0), p + Point(-1, 0)],
            Pipe::NE => [p + Point(0, -1), p + Point(1, 0)],
            Pipe::NW => [p + Point(0, -1), p + Point(-1, 0)],
            Pipe::SE => [p + Point(0, 1), p + Point(1, 0)],
            Pipe::SW => [p + Point(0, 1), p + Point(-1, 0)],
            _ => panic!("{p}"),
        }
    }

    fn left_right(self, p: Point, from: Dir) -> (Vec<Point>, Vec<Point>) {
        let n = p + Point(0, -1);
        let e = p + Point(1, 0);
        let s = p + Point(0, 1);
        let w = p + Point(-1, 0);
        match (self, from) {
            (Pipe::NS, Dir::N) => (vec![e], vec![w]),
            (Pipe::NS, Dir::S) => (vec![w], vec![e]),
            (Pipe::EW, Dir::E) => (vec![s], vec![n]),
            (Pipe::EW, Dir::W) => (vec![n], vec![s]),
            (Pipe::NE, Dir::N) => (vec![], vec![s, w]),
            (Pipe::NE, Dir::E) => (vec![s, w], vec![]),
            (Pipe::NW, Dir::N) => (vec![s, e], vec![]),
            (Pipe::NW, Dir::W) => (vec![], vec![s, e]),
            (Pipe::SE, Dir::S) => (vec![n, w], vec![]),
            (Pipe::SE, Dir::E) => (vec![], vec![n, w]),
            (Pipe::SW, Dir::S) => (vec![], vec![n, e]),
            (Pipe::SW, Dir::W) => (vec![n, e], vec![]),
            _ => panic!("left_right {p} {from:?}"),
        }
    }

    fn go(self, p: Point, from: Dir) -> (Point, Dir) {
        let n = (p + Point(0, -1), Dir::S);
        let e = (p + Point(1, 0), Dir::W);
        let s = (p + Point(0, 1), Dir::N);
        let w = (p + Point(-1, 0), Dir::E);
        match (self, from) {
            (Pipe::NS, Dir::N) => s,
            (Pipe::NS, Dir::S) => n,
            (Pipe::EW, Dir::E) => w,
            (Pipe::EW, Dir::W) => e,
            (Pipe::NE, Dir::N) => e,
            (Pipe::NE, Dir::E) => n,
            (Pipe::NW, Dir::N) => w,
            (Pipe::NW, Dir::W) => n,
            (Pipe::SE, Dir::S) => e,
            (Pipe::SE, Dir::E) => s,
            (Pipe::SW, Dir::S) => w,
            (Pipe::SW, Dir::W) => s,
            _ => panic!("go {p} {from:?}"),
        }
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        use Pipe::*;
        match value {
            '|' => NS,
            '-' => EW,
            'L' => NE,
            'J' => NW,
            'F' => SE,
            '7' => SW,
            '.' => Ground,
            'S' => Start,
            _ => panic!("{value}"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

fn flood(grid: &Grid<Pipe>, start: Vec<Point>, flooded: &mut HashSet<Point>) {
    let mut q = start;
    while let Some(p) = q.pop() {
        if grid.get(p) != Some(&Pipe::Ground) {
            continue;
        }
        if !flooded.insert(p) {
            continue;
        }
        for a in p.adj() {
            q.push(a);
        }
    }
}
