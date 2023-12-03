use crate::util::Point;
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (numbers, symbols) = find_things(&input);
        let symbols: HashSet<_> = symbols.into_keys().collect();

        numbers
            .into_iter()
            .filter(|it| it.neighbors().iter().any(|np| symbols.contains(np)))
            .map(|it| it.value_as_i32())
            .sum::<i32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let (numbers, symbols) = find_things(&input);
        symbols
            .into_iter()
            .filter(|(_, c)| *c == '*')
            .map(|(pos, _)| {
                let adj_points = pos.adj_diag();
                let adj: Vec<_> = numbers
                    .iter()
                    .filter(|n| {
                        n.occupied_points()
                            .any(|np| adj_points.iter().any(|ap| ap == &np))
                    })
                    .collect();
                if adj.len() != 2 {
                    0
                } else {
                    let a = adj[0];
                    let b = adj[1];
                    a.value_as_i32() * b.value_as_i32()
                }
            })
            .sum::<i32>()
            .to_string()
    }
}

#[derive(Debug)]
struct Number {
    pos: Point, // pos of leftmost digit
    value: String,
}

impl Number {
    fn value_as_i32(&self) -> i32 {
        self.value.parse().unwrap()
    }

    fn neighbors(&self) -> Vec<Point> {
        let l = self.value.len() as i32;
        let mut res = vec![
            self.pos + Point(-1, -1),
            self.pos + Point(-1, 0),
            self.pos + Point(-1, 1),
        ];
        for dx in 0..l {
            res.push(self.pos + Point(dx, -1));
            res.push(self.pos + Point(dx, 1));
        }
        res.push(self.pos + Point(l, -1));
        res.push(self.pos + Point(l, 0));
        res.push(self.pos + Point(l, 1));
        res
    }

    fn occupied_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.value.len()).map(|dx| self.pos + Point(dx as i32, 0))
    }
}

fn find_things(input: &str) -> (Vec<Number>, HashMap<Point, char>) {
    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        let mut number: Option<Number> = None;
        for (x, c) in line.char_indices() {
            let pos = Point(x as i32, y as i32);
            if c.is_ascii_digit() {
                if let Some(n) = number.as_mut() {
                    n.value.push(c);
                } else {
                    let mut n = Number {
                        pos,
                        value: String::new(),
                    };
                    n.value.push(c);
                    number = Some(n);
                }
            } else {
                if let Some(n) = number {
                    numbers.push(n);
                    number = None;
                }
                if c != '.' {
                    symbols.insert(pos, c);
                } else {
                    continue;
                }
            }
        }
        if let Some(n) = number {
            numbers.push(n);
        }
    }
    (numbers, symbols)
}

#[cfg(test)]
mod test {
    use crate::day3::Solution;

    #[test]
    fn ex1() {
        use crate::Solution;
        let inp = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .to_string();
        let res = Solution.solve_1(inp);
        assert_eq!(res, "4361");
    }
}
