#![doc = include_str!("../README.md")]

use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

/// See [crate level documentation](crate).
pub use michie_macro::memoized;

pub trait MemoizationStore<K, R, Q = K>
where
    Q: ?Sized,
    K: Borrow<Q>
{
    fn insert(&mut self, key: K, value: R);
    fn get(&self, key: &Q) -> Option<&R>;
}

impl<K, R, Q> MemoizationStore<K, R, Q> for HashMap<K, R>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Hash + Eq + ?Sized,
{
    fn insert(&mut self, key: K, value: R) {
        HashMap::insert(self, key, value);
    }
    fn get(&self, key: &Q) -> Option<&R>
    {
        HashMap::get(self, key)
    }
}

impl<K, R, Q> MemoizationStore<K, R, Q> for BTreeMap<K, R>
where
    K: Ord + Borrow<Q>,
    Q: Ord + ?Sized
{
    fn insert(&mut self, key: K, value: R) {
        BTreeMap::insert(self, key, value);
    }
    fn get(&self, key: &Q) -> Option<&R> {
        BTreeMap::get(self, key)
    }
}
