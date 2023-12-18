use crate::util::{parse_lines, Dir, Point};
use itertools::Itertools;
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        find_area(parse_lines(&input)).to_string()
    }

    fn solve_2(&self, input: String) -> String {
        find_area(parse_lines(&input).update(Instruction::fix_dists)).to_string()
    }
}

struct Instruction {
    dir: Dir,
    dist: i32,
    color: (i32, Dir),
}

impl Instruction {
    fn fix_dists(&mut self) {
        self.dist = self.color.0;
        self.dir = self.color.1;
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let dir_str = parts.next().unwrap();
        let dir = match dir_str {
            "U" => Dir::N,
            "D" => Dir::S,
            "L" => Dir::W,
            "R" => Dir::E,
            _ => panic!("what is this {dir_str}"),
        };
        let dist = parts.next().unwrap().parse().unwrap();
        let color_str = parts.next().unwrap().trim_matches(&['#', '(', ')'][..]);
        let color_dist = i32::from_str_radix(&color_str[0..5], 16).unwrap();
        let color_dir = match &color_str[5..6] {
            "0" => Dir::E,
            "1" => Dir::S,
            "2" => Dir::W,
            "3" => Dir::N,
            _ => panic!("srsly what is this"),
        };
        Ok(Instruction {
            dir,
            dist,
            color: (color_dist, color_dir),
        })
    }
}

struct Segment {
    start: Point,
    end: Point,
}

fn find_border(instructions: impl Iterator<Item = Instruction>) -> Vec<Segment> {
    let mut res = Vec::new();
    let mut current = Point::zero();

    for instr in instructions {
        let end = current + instr.dir.diff() * instr.dist;
        res.push(Segment {
            start: current,
            end,
        });
        current = end;
    }

    res
}

fn find_area(instructions: impl Iterator<Item = Instruction>) -> i64 {
    let border = find_border(instructions);

    let (min_x, max_x) = border
        .iter()
        .flat_map(|seg| [seg.start.0, seg.end.0])
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = border
        .iter()
        .flat_map(|seg| [seg.start.1, seg.end.1])
        .minmax()
        .into_option()
        .unwrap();
    let center = Point((min_x + max_x) / 2, (min_y + max_y) / 2);

    let mut area = 0;
    let mut border_area = 0; // only counts bottom and left borders
    for seg in border {
        let (base, height) = if seg.start.0 == seg.end.0 {
            (
                i64::from(seg.start.1 - seg.end.1),
                i64::from(center.0 - seg.start.0),
            )
        } else {
            (
                i64::from(seg.start.0 - seg.end.0),
                i64::from(seg.start.1 - center.1),
            )
        };
        log::debug!("base {base} height {height}");
        area += base * height;

        if seg.start.0 == seg.end.0 && seg.start.1 > seg.end.1 {
            // left border
            border_area += i64::from(seg.start.1 - seg.end.1);
        } else if seg.start.1 == seg.end.1 && seg.start.0 > seg.end.0 {
            // bottom border
            border_area += i64::from(seg.start.0 - seg.end.0);
        }
    }

    // the 1 is for the center I think
    area / 2 + border_area + 1
}
