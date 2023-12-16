use crate::util::{Grid, Point};
use std::collections::HashSet;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let map = input.parse().unwrap();
        energy(trace_light(&map, Point(0, 0), Dir::E)).to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let map: Grid<Tile> = input.parse().unwrap();
        let w = map.width();
        let h = map.height();
        let top = (0..w).map(|x| (Point(x as i32, 0), Dir::S));
        let bot = (0..w).map(|x| (Point(x as i32, (h as i32) - 1), Dir::N));
        let lft = (0..h).map(|y| (Point(0, y as i32), Dir::E));
        let rgt = (0..h).map(|y| (Point((h as i32) - 1, y as i32), Dir::W));
        top.chain(bot)
            .chain(lft)
            .chain(rgt)
            .map(|(start_pos, start_dir)| energy(trace_light(&map, start_pos, start_dir)))
            .max()
            .unwrap()
            .to_string()
    }
}

enum Tile {
    Empty,
    MirrorDown, // \
    MirrorUp,   // /
    SplitterH,  // -
    SplitterV,  // |
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '\\' => Tile::MirrorDown,
            '/' => Tile::MirrorUp,
            '-' => Tile::SplitterH,
            '|' => Tile::SplitterV,
            _ => Tile::Empty,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn diff(&self) -> Point {
        match self {
            Dir::N => Point(0, -1),
            Dir::E => Point(1, 0),
            Dir::S => Point(0, 1),
            Dir::W => Point(-1, 0),
        }
    }
}

fn trace_light(map: &Grid<Tile>, start_pos: Point, start_dir: Dir) -> Grid<HashSet<Dir>> {
    let mut trace = Grid::build(map.width(), map.height(), |_| HashSet::new());
    let mut q = vec![(start_pos, start_dir)];

    while let Some((p, current_dir)) = q.pop() {
        let Some(tile) = map.get(p) else {
            continue;
        };
        if !trace[p].insert(current_dir) {
            continue;
        }
        match tile {
            Tile::Empty => {
                q.push((p + current_dir.diff(), current_dir));
            }
            Tile::MirrorDown => {
                let new_dir = match current_dir {
                    Dir::N => Dir::W,
                    Dir::E => Dir::S,
                    Dir::S => Dir::E,
                    Dir::W => Dir::N,
                };
                q.push((p + new_dir.diff(), new_dir));
            }
            Tile::MirrorUp => {
                let new_dir = match current_dir {
                    Dir::N => Dir::E,
                    Dir::E => Dir::N,
                    Dir::S => Dir::W,
                    Dir::W => Dir::S,
                };
                q.push((p + new_dir.diff(), new_dir));
            }
            Tile::SplitterH => match current_dir {
                Dir::N | Dir::S => {
                    q.push((p + Dir::E.diff(), Dir::E));
                    q.push((p + Dir::W.diff(), Dir::W));
                }
                _ => {
                    q.push((p + current_dir.diff(), current_dir));
                }
            },
            Tile::SplitterV => match current_dir {
                Dir::E | Dir::W => {
                    q.push((p + Dir::N.diff(), Dir::N));
                    q.push((p + Dir::S.diff(), Dir::S));
                }
                _ => {
                    q.push((p + current_dir.diff(), current_dir));
                }
            },
        }
    }

    trace
}

fn energy(trace: Grid<HashSet<Dir>>) -> usize {
    trace
        .points_with_item()
        .filter(|(_, set)| !set.is_empty())
        .count()
}
