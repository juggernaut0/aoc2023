use crate::util::Point;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub const fn values() -> [Dir; 4] {
        [Dir::N, Dir::E, Dir::S, Dir::W]
    }

    pub fn turn_left(self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }

    pub fn turn_right(self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    pub fn opposite(self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }

    pub fn diff(self) -> Point {
        match self {
            Dir::N => Point(0, -1),
            Dir::E => Point(1, 0),
            Dir::S => Point(0, 1),
            Dir::W => Point(-1, 0),
        }
    }
}
