use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::ops::Add;

pub struct Counter<T, C = u64> {
    counts: HashMap<T, C>,
}

impl<T, C> Counter<T, C> {
    pub fn new() -> Counter<T, C> {
        Counter {
            counts: HashMap::new(),
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<T, C> {
        self.counts.iter()
    }
}

impl<T, C> Default for Counter<T, C> {
    fn default() -> Self {
        Counter::new()
    }
}

impl<T: Hash + Eq, C: Add<Output = C> + Default + Copy + Eq> Counter<T, C> {
    pub fn count_n(&mut self, k: T, n: C) -> C {
        let entry = self.counts.entry(k).or_default();
        *entry = *entry + n;
        *entry
    }

    pub fn is_empty(&self) -> bool {
        self.counts.is_empty() || self.counts.values().all(|v| v == &C::default())
    }
}

impl<T: Hash + Eq> Counter<T, u64> {
    pub fn count(&mut self, k: T) -> u64 {
        self.count_n(k, 1)
    }
}

impl<T: Debug, C: Debug> Debug for Counter<T, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.counts)
    }
}

impl<'a, T, C> IntoIterator for &'a Counter<T, C> {
    type Item = (&'a T, &'a C);
    type IntoIter = std::collections::hash_map::Iter<'a, T, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, C> IntoIterator for Counter<T, C> {
    type Item = (T, C);
    type IntoIter = std::collections::hash_map::IntoIter<T, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.into_iter()
    }
}
