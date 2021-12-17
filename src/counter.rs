use crate::ops::Incrementable;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator};
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct Counter<A, B>
where
    A: Eq + Hash,
    B: Incrementable + Default,
{
    data: HashMap<A, B>,
}

impl<A, B> Counter<A, B>
where
    A: Eq + Hash,
    B: Incrementable + Default,
{
    pub fn new<I: Iterator<Item = A>>(data: I) -> Self {
        let mut map: HashMap<A, B> = HashMap::new();
        for i in data {
            let cnt = map.entry(i).or_insert(B::default());
            cnt.increment();
        }
        Counter { data: map }
    }
}

impl<A, B> Deref for Counter<A, B>
where
    A: Eq + Hash,
    B: Incrementable + Default,
{
    type Target = HashMap<A, B>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<A, B> FromIterator<A> for Counter<A, B>
where
    A: Eq + Hash,
    B: Incrementable + Default,
{
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        Self::new(iter.into_iter())
    }
}

#[cfg(test)]
mod test {
    use super::{Counter, HashMap};

    #[test]
    fn test_new_counter() {
        assert_eq!(
            *Counter::new("some small string".chars()),
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
        let c: Counter<char, u64> = "some small string".chars().collect();
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
