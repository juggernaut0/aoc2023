use std::str::FromStr;

struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T: FromStr> FromStr for Grid<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
