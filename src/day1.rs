#![allow(unused_variables)]

use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::util::{parse_lines, search, Searchable};

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let answer: i32 = parse_lines(&input)
            .map(|mut it: Blueprint| {
                it.max_time = 24;
                let id = it.id;
                let (_, v) = search(it).unwrap();
                log::info!("{id}: {v}");
                id * v
            })
            .sum();
        answer.to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let answer: i32 = parse_lines(&input)
            .take(3)
            .map(|mut it: Blueprint| {
                it.max_time = 32;
                let id = it.id;
                let (_, v) = search(it).unwrap();
                log::info!("{id}: {v}");
                v
            })
            .product();
        answer.to_string()
    }
}

struct Blueprint {
    id: i32,
    ore_ore: i32,
    clay_ore: i32,
    obs_ore: i32,
    obs_clay: i32,
    geo_ore: i32,
    geo_obs: i32,
    max_ore_cost: i32,
    max_time: i32,
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new("Blueprint (\\d+): Each ore robot costs (\\d+) ore. Each clay robot costs (\\d+) ore. Each obsidian robot costs (\\d+) ore and (\\d+) clay. Each geode robot costs (\\d+) ore and (\\d+) obsidian.").unwrap()
        });

        let m = RE.captures(s).ok_or_else(|| s.to_string())?;
        let clay_ore = m[3].parse().unwrap();
        let obs_ore = m[4].parse().unwrap();
        let geo_ore = m[6].parse().unwrap();
        let b = Blueprint {
            id: m[1].parse().unwrap(),
            ore_ore: m[2].parse().unwrap(),
            clay_ore,
            obs_ore,
            obs_clay: m[5].parse().unwrap(),
            geo_ore,
            geo_obs: m[7].parse().unwrap(),
            max_ore_cost: [clay_ore, obs_ore, geo_ore].into_iter().max().unwrap(),
            max_time: -1,
        };
        Ok(b)
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    ore: i32,
    clay: i32,
    obs: i32,
    geo: i32,
    ore_robots: i32,
    clay_robots: i32,
    obs_robots: i32,
    geo_robots: i32,
    time: i32,
}

struct IncomeDiffs {
    new_ore: i32,
    new_clay: i32,
    new_obs: i32,
    new_ore_robots: i32,
    new_clay_robots: i32,
    new_obs_robots: i32,
    new_geo_robots: i32,
}

impl IncomeDiffs {
    fn zero() -> IncomeDiffs {
        IncomeDiffs {
            new_ore: 0,
            new_clay: 0,
            new_obs: 0,
            new_ore_robots: 0,
            new_clay_robots: 0,
            new_obs_robots: 0,
            new_geo_robots: 0,
        }
    }
}

impl State {
    fn initial() -> State {
        State {
            ore: 0,
            clay: 0,
            obs: 0,
            geo: 0,
            ore_robots: 1,
            clay_robots: 0,
            obs_robots: 0,
            geo_robots: 0,
            time: 0,
        }
    }

    fn income(&self, income: IncomeDiffs) -> State {
        State {
            ore: self.ore + self.ore_robots + income.new_ore,
            clay: self.clay + self.clay_robots + income.new_clay,
            obs: self.obs + self.obs_robots + income.new_obs,
            geo: self.geo + self.geo_robots,
            ore_robots: self.ore_robots + income.new_ore_robots,
            clay_robots: self.clay_robots + income.new_clay_robots,
            obs_robots: self.obs_robots + income.new_obs_robots,
            geo_robots: self.geo_robots + income.new_geo_robots,
            time: self.time + 1,
        }
    }
}

impl Searchable for Blueprint {
    type State = State;
    type Key = State;
    type Value = i32;

    fn initial_state(&self) -> Self::State {
        State::initial()
    }

    fn successors(&self, state: Self::State) -> Vec<Self::State> {
        let mut res = Vec::new();

        res.push(state.income(IncomeDiffs::zero()));
        if state.ore >= self.ore_ore && state.obs_robots < self.max_ore_cost {
            res.push(state.income(IncomeDiffs {
                new_ore: -self.ore_ore,
                new_ore_robots: 1,
                ..IncomeDiffs::zero()
            }));
        }
        if state.ore >= self.clay_ore && state.clay_robots < self.obs_clay {
            res.push(state.income(IncomeDiffs {
                new_ore: -self.clay_ore,
                new_clay_robots: 1,
                ..IncomeDiffs::zero()
            }));
        }
        if state.ore >= self.obs_ore && state.clay >= self.obs_clay && state.obs_robots < self.geo_obs {
            res.push(state.income(IncomeDiffs {
                new_ore: -self.obs_ore,
                new_clay: -self.obs_clay,
                new_obs_robots: 1,
                ..IncomeDiffs::zero()
            }));
        }
        if state.ore >= self.geo_ore && state.obs >= self.geo_obs {
            res.push(state.income(IncomeDiffs {
                new_ore: -self.geo_ore,
                new_obs: -self.geo_obs,
                new_geo_robots: 1,
                ..IncomeDiffs::zero()
            }));
        }
        res
    }

    fn key(&self, state: &Self::State) -> State {
        state.clone()
    }

    fn value(&self, state: &Self::State) -> i32 {
        state.geo
    }

    fn value_estimate(&self, state: &Self::State) -> i32 {
        let mut est_clay = state.clay;
        let mut est_clay_robots = state.clay_robots;
        let mut est_obs = state.obs;
        let mut est_obs_robots = state.obs_robots;
        let mut est_geo = state.geo;
        let mut est_geo_robots = state.geo_robots;

        for t in state.time..self.max_time {
            est_clay += est_clay_robots;
            est_clay_robots += 1;

            est_obs += est_obs_robots;
            if est_clay >= self.obs_clay {
                est_clay -= self.obs_clay;
                est_obs_robots += 1;
            }

            est_geo += est_geo_robots;
            if est_obs >= self.geo_obs {
                est_obs -= self.geo_obs;
                est_geo_robots += 1;
            }
        }

        est_geo
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        state.time == self.max_time
    }
}