#![doc = include_str!("../README.md")]

use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

/// See [crate level documentation](crate).
pub use michie_macro::memoized;

pub trait MemoizationStore<K, Q, R>
where
    Q: ?Sized,
{
    fn insert(&mut self, key: K, value: R);
    fn get(&self, key: &Q) -> Option<&R>
    where
        K: Borrow<Q>;
}

impl<K, Q, R> MemoizationStore<K, Q, R> for HashMap<K, R>
where
    K: Eq + Hash,
    Q: Hash + Eq,
    Q: ?Sized,
{
    fn insert(&mut self, key: K, value: R) {
        HashMap::insert(self, key, value);
    }
    fn get(&self, key: &Q) -> Option<&R>
    where
        K: Borrow<Q>,
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
