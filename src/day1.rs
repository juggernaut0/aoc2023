use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        process(&input, &Regex::new("\\d").unwrap())
    }

    fn solve_2(&self, input: String) -> String {
        process(
            &input,
            &Regex::new("\\d|one|two|three|four|five|six|seven|eight|nine").unwrap(),
        )
    }
}

static NUMS: Lazy<HashMap<String, i32>> = Lazy::new(|| {
    let mut nums = HashMap::new();
    nums.insert("one".to_string(), 1);
    nums.insert("two".to_string(), 2);
    nums.insert("three".to_string(), 3);
    nums.insert("four".to_string(), 4);
    nums.insert("five".to_string(), 5);
    nums.insert("six".to_string(), 6);
    nums.insert("seven".to_string(), 7);
    nums.insert("eight".to_string(), 8);
    nums.insert("nine".to_string(), 9);
    nums
});

fn process(input: &str, re: &Regex) -> String {
    input
        .lines()
        .map(|line| process_line(line, re))
        .sum::<i32>()
        .to_string()
}

fn process_line(line: &str, re: &Regex) -> i32 {
    let a = find_first_digit(line, re);
    let b = find_last_digit(line, re);
    a * 10 + b
}

fn find_first_digit(s: &str, re: &Regex) -> i32 {
    re.find(s)
        .map(|m| {
            let s = m.as_str();
            NUMS.get(s).copied().unwrap_or_else(|| s.parse().unwrap())
        })
        .unwrap()
}

fn find_last_digit(s: &str, re: &Regex) -> i32 {
    for i in (0..=s.len()).rev() {
        if let Some(m) = re.find(&s[i..]) {
            let s = m.as_str();
            return NUMS.get(s).copied().unwrap_or_else(|| s.parse().unwrap());
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Solution;

    #[test]
    fn ex2() {
        let inp = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();
        let res = Solution.solve_2(inp);
        assert_eq!(res, "281");
    }
}
