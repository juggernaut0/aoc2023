use crate::util::{parse_lines, split_once};
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        parse_lines(&input)
            .filter(|game: &Game| {
                game.picks
                    .iter()
                    .all(|pick| pick.red <= 12 && pick.green <= 13 && pick.blue <= 14)
            })
            .map(|game| game.id)
            .sum::<i32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        parse_lines(&input)
            .map(|game: Game| {
                game.picks
                    .into_iter()
                    .fold(Pick::default(), |a, b| a.min_colors(&b))
                    .power()
            })
            .sum::<i64>()
            .to_string()
    }
}

struct Game {
    id: i32,
    picks: Vec<Pick>,
}

#[derive(Default)]
struct Pick {
    red: i32,
    green: i32,
    blue: i32,
}

impl Pick {
    fn min_colors(&self, other: &Pick) -> Pick {
        Pick {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> i64 {
        self.red as i64 * self.green as i64 * self.blue as i64
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = split_once(s, ":").ok_or_else(|| s.to_string())?;

        let id: i32 = head
            .chars()
            .skip(5)
            .collect::<String>()
            .parse()
            .map_err(|_| s.to_string())?;

        let mut picks = Vec::new();
        for pick_str in tail.split(';') {
            let mut pick = Pick::default();
            for color_n_str in pick_str.split(',') {
                let (n_str, color) =
                    split_once(color_n_str.trim(), " ").ok_or_else(|| color_n_str.to_string())?;
                let n: i32 = n_str.parse().map_err(|_| s.to_string())?;

                match color {
                    "red" => pick.red = n,
                    "green" => pick.green = n,
                    "blue" => pick.blue = n,
                    _ => return Err(format!("unknown color {color}")),
                }
            }
            picks.push(pick);
        }

        Ok(Game { id, picks })
    }
}
