use crate::util::parse_lines;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (workflows, parts) = parse_input(&input);

        parts
            .into_iter()
            .filter(|part| {
                let mut wf = &workflows["in"];
                loop {
                    let dest = wf.process(part);
                    match dest {
                        "A" => return true,
                        "R" => return false,
                        _ => wf = &workflows[dest],
                    }
                }
            })
            .map(|part| part.total_ratings())
            .sum::<i32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let (workflows, _) = parse_input(&input);
        let in_play = [
            1..4001, // x
            1..4001, // m
            1..4001, // a
            1..4001, // s
        ];
        accept_count(&workflows, "in", in_play).to_string()
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (workflow_strs, part_strs) = input.split_once("\n\n").unwrap();
    let workflows = parse_lines::<Workflow>(workflow_strs)
        .map(|wf| (wf.name.clone(), wf))
        .collect();
    let parts = parse_lines(part_strs).collect();
    (workflows, parts)
}

struct Part {
    values: HashMap<char, i32>,
}

impl Part {
    fn total_ratings(&self) -> i32 {
        self.values.values().sum::<i32>()
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .trim_matches(&['{', '}'][..])
            .split(',')
            .map(|kv| {
                let (k, v) = kv.split_once('=').unwrap();
                (k.chars().next().unwrap(), v.parse().unwrap())
            })
            .collect();
        Ok(Part { values })
    }
}

struct Workflow {
    name: String,
    conditions: Vec<Condition>,
    final_dest: String,
}

impl Workflow {
    fn process(&self, part: &Part) -> &str {
        for cond in &self.conditions {
            if cond.accepts(part) {
                return &cond.dest;
            }
        }
        &self.final_dest
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static COND_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w)([><])(\d+):(\w+)").unwrap());

        let (name_str, conditions_str) = s.trim_end_matches('}').split_once('{').unwrap();
        let name = name_str.to_string();
        let conditions_raw: Vec<_> = conditions_str.split(',').collect();
        let conditions = conditions_raw[..conditions_raw.len() - 1]
            .iter()
            .map(|cond_str| {
                let m = COND_RE.captures(cond_str).unwrap();
                Condition {
                    var: m[1].chars().next().unwrap(),
                    op: match &m[2] {
                        ">" => Op::Greater,
                        "<" => Op::Less,
                        _ => unreachable!(),
                    },
                    test: m[3].parse().unwrap(),
                    dest: m[4].to_string(),
                }
            })
            .collect();
        let final_dest = conditions_raw[conditions_raw.len() - 1].to_string();
        Ok(Workflow {
            name,
            conditions,
            final_dest,
        })
    }
}

struct Condition {
    var: char,
    op: Op,
    test: i32,
    dest: String,
}

impl Condition {
    fn accepts(&self, part: &Part) -> bool {
        self.op.compare(part.values[&self.var], self.test)
    }
}

enum Op {
    Greater,
    Less,
}

impl Op {
    fn compare(&self, value: i32, test: i32) -> bool {
        match self {
            Op::Greater => value > test,
            Op::Less => value < test,
        }
    }
}

fn accept_count(
    workflows: &HashMap<String, Workflow>,
    name: &str,
    mut in_play: [Range<i64>; 4],
) -> i64 {
    let wf = &workflows[name];
    let mut total = 0;

    for cond in &wf.conditions {
        let mut matches = in_play.clone();
        let ipi = ['x', 'm', 'a', 's']
            .iter()
            .position(|c| c == &cond.var)
            .unwrap();
        matches[ipi] = match cond.op {
            Op::Greater => cut_top(&mut in_play[ipi], cond.test),
            Op::Less => cut_bottom(&mut in_play[ipi], cond.test),
        };
        total += match cond.dest.as_str() {
            "A" => count(&matches),
            "R" => 0,
            dep => accept_count(workflows, dep, matches),
        }
    }

    total += match wf.final_dest.as_str() {
        "A" => count(&in_play),
        "R" => 0,
        final_dest => accept_count(workflows, final_dest, in_play),
    };

    log::debug!("{name} -> {total}");
    total
}

fn count(ranges: &[Range<i64>]) -> i64 {
    ranges.iter().map(|r| r.end - r.start).product::<i64>()
}

// cuts the top off a range, returning the range that it cut
fn cut_top(range: &mut Range<i64>, v: i32) -> Range<i64> {
    let v = i64::from(v) + 1;
    if range.start > v {
        let old = range.clone();
        range.end = range.start;
        old
    } else if range.end > v {
        let old_end = range.end;
        range.end = v;
        v..old_end
    } else {
        v..v
    }
}

fn cut_bottom(range: &mut Range<i64>, v: i32) -> Range<i64> {
    let v = i64::from(v);
    if range.end < v {
        let old = range.clone();
        range.start = range.end;
        old
    } else if range.start < v {
        let old_start = range.start;
        range.start = v;
        old_start..v
    } else {
        v..v
    }
}
