#![doc = include_str!("../README.md")]

use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash, borrow::Borrow,
};

/// See [crate level documentation](crate).
pub use michie_macro::memoized;

pub trait MemoizationStore<K, R> {
    fn insert(&mut self, key: K, value: R);
    fn get<Q>(&self, key: &Q) -> Option<&R>
        where K: Borrow<Q>,
              Q: ?Sized;
}

impl<K, R> MemoizationStore<K, R> for HashMap<K, R>
where
    K: Eq + Hash,
{
    fn insert(&mut self, key: K, value: R) {
        HashMap::insert(self, key, value);
    }
    fn get<Q>(&self, key: &Q) -> Option<&R>
        where K: Borrow<Q>,
              Q: ?Sized,
              Q: Hash + Eq, 
    {
        HashMap::get(self, key)
    }
}

impl<K, R> MemoizationStore<K, R> for BTreeMap<K, R>
where
    K: Ord,
{
    fn insert(&mut self, key: K, value: R) {
        BTreeMap::insert(self, key, value);
    }
    fn get(&self, key: &K) -> Option<&R> {
        BTreeMap::get(self, key)
    }
}
