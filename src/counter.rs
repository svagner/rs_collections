use std::collections::HashMap;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator};
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct Counter<T>
where
    T: Eq + Hash,
{
    data: HashMap<T, u64>,
}

impl<T> Counter<T>
where
    T: Eq + Hash,
{
    pub fn new<I: Iterator<Item = T>>(data: I) -> Self {
        let mut map: HashMap<T, u64> = HashMap::new();
        for i in data {
            let cnt = map.entry(i).or_insert(0);
            *cnt += 1;
        }
        Counter { data: map }
    }
}

impl<T> Deref for Counter<T>
where
    T: Eq + Hash,
{
    type Target = HashMap<T, u64>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> FromIterator<T> for Counter<T>
where
    T: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter.into_iter())
    }
}

#[cfg(test)]
mod test {
    use super::{Counter, Deref, HashMap};

    #[test]
    fn test_new_counter() {
        assert_eq!(
            *Counter::new("some small string".chars()).deref(),
            HashMap::<char, u64>::from_iter([
                ('s', 3),
                ('o', 1),
                ('m', 2),
                ('e', 1),
                (' ', 2),
                ('a', 1),
                ('l', 2),
                ('t', 1),
                ('r', 1),
                ('i', 1),
                ('n', 1),
                ('g', 1),
            ]),
        );
    }

    #[test]
    fn counter_from_iter() {
        let c: Counter<char> = "some small string".chars().collect();
        assert_eq!(
            *c,
            HashMap::<char, u64>::from_iter([
                ('s', 3),
                ('o', 1),
                ('m', 2),
                ('e', 1),
                (' ', 2),
                ('a', 1),
                ('l', 2),
                ('t', 1),
                ('r', 1),
                ('i', 1),
                ('n', 1),
                ('g', 1),
            ]),
        );
    }
}
