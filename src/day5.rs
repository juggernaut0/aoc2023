use std::cell::RefCell;
use std::ops::Range;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (seeds, maps) = parse_input(input);
        seeds
            .into_iter()
            .map(|seed| maps.iter().fold(seed, |n, map| map.apply(n)))
            .min()
            .unwrap()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let (seeds, maps) = parse_input(input);
        let maps: Vec<_> = maps.into_iter().map(RefCell::new).collect();

        let seed_map_entries = seeds
            .chunks(2)
            .map(|it| MapEntry {
                source: it[0]..(it[0] + it[1]),
                dest_start: it[0],
            })
            .collect();
        let mut seed_map = Map {
            entries: seed_map_entries,
        };

        for wind in maps.windows(2).rev() {
            split_map(&mut wind[0].borrow_mut(), &wind[1].borrow());
        }
        split_map(&mut seed_map, &maps[0].borrow());

        seed_map
            .entries
            .into_iter()
            .map(|e| e.dest_start)
            .map(|seed| maps.iter().fold(seed, |n, map| map.borrow().apply(n)))
            .min()
            .unwrap()
            .to_string()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct MapEntry {
    source: Range<u64>,
    dest_start: u64,
}

impl MapEntry {
    fn dest_end(&self) -> u64 {
        self.dest_start + (self.source.end - self.source.start)
    }
}

#[derive(Debug)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn apply(&self, n: u64) -> u64 {
        for entry in &self.entries {
            if entry.source.contains(&n) {
                let d = n - entry.source.start;
                return entry.dest_start + d;
            }
        }
        n
    }

    fn new(mut entries: Vec<MapEntry>) -> Map {
        entries.sort_by_key(|it| it.source.start);

        // fill holes
        let mut new_entries = Vec::new();
        let mut last = None;
        for e in entries {
            if let Some(last) = last {
                if e.source.start != last {
                    new_entries.push(MapEntry {
                        source: last..e.source.start,
                        dest_start: last,
                    })
                }
            }
            last = Some(e.source.end);
            new_entries.push(e);
        }
        entries = new_entries;

        log::debug!("{entries:#?}");

        Map { entries }
    }
}

fn parse_input(input: String) -> (Vec<u64>, Vec<Map>) {
    let mut lines = input.lines();
    let seeds_str = &lines.next().unwrap()[7..];
    let seeds = seeds_str
        .split_ascii_whitespace()
        .map(|it| it.parse().unwrap())
        .collect();

    lines.next().unwrap();

    let mut maps = Vec::new();
    let mut current = Vec::new();
    for line in lines {
        if line.is_empty() {
            maps.push(Map::new(current));
            current = Vec::new();
            continue;
        }

        if line.starts_with(|c: char| c.is_alphabetic()) {
            continue;
        }

        let mut parts = line.split_ascii_whitespace().map(|s| s.parse().unwrap());
        let dest_start = parts.next().unwrap();
        let source_start = parts.next().unwrap();
        let len = parts.next().unwrap();
        let entry = MapEntry {
            source: source_start..(source_start + len),
            dest_start,
        };
        current.push(entry);
    }
    maps.push(Map::new(current));
    (seeds, maps)
}

/**
Modifies earlier but with entries split such that every entry in the
earlier map corresponds to exactly one entry in the later map
*/
fn split_map(earlier: &mut Map, later: &Map) {
    // maps are sorted by source start
    let cut_points: Vec<_> = later
        .entries
        .iter()
        .map(|le| le.source.start)
        .chain([later.entries.last().unwrap().source.end])
        .collect();

    let mut res = Vec::new();

    let mut entries = std::mem::take(&mut earlier.entries);

    while let Some(entry) = entries.pop() {
        let cutter = cut_points
            .iter()
            .find(|c| ((entry.dest_start + 1)..entry.dest_end()).contains(c));
        if let Some(cutter) = cutter {
            log::debug!(
                "cutting {}..{} at {}",
                entry.dest_start,
                entry.dest_end(),
                cutter,
            );
            let d = cutter - entry.dest_start;
            let prefix = MapEntry {
                source: entry.source.start..(entry.source.start + d),
                dest_start: entry.dest_start,
            };
            let suffix = MapEntry {
                source: (entry.source.start + d)..entry.source.end,
                dest_start: entry.dest_start + d,
            };
            entries.push(prefix);
            entries.push(suffix);
        } else {
            log::debug!("{} was not in a range", entry.dest_start);

            res.push(entry);
        }
    }

    #[cfg(debug_assertions)]
    {
        for e in &res {
            let a = later
                .entries
                .iter()
                .find(|it| it.source.contains(&e.dest_start));
            let b = later
                .entries
                .iter()
                .find(|it| it.source.contains(&(e.dest_end() - 1)));

            assert_eq!(
                a,
                b,
                "ds {} de {} later {:#?}",
                e.dest_start,
                e.dest_end(),
                later.entries
            );
            assert_eq!(later.apply(e.dest_start + 1), later.apply(e.dest_start) + 1);
        }
    }

    *earlier = Map::new(res);
}

#[cfg(test)]
mod test {
    use crate::day5::{split_map, Map, MapEntry};

    #[test]
    fn overlap() {
        let mut earlier = Map {
            entries: vec![
                MapEntry {
                    source: 0..69,
                    dest_start: 10,
                },
                MapEntry {
                    source: 69..79,
                    dest_start: 0,
                },
            ],
        };
        let later = Map {
            entries: vec![
                MapEntry {
                    source: 56..93,
                    dest_start: 60,
                },
                MapEntry {
                    source: 93..97,
                    dest_start: 56,
                },
            ],
        };

        split_map(&mut earlier, &later);

        assert_eq!(
            vec![
                MapEntry {
                    source: 0..46,
                    dest_start: 10,
                },
                MapEntry {
                    source: 46..69,
                    dest_start: 56,
                },
                MapEntry {
                    source: 69..79,
                    dest_start: 0,
                },
            ],
            earlier.entries,
        );
    }
}
