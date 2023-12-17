use crate::util::{search, Dir, Grid, Point, Searchable};

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let city = City {
            map: input.parse().unwrap(),
            min_straight: 1,
            max_straight: 3,
        };
        let (best, _) = search(&city).unwrap();
        best.total_cost.to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let city = City {
            map: input.parse().unwrap(),
            min_straight: 4,
            max_straight: 10,
        };
        let (best, _) = search(&city).unwrap();
        best.total_cost.to_string()
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Cost(u8);

impl From<char> for Cost {
    fn from(value: char) -> Self {
        Cost(value.to_digit(10).unwrap() as u8)
    }
}

#[derive(Debug)]
struct State {
    key: StateKey,
    total_cost: i32,
}

impl State {
    fn turn_left(&self) -> State {
        State {
            key: StateKey {
                pos: self.key.pos,
                dir: self.key.dir.turn_left(),
            },
            total_cost: self.total_cost,
        }
    }

    fn turn_right(&self) -> State {
        State {
            key: StateKey {
                pos: self.key.pos,
                dir: self.key.dir.turn_right(),
            },
            total_cost: self.total_cost,
        }
    }

    fn straight(&self, city: &City, n: u8) -> State {
        let pos = self.key.pos + self.key.dir.diff();
        let new_state = State {
            key: StateKey {
                pos,
                dir: self.key.dir,
            },
            total_cost: self.total_cost + city.map.get(pos).map_or(10000, |it| it.0 as i32),
        };
        if n == 1 {
            new_state
        } else {
            new_state.straight(city, n - 1)
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct StateKey {
    pos: Point,
    dir: Dir,
}

struct City {
    map: Grid<Cost>,
    min_straight: u8,
    max_straight: u8,
}

impl City {
    fn goal_point(&self) -> Point {
        Point(self.map.width() as i32 - 1, self.map.height() as i32 - 1)
    }
}

impl Searchable for City {
    type State = State;
    type Key = StateKey;
    type Value = i32;

    fn initial_state(&self) -> Self::State {
        State {
            key: StateKey {
                pos: Point(0, 0),
                dir: Dir::E,
            },
            total_cost: 0,
        }
    }

    fn successors(&self, state: Self::State) -> Vec<Self::State> {
        let mut res = Vec::new();

        if state.total_cost == 0 {
            // special condition for start: it can go east or south
            res.push(state.straight(self, self.min_straight));
            res.push(state.turn_right().straight(self, self.min_straight));
            return res;
        }

        let mut current = state;
        for _ in self.min_straight..=self.max_straight {
            if self.is_goal(&current) {
                res.push(current);
                break;
            }
            res.push(current.turn_left().straight(self, self.min_straight));
            res.push(current.turn_right().straight(self, self.min_straight));
            current = current.straight(self, 1);
        }

        res
    }

    fn key(&self, state: &Self::State) -> Self::Key {
        state.key.clone()
    }

    fn value(&self, state: &Self::State) -> Self::Value {
        -state.total_cost
    }

    fn value_estimate(&self, state: &Self::State) -> Self::Value {
        -state.total_cost - (state.key.pos.l1dist(self.goal_point()))
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        state.key.pos == self.goal_point()
    }

    fn break_on_goal() -> bool {
        true
    }
}
