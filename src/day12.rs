use crate::util::{parse_lines, split_once};
use itertools::Itertools;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        parse_lines(&input)
            .map(|springs: Springs| {
                let count = pi(&springs.conditions, &springs.checks);
                log::info!("{springs:?} -> {count}");
                count
            })
            .sum::<u64>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        parse_lines::<Springs>(&input)
            .update(|springs| springs.expand())
            .map(|springs: Springs| {
                let count = pi(&springs.conditions, &springs.checks);
                log::info!("{springs:?} -> {count}");
                count
            })
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
        let (conditions_str, checks_str) = split_once(s, " ").ok_or_else(|| s.to_string())?;
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

fn pi(conds: &[Condition], checks: &[usize]) -> u64 {
    let mut table = vec![vec![0; checks.len() + 1]; conds.len() + 1];
    // component of each table entry that comes from the possibility of condition == damaged
    let mut da_comp = vec![vec![0; checks.len() + 1]; conds.len() + 1];

    for check_i in 0..=checks.len() {
        if check_i == 0 {
            for cond_i in 0..=conds.len() {
                table[cond_i][0] = if cond_i == 0 {
                    1
                } else if conds[cond_i - 1] == Condition::Damaged {
                    0
                } else {
                    table[cond_i - 1][0]
                };
            }
        } else {
            let check = checks[check_i - 1];
            for cond_i in 1..=conds.len() {
                if cond_i >= check {
                    let op = table[cond_i - 1][check_i];
                    let da = if conds[cond_i - check..cond_i].contains(&Condition::Operational) {
                        0
                    } else {
                        let x = table[cond_i - check][check_i - 1];
                        let da_mod = da_comp[cond_i - check][check_i - 1];
                        x - da_mod
                    };
                    match conds[cond_i - 1] {
                        Condition::Operational => {
                            table[cond_i][check_i] = op;
                        }
                        Condition::Damaged => {
                            table[cond_i][check_i] = da;
                            da_comp[cond_i][check_i] = da;
                        }
                        Condition::Unknown => {
                            table[cond_i][check_i] = op + da;
                            da_comp[cond_i][check_i] = da;
                        }
                    }
                }
            }
        }
    }
    table[conds.len()][checks.len()]
}
