use crate::util::Point;
use std::fmt::{Debug, Formatter};
use std::ops::Index;
use std::str::FromStr;

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn build(width: usize, height: usize, elem: impl Fn(Point) -> T) -> Grid<T> {
        let mut data = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                row.push(elem(Point(x as i32, y as i32)));
            }
            data.push(row);
        }
        Grid { data }
    }

    pub fn get(&self, p: Point) -> Option<&T> {
        self.data
            .get(p.1 as usize)
            .and_then(|row| row.get(p.0 as usize))
    }

    pub fn set(&mut self, p: Point, t: T) -> Option<T> {
        let Point(x, y) = p;
        if let Some(row) = self.data.get_mut(y as usize) {
            if let Some(old) = row.get_mut(x as usize) {
                let old = std::mem::replace(old, t);
                Some(old)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn points<'a>(&self) -> impl Iterator<Item = Point> + 'a {
        let height = self.data.len();
        let width = self.data[0].len();
        (0..height).flat_map(move |y| (0..width).map(move |x| Point(x as i32, y as i32)))
    }

    pub fn points_with_item(&self) -> impl Iterator<Item = (Point, &T)> {
        self.data.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, t)| (Point(x as i32, y as i32), t))
        })
    }
}

impl<T: From<char>> FromStr for Grid<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
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
