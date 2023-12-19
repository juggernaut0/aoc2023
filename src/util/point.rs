use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point<T = i32>(pub T, pub T);

impl Point {
    pub fn of<X: TryInto<i32>, Y: TryInto<i32>>(x: X, y: Y) -> Point
    where
        X::Error: Debug,
        Y::Error: Debug,
    {
        Point(x.try_into().unwrap(), y.try_into().unwrap())
    }

    pub fn l1dist(self, other: Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    pub fn adj(self) -> [Point; 4] {
        [
            Point(self.0 + 1, self.1),
            Point(self.0, self.1 + 1),
            Point(self.0 - 1, self.1),
            Point(self.0, self.1 - 1),
        ]
    }

    pub fn adj_diag(self) -> [Point; 8] {
        [
            Point(self.0 - 1, self.1 - 1),
            Point(self.0 - 1, self.1),
            Point(self.0 - 1, self.1 + 1),
            Point(self.0, self.1 - 1),
            Point(self.0, self.1 + 1),
            Point(self.0 + 1, self.1 - 1),
            Point(self.0 + 1, self.1),
            Point(self.0 + 1, self.1 + 1),
        ]
    }
}

impl<T> Point<T> {
    // pseudo Into impl
    pub fn into<U: From<T>>(self) -> Point<U> {
        Point(self.0.into(), self.1.into())
    }
}

impl<T: Default> Point<T> {
    pub fn zero() -> Self {
        Point(T::default(), T::default())
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Point<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T: Display> Debug for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
