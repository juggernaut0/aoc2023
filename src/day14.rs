use crate::util::{Grid, Point};
use std::collections::HashMap;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let mut map: Grid<Tile> = input.parse().unwrap();
        full_tilt(&mut map, Point(0, -1));
        load(&map).to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let mut map: Grid<Tile> = input.parse().unwrap();
        let mut seen: HashMap<u32, u32> = HashMap::new();
        let mut cycle_start = 0;
        let mut cycle_length = 0;
        for i in 1.. {
            spin_cycle(&mut map);
            let key = key(&map);
            if let Some(old) = seen.insert(key, i) {
                log::debug!("duplicate {key} at {i}, prev at {old}");
                cycle_start = old;
                cycle_length = i - old;
                break;
            }
        }
        let rem = (1_000_000_000 - cycle_start) % cycle_length;
        for _ in 0..rem {
            spin_cycle(&mut map);
        }
        load(&map).to_string()
    }
}

enum Tile {
    Empty,
    Rock,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            'O' => Tile::Rock,
            '#' => Tile::Wall,
            _ => panic!("unknown tile {value}"),
        }
    }
}

fn load(map: &Grid<Tile>) -> u32 {
    map.points_with_item()
        .map(|(p, t)| {
            if let Tile::Rock = t {
                100 - (p.1 as u32)
            } else {
                0
            }
        })
        .sum()
}

fn key(map: &Grid<Tile>) -> u32 {
    map.points_with_item()
        .map(|(p, t)| {
            if let Tile::Rock = t {
                (p.0 + 1000 * p.1) as u32
            } else {
                0
            }
        })
        .sum()
}

fn tilt(map: &mut Grid<Tile>, dir: Point) -> bool {
    let mut tilted = false;
    for p in map.points() {
        if let Tile::Rock = map[p] {
            let to = p + dir;
            if let Some(Tile::Empty) = map.get(to) {
                map[p] = Tile::Empty;
                map[to] = Tile::Rock;
                tilted = true;
            }
        }
    }
    tilted
}

fn full_tilt(map: &mut Grid<Tile>, dir: Point) {
    while tilt(map, dir) {}
}

fn spin_cycle(map: &mut Grid<Tile>) {
    full_tilt(map, Point(0, -1));
    full_tilt(map, Point(-1, 0));
    full_tilt(map, Point(0, 1));
    full_tilt(map, Point(1, 0));
}
