use crate::util::{Grid, Point};

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let map = parse_input(&input);

        let open_counts = open_counts(&map);
        open_counts.center.to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let map = parse_input(&input);

        let open_counts = open_counts(&map);
        let steps = 26_501_365;
        let si = u64::try_from(map.width()).unwrap();
        let chunks = (steps - (si - 1) / 2) / si; // how many axial chunks
        log::info!("chunks {chunks}");

        let full_even_count = chunks * chunks;
        let full_odd_count = (chunks - 1) * (chunks - 1);
        let total = full_even_count * open_counts.full_even
            + full_odd_count * open_counts.full_odd
            + open_counts.n
            + open_counts.e
            + open_counts.s
            + open_counts.w
            + chunks * open_counts.ne_small
            + (chunks - 1) * open_counts.ne_big
            + chunks * open_counts.se_small
            + (chunks - 1) * open_counts.se_big
            + chunks * open_counts.sw_small
            + (chunks - 1) * open_counts.sw_big
            + chunks * open_counts.nw_small
            + (chunks - 1) * open_counts.nw_big;

        total.to_string()
    }
}

fn parse_input(input: &str) -> Grid<Tile> {
    let mut map: Grid<Tile> = input.parse().unwrap();
    let start = map
        .points_with_item()
        .find_map(|(p, t)| if let Tile::Start = t { Some(p) } else { None })
        .unwrap();
    map[start] = Tile::Open;
    map
}

enum Tile {
    Open,
    Rock,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Open,
            '#' => Tile::Rock,
            'S' => Tile::Start,
            _ => panic!("{value}"),
        }
    }
}

#[derive(Default, Debug)]
struct OpenCounts {
    center: u64,
    full_even: u64,
    full_odd: u64,
    n: u64,
    e: u64,
    s: u64,
    w: u64,
    ne_small: u64,
    ne_big: u64,
    se_small: u64,
    se_big: u64,
    sw_small: u64,
    sw_big: u64,
    nw_small: u64,
    nw_big: u64,
}

fn open_counts(map: &Grid<Tile>) -> OpenCounts {
    assert_eq!(map.width(), map.height());
    let size = map.height() - 1;
    let hs = size / 2;
    let mut counts = OpenCounts::default();
    for (p, tile) in map.points_with_item() {
        if let Tile::Rock = tile {
            continue;
        }
        if p.adj()
            .into_iter()
            .all(|a| matches!(map.get(a), Some(Tile::Rock)))
        {
            continue;
        }
        let tests = [
            (Point(hs, hs), 64, 0, &mut counts.center),
            (Point(hs, hs), size + 1, 0, &mut counts.full_even),
            (Point(hs, hs), size + 1, 1, &mut counts.full_odd),
            (Point(hs, size), size, 1, &mut counts.n),
            (Point(0, hs), size, 1, &mut counts.e),
            (Point(hs, 0), size, 1, &mut counts.s),
            (Point(size, hs), size, 1, &mut counts.w),
            (Point(0, size), hs, 0, &mut counts.ne_small),
            (Point(0, size), size + hs, 1, &mut counts.ne_big),
            (Point(0, 0), hs, 0, &mut counts.se_small),
            (Point(0, 0), size + hs, 1, &mut counts.se_big),
            (Point(size, 0), hs, 0, &mut counts.sw_small),
            (Point(size, 0), size + hs, 1, &mut counts.sw_big),
            (Point(size, size), hs, 0, &mut counts.nw_small),
            (Point(size, size), size + hs, 1, &mut counts.nw_big),
        ];
        for (start, dist, eo, dest) in tests {
            if (p.0 + p.1) % 2 == eo && start.l1dist(p) <= dist {
                *dest += 1;
            }
        }
    }
    counts
}
