use crate::util::{search, Dir, Grid, Point, Searchable};

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let city = City {
            map: input.parse().unwrap(),
            min_straight: 0,
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
    fn next(&self, dir: Dir, city: &City) -> State {
        let new_pos = self.key.pos + dir.diff();
        let straights = if dir == self.key.dir {
            self.key.straights + 1
        } else {
            1
        };
        State {
            key: StateKey {
                pos: new_pos,
                dir,
                straights,
            },
            total_cost: self.total_cost + city.map.get(new_pos).map_or(1000, |it| it.0 as i32),
        }
    }

    fn next_left(&self, city: &City) -> State {
        self.next(self.key.dir.turn_left(), city)
    }

    fn next_right(&self, city: &City) -> State {
        self.next(self.key.dir.turn_right(), city)
    }

    fn next_straight(&self, city: &City) -> State {
        self.next(self.key.dir, city)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct StateKey {
    pos: Point,
    dir: Dir,
    straights: u8,
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
                straights: 0,
            },
            total_cost: 0,
        }
    }

    fn successors(&self, state: Self::State) -> Vec<Self::State> {
        let mut res = Vec::new();

        if state.total_cost == 0 {
            // special condition for start: it can go right or straight
            res.push(state.next_right(self));
            res.push(state.next_straight(self));
            return res;
        }

        if state.key.straights >= self.min_straight {
            res.push(state.next_left(self));
            res.push(state.next_right(self));
        }

        if state.key.straights < self.max_straight {
            res.push(state.next_straight(self));
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
        state.key.pos == self.goal_point() && state.key.straights > self.min_straight
    }

    fn break_on_goal() -> bool {
        true
    }
}
