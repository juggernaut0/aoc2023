use crate::util::parse_lines_with;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        parse_lines_with(&input, parse_value_seq)
            .map(predict_value)
            .sum::<i32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        parse_lines_with(&input, parse_value_seq)
            .map(|mut it| {
                it.reverse();
                it
            })
            .map(predict_value)
            .sum::<i32>()
            .to_string()
    }
}

fn parse_value_seq(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|it| it.parse().unwrap())
        .collect()
}

fn predict_value(seq: Vec<i32>) -> i32 {
    if seq.iter().copied().all(|v| v == 0) {
        return 0;
    }
    let diffs = seq.windows(2).map(|w| w[1] - w[0]).collect();
    let diff = predict_value(diffs);
    seq[seq.len() - 1] + diff
}
