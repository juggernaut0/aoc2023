use crate::util::parse_lines;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        parse_lines(&input)
            .map(|card: Card| card.points())
            .sum::<i32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let cards: Vec<Card> = parse_lines(&input).collect();
        let mut counts = vec![1; cards.len()];
        for card in cards {
            let id = card.id;
            let count = counts[id - 1];
            let matches = card.matches();
            for i in 0..matches {
                counts[id + i] += count;
            }
        }
        counts.into_iter().sum::<usize>().to_string()
    }
}

struct Card {
    id: usize,
    winning: HashSet<i32>,
    picked: Vec<i32>,
}

impl Card {
    fn matches(&self) -> usize {
        self.picked
            .iter()
            .filter(|it| self.winning.contains(it))
            .count()
    }

    fn points(&self) -> i32 {
        let exp = self.matches() as u32;
        if exp == 0 {
            0
        } else {
            2i32.pow(exp - 1)
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(':').ok_or_else(|| s.to_string())?;

        let id = head
            .chars()
            .skip(5)
            .collect::<String>()
            .trim()
            .parse()
            .map_err(|_| s.to_string())?;

        let (winning_str, picked_str) = tail.split_once('|').ok_or_else(|| s.to_string())?;
        let winning = winning_str
            .split_ascii_whitespace()
            .map(|it| it.parse().unwrap())
            .collect();
        let picked = picked_str
            .split_ascii_whitespace()
            .map(|it| it.parse().unwrap())
            .collect();
        Ok(Card {
            id,
            winning,
            picked,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::day4::Solution;

    #[test]
    fn ex1() {
        use crate::Solution;
        let inp = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();

        let res = Solution.solve_1(inp);
        assert_eq!("13", res);
    }
}
