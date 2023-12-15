use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        input.trim().split(',').map(hash).sum::<usize>().to_string()
    }

    fn solve_2(&self, input: String) -> String {
        const EMPTY_VEC: Vec<Lens> = Vec::new();
        let mut boxes = [EMPTY_VEC; 256];

        for instr in input.split(',').map(|s| s.parse::<Instruction>().unwrap()) {
            let label = instr.label;
            let hash = hash(&label);
            let maybe_pos = boxes[hash].iter().position(|l| l.label == label);

            match instr.typ {
                InstructionType::Remove => {
                    if let Some(pos) = maybe_pos {
                        boxes[hash].remove(pos);
                    }
                }
                InstructionType::Insert(focal) => {
                    if let Some(pos) = maybe_pos {
                        boxes[hash][pos].focal = focal;
                    } else {
                        boxes[hash].push(Lens { label, focal });
                    }
                }
            }
        }

        boxes
            .into_iter()
            .enumerate()
            .map(|(bi, b)| {
                b.into_iter()
                    .enumerate()
                    .map(|(li, l)| (bi + 1) * (li + 1) * (l.focal as usize))
                    .sum::<usize>()
            })
            .sum::<usize>()
            .to_string()
    }
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| (acc + (c as usize)) * 17 % 256)
}

struct Instruction {
    label: String,
    typ: InstructionType,
}

enum InstructionType {
    Remove,
    Insert(u8),
}

struct Lens {
    label: String,
    focal: u8,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr = if s.ends_with(|c: char| c.is_ascii_digit()) {
            let (label, focal_str) = s.split_once('=').unwrap();
            Instruction {
                label: label.to_string(),
                typ: InstructionType::Insert(focal_str.parse().unwrap()),
            }
        } else {
            Instruction {
                label: s.trim_end_matches('-').to_string(),
                typ: InstructionType::Remove,
            }
        };
        Ok(instr)
    }
}
