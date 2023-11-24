use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn zero() -> Point {
        Point(0, 0)
    }

    fn l1dist(self, other: Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    fn adj(self) -> [Point; 4] {
        [
            Point(self.0 + 1, self.1),
            Point(self.0, self.1 + 1),
            Point(self.0 - 1, self.1),
            Point(self.0, self.1 - 1),
        ]
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x, y) = (self.0, self.1);
        write!(f, "({x}, {y})")
    }
}
