use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Mutex;

use rayon::iter::ParallelIterator;

pub trait IteratorExt: Iterator
where
    Self::Item: Eq + Hash + Clone,
{
    fn duplicates(&mut self) -> impl Iterator<Item = Self::Item>;
}

impl<I: Iterator> IteratorExt for I
where
    I::Item: Eq + Hash + Clone,
{
    fn duplicates(&mut self) -> impl Iterator<Item = Self::Item> {
        let mut used = HashSet::new();
        self.filter(move |elt| !used.insert(elt.clone()))
    }
}

pub trait ParallelIteratorExt: ParallelIterator
where
    Self::Item: Eq + Hash + Clone + Sync,
{
    fn duplicates(self) -> impl ParallelIterator<Item = Self::Item>;
}

impl<PI: ParallelIterator> ParallelIteratorExt for PI
where
    PI::Item: Eq + Hash + Clone + Sync,
{
    fn duplicates(self) -> impl ParallelIterator<Item = Self::Item> {
        let used = Mutex::new(HashSet::new());
        self.filter(move |elt| !used.lock().unwrap().insert(elt.clone()))
    }
}
