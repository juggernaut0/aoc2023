use crate::util::{Dir, Grid, Point};
use itertools::Itertools;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let maze = Maze::new(input.parse().unwrap(), false);
        log::debug!("nodes: {:#?}", maze.nodes);
        log::debug!("matrix: {:#?}", maze.matrix);
        maze.search(0, &mut vec![false; maze.nodes.len()])
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let maze = Maze::new(input.parse().unwrap(), true);
        log::info!("{} nodes", maze.nodes.len());
        log::debug!("matrix: {:#?}", maze.matrix);
        maze.search(0, &mut vec![false; maze.nodes.len()])
            .to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open,
    Wall,
    Slope(Dir),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Open,
            '#' => Tile::Wall,
            '>' => Tile::Slope(Dir::E),
            'v' => Tile::Slope(Dir::S),
            _ => panic!("unknown tile {value}"),
        }
    }
}

struct Maze {
    nodes: Vec<Point>,
    matrix: Vec<Vec<Option<u32>>>,
}

impl Maze {
    fn new(mut map: Grid<Tile>, ignore_slopes: bool) -> Maze {
        let start = Point(1, 0);
        let goal = Point::of(map.width() - 2, map.height() - 1);

        // hack
        map[start] = Tile::Slope(Dir::S);

        let nodes = {
            let mut nodes = map
                .points()
                .filter(|p| {
                    map[*p] == Tile::Open
                        && Dir::values()
                            .into_iter()
                            .filter(|d| map.get(*p + d.diff()) != Some(&Tile::Wall))
                            .count()
                            > 2
                })
                .collect_vec();
            nodes.insert(0, start); // start
            nodes.push(goal); // goal
            nodes
        };

        if ignore_slopes {
            // make all slopes face "outward" from nodes. This simplifies the walk function
            for node in &nodes {
                if let Some(Tile::Slope(d)) = map.get_mut(*node + Dir::N.diff()) {
                    *d = Dir::N;
                }
                if let Some(Tile::Slope(d)) = map.get_mut(*node + Dir::W.diff()) {
                    *d = Dir::W;
                }
            }
        }

        let mut matrix = vec![vec![None; nodes.len()]; nodes.len()];
        {
            let (distance, dest) = walk(&map, &nodes, start);
            let dest_i = nodes
                .iter()
                .position(|n| n == &dest)
                .expect("found a dest that wasn't a node");
            matrix[0][dest_i] = Some(distance - 1);
        }
        for (node_i, node) in nodes.iter().copied().enumerate() {
            if node == start || node == goal {
                continue;
            }
            let e = node + Dir::E.diff();
            let s = node + Dir::S.diff();
            if let Tile::Slope(_) = map[e] {
                let (distance, dest) = walk(&map, &nodes, e);
                let dest_i = nodes
                    .iter()
                    .position(|n| n == &dest)
                    .expect("found a dest that wasn't a node");
                matrix[node_i][dest_i] = Some(distance);
            }
            if let Tile::Slope(_) = map[s] {
                let (distance, dest) = walk(&map, &nodes, s);
                let dest_i = nodes
                    .iter()
                    .position(|n| n == &dest)
                    .expect("found a dest that wasn't a node");
                matrix[node_i][dest_i] = Some(distance);
            }
            if ignore_slopes {
                let n = node + Dir::N.diff();
                let w = node + Dir::W.diff();
                if let Tile::Slope(_) = map[n] {
                    let (distance, dest) = walk(&map, &nodes, n);
                    let dest_i = nodes
                        .iter()
                        .position(|n| n == &dest)
                        .expect("found a dest that wasn't a node");
                    matrix[node_i][dest_i] = Some(distance);
                }
                if let Tile::Slope(_) = map[w] {
                    let (distance, dest) = walk(&map, &nodes, w);
                    let dest_i = nodes
                        .iter()
                        .position(|n| n == &dest)
                        .expect("found a dest that wasn't a node");
                    matrix[node_i][dest_i] = Some(distance);
                }
            }
        }
        Maze { nodes, matrix }
    }

    fn search(&self, node: usize, visited: &mut [bool]) -> u32 {
        if node == self.nodes.len() - 1 {
            return 0;
        }

        visited[node] = true;

        let res = self.matrix[node]
            .iter()
            .enumerate()
            .filter_map(|(i, it)| {
                it.filter(|_| !visited[i])
                    .map(|dist| self.search(i, visited) + dist)
            })
            .max()
            .unwrap_or_default();

        visited[node] = false;

        res
    }
}

fn walk(map: &Grid<Tile>, nodes: &[Point], start: Point) -> (u32, Point) {
    let Tile::Slope(start_dir) = map[start] else {
        panic!("not starting on slope");
    };
    let mut res = 2;
    let mut last = start;
    let mut current = start + start_dir.diff();
    let dest = loop {
        let next = Dir::values()
            .into_iter()
            .map(|d| current + d.diff())
            .find(|p| map[*p] != Tile::Wall && p != &last)
            .expect("expected to continue");
        res += 1;

        if nodes.contains(&next) {
            break next;
        }

        last = current;
        current = next;
    };
    (res, dest)
}
