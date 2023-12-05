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
        let (seeds, mut maps) = parse_input(input);

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
        seed_map.entries.sort_by_key(|it| it.source.start);

        let m6 = maps.pop().unwrap();
        let mut m5 = maps.pop().unwrap();
        let mut m4 = maps.pop().unwrap();
        let mut m3 = maps.pop().unwrap();
        let mut m2 = maps.pop().unwrap();
        let mut m1 = maps.pop().unwrap();
        let mut m0 = maps.pop().unwrap();

        assert!(maps.is_empty(), "map isn't empty!");

        split_map(&mut m5, &m6);
        split_map(&mut m4, &m5);
        split_map(&mut m3, &m4);
        split_map(&mut m2, &m3);
        split_map(&mut m1, &m2);
        split_map(&mut m0, &m1);
        split_map(&mut seed_map, &m0);

        maps = vec![m0, m1, m2, m3, m4, m5, m6];

        seed_map
            .entries
            .into_iter()
            .map(|e| e.dest_start)
            .map(|seed| maps.iter().fold(seed, |n, map| map.apply(n)))
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

#[derive(Default, Debug)]
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
    let mut res = Map::default();

    let mut entries = std::mem::take(&mut earlier.entries).into_iter();
    let mut entry = entries.next();

    while let Some(remaining) = entry {
        let containing = later
            .entries
            .iter()
            .find(|later_entry| later_entry.source.contains(&remaining.dest_start));
        if let Some(containing) = containing {
            if remaining.dest_end() <= containing.source.end {
                res.entries.push(remaining);
                entry = entries.next();
                continue;
            }

            log::debug!(
                "cutting {}..{} at {}",
                remaining.dest_start,
                remaining.dest_end(),
                containing.source.end
            );
            let d = containing.source.end - remaining.dest_start;
            let prefix = MapEntry {
                source: remaining.source.start..(remaining.source.start + d),
                dest_start: remaining.dest_start,
            };
            res.entries.push(prefix);
            entry = Some(MapEntry {
                source: (remaining.source.start + d)..remaining.source.end,
                dest_start: remaining.dest_start + d,
            });
        } else {
            log::debug!("{} was not in a range", remaining.dest_start);

            let first_later = match later.entries.iter().find(|e| {
                e.source.start > remaining.dest_start && e.source.start < remaining.dest_end()
            }) {
                Some(e) => e,
                None => {
                    res.entries.push(remaining);
                    entry = entries.next();
                    continue;
                }
            };

            log::debug!(
                "cutting {}..{} at {}",
                remaining.dest_start,
                remaining.dest_end(),
                first_later.source.start
            );

            let d = first_later.source.start - remaining.dest_start;
            let prefix = MapEntry {
                source: remaining.source.start..(remaining.source.start + d),
                dest_start: remaining.dest_start,
            };
            res.entries.push(prefix);
            entry = Some(MapEntry {
                source: (remaining.source.start + d)..remaining.source.end,
                dest_start: remaining.dest_start + d,
            })
        }
    }
    res.entries.sort_by_key(|it| it.source.start);

    *earlier = res
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
