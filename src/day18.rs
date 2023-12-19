use crate::util::{parse_lines, Dir, Point};
use itertools::Itertools;
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        find_area(parse_lines(&input)).to_string()
    }

    fn solve_2(&self, input: String) -> String {
        find_area(parse_lines(&input).update(Instruction::fix)).to_string()
    }
}

struct Instruction {
    dir: Dir,
    dist: i32,
    color: (i32, Dir),
}

impl Instruction {
    fn fix(&mut self) {
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
    start: Point<i64>,
    end: Point<i64>,
}

fn find_area(instructions: impl Iterator<Item = Instruction>) -> i64 {
    let area = instructions
        .scan(Point::zero(), |start, instr| {
            let end = *start + instr.dir.diff().into() * i64::from(instr.dist);
            let seg = Segment { start: *start, end };
            *start = end;
            Some(seg)
        })
        .map(|seg| {
            if seg.start.0 == seg.end.0 {
                (seg.start.1 - seg.end.1) * -seg.start.0 + (seg.start.1 - seg.end.1).abs()
            } else {
                (seg.start.0 - seg.end.0) * seg.start.1 + (seg.start.0 - seg.end.0).abs()
            }
        })
        .sum::<i64>();

    area / 2 + 1
}
