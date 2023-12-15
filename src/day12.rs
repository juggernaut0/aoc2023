use crate::util::parse_lines;
use itertools::Itertools;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        parse_lines::<Springs>(&input)
            .map(|springs| springs.possibilities())
            .sum::<u64>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        parse_lines::<Springs>(&input)
            .update(|springs| springs.expand())
            .map(|springs| springs.possibilities())
            .sum::<u64>()
            .to_string()
    }
}

struct Springs {
    conditions: Vec<Condition>,
    checks: Vec<usize>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Debug for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Condition::Operational => '.',
            Condition::Damaged => '#',
            Condition::Unknown => '?',
        };
        write!(f, "{c}")
    }
}

impl FromStr for Springs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (conditions_str, checks_str) = s.split_once(' ').ok_or_else(|| s.to_string())?;
        let conditions = conditions_str
            .chars()
            .map(|c| match c {
                '.' => Condition::Operational,
                '#' => Condition::Damaged,
                '?' => Condition::Unknown,
                _ => panic!("{s}"),
            })
            .collect();
        let checks = checks_str
            .split(',')
            .map(|ss| ss.parse().unwrap())
            .collect();
        Ok(Springs { conditions, checks })
    }
}

impl Debug for Springs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in &self.conditions {
            write!(f, "{c:?}")?;
        }
        let checks_str = self.checks.iter().join(",");
        write!(f, " {checks_str}")?;
        Ok(())
    }
}

impl Springs {
    fn expand(&mut self) {
        let conditions = self.conditions.clone();
        let checks = self.checks.clone();
        for _ in 0..4 {
            self.conditions.push(Condition::Unknown);
            self.conditions.extend(conditions.iter().copied());
            self.checks.extend(checks.iter().copied());
        }
    }

    fn possibilities(&self) -> u64 {
        let conds = &self.conditions;
        let checks = &self.checks;
        let mut table = vec![vec![0; checks.len() + 1]; conds.len() + 1];
        table[0][0] = 1;

        for check_i in 0..=checks.len() {
            let check = if check_i == 0 { 0 } else { checks[check_i - 1] };
            for cond_i in 1..=conds.len() {
                if cond_i < check {
                    continue;
                }

                let op = table[cond_i - 1][check_i];
                let is_blocked = conds[cond_i - check..cond_i].contains(&Condition::Operational);
                let da = if check == 0 || is_blocked {
                    0
                } else if cond_i == check {
                    table[0][check_i - 1]
                } else {
                    let target_cond = conds[cond_i - check - 1];
                    match target_cond {
                        Condition::Operational | Condition::Unknown => {
                            table[cond_i - check - 1][check_i - 1]
                        }
                        Condition::Damaged => 0,
                    }
                };
                table[cond_i][check_i] = match conds[cond_i - 1] {
                    Condition::Operational => op,
                    Condition::Damaged => da,
                    Condition::Unknown => op + da,
                }
            }
        }
        log::debug!("{table:?}");
        let count = table[conds.len()][checks.len()];
        log::info!("{self:?} -> {count}");
        count
    }
}
