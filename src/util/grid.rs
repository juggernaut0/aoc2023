use crate::util::Point;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn build(width: i32, height: i32, elem: impl Fn(Point) -> T) -> Grid<T> {
        let width_size = width.try_into().unwrap();
        let height_size = height.try_into().unwrap();
        let mut data = Vec::with_capacity(height_size);
        for y in 0..height {
            let mut row = Vec::with_capacity(width_size);
            for x in 0..width {
                row.push(elem(Point(x, y)));
            }
            data.push(row);
        }
        Grid { data }
    }

    pub fn width(&self) -> i32 {
        self.data[0].len().try_into().unwrap()
    }

    pub fn height(&self) -> i32 {
        self.data.len().try_into().unwrap()
    }

    pub fn get(&self, p: Point) -> Option<&T> {
        let x: usize = p.0.try_into().ok()?;
        let y: usize = p.1.try_into().ok()?;
        self.data.get(y).and_then(|row| row.get(x))
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        let x: usize = p.0.try_into().ok()?;
        let y: usize = p.1.try_into().ok()?;
        self.data.get_mut(y).and_then(|row| row.get_mut(x))
    }

    pub fn set(&mut self, p: Point, t: T) -> Option<T> {
        let elem = self.get_mut(p)?;
        let old = std::mem::replace(elem, t);
        Some(old)
    }

    pub fn points<'a>(&self) -> impl Iterator<Item = Point> + 'a {
        let height: i32 = self.data.len().try_into().expect("my height is too big");
        let width: i32 = self.data[0].len().try_into().expect("my width is too big");
        (0..height).flat_map(move |y| (0..width).map(move |x| Point(x, y)))
    }

    pub fn points_with_item(&self) -> impl Iterator<Item = (Point, &T)> {
        self.points().map(|p| (p, &self[p]))
    }
}

impl<T: From<char>> FromStr for Grid<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| line.chars().map(char::into).collect())
            .collect();
        Ok(Grid { data })
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for t in row {
                write!(f, "{t:?}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("Cannot index grid: {index}"))
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.get_mut(index)
            .unwrap_or_else(|| panic!("Cannot index grid: {index}"))
    }
}
