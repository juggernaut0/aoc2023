pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        parse_input(&input)
            .into_iter()
            .map(|race| {
                let h = hold_time(race);
                race.0 - 2 * h + 1
            })
            .product::<i32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let race = parse_input_2(&input);
        let h = hold_time_2(race);
        let ways = race.0 - 2 * h + 1;
        ways.to_string()
    }
}

fn parse_input(s: &str) -> Vec<(i32, i32)> {
    let mut lines = s.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|it| it.parse().unwrap());
    let dists = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|it| it.parse().unwrap());
    times.zip(dists).collect()
}

fn parse_input_2(s: &str) -> (i128, i128) {
    let mut lines = s.lines();
    let time = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    let dist = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    (time, dist)
}

fn hold_time((t, d): (i32, i32)) -> i32 {
    for h in 1..t {
        let hd = h * (t - h);
        if hd > d {
            return h;
        }
    }
    unreachable!()
}
fn hold_time_2((t, d): (i128, i128)) -> i128 {
    for h in 1..t {
        let hd = h * (t - h);
        if hd > d {
            return h;
        }
    }
    unreachable!()
}
